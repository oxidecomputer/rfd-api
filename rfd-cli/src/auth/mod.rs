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

pub use link::Link;
pub use login::Login;
