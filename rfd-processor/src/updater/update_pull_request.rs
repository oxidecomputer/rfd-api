// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use async_trait::async_trait;
use std::cmp::Ordering;
use tap::TapFallible;
use tracing::instrument;

use crate::rfd::PersistedRfd;

use super::{
    RfdUpdateAction, RfdUpdateActionContext, RfdUpdateActionErr, RfdUpdateActionResponse,
    RfdUpdateMode,
};

#[derive(Debug)]
pub struct UpdatePullRequest;

#[async_trait]
impl RfdUpdateAction for UpdatePullRequest {
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
            ..
        } = ctx;

        // We only want to operate on open pull requests
        let open_prs = pull_requests
            .iter()
            .filter(|pr| pr.state == "open")
            .collect::<Vec<_>>();

        // Explicitly we will only update a pull request if it is the only open pull request for the
        // branch that we are working on
        match open_prs.len().cmp(&1) {
            Ordering::Equal => {
                if let Some(pull_request) = open_prs.first() {
                    let rfd_name = new.name();

                    // Let's make sure the title of the pull request is what it should be.
                    // The pull request title should be equal to the name of the pull request.
                    if rfd_name != pull_request.title {
                        // TODO: Is this call necessary?
                        // Get the current set of settings for the pull request.
                        // We do this because we want to keep the current state for body.
                        let pull_content = ctx
                            .github
                            .client
                            .pulls()
                            .get(
                                &update.location.owner,
                                &update.location.repo,
                                pull_request.number,
                            )
                            .await
                            .map_err(|err| RfdUpdateActionErr::Continue(Box::new(err)))?;

                        if mode == RfdUpdateMode::Write {
                            ctx.github
                                .client
                                .pulls()
                                .update(
                                    &update.location.owner,
                                    &update.location.repo,
                                    pull_request.number,
                                    &octorust::types::PullsUpdateRequest {
                                        title: rfd_name.to_string(),
                                        body: pull_content.body.body,
                                        base: "".to_string(),
                                        maintainer_can_modify: None,
                                        state: None,
                                    },
                                )
                                .await
                                .tap_err(|err| {
                                    tracing::error!(
                                        ?err,
                                        current_title = pull_request.title,
                                        new_title = rfd_name,
                                        pull_request = pull_request.number,
                                        "Failed to update title of pull request"
                                    )
                                })
                                .map_err(|err| RfdUpdateActionErr::Continue(Box::new(err)))?;
                        }

                        tracing::info!(new_title = ?rfd_name, "Updated title");
                    } else {
                        tracing::debug!("Title is valid. No update needed");
                    }

                    // Update the labels for the pull request.
                    let mut labels: Vec<String> = Default::default();

                    if new.is_state("discussion")
                        && !pull_request
                            .labels
                            .iter()
                            .any(|label| label.name.ends_with("discussion"))
                    {
                        labels.push(":thought_balloon: discussion".to_string());
                        tracing::info!("Discussion label is missing");
                    } else if new.is_state("ideation")
                        && !pull_request
                            .labels
                            .iter()
                            .any(|label| label.name.ends_with("ideation"))
                    {
                        labels.push(":hatching_chick: ideation".to_string());
                        tracing::info!("Ideation label is missing");
                    }

                    if mode == RfdUpdateMode::Write {
                        // Only add a label if there is label missing.
                        if !labels.is_empty() {
                            ctx.github
                                .client
                                .issues()
                                .add_labels(
                                    &update.location.owner,
                                    &update.location.repo,
                                    pull_request.number,
                                    &octorust::types::IssuesAddLabelsRequestOneOf::StringVector(
                                        labels,
                                    ),
                                )
                                .await
                                .map_err(|err| RfdUpdateActionErr::Continue(Box::new(err)))?;

                            tracing::info!("Updated pull request labels");
                        } else {
                            tracing::debug!("Labels are valid. No changes needed");
                        }
                    }
                }
            }
            Ordering::Greater => {
                tracing::info!("Found multiple open pull requests for RFD. Unable to perform pull request updates");
            }
            Ordering::Less => {
                // Nothing to do, there are no PRs
                tracing::debug!(
                    "No pull requests found for RFD. No pull requests updates will be made"
                );
            }
        }

        Ok(RfdUpdateActionResponse::default())
    }
}
