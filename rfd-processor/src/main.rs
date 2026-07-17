// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use clap::Parser;
use config::{Config, ConfigError, Environment, File};
use processor::{processor, JobError};
use serde::{Deserialize, Serialize};
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};
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
    #[serde(default)]
    pub log_format: LogFormat,
    pub processor_enabled: bool,
    pub processor_batch_size: i64,
    pub processor_interval: u64,
    pub processor_capacity: u64,
    pub processor_update_mode: RfdUpdateMode,
    pub scanner_enabled: bool,
    pub scanner_interval: u64,
    pub database_url: String,
    pub actions: Vec<String>,
    pub auth: AuthConfig,
    pub source: GitHubSourceRepo,
    #[serde(default)]
    pub static_storage: Vec<StaticStorageConfig>,
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
    pub default_branch: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct StaticStorageConfig {
    pub bucket: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PdfStorageConfig {
    pub drive: Option<String>,
    pub folder: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SearchConfig {
    pub host: String,
    pub key: String,
    pub index: String,
}

const DEFAULT_CONFIG_PATHS: &[&str] = &[
    "/etc/rfd-processor/config.toml",
    "rfd-processor/config.toml",
];

impl AppConfig {
    pub fn new(config_sources: Option<Vec<String>>) -> Result<Self, ConfigError> {
        let mut config = Config::builder();

        for path in Self::candidate_paths(&config_sources) {
            config = config.add_source(File::with_name(&path).required(false));
        }

        config
            .add_source(Environment::default())
            .build()?
            .try_deserialize()
    }

    /// The configuration file paths that will be consulted, in priority order (later entries
    /// override earlier ones). An explicit path replaces the default search locations entirely,
    /// rather than layering on top of them.
    pub fn candidate_paths(config_sources: &Option<Vec<String>>) -> Vec<String> {
        match config_sources {
            Some(sources) if !sources.is_empty() => sources.clone(),
            _ => DEFAULT_CONFIG_PATHS.iter().map(|s| s.to_string()).collect(),
        }
    }
}

const AFTER_HELP: &str = "\
Examples:
  rfd-processor start    [--config PATH]
  rfd-processor validate [--config PATH]
  rfd-processor version
  rfd-processor pdf       <directory> -o <output.pdf>

If --config is omitted, configuration is read from ./rfd-processor/config.toml or /etc/rfd-processor/config.toml.";

/// RFD processor worker
#[derive(Parser)]
#[command(disable_help_subcommand = true, after_help = AFTER_HELP)]
struct Args {
    #[command(subcommand)]
    command: ServerCommand,
}

#[derive(Parser)]
enum ServerCommand {
    /// Start the processor
    Start {
        /// Path to the configuration file [default: ./rfd-processor/config.toml or /etc/rfd-processor/config.toml]
        #[arg(short, long)]
        config: Option<String>,
    },
    /// Validate a configuration file
    Validate {
        /// Path to the configuration file [default: ./rfd-processor/config.toml or /etc/rfd-processor/config.toml]
        #[arg(short, long)]
        config: Option<String>,
    },
    /// Print the version
    Version,
    /// Render RFD content in a directory to a PDF file
    Pdf {
        /// Directory containing the RFD content to render
        directory: String,
        /// Path to write the rendered PDF to
        #[arg(short = 'o', long)]
        output: String,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    match args.command {
        ServerCommand::Version => {
            println!(
                "{} ({}, {})",
                env!("CARGO_PKG_VERSION"),
                env!("RFD_PROCESSOR_GIT_HASH"),
                env!("RFD_PROCESSOR_BUILD_TYPE"),
            );
            Ok(())
        }
        ServerCommand::Validate { config } => {
            let config_sources = config.map(|path| vec![path]);
            let candidate_paths = AppConfig::candidate_paths(&config_sources);
            AppConfig::new(config_sources).map_err(|err| {
                format!(
                    "Configuration is invalid ({}): {err}",
                    describe_config_paths(&candidate_paths)
                )
            })?;
            println!(
                "Configuration is valid ({})",
                describe_config_paths(&candidate_paths)
            );
            Ok(())
        }
        ServerCommand::Pdf { directory, output } => render_pdf_command(directory, output).await,
        ServerCommand::Start { config } => run_processor(config).await,
    }
}

fn describe_config_paths(paths: &[String]) -> String {
    paths
        .iter()
        .map(|path| {
            if Path::new(path).is_file() {
                path.clone()
            } else {
                format!("{path} (not found)")
            }
        })
        .collect::<Vec<_>>()
        .join(", ")
}

async fn run_processor(config_path: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
    rustls::crypto::ring::default_provider()
        .install_default()
        .expect("Failed to install rustls crypto provider");

    let config_sources = config_path.map(|path| vec![path]);
    let candidate_paths = AppConfig::candidate_paths(&config_sources);
    let config = AppConfig::new(config_sources)?;

    let (writer, _guard) = if let Some(log_directory) = &config.log_directory {
        let file_appender = tracing_appender::rolling::daily(log_directory, "rfd-processor.log");
        tracing_appender::non_blocking(file_appender)
    } else {
        NonBlocking::new(std::io::stdout())
    };

    let subscriber = tracing_subscriber::fmt()
        .with_file(false)
        .with_line_number(false)
        .with_env_filter(EnvFilter::from_default_env())
        .with_writer(writer);

    match config.log_format {
        LogFormat::Pretty => subscriber.pretty().init(),
        LogFormat::Json => subscriber.json().init(),
    }

    tracing::info!("Initialized logger");
    tracing::info!(
        config_file = %describe_config_paths(&candidate_paths),
        "Loaded configuration"
    );

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

async fn render_pdf_command(
    directory: String,
    output: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let pdf = content::render_pdf_from_dir(PathBuf::from(directory)).await?;
    std::fs::write(output, pdf.into_inner())?;

    Ok(())
}
