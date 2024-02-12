// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use anyhow::Result;
use clap::{Parser, Subcommand};
use progenitor_client::Error;
use rfd_sdk::types::{self, MappingRules};
use uuid::Uuid;

use crate::{generated::cli::CliOutput, Context};

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
                .rule(MappingRules::GithubUsername {
                    github_username: self.username.clone(),
                    groups: vec![self.group.to_string()],
                    permissions: vec![].into(),
                })
        });

        let result = request.send().await;
        match result {
            Ok(r) => ctx.printer()?.output_create_mapper(Ok(r.into_inner())),
            Err(r) => ctx.printer()?.output_create_mapper(Err(r)),
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
                .rule(MappingRules::EmailAddress {
                    email: self.email.clone(),
                    groups: vec![self.group.to_string()],
                    permissions: vec![].into(),
                })
        });

        let result = request.send().await;
        match result {
            Ok(r) => ctx.printer()?.output_create_mapper(Ok(r.into_inner())),
            Err(r) => ctx.printer()?.output_create_mapper(Err(r)),
        }

        Ok(())
    }
}
