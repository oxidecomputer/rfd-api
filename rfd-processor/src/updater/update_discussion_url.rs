use async_trait::async_trait;
use std::cmp::Ordering;
use tracing::instrument;

use crate::rfd::PersistedRfd;

use super::{
    RfdUpdateAction, RfdUpdateActionContext, RfdUpdateActionErr, RfdUpdateActionResponse,
    RfdUpdateMode,
};

#[derive(Debug)]
pub struct UpdateDiscussionUrl;

#[async_trait]
impl RfdUpdateAction for UpdateDiscussionUrl {
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
        let open_prs = pull_requests
            .iter()
            .filter(|pr| pr.state == "open")
            .collect::<Vec<_>>();

        // Explicitly we will only update a pull request if it is the only open pull request for the
        // branch that we are working on
        match open_prs.len().cmp(&1) {
            Ordering::Equal => {
                if let Some(pull_request) = open_prs.get(0) {
                    tracing::debug!(current = ?new.revision.discussion, pr = ?pull_request.html_url, "Found discussion url for pull request. Testing if it matches the current url");

                    // If the stored discussion link does not match the PR we found, then and
                    // update is required
                    if !pull_request.html_url.is_empty()
                        && new
                            .revision
                            .discussion
                            .as_ref()
                            .map(|link| link != &pull_request.html_url)
                            .unwrap_or(true)
                    {
                        tracing::info!(
                            new.revision.discussion,
                            pull_request.html_url,
                            "Stored discussion link does not match the pull request found"
                        );

                        new.update_discussion(&pull_request.html_url)
                            .map_err(|err| RfdUpdateActionErr::Continue(Box::new(err)))?;

                        tracing::info!("Updated RFD file in GitHub with discussion link change");

                        requires_source_commit = true;
                    }
                }
            }
            Ordering::Greater => {
                tracing::warn!(
                    "Found multiple open pull requests for RFD. Unable to update discussion url"
                );
            }
            Ordering::Less => {
                // Nothing to do, there are no PRs
                tracing::debug!(
                    "No pull requests found for RFD. Discussion url can not be updated"
                );
            }
        }

        Ok(RfdUpdateActionResponse {
            requires_source_commit,
        })
    }
}
