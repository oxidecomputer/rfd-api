use async_trait::async_trait;
use tracing::instrument;

use crate::rfd::PersistedRfd;

use super::{
    RfdUpdateAction, RfdUpdateActionContext, RfdUpdateActionErr, RfdUpdateActionResponse,
    RfdUpdateMode,
};

#[derive(Debug)]
pub struct CreatePullRequest;

#[async_trait]
impl RfdUpdateAction for CreatePullRequest {
    #[instrument(skip(self, ctx, new), err(Debug))]
    async fn run(
        &self,
        ctx: &mut RfdUpdateActionContext,
        new: &mut PersistedRfd,
        mode: RfdUpdateMode,
    ) -> Result<RfdUpdateActionResponse, RfdUpdateActionErr> {
        let RfdUpdateActionContext {
            ctx,
            update,
            pull_requests,
            previous,
            ..
        } = ctx;

        // We only ever create pull requests if the RFD is in the discussion state, we are not
        // handling an update on the default branch, and there are no previous pull requests for
        // for this branch. This includes Closed pull requests, therefore this action will not
        // re-open or create a new pull request for a branch that previously had an open PR
        if update.location.branch != update.location.default_branch
            && new.is_state("discussion")
            && pull_requests.is_empty()
        {
            tracing::info!("RFD is in the discussion state but there are no open pull requests, creating a new pull request");

            if mode == RfdUpdateMode::Write {
                let pull = ctx
                    .github
                    .client
                    .pulls()
                    .create(
                        &update.location.owner,
                        &update.location.repo,
                        &octorust::types::PullsCreateRequest {
                            title: new.name(),
                            head: format!(
                                "{}:{}",
                                ctx.github.repository.owner, update.location.branch
                            ),
                            base: update.location.default_branch.to_string(),
                            body: "Automatically opening the pull request since the document \
                                is marked as being in discussion. If you wish to not have \
                                a pull request open, change the state of your document and \
                                close this pull request."
                                .to_string(),
                            draft: Some(false),
                            maintainer_can_modify: Some(true),
                            issue: 0,
                        },
                    )
                    .await
                    .map_err(|err| RfdUpdateActionErr::Continue(Box::new(err)))?
                    .body;

                tracing::info!(
                    old_state = ?previous.map(|rfd| &rfd.revision.state), new_state = new.revision.state, new_pr = pull.number,
                    "Opened new pull request for discussion"
                );

                // Add the newly created pull request into the context for future actions
                pull_requests.push(pull.into());
            }
        } else {
            tracing::debug!("RFD does not require a pull request or one already exists");
        }

        Ok(RfdUpdateActionResponse::default())
    }
}
