use std::collections::BTreeSet;

use async_trait::async_trait;
use rfd_model::storage::StoreError;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{context::ApiContext, endpoints::login::UserInfo, ApiPermissions};

use self::email_domain::EmailDomainMapper;

pub mod email_address;
pub mod email_domain;

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

#[derive(Debug, Deserialize, Serialize)]
pub enum MappingRules {
    EmailDomain(EmailDomainMapper),
}

#[async_trait]
impl MapperRule for MappingRules {
    async fn permissions_for(
        &self,
        ctx: &ApiContext,
        user: &UserInfo,
    ) -> Result<ApiPermissions, StoreError> {
        match self {
            Self::EmailDomain(rule) => rule.permissions_for(ctx, user).await,
        }
    }

    async fn groups_for(
        &self,
        ctx: &ApiContext,
        user: &UserInfo,
    ) -> Result<BTreeSet<Uuid>, StoreError> {
        match self {
            Self::EmailDomain(rule) => rule.groups_for(ctx, user).await,
        }
    }
}
