// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use clap::Parser;
use context::RfdContext;
use minijinja::Environment;
use rustls;
use server::{server, ServerConfig};
use std::{
    net::{SocketAddr, SocketAddrV4},
    path::Path,
    sync::Arc,
};
use strum::IntoEnumIterator;
use tap::TapFallible;
use tracing_appender::non_blocking::NonBlocking;
use tracing_subscriber::EnvFilter;
use v_api::{
    endpoints::login::oauth::{
        remote::{github::GitHubOAuthProvider, google::GoogleOAuthProvider},
        OAuthProviderName,
    },
    ApiContext, MagicLinkTarget, VContextBuilder,
};
use v_model::{schema_ext::MagicLinkMedium, storage::postgres::PostgresStore as VApiPostgresStore};

use crate::{
    config::{AppConfig, EmailService, ServerLogFormat},
    initial_data::InitialData,
    magic_link::{MagicLinkMessageBuilder, ResendMagicLink},
};

mod caller;
mod config;
mod context;
mod endpoints;
mod error;
mod initial_data;
mod magic_link;
mod permissions;
use permissions::RfdPermission;
mod search;
mod secrets;
mod server;
mod util;

const AFTER_HELP: &str = "\
Examples:
  rfd-api start    [--config PATH]
  rfd-api validate [--config PATH]
  rfd-api describe
  rfd-api version
  rfd-api migrate  [--database-url URL] [--v-only]

If --config is omitted, configuration is read from ./rfd-api/config.toml or /etc/rfd-api/config.toml.";

/// RFD API server
#[derive(Parser)]
#[command(disable_help_subcommand = true, after_help = AFTER_HELP)]
struct Args {
    #[command(subcommand)]
    command: ServerCommand,
}

