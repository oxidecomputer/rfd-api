use std::collections::BTreeSet;

use async_trait::async_trait;
use rfd_model::storage::StoreError;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{context::ApiContext, endpoints::login::UserInfo, ApiPermissions};

use super::MapperRule;

#[derive(Debug, Deserialize, Serialize)]
pub struct GitHubUsernameMapper {
    github_username: String,
    #[serde(default)]
    permissions: ApiPermissions,
    #[serde(default)]
    groups: Vec<String>,
}

#[async_trait]
impl MapperRule for GitHubUsernameMapper {
    async fn permissions_for(
        &self,
        _ctx: &ApiContext,
        user: &UserInfo,
    ) -> Result<ApiPermissions, StoreError> {
        if user.github_username.as_ref().map(|u| u == &self.github_username).unwrap_or(false)
        {
            Ok(self.permissions.clone())
        } else {
            Ok(ApiPermissions::new())
        }
    }

    async fn groups_for(
        &self,
        ctx: &ApiContext,
        user: &UserInfo,
    ) -> Result<BTreeSet<Uuid>, StoreError> {
        if user.github_username.as_ref().map(|u| u == &self.github_username).unwrap_or(false) {
            let groups = ctx
                .get_groups()
                .await?
                .into_iter()
                .filter_map(|group| {
                    if self.groups.contains(&group.name) {
                        Some(group.id)
                    } else {
                        None
                    }
                })
                .collect();
            Ok(groups)
        } else {
            Ok(BTreeSet::new())
        }
    }
}
