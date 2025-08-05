// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use anyhow::{bail, Ok, Result};
use clap::{Parser, Subcommand};
use rfd_sdk::types::RfdPermission;

use crate::Context;

#[derive(Debug, Parser)]
pub struct AccessShortcut {
    #[clap(subcommand)]
    pub access: AccessShortcuts,
}

#[derive(Debug, Subcommand)]
pub enum AccessShortcuts {
    /// Grant and revoke access to RFDs
    Rfd(RfdAccessShortcut),
}

#[derive(Debug, Parser)]
pub struct RfdAccessShortcut {
    #[clap(subcommand)]
    pub rfd: RfdAccessShortcuts,
}

#[derive(Debug, Subcommand)]
pub enum RfdAccessShortcuts {
    /// Grant access to an RFD
    Add(AddRfdAccessShortcut),
    /// Revoke access to an RFD
    Remove(RemoveRfdAccessShortcut),
}

#[derive(Debug, Parser)]
pub struct AddRfdAccessShortcut {
    /// Group name or id to grant access to
    pub group: String,
    /// RFD to grant access to
    pub number: i32,
}

#[derive(Debug, Parser)]
pub struct RemoveRfdAccessShortcut {
    /// Group name or id to revoke access to
    pub group: String,
    /// RFD to revoke access to
    pub number: i32,
}

impl AddRfdAccessShortcut {
    pub async fn run(&self, ctx: &mut Context) -> Result<()> {
        let client = ctx.client()?;
        let groups = client.get_groups().send().await?.into_inner();
        let group = groups
            .iter()
            .find(|g| g.id.to_string() == self.group)
            .or_else(|| groups.iter().find(|g| g.name == self.group));

        if let Some(mut group) = group.cloned() {
            group.permissions.0.push(RfdPermission::GetRfd(self.number));
            let response = client
                .update_group()
                .group_id(group.id)
                .body_map(|body| body.name(group.name).permissions(group.permissions))
                .send()
                .await?
                .into_inner();

            println!("Added access to RFD {} to {}", self.number, self.group);
            Ok(())
        } else {
            bail!("Unable to find requested group")
        }
    }
}

impl RemoveRfdAccessShortcut {
    pub async fn run(&self, ctx: &mut Context) -> Result<()> {
        let client = ctx.client()?;
        let groups = client.get_groups().send().await?.into_inner();
        let group = groups
            .iter()
            .find(|g| g.id.to_string() == self.group)
            .or_else(|| groups.iter().find(|g| g.name == self.group));

        if let Some(mut group) = group.cloned() {
            group.permissions.0 = group
                .permissions
                .0
                .into_iter()
                .filter(|permission| match permission {
                    RfdPermission::GetRfd(number) if *number == self.number => false,
                    _ => true,
                })
                .collect::<Vec<_>>();

            let response = client
                .update_group()
                .group_id(group.id)
                .body_map(|body| body.name(group.name).permissions(group.permissions))
                .send()
                .await?
                .into_inner();

            println!("Removed access to RFD {} from {}", self.number, self.group);
            Ok(())
        } else {
            bail!("Unable to find requested group")
        }
    }
}
