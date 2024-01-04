// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::collections::BTreeSet;

use async_trait::async_trait;
use rfd_model::storage::StoreError;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    context::ApiContext, endpoints::login::UserInfo, util::response::ResourceResult, ApiPermissions,
};

use super::MapperRule;

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct DefaultMapper {
    #[serde(default)]
    permissions: ApiPermissions,
    #[serde(default)]
    groups: Vec<String>,
}

#[async_trait]
impl MapperRule for DefaultMapper {
    async fn permissions_for(
        &self,
        _ctx: &ApiContext,
        _user: &UserInfo,
    ) -> Result<ApiPermissions, StoreError> {
        Ok(self.permissions.clone())
    }

    async fn groups_for(
        &self,
        ctx: &ApiContext,
        _user: &UserInfo,
    ) -> ResourceResult<BTreeSet<Uuid>, StoreError> {
        let groups = ctx
            .get_groups(&ctx.builtin_registration_user())
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
    }
}
