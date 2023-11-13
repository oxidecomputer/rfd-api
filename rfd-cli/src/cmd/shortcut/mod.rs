// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::Context;

use self::mapper::{EmailMapper, GitHubMapper};

mod mapper;

#[derive(Debug, Parser)]
#[clap(name = "shortcut", short_flag = 's')]
/// Shorthand commands for commonly used features
pub struct ShortcutCmd {
    #[clap(subcommand)]
    shortcut: Shortcut,
}

#[derive(Debug, Subcommand)]
pub enum Shortcut {
    EmailMapper(EmailMapper),
    #[command(name = "github-mapper")]
    GitHubMapper(GitHubMapper),
}

impl ShortcutCmd {
    pub async fn run(&self, ctx: &mut Context) -> Result<()> {
        match &self.shortcut {
            Shortcut::EmailMapper(cmd) => cmd.run(ctx).await?,
            Shortcut::GitHubMapper(cmd) => cmd.run(ctx).await?,
        }

        Ok(())
    }
}
