// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::collections::BTreeSet;

use async_trait::async_trait;
use rfd_model::storage::StoreError;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{context::ApiContext, endpoints::login::UserInfo, ApiPermissions};

use super::MapperRule;

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct EmailAddressMapper {
    email: String,
    #[serde(default)]
    permissions: ApiPermissions,
    #[serde(default)]
    groups: Vec<String>,
}

#[async_trait]
impl MapperRule for EmailAddressMapper {
    async fn permissions_for(
        &self,
        _ctx: &ApiContext,
        user: &UserInfo,
    ) -> Result<ApiPermissions, StoreError> {
        if user
            .verified_emails
            .iter()
            .fold(false, |found, email| found || email == &self.email)
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
        let found_email = user
            .verified_emails
            .iter()
            .fold(false, |found, email| found || email == &self.email);

        if found_email {
            let groups = ctx
                .get_groups(&ctx.system_caller)
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
