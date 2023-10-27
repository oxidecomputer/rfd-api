use anyhow::Result;
use chrono::{DateTime, Duration, Utc};
use clap::{Parser, Subcommand};
use oauth2::TokenResponse;
use rfd_sdk::types::OAuthProviderName;
use std::ops::Add;

use crate::{err::format_api_err, Context};

mod link;
mod login;
mod oauth;

// Authenticate against the RFD API
#[derive(Parser, Debug)]
#[clap(name = "auth")]
pub struct Auth {
    #[command(subcommand)]
    auth: AuthCommands,
}

#[derive(Subcommand, Debug, Clone)]
enum AuthCommands {
    /// Link an authentication provider to an account
    Link(link::Link),
    /// Login via an authentication provider
    Login(login::Login),
}

impl Auth {
    pub async fn run(&self, ctx: &mut Context) -> Result<()> {
        match &self.auth {
            AuthCommands::Link(link) => link.run(ctx).await,
            AuthCommands::Login(login) => login.run(ctx).await,
        }
    }
}