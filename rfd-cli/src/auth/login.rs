use anyhow::Result;
use clap::{Parser, Subcommand};
use oauth2::TokenResponse;
use rfd_sdk::types::OAuthProviderName;

use crate::{Context, auth::oauth};

// Authenticates and generates an access token for interacting with the api
#[derive(Parser, Debug)]
#[clap(name = "login")]
pub struct Login {
    #[command(subcommand)]
    provider: LoginProvider,
}

impl Login {
    pub async fn run(&self, ctx: &mut Context) -> Result<()> {
        let access_token = self.provider.run(ctx).await?;

        ctx.config.set_token(access_token);
        ctx.config.save()?;

        Ok(())
    }
}

#[derive(Subcommand, Debug)]
pub enum LoginProvider {
    /// Login via GitHub
    #[command(name = "github")]
    GitHub,
    /// Login via Google
    Google,
}

impl LoginProvider {
    fn as_name(&self) -> OAuthProviderName {
        match self {
            Self::GitHub => OAuthProviderName::Github,
            Self::Google => OAuthProviderName::Google,
        }
    }

    pub async fn run(&self, ctx: &mut Context) -> Result<String> {
        let provider = ctx
            .client()?
            .get_device_provider()
            .provider(self.as_name())
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

        let token = match token_response {
            Ok(token) => Ok(token.access_token().to_owned()),
            Err(err) => Err(anyhow::anyhow!("Authentication failed: {}", err)),
        }?;

        Ok(token.secret().to_string())
    }
}
