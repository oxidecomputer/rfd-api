use anyhow::Result;
use chrono::{DateTime, Duration, Utc};
use clap::{Parser, Subcommand};
use oauth2::TokenResponse;
use rfd_sdk::types::{
    AccessTokenProviderLogin, AccessTokenProviderName, JwtProviderLogin, JwtProviderName,
    LoginPermissions, OAuthProviderName,
};
use std::ops::Add;

use crate::{err::format_api_err, Context};

mod oauth;

/// Authenticates and generates an access token for interacting with the api
#[derive(Parser, Debug)]
#[clap(name = "login")]
pub struct Login {
    #[clap(subcommand)]
    provider: LoginProvider,
}

#[derive(Subcommand, Debug)]
pub enum LoginProvider {
    /// Login via GitHub
    #[clap(subcommand, name = "github")]
    GitHub(GitHubLoginMethod),
    /// Login via Google
    #[clap(subcommand)]
    Google(GoogleLoginMethod),
}

#[derive(Subcommand, Debug)]
pub enum GitHubLoginMethod {
    /// Authenticate against a GitHub account with an access, personal, or fine-grained token
    Token {
        /// An access, personal, or fine-grained token
        #[clap(short = 't', long, env = "RFD_GITHUB_TOKEN")]
        token: String,
        /// An expiration date for generated RFD API access token. If none is specified the
        /// generated token will last for one week
        #[clap(short = 'e', long)]
        expiration: Option<DateTime<Utc>>,
    },
}

#[derive(Subcommand, Debug)]
pub enum GoogleLoginMethod {
    /// Authenticate against a Google account with an id token
    Id {
        /// An id token signed by Google. The token must have the openid and email scopes
        #[clap(short = 't', long, env = "RFD_GOOGLE_ID_TOKEN")]
        token: String,

        /// An expiration date for generated RFD API access token. If none is specified the
        /// generated token will last for one week
        #[clap(short = 'e', long)]
        expiration: Option<DateTime<Utc>>,
    },
    #[clap(name = "oauth")]
    /// Authenticate against a Google account via an OAuth flow
    OAuth,
}

impl Login {
    pub async fn run(&self, ctx: &mut Context) -> Result<()> {
        let access_token = match &self.provider {
            LoginProvider::GitHub(github) => github.run(ctx).await,
            LoginProvider::Google(google) => google.run(ctx).await,
        }?;

        ctx.config.set_token(access_token);
        ctx.config.save()?;

        Ok(())
    }
}

impl GitHubLoginMethod {
    async fn run(&self, ctx: &mut Context) -> Result<String> {
        match self {
            GitHubLoginMethod::Token { token, expiration } => {
                let response = ctx
                    .client()?
                    .access_token_login()
                    .provider(AccessTokenProviderName::Github)
                    .body(AccessTokenProviderLogin {
                        token: token.to_string(),
                        expiration: expiration.or_else(|| Some(Utc::now().add(Duration::weeks(1)))),
                        permissions: LoginPermissions::All,
                    })
                    .send()
                    .await;

                match response {
                    Ok(response) => Ok(response.into_inner().token),
                    Err(err) => Err(format_api_err(&ctx, err)),
                }
            }
        }
    }
}

impl GoogleLoginMethod {
    async fn run(&self, ctx: &mut Context) -> Result<String> {
        match self {
            GoogleLoginMethod::Id { token, expiration } => {
                let response = ctx
                    .client()?
                    .jwt_login()
                    .provider(JwtProviderName::Google)
                    .body(JwtProviderLogin {
                        token: token.to_string(),
                        expiration: expiration.or_else(|| Some(Utc::now().add(Duration::weeks(1)))),
                        permissions: LoginPermissions::All,
                    })
                    .send()
                    .await;

                match response {
                    Ok(response) => Ok(response.into_inner().token),
                    Err(err) => Err(format_api_err(&ctx, err)),
                }
            }
            GoogleLoginMethod::OAuth => {
                let provider = ctx
                    .client()?
                    .get_device_provider()
                    .provider(OAuthProviderName::Google)
                    .send()
                    .await?;
                let oauth_client = oauth::GoogleDeviceAuth::new(provider.into_inner())?;
                let details = oauth_client.get_device_authorization().await?;

                println!(
                    "To complete login visit: {} and enter {}",
                    details.verification_uri().as_str(),
                    details.user_code().secret()
                );

                let token_response = oauth_client.login(&details).await;

                let token = match token_response {
                    Ok(token) => Ok(token.access_token().to_owned()),
                    Err(err) => Err(anyhow::anyhow!("Authentication failed: {}", err)),
                }?;

                Ok(token.secret().to_string())
            }
        }
    }
}
