// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use config::{Config, ConfigError, Environment, File};
use diesel::result::{DatabaseErrorKind, Error as DieselError};
use rfd_model::{storage::StoreError, NewAccessGroup, NewMapper};
use serde::Deserialize;
use thiserror::Error;
use tracing::Instrument;
use uuid::Uuid;

use crate::{context::ApiContext, mapper::MappingRules, ApiPermissions};

#[derive(Debug, Deserialize)]
pub struct InitialData {
    pub groups: Vec<InitialGroup>,
    pub mappers: Vec<InitialMapper>,
}

#[derive(Debug, Deserialize)]
pub struct InitialGroup {
    pub name: String,
    pub permissions: ApiPermissions,
}

#[derive(Debug, Deserialize)]
pub struct InitialMapper {
    pub name: String,
    #[serde(flatten)]
    pub rule: MappingRules,
    pub max_activations: Option<u32>,
}

#[derive(Debug, Error)]
pub enum InitError {
    #[error("Failed to parse configuration file for initial data: {0}")]
    Config(#[from] ConfigError),
    #[error("Failed to serialize rule for storage: {0}")]
    Rule(#[from] serde_json::Error),
    #[error("Failed to store initial rule: {0}")]
    Storage(#[from] StoreError),
}

impl InitialData {
    pub fn new(config_sources: Option<Vec<String>>) -> Result<Self, InitError> {
        let mut config = Config::builder()
            .add_source(File::with_name("mappers.toml").required(false))
            .add_source(File::with_name("rfd-api/mappers.toml").required(false));

        for source in config_sources.unwrap_or_default() {
            config = config.add_source(File::with_name(&source).required(false));
        }

        Ok(config
            .add_source(Environment::default())
            .build()?
            .try_deserialize()?)
    }

    pub async fn initialize(self, ctx: &ApiContext) -> Result<(), InitError> {
        let existing_groups = ctx.get_groups(&ctx.system_caller).await?;

        for group in self.groups {
            let span = tracing::info_span!("Initializing group", group = ?group);

            async {
                let id = existing_groups
                    .iter()
                    .find(|g| g.name == group.name)
                    .map(|g| g.id)
                    .unwrap_or(Uuid::new_v4());

                ctx.create_group(
                    ctx.builtin_registration_user(),
                    NewAccessGroup {
                        id,
                        name: group.name,
                        permissions: group.permissions,
                    },
                )
                .await
                .map(|_| ())
                .or_else(handle_unique_violation_error)
            }
            .instrument(span)
            .await?
        }

        for mapper in self.mappers {
            let span = tracing::info_span!("Initializing mapper", mapper = ?mapper);
            async {
                let new_mapper = NewMapper {
                    id: Uuid::new_v4(),
                    name: mapper.name,
                    rule: serde_json::to_value(&mapper.rule)?,
                    activations: None,
                    max_activations: mapper.max_activations.map(|i| i as i32),
                };

                ctx.add_mapper(ctx.builtin_registration_user(), &new_mapper)
                    .await
                    .map(|_| ())
                    .or_else(handle_unique_violation_error)?;

                Ok::<(), InitError>(())
            }
            .instrument(span)
            .await?;
        }

        Ok(())
    }
}

fn handle_unique_violation_error(err: StoreError) -> Result<(), StoreError> {
    match err {
        StoreError::Db(DieselError::DatabaseError(DatabaseErrorKind::UniqueViolation, info)) => {
            tracing::info!(?info, "Record already exists. Skipping.");
            Ok(())
        }
        err => {
            tracing::error!(?err, "Failed to store record");
            Err(err)
        }
    }
}
