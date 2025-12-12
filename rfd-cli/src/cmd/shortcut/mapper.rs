// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use anyhow::Result;
use clap::{Parser, Subcommand};
use progenitor_client::Error;
use rfd_sdk::types::RfdPermission;
use serde_json::json;
use uuid::Uuid;

use crate::{printer::CliOutput, Context};

#[derive(Debug, Parser)]
pub struct MapperShortcut {
    #[clap(subcommand)]
    pub mapper: MapperShortcuts,
}

#[derive(Debug, Subcommand)]
pub enum MapperShortcuts {
    Email(EmailMapper),
    #[command(name = "github")]
    GitHub(GitHubMapper),
}

#[derive(Debug, Parser)]
/// Add a new one-time mapping for for a GitHub user
pub struct GitHubMapper {
    /// The GitHub username of the user to create a mapping rule for
    #[clap(index = 1)]
    username: String,
    /// The group to add this user to
    #[clap(index = 2)]
    group: String,
}

#[derive(Debug, Parser)]
/// Add a new one-time mapping for for an email address
pub struct EmailMapper {
    /// The email of the user to create a mapping rule for
    #[clap(index = 1)]
    email: String,
    /// The group to add this user to
    #[clap(index = 2)]
    group: String,
}

impl GitHubMapper {
    pub async fn run(&self, ctx: &mut Context) -> Result<()> {
        let mut request = ctx.client()?.create_mapper();
        request = request.body_map(|body| {
            body.max_activations(1)
                .name(format!("map-github-{}", Uuid::new_v4()))
                .rule(json!({
                    "rule": "github_username",
                    "github_username": self.username.clone(),
                    "groups": vec![self.group.to_string()],
                    "permissions": Vec::<RfdPermission>::new(),
                }))
        });

        let result = request.send().await;
        match result {
            Ok(r) => ctx.printer()?.output_mapper(r.into_inner()),
            Err(r) => {
                // ctx.printer()?.output_create_mapper(Err(r))
            }
        }

        Ok(())
    }
}

impl EmailMapper {
    pub async fn run(&self, ctx: &mut Context) -> Result<()> {
        let mut request = ctx.client()?.create_mapper();
        request = request.body_map(|body| {
            body.max_activations(1)
                .name(format!("map-email-{}", Uuid::new_v4()))
                .rule(json!({
                    "rule": "email_address",
                    "email": self.email.clone(),
                    "groups": vec![self.group.to_string()],
                    "permissions": Vec::<RfdPermission>::new(),
                }))
        });

        let result = request.send().await;
        match result {
            Ok(r) => ctx.printer()?.output_mapper(r.into_inner()),
            Err(r) => {
                // ctx.printer()?.output_create_mapper(Err(r))
            }
        }

        Ok(())
    }
}
