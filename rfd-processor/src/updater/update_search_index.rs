use async_trait::async_trait;
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
            tracing::info!("Updating search index");

            if mode == RfdUpdateMode::Write {
                if let Err(err) = index
                    .index_rfd(&new.rfd.rfd_number.into(), &new.revision.content)
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