#[derive(Parser)]
enum ServerCommand {
    /// Start the server
    Start {
        /// Path to the configuration file [default: ./rfd-api/config.toml or /etc/rfd-api/config.toml]
        #[arg(short, long)]
        config: Option<String>,
    },
    /// Validate a configuration file
    Validate {
        /// Path to the configuration file [default: ./rfd-api/config.toml or /etc/rfd-api/config.toml]
        #[arg(short, long)]
        config: Option<String>,
    },
    /// Print the version
    Version,
    /// Print the OpenAPI JSON schema to stdout
    Describe,
    /// Run database migrations
    Migrate {
        /// Database connection string [default: $DATABASE_URL]
        #[arg(long)]
        database_url: Option<String>,
        /// Only run v-api migrations, skip RFD-specific migrations
        #[arg(long)]
        v_only: bool,
    },
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

fn resolve_database_url(database_url: &Option<String>) -> anyhow::Result<String> {
    database_url
        .clone()
        .or_else(|| std::env::var("DATABASE_URL").ok())
        .ok_or_else(|| {
            anyhow::anyhow!(
                "Database URL must be specified via --database-url or the DATABASE_URL \
                 environment variable"
            )
        })
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    rustls::crypto::aws_lc_rs::default_provider()
        .install_default()
        .expect("Failed to install rustls crypto provider");

    let args = Args::parse();

    match args.command {
        ServerCommand::Version => {
            println!(
                "{} ({}, {})",
                env!("CARGO_PKG_VERSION"),
                env!("RFD_API_GIT_HASH"),
                env!("RFD_API_BUILD_TYPE"),
            );
            Ok(())
        }
        // Print the OpenAPI document to stdout and exit, without requiring the rest of the
        // server's runtime configuration (database, secrets, etc). Used to keep the checked-in
        // rfd-api-spec.json up to date via `cargo xtask generate`.
        ServerCommand::Describe => {
            server::write_openapi(&mut std::io::stdout()).map_err(|err| anyhow::anyhow!(err))
        }
        ServerCommand::Validate { config } => {
            let config_sources = config.map(|path| vec![path]);
            let candidate_paths = AppConfig::candidate_paths(&config_sources);
            AppConfig::new(config_sources).map_err(|err| {
                anyhow::anyhow!(
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
        ServerCommand::Migrate {
            database_url,
            v_only,
        } => {
            let url = resolve_database_url(&database_url)?;
            rfd_model::migrations::run_migrations(&url, v_only);
            println!("Migrations completed successfully");
            Ok(())
        }
        ServerCommand::Start { config } => run_server(config).await,
    }
}

async fn run_server(config_path: Option<String>) -> anyhow::Result<()> {
    // yup_oauth2 panics unless an application level default crypto provider is installed.
    rustls::crypto::aws_lc_rs::default_provider()
        .install_default()
        .expect("Failed to install rustls crypto provider");
    jsonwebtoken::crypto::aws_lc::DEFAULT_PROVIDER
        .install_default()
        .expect("Failed to install jsonwebtoken crypto provider");

    let param_path = config_path
        .as_deref()
        .and_then(|path| Path::new(path).parent())
        .filter(|path| !path.as_os_str().is_empty())
        .map(|path| path.to_path_buf());

    let config_sources = config_path.map(|path| vec![path]);
    let candidate_paths = AppConfig::candidate_paths(&config_sources);
    let mut config = AppConfig::new(config_sources)?;

    let (writer, _guard) = if let Some(log_directory) = config.log_directory {
        let file_appender = tracing_appender::rolling::daily(log_directory, "rfd-api.log");
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
        ServerLogFormat::Json => subscriber.json().init(),
        ServerLogFormat::Pretty => subscriber.pretty().init(),
    };

    tracing::info!("Initialized logger");
    tracing::info!(
        config_file = %describe_config_paths(&candidate_paths),
        "Loaded configuration"
    );

    let storage = Arc::new(
        VApiPostgresStore::new(&config.database_url)
            .await
            .tap_err(|err| {
                tracing::error!(?err, "Failed to establish initial database connection");
            })?,
    );
    let mut v_ctx_builder = VContextBuilder::<RfdPermission>::new()
        .with_public_url(config.public_url.clone())
        .with_storage(storage.clone())
        .with_jwt_expiration(config.jwt.default_expiration)
        .with_keys(std::mem::take(&mut config.keys))
        .with_additional_builtin_permissions(RfdPermission::iter().collect());
    if let Some(param_path) = param_path.clone() {
        v_ctx_builder = v_ctx_builder.with_param_path(param_path);
    }
    let mut v_ctx = v_ctx_builder.build().await?;

    if let Some(github) = config.authn.oauth.github {
        let github_config = github.resolve(param_path.as_deref())?;
        let public_url = config.public_url.clone();
        v_ctx.insert_oauth_provider(
            OAuthProviderName::GitHub,
            Box::new(move || {
                Box::new(GitHubOAuthProvider::new(
                    github_config.clone(),
                    public_url.clone(),
                    None,
                ))
            }),
        );

        tracing::info!("Added GitHub OAuth provider");
    }

    if let Some(google) = config.authn.oauth.google {
        let google_config = google.resolve(param_path.as_deref())?;
        let public_url = config.public_url.clone();
        v_ctx.insert_oauth_provider(
            OAuthProviderName::Google,
            Box::new(move || {
                Box::new(GoogleOAuthProvider::new(
                    google_config.clone(),
                    public_url.clone(),
                    None,
                ))
            }),
        );

        tracing::info!("Added Google OAuth provider");
    }

    // Install magic link support
    for template in config.magic_link.templates {
        let mut email_message_env = Environment::new();

        email_message_env.add_template_owned("text", template.text)?;
        if let Some(subject) = template.subject {
            email_message_env.add_template_owned("subject", subject)?;
        }
        if let Some(html) = template.html {
            email_message_env.add_template_owned("html", html)?;
        }
        let target = MagicLinkTarget {
            medium: MagicLinkMedium::Email,
            channel: template.channel,
        };

        v_ctx.magic_link.set_message_builder(
            target.clone(),
            MagicLinkMessageBuilder {
                env: email_message_env,
            },
        );

        if let Some(service) = &config.magic_link.email_service {
            match service {
                EmailService::Resend { key } => {
                    v_ctx.magic_link.set_messenger(
                        target,
                        ResendMagicLink::new(key.to_string(), template.from),
                    );
                }
            }
        }
    }

    // Configure permissions for the default unauthenticated user
    v_ctx.add_unauthenticated_caller_permission(RfdPermission::SearchRfds);

    let context = RfdContext::new(
        config.public_url,
        storage,
        config.search,
        config.content,
        config.services,
        v_ctx,
    )
    .await?;

    tracing::info!("Configured server context");

    let init_data = InitialData::new(config.initial_mappers.map(|p| vec![p])).tap_err(|err| {
        tracing::error!(?err, "Failed to load initial data from configuration");
    })?;
    init_data.initialize(context.v_ctx()).await.tap_err(|err| {
        tracing::error!(?err, "Failed to install initial data");
    })?;

    tracing::info!("Loaded initial data");

    tracing::debug!(?config.spec, "Spec configuration");

    let config = ServerConfig {
        context,
        server_address: SocketAddr::V4(SocketAddrV4::new(
            "0.0.0.0"
                .parse()
                .tap_err(|err| tracing::error!(?err, "Failed to parse server address"))?,
            config.server_port,
        )),
        spec_output: config.spec,
    };

    let server = server(config)
        .tap_err(|err| {
            tracing::error!(?err, "Failed to construct server");
        })
        .expect("Failed to construct server")
        .start();

    server?
        .await
        .tap_err(|err| {
            tracing::error!(?err, "Server exited with an error");
        })
        .expect("Failed to start server");

    tracing::error!("Server completed without an error");

    Ok(())
}
