// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use async_trait::async_trait;
use std::cmp::Ordering;
use tracing::instrument;

use crate::rfd::PersistedRfd;

use super::{
    RfdUpdateAction, RfdUpdateActionContext, RfdUpdateActionErr, RfdUpdateActionResponse,
    RfdUpdateMode,
};

#[derive(Debug)]
pub struct EnsureRfdWithPullRequestIsInValidState;

#[async_trait]
impl RfdUpdateAction for EnsureRfdWithPullRequestIsInValidState {
    #[instrument(skip(self, ctx, new), err(Debug))]
    async fn run(
        &self,
        ctx: &mut RfdUpdateActionContext,
        new: &mut PersistedRfd,
        _mode: RfdUpdateMode,
    ) -> Result<RfdUpdateActionResponse, RfdUpdateActionErr> {
        let RfdUpdateActionContext { pull_requests, .. } = ctx;

        let mut requires_source_commit = false;

        // We only want to operate on open pull requests
        let open_prs = pull_requests.iter().filter(|pr| pr.state == "open");

        // Explicitly we will only update a pull request if it is the only open pull request for the
        // branch that we are working on
        match open_prs.count().cmp(&1) {
            Ordering::Equal => {
                // If there is a pull request open for this branch, then check to ensure that it is in one
                // of these valid states:
                //   * published  - A RFD may be in this state if it had previously been published and an
                //                  an update is being made, Or the RFD may be in the process of being
                //                  published
                //   * committed  - A RFD may be in this state if it had previously been committed and an
                //                  an update is being made. Or the RFD may be in the process of being
                //                  committed
                //   * discussion - The default state for a RFD that has an open pull request and has yet to
                //                  to be merged. If the document on this branch is found to be in an
                //                  invalid state, it will be set back to the discussion state
                //   * ideation   - An alternative state to discussion where the RFD is not yet merged, but
                //                  may not be ready for discussion. A pull request is being used to share
                //                  initial thoughts on an idea
                //   * abandoned  - A RFD may be in this state if it had previously been abandoned or is in
                //                  the process of being abandoned
                if !new.is_state("discussion")
                    && !new.is_state("published")
                    && !new.is_state("committed")
                    && !new.is_state("ideation")
                    && !new.is_state("abandoned")
                {
                    new.update_state("discussion")
                        .map_err(|err| RfdUpdateActionErr::Stop(Box::new(err)))?;
                    requires_source_commit = true;
                } else {
                    tracing::debug!("RFD is in a valid state and does not need to be updated");
                }
            }
            Ordering::Greater => {
                tracing::info!(
                    "Found multiple pull requests for RFD. Unable to update state to discussion"
                );
            }
            Ordering::Less => {
                // Nothing to do, there are no PRs
                tracing::debug!("No pull requests found for RFD. State can not be updated");
            }
        }

        Ok(RfdUpdateActionResponse {
            requires_source_commit,
        })
    }
}
