// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use config::{Config, ConfigError, Environment, File};
use processor::{processor, JobError};
use rfd_secret::SecretString;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use thiserror::Error;
use tokio::select;
use tracing_appender::non_blocking::NonBlocking;
use tracing_subscriber::EnvFilter;
use updater::RfdUpdateMode;

use crate::{
    context::{Context, Database},
    scanner::{scanner, ScannerError},
};

mod content;
mod context;
// mod github;
mod pdf;
mod processor;
mod rfd;
mod scanner;
mod search;
mod updater;
mod util;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub log_directory: Option<String>,
    pub log_filter: Option<String>,
    #[serde(default)]
    pub log_format: LogFormat,
    pub processor_enabled: bool,
    pub processor_batch_size: i64,
    pub processor_interval: u64,
    pub processor_capacity: u64,
    pub processor_update_mode: RfdUpdateMode,
    pub scanner_enabled: bool,
    pub scanner_interval: u64,
    pub database: DatabaseConfig,
    pub actions: Vec<String>,
    pub auth: AuthConfig,
    pub source: GitHubSourceRepo,
    #[serde(default)]
    pub gcs_storage: Vec<GcsStorageConfig>,
    #[serde(default)]
    pub s3_storage: Vec<S3StorageConfig>,
    #[serde(default)]
    pub pdf_storage: Option<PdfStorageConfig>,
    #[serde(default)]
    pub search_storage: Vec<SearchConfig>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LogFormat {
    Pretty,
    // The default value is used to avoid breaking old configuration files.
    #[default]
    Json,
}

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Job task failed")]
    Job(#[source] JobError),
    #[error("Scanner task failed")]
    Scanner(#[source] ScannerError),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthConfig {
    pub github: GitHubAuthConfig,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum GitHubAuthConfig {
    Installation {
        app_id: i64,
        installation_id: i64,
        private_key: SecretString,
    },
    User {
        token: SecretString,
    },
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GitHubSourceRepo {
    pub owner: String,
    pub repo: String,
    pub path: String,
    pub default_branch: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GcsStorageConfig {
    pub bucket: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct S3StorageConfig {
    pub bucket: String,
    pub region: String,
    pub endpoint: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PdfStorageConfig {
    pub drive: Option<String>,
    pub folder: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SearchConfig {
    pub host: String,
    pub key: SecretString,
    pub index: String,
}

#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: SecretString,
    pub database: String,
}

impl DatabaseConfig {
    pub fn to_url(&self) -> Result<String, rfd_secret::SecretResolutionError> {
        let password = self.password.resolve()?;
        Ok(format!(
            "postgres://{}:{}@{}:{}/{}",
            self.user, password, self.host, self.port, self.database
        ))
    }
}

impl AppConfig {
    pub fn new(config_sources: Option<Vec<String>>) -> Result<Self, ConfigError> {
        let mut config = Config::builder()
            .add_source(File::with_name("config.toml").required(false))
            .add_source(File::with_name("rfd-processor/config.toml").required(false));

        for source in config_sources.unwrap_or_default() {
            config = config.add_source(File::with_name(&source).required(false));
        }

        config
            .add_source(Environment::default())
            .build()?
            .try_deserialize()
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args();
    let _ = args.next();
    let config_path = args.next();

    let config = AppConfig::new(config_path.map(|path| vec![path]))?;

    let (writer, _guard) = if let Some(log_directory) = &config.log_directory {
        let file_appender = tracing_appender::rolling::daily(log_directory, "rfd-processor.log");
        tracing_appender::non_blocking(file_appender)
    } else {
        NonBlocking::new(std::io::stdout())
    };

    let env_filter = match config.log_filter {
        Some(ref filter) => EnvFilter::new(filter),
        None => EnvFilter::from_default_env(),
    };

    let subscriber = tracing_subscriber::fmt()
        .with_file(false)
        .with_line_number(false)
        .with_env_filter(env_filter)
        .with_writer(writer);

    match config.log_format {
        LogFormat::Pretty => subscriber.pretty().init(),
        LogFormat::Json => subscriber.json().init(),
    }

    let database_url = config.database.to_url()?;
    let ctx = Arc::new(Context::new(Database::new(&database_url).await, &config).await?);

    let scanner_ctx = ctx.clone();
    let scanner_handle = tokio::spawn(async move {
        scanner(scanner_ctx).await?;
        Ok::<_, ScannerError>(())
    });

    let processor_ctx = ctx.clone();
    let processor_handle = tokio::spawn(async move {
        processor(processor_ctx).await?;
        Ok::<_, JobError>(())
    });

    // Tasks should run for the lifetime of the program. If any of them complete for any reason
    // then the entire application should exit
    let error = select! {
        value = processor_handle => {
            tracing::info!(?value, "Processor task exited");
            value?.map_err(AppError::Job)
        }
        value = scanner_handle => {
            tracing::info!(?value, "Scanner task exited");
            value?.map_err(AppError::Scanner)
        }
    };

    Ok(error?)
}
