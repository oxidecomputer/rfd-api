use config::{Config, ConfigError, Environment, File};
use rfd_model::{storage::StoreError, NewAccessGroup, NewMapper};
use serde::Deserialize;
use thiserror::Error;
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
            .add_source(File::with_name("baseline.toml").required(false))
            .add_source(File::with_name("rfd-api/baseline.toml").required(false))
            .add_source(Environment::default())
            .build()?;

        Ok(config.try_deserialize()?)
    }

    pub async fn initialize(self, ctx: &ApiContext) -> Result<(), InitError> {
        for group in self.groups {
            ctx.create_group(NewAccessGroup {
                id: Uuid::new_v4(),
                name: group.name,
                permissions: group.permissions,
            })
            .await?;
        }

        for mapper in self.mappers {
            ctx.add_mapper(&NewMapper {
                id: Uuid::new_v4(),
                name: mapper.name,
                rule: serde_json::to_value(&mapper.rule)?,
                activations: None,
                max_activations: mapper.max_activations.map(|i| i as i32),
            })
            .await?;
        }

        Ok(())
    }
}
