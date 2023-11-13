// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::{
    error::Error,
    net::{SocketAddr, SocketAddrV4},
};

use context::Context;
use regex::Regex;
use tracing_subscriber::EnvFilter;

use crate::{
    config::{AppConfig, ServerLogFormat},
    server::{server, ServerConfig},
};

mod config;
mod context;
mod endpoints;
mod server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let config = AppConfig::new()?;

    let subscriber = tracing_subscriber::fmt()
        .with_file(false)
        .with_line_number(false)
        .with_env_filter(EnvFilter::from_default_env());

    match config.log_format {
        ServerLogFormat::Json => subscriber.json().init(),
        ServerLogFormat::Pretty => subscriber.pretty().init(),
    };

    let config = ServerConfig {
        context: Context {
            host_regex: Regex::new(&config.host_regex)?,
            github_template: config.github_template.to_string(),
        },
        server_address: SocketAddr::V4(SocketAddrV4::new("0.0.0.0".parse()?, config.server_port)),
    };

    let server = server(config)?.start();

    server.await?;

    Ok(())
}
