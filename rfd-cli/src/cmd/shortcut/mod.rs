// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::{
    cmd::shortcut::{
        access::{AccessShortcut, AccessShortcuts, RfdAccessShortcut, RfdAccessShortcuts},
        mapper::{MapperShortcut, MapperShortcuts},
    },
    Context,
};

use self::mapper::{EmailMapper, GitHubMapper};

mod access;
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
    /// Grant and revoke access to resources for groups
    Access(AccessShortcut),
    /// Create new mappers
    Mapper(MapperShortcut),
}

impl ShortcutCmd {
    pub async fn run(&self, ctx: &mut Context) -> Result<()> {
        match &self.shortcut {
            Shortcut::Access(shortcut) => match &shortcut.access {
                AccessShortcuts::Rfd(method) => match &method.rfd {
                    RfdAccessShortcuts::Add(cmd) => {
                        cmd.run(ctx).await?;
                    }
                    RfdAccessShortcuts::Remove(cmd) => {
                        cmd.run(ctx).await?;
                    }
                },
            },
            Shortcut::Mapper(shortcut) => match &shortcut.mapper {
                MapperShortcuts::Email(cmd) => cmd.run(ctx).await?,
                MapperShortcuts::GitHub(cmd) => cmd.run(ctx).await?,
            },
        }

        Ok(())
    }
}
