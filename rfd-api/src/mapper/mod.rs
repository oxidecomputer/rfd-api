use std::collections::BTreeSet;

use async_trait::async_trait;
use rfd_model::{storage::StoreError, Mapper};
use serde::{Deserialize, Serialize};
use tap::TapFallible;
use uuid::Uuid;

use crate::{context::ApiContext, endpoints::login::UserInfo, ApiPermissions};

use self::{
    email_address::EmailAddressMapper, email_domain::EmailDomainMapper,
    github_username::GitHubUsernameMapper,
};

pub mod email_address;
pub mod email_domain;
pub mod github_username;

#[async_trait]
pub trait MapperRule: Send + Sync {
    async fn permissions_for(
        &self,
        ctx: &ApiContext,
        user: &UserInfo,
    ) -> Result<ApiPermissions, StoreError>;
    async fn groups_for(
        &self,
        ctx: &ApiContext,
        user: &UserInfo,
    ) -> Result<BTreeSet<Uuid>, StoreError>;
}

#[derive(Debug, Serialize)]
pub struct Mapping {
    pub id: Uuid,
    pub name: String,
    pub rule: MappingRules,
    pub activations: Option<i32>,
    pub max_activations: Option<i32>,
}

impl TryFrom<Mapper> for Mapping {
    type Error = serde_json::Error;

    fn try_from(value: Mapper) -> Result<Self, Self::Error> {
        serde_json::from_value::<MappingRules>(value.rule)
            .tap_err(|err| {
                tracing::error!(?err, "Failed to translate stored rule to mapper");
            })
            .map(|rule| Mapping {
                id: value.id,
                name: value.name,
                rule,
                activations: value.activations,
                max_activations: value.max_activations,
            })
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "rule", rename_all = "snake_case")]
pub enum MappingRules {
    EmailAddress(EmailAddressMapper),
    EmailDomain(EmailDomainMapper),
    GitHubUsername(GitHubUsernameMapper),
}

#[async_trait]
impl MapperRule for MappingRules {
    async fn permissions_for(
        &self,
        ctx: &ApiContext,
        user: &UserInfo,
    ) -> Result<ApiPermissions, StoreError> {
        match self {
            Self::EmailAddress(rule) => rule.permissions_for(ctx, user).await,
            Self::EmailDomain(rule) => rule.permissions_for(ctx, user).await,
            Self::GitHubUsername(rule) => rule.permissions_for(ctx, user).await,
        }
    }

    async fn groups_for(
        &self,
        ctx: &ApiContext,
        user: &UserInfo,
    ) -> Result<BTreeSet<Uuid>, StoreError> {
        match self {
            Self::EmailAddress(rule) => rule.groups_for(ctx, user).await,
            Self::EmailDomain(rule) => rule.groups_for(ctx, user).await,
            Self::GitHubUsername(rule) => rule.groups_for(ctx, user).await,
        }
    }
}
