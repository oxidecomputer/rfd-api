use config::{Config, ConfigError, Environment, File};
use rfd_model::{storage::StoreError, NewAccessGroup};
use serde::Deserialize;
use uuid::Uuid;

use crate::{context::ApiContext, mapper::MapperRules, ApiPermissions};

#[derive(Debug, Deserialize)]
pub struct InitialData {
    pub groups: Vec<InitialGroup>,
    pub mappers: Vec<MapperRules>,
}

#[derive(Debug, Deserialize)]
pub struct InitialGroup {
    pub name: String,
    pub permissions: ApiPermissions,
}

impl InitialData {
    pub fn new() -> Result<Self, ConfigError> {
        let config = Config::builder()
            .add_source(File::with_name("baseline.toml").required(false))
            .add_source(File::with_name("rfd-api/baseline.toml").required(false))
            .add_source(Environment::default())
            .build()?;

        config.try_deserialize()
    }

    pub async fn initialize(&self, ctx: &ApiContext) -> Result<(), StoreError> {
        for group in &self.groups {
            ctx.create_group(NewAccessGroup {
                id: Uuid::new_v4(),
                name: group.name.clone(),
                permissions: group.permissions.clone(),
            })
            .await?;
        }

        for _mapper in &self.mappers {
            // TODO: Configure initial mappers
        }

        Ok(())
    }
}
