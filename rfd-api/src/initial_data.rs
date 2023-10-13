use config::{Config, ConfigError, Environment, File};
use diesel::result::{DatabaseErrorKind, Error as DieselError};
use rfd_model::{
    storage::StoreError,
    NewAccessGroup, NewMapper,
};
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
    pub fn new() -> Result<Self, InitError> {
        let config = Config::builder()
            .add_source(File::with_name("mappers.toml").required(false))
            .add_source(File::with_name("rfd-api/mappers.toml").required(false))
            .add_source(Environment::default())
            .build()?;

        Ok(config.try_deserialize()?)
    }

    pub async fn initialize(self, ctx: &ApiContext) -> Result<(), InitError> {
        let existing_groups = ctx.get_groups().await?;

        for group in self.groups {
            let span = tracing::info_span!("Initializing group", group = ?group);

            async {
                let id = existing_groups
                    .iter()
                    .find(|g| g.name == group.name)
                    .map(|g| g.id)
                    .unwrap_or(Uuid::new_v4());

                ctx.create_group(NewAccessGroup {
                    id,
                    name: group.name,
                    permissions: group.permissions,
                })
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

                ctx.add_mapper(&new_mapper)
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
