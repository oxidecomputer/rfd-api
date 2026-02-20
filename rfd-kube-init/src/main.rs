// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

mod kube;
mod meilisearch;
mod meilisearch_client;
mod oauth_init;

use anyhow::Result;
use clap::{Parser, Subcommand};
use tracing_subscriber::{filter::LevelFilter, EnvFilter};

use crate::meilisearch::MeilisearchArgs;
use crate::oauth_init::OAuthInitArgs;

#[derive(Parser)]
#[command(name = "rfd-kube-init")]
#[command(about = "Kubernetes initialization tool for RFD services")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize Meilisearch secrets across target namespaces
    Meilisearch(MeilisearchArgs),
    /// Initialize OAuth client and distribute credentials to target namespaces
    OauthInit(OAuthInitArgs),
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        .with_writer(std::io::stdout)
        .init();

    tracing::info!("Starting rfd-kube-init");

    let cli = Cli::parse();

    match cli.command {
        Commands::Meilisearch(args) => {
            tracing::info!("Running meilisearch initialization");
            tracing::debug!("Initializing Kubernetes client");
            let kube_client = ::kube::Client::try_default().await?;
            tracing::debug!("Kubernetes client initialized");
            let result = meilisearch::init(&kube_client, &args).await;
            if result.is_ok() {
                tracing::info!("Meilisearch initialization completed successfully");
            }
            result
        }
        Commands::OauthInit(args) => {
            tracing::info!("Running OAuth client initialization");
            tracing::debug!("Initializing Kubernetes client");
            let kube_client = ::kube::Client::try_default().await?;
            tracing::debug!("Kubernetes client initialized");
            let result = oauth_init::init(&kube_client, &args).await;
            if result.is_ok() {
                tracing::info!("OAuth client initialization completed successfully");
            }
            result
        }
    }
}
