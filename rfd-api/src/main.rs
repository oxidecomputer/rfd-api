// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use context::RfdContext;
use minijinja::Environment;
use server::{server, ServerConfig};
use std::{
    net::{SocketAddr, SocketAddrV4},
    sync::Arc,
};
use tap::TapFallible;
use tracing_appender::non_blocking::NonBlocking;
use tracing_subscriber::EnvFilter;
use v_api::{
    endpoints::login::oauth::{
        github::GitHubOAuthProvider, google::GoogleOAuthProvider, OAuthProviderName,
    },
    ApiContext, MagicLinkTarget, VContext,
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

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut args = std::env::args();
    let _ = args.next();
    let config_path = args.next();

    let config = AppConfig::new(config_path.map(|path| vec![path]))?;

    let (writer, _guard) = if let Some(log_directory) = config.log_directory {
        let file_appender = tracing_appender::rolling::daily(log_directory, "rfd-api.log");
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
        ServerLogFormat::Json => subscriber.json().init(),
        ServerLogFormat::Pretty => subscriber.pretty().init(),
    };

    tracing::info!("Initialized logger");

    // Resolve path-based secrets for asymmetric keys
    let resolved_keys: Vec<_> = config
        .keys
        .into_iter()
        .map(|key| {
            key.resolve().tap_err(|err| {
                tracing::error!(?err, "Failed to resolve asymmetric key secret");
            })
        })
        .collect::<Result<_, _>>()?;

    // Resolve database URL from config
    let database_url = config.database.to_url().tap_err(|err| {
        tracing::error!(?err, "Failed to resolve database password secret");
    })?;

    let mut v_ctx = VContext::new(
        config.public_url.clone(),
        Arc::new(
            VApiPostgresStore::new(&database_url)
                .await
                .tap_err(|err| {
                    tracing::error!(?err, "Failed to establish initial database connection");
                })?,
        ),
        config.jwt,
        resolved_keys,
    )
    .await?;

    if let Some(github) = config.authn.oauth.github {
        let device_client_id = github.device.client_id.resolve()?;
        let device_client_secret = github.device.client_secret.resolve()?;
        let web_client_id = github.web.client_id.resolve()?;
        let web_client_secret = github.web.client_secret.resolve()?;

        v_ctx.insert_oauth_provider(
            OAuthProviderName::GitHub,
            Box::new(move || {
                Box::new(GitHubOAuthProvider::new(
                    device_client_id.clone(),
                    device_client_secret.clone().into(),
                    web_client_id.clone(),
                    web_client_secret.clone().into(),
                    None,
                ))
            }),
        );

        tracing::info!("Added GitHub OAuth provider");
    }

    if let Some(google) = config.authn.oauth.google {
        let device_client_id = google.device.client_id.resolve()?;
        let device_client_secret = google.device.client_secret.resolve()?;
        let web_client_id = google.web.client_id.resolve()?;
        let web_client_secret = google.web.client_secret.resolve()?;

        v_ctx.insert_oauth_provider(
            OAuthProviderName::Google,
            Box::new(move || {
                Box::new(GoogleOAuthProvider::new(
                    device_client_id.clone(),
                    device_client_secret.clone().into(),
                    web_client_id.clone(),
                    web_client_secret.clone().into(),
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
                    v_ctx
                        .magic_link
                        .set_messenger(target, ResendMagicLink::new(key.resolve()?, template.from));
                }
            }
        }
    }

    // Configure permissions for the default unauthenticated user
    v_ctx.add_unauthenticated_caller_permission(RfdPermission::SearchRfds);

    let context = RfdContext::new(
        config.public_url,
        Arc::new(
            VApiPostgresStore::new(&database_url)
                .await
                .tap_err(|err| {
                    tracing::error!(?err, "Failed to establish initial database connection");
                })?,
        ),
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
