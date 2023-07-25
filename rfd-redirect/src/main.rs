/// Handle redirecting urls of the form https://{rfd_number}.rfd.oxide.computer to GitHub to
/// maintain backwards compatibility
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
            max_number: 100,
            host_regex: Regex::new(r#"^(\d{1,4}).rfd.oxide.computer$"#)?,
            github_template:
                "https://github.com/oxidecomputer/rfd/tree/{rfd_number}/rfd/{rfd_number}"
                    .to_string(),
        },
        server_address: SocketAddr::V4(SocketAddrV4::new("0.0.0.0".parse()?, config.server_port)),
    };

    let server = server(config)?.start();

    server.await?;

    Ok(())
}
