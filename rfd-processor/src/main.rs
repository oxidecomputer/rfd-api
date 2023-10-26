use config::{Config, ConfigError, Environment, File};
use processor::{processor, JobError};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::select;
use tracing_subscriber::EnvFilter;

use crate::{
    context::{Context, Database},
    scanner::{scanner, ScannerError},
};

mod content;
mod context;
mod features;
mod github;
mod pdf;
mod processor;
mod rfd;
mod scanner;
mod search;
mod updater;
mod util;

#[derive(Debug, Deserialize, Serialize)]
pub struct AppConfig {
    pub processor_batch_size: i64,
    pub processor_interval: u64,
    pub scanner_interval: u64,
    pub database_url: String,
    pub actions: Vec<String>,
    pub auth: AuthConfig,
    pub source: GitHubSourceRepo,
    #[serde(default)]
    pub static_storage: Vec<StaticStorageConfig>,
    #[serde(default)]
    pub pdf_storage: Vec<PdfStorageConfig>,
    #[serde(default)]
    pub search_storage: Vec<SearchConfig>,
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
        private_key: String,
    },
    User {
        token: String,
    },
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GitHubSourceRepo {
    pub owner: String,
    pub repo: String,
    pub path: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct StaticStorageConfig {
    pub bucket: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PdfStorageConfig {
    pub drive: String,
    pub folder: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SearchConfig {
    pub host: String,
    pub key: String,
    pub index: String,
}

impl AppConfig {
    pub fn new() -> Result<Self, ConfigError> {
        let config = Config::builder()
            .add_source(File::with_name("config.toml").required(false))
            .add_source(File::with_name("rfd-processor/config.toml").required(false))
            .add_source(Environment::default())
            .build()?;

        config.try_deserialize()
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _subscriber = tracing_subscriber::fmt()
        .with_file(false)
        .with_line_number(false)
        .with_env_filter(EnvFilter::from_default_env())
        .pretty()
        .init();

    let config = AppConfig::new()?;

    let ctx = Arc::new(Context::new(Database::new(&config.database_url).await, &config).await?);

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
    select! {
        value = processor_handle => {
            tracing::info!(?value, "Processor task exited")
        }
        value = scanner_handle => {
            tracing::info!(?value, "Scanner task exited")
        }
    };

    Ok(())
}
