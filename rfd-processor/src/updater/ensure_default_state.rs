use async_trait::async_trait;
use tracing::instrument;

use crate::rfd::PersistedRfd;

use super::{
    RfdUpdateAction, RfdUpdateActionContext, RfdUpdateActionErr, RfdUpdateActionResponse,
    RfdUpdateMode,
};

#[derive(Debug)]
pub struct EnsureRfdOnDefaultIsInValidState;

#[async_trait]
impl RfdUpdateAction for EnsureRfdOnDefaultIsInValidState {
    #[instrument(skip(self, ctx, new), err(Debug))]
    async fn run(
        &self,
        ctx: &mut RfdUpdateActionContext,
        new: &mut PersistedRfd,
        _mode: RfdUpdateMode,
    ) -> Result<RfdUpdateActionResponse, RfdUpdateActionErr> {
        let RfdUpdateActionContext { update, .. } = ctx;

        // If an RFD exists on the default branch then it should be in either the published or
        // abandoned state
        if update.location.branch == update.location.default_branch {
            if !new.is_state("published")
                && !new.is_state("committed")
                && !new.is_state("abandoned")
            {
                tracing::warn!(state = ?new.revision.state, "RFD on the default branch is in an invalid state. It needs to be updated to either published or abandoned");
            } else {
                tracing::debug!(
                    "RFD on the default branch is in a valid state. No updates are needed"
                );
            }
        }

        Ok(RfdUpdateActionResponse::default())
    }
}
