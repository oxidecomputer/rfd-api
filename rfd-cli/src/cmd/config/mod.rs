use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::Context;

#[derive(Debug, Parser)]
#[clap(name = "config")]
pub struct ConfigCmd {
    #[clap(subcommand)]
    setting: SettingCmd,
}

#[derive(Debug, Subcommand)]
pub enum SettingCmd {
    /// Gets a setting
    #[clap(subcommand, name = "get")]
    Get(GetCmd),
    /// Sets a setting
    #[clap(subcommand, name = "set")]
    Set(SetCmd),
}

#[derive(Debug, Subcommand)]
pub enum GetCmd {
    /// Get the configured API host in use
    #[clap(name = "host")]
    Host,
    /// Get the configured access token
    #[clap(name = "token")]
    Token,
}

#[derive(Debug, Subcommand)]
pub enum SetCmd {
    /// Set the configured API host to use
    #[clap(name = "host")]
    Host { host: String },
}

impl ConfigCmd {
    pub async fn run(&self, ctx: &mut Context) -> Result<()> {
        match &self.setting {
            SettingCmd::Get(get) => get.run(ctx).await?,
            SettingCmd::Set(set) => set.run(ctx).await?,
        }

        Ok(())
    }
}

impl GetCmd {
    pub async fn run(&self, ctx: &mut Context) -> Result<()> {
        match &self {
            GetCmd::Host => {
                println!("{}", ctx.config.host().unwrap_or("None"));
            }
            GetCmd::Token => {
                println!("{}", ctx.config.token().unwrap_or("None"));
            }
        }

        Ok(())
    }
}

impl SetCmd {
    pub async fn run(&self, ctx: &mut Context) -> Result<()> {
        match &self {
            SetCmd::Host { host } => {
                ctx.config.set_host(host.to_string());
                ctx.config.save()?;
            }
        }

        Ok(())
    }
}
