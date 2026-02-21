// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use async_trait::async_trait;
use rfd_model::schema_ext::Visibility;
use tracing::instrument;

use crate::rfd::PersistedRfd;

use super::{
    RfdUpdateAction, RfdUpdateActionContext, RfdUpdateActionErr, RfdUpdateActionResponse,
    RfdUpdateMode,
};

#[derive(Debug)]
pub struct UpdateSearch;

#[async_trait]
impl RfdUpdateAction for UpdateSearch {
    #[instrument(skip(self, ctx, new), err(Debug))]
    async fn run(
        &self,
        ctx: &mut RfdUpdateActionContext,
        new: &mut PersistedRfd,
        mode: RfdUpdateMode,
    ) -> Result<RfdUpdateActionResponse, RfdUpdateActionErr> {
        let RfdUpdateActionContext { ctx, .. } = ctx;

        for (i, index) in ctx.search.indexes.iter().enumerate() {
            tracing::info!("Updating search index {}", index.index);

            if mode == RfdUpdateMode::Write {
                let public = match new.rfd.visibility {
                    Visibility::Private => false,
                    Visibility::Public => true,
                };

                if let Err(err) = index
                    .index_rfd(&new.rfd.rfd_number.into(), &new.revision.content, public)
                    .await
                {
                    tracing::error!(?err, search_index = i, "Failed to add RFD to search index");
                }
            }
        }

        tracing::info!("Finished updating search indexes");

        Ok(RfdUpdateActionResponse::default())
    }
}
