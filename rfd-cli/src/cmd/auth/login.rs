// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::ops::Add;

use anyhow::Result;
use chrono::{Duration, Utc};
use clap::{Parser, Subcommand, ValueEnum};
use oauth2::{basic::BasicTokenType, EmptyExtraTokenFields, StandardTokenResponse, TokenResponse};
use rfd_sdk::types::OAuthProviderName;
#[cfg(feature = "local-dev")]
use serde::Serialize;

use crate::{cmd::auth::oauth, Context};

// Authenticates and generates an access token for interacting with the api
#[derive(Parser, Debug, Clone)]
#[clap(name = "login")]
pub struct Login {
    #[command(subcommand)]
    provider: LoginProviderCommand,
    #[arg(short = 'm', default_value = "id")]
    mode: AuthenticationMode,
}

impl Login {
    pub async fn run(&self, ctx: &mut Context) -> Result<()> {
        let access_token = self.provider.run(ctx, &self.mode).await?;

        ctx.config.set_token(access_token);
        ctx.config.save()?;

        Ok(())
    }
}

#[derive(Debug, Clone, Subcommand)]
pub enum LoginProviderCommand {
    #[clap(name = "github")]
    /// Login via GitHub
    GitHub,
    /// Login via Google
    Google,
    /// Login with arbitrary details for local development
    #[cfg(feature = "local-dev")]
    Local {
        /// The email to authenticate as
        email: String,
        /// An arbitrary external id to uniquely identify this user
        external_id: String,
    },
}

#[derive(ValueEnum, Debug, Clone, PartialEq)]
pub enum AuthenticationMode {
    /// Retrieve and store an identity token. Identity mode is the default and should be used to
    /// when you do not require extended (multi-day) access
    #[value(name = "id")]
    Identity,
    /// Retrieve and store an api token. Token mode should be used when you want to authenticate
    /// a machine for continued access. This requires the permission to create api tokens
    #[value(name = "token")]
    Token,
}

pub struct OAuthProviderRunner(OAuthProviderName);

#[cfg(feature = "local-dev")]
pub struct LocalProviderRunner {
    email: String,
    external_id: String,
}

#[cfg(feature = "local-dev")]
#[derive(Serialize)]
struct LocalLoginBody<'a> {
    external_id: &'a str,
    email: &'a str,
}

pub trait ProviderRunner {
    async fn run(&self, ctx: &mut Context, mode: &AuthenticationMode) -> Result<String>;
}

impl ProviderRunner for OAuthProviderRunner {
    async fn run(&self, ctx: &mut Context, mode: &AuthenticationMode) -> Result<String> {
        let provider = ctx
            .client()?
            .get_device_provider()
            .provider(self.0.to_string())
            .send()
            .await?;

        let oauth_client = oauth::DeviceOAuth::new(provider.into_inner())?;
        let details = oauth_client.get_device_authorization().await?;

        println!(
            "To complete login visit: {} and enter {}",
            details.verification_uri().as_str(),
            details.user_code().secret()
        );

        let token_response = oauth_client.login(&details).await;

        let identity_token = match token_response {
            Ok(token) => Ok(token.access_token().to_owned()),
            Err(err) => Err(anyhow::anyhow!("Authentication failed: {}", err)),
        }?;

        if mode == &AuthenticationMode::Token {
            let client = ctx.new_client(Some(identity_token.secret()))?;
            let user = client.get_self().send().await?;
            Ok(client
                .create_api_user_token()
                .user_id(&user.info.id)
                .body_map(|body| body.expires_at(Utc::now().add(Duration::days(365))))
                .send()
                .await?
                .key
                .to_string())
        } else {
            Ok(identity_token.secret().to_string())
        }
    }
}

#[cfg(feature = "local-dev")]
impl ProviderRunner for LocalProviderRunner {
    async fn run(&self, ctx: &mut Context, mode: &AuthenticationMode) -> Result<String> {
        // The `/login/local` endpoint is registered by v-api only when its
        // `local-dev` feature is enabled, so it is not present in the public
        // OpenAPI spec and not exposed through the generated SDK. Hit it
        // directly via reqwest instead.
        let host = ctx.config.host()?;
        let url = format!("{}/login/local", host.trim_end_matches('/'));

        let response = reqwest::Client::new()
            .post(&url)
            .json(&LocalLoginBody {
                external_id: &self.external_id,
                email: &self.email,
            })
            .send()
            .await?
            .error_for_status()?
            .json::<StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>>()
            .await
            .map_err(|err| anyhow::anyhow!("Authentication failed: {}", err))?;

        let identity_token = response.access_token().to_owned();

        if mode == &AuthenticationMode::Token {
            let client = ctx.new_client(Some(identity_token.secret()))?;
            let user = client.get_self().send().await?;
            Ok(client
                .create_api_user_token()
                .user_id(&user.info.id)
                .body_map(|body| body.expires_at(Utc::now().add(Duration::days(365))))
                .send()
                .await?
                .key
                .to_string())
        } else {
            Ok(identity_token.secret().to_string())
        }
    }
}

impl ProviderRunner for LoginProviderCommand {
    async fn run(&self, ctx: &mut Context, mode: &AuthenticationMode) -> Result<String> {
        match self {
            LoginProviderCommand::GitHub => {
                OAuthProviderRunner(OAuthProviderName::Github)
                    .run(ctx, mode)
                    .await
            }
            LoginProviderCommand::Google => {
                OAuthProviderRunner(OAuthProviderName::Google)
                    .run(ctx, mode)
                    .await
            }
            #[cfg(feature = "local-dev")]
            LoginProviderCommand::Local { email, external_id } => {
                LocalProviderRunner {
                    email: email.to_string(),
                    external_id: external_id.to_string(),
                }
                .run(ctx, mode)
                .await
            }
        }
    }
}
