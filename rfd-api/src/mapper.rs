use async_trait::async_trait;
use uuid::Uuid;

use crate::{context::ApiContext, endpoints::login::UserInfo, ApiPermissions};

#[async_trait]
pub trait ApiPermissionMapper: Send + Sync {
    async fn permissions_for(&self, ctx: &ApiContext, user: &UserInfo) -> ApiPermissions;
}

#[async_trait]
pub trait GroupMapper: Send + Sync {
    async fn groups_for(&self, ctx: &ApiContext, user: &UserInfo) -> Vec<Uuid>;
}
