// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use async_trait::async_trait;
use newtype_uuid::TypedUuid;
use octorust::types::{LabelsData, PullRequestData, PullRequestSimple};
use process_includes::ProcessIncludes;
use rfd_data::content::RfdDocument;
use rfd_github::{GitHubError, GitHubRfdUpdate};
use rfd_model::RfdId;
use serde::Deserialize;
use std::fmt::Debug;
use thiserror::Error;
use tracing::instrument;
use v_model::storage::StoreError;

use crate::{
    context::Context,
    rfd::{FetchRemoteRfdError, PersistedRfd, RemoteRfd, RemoteRfdError, RfdError},
};

use self::{
    copy_images_to_storage::CopyImagesToStorage, create_pull_request::CreatePullRequest,
    ensure_default_state::EnsureRfdOnDefaultIsInValidState,
    ensure_pr_state::EnsureRfdWithPullRequestIsInValidState,
    update_discussion_url::UpdateDiscussionUrl, update_pdfs::UpdatePdfs,
    update_pull_request::UpdatePullRequest, update_search_index::UpdateSearch,
};

mod copy_images_to_storage;
mod create_pull_request;
mod ensure_default_state;
mod ensure_pr_state;
mod process_includes;
mod update_discussion_url;
mod update_pdfs;
mod update_pull_request;
mod update_search_index;

#[derive(Debug, Error)]
pub enum RfdUpdaterError {
    #[error("Update action failed {0}")]
    ActionError(#[from] Box<dyn std::error::Error + Send>),
    #[error("Failed to lookup existing RFD revision {0}")]
    ExistingLookup(StoreError),
    #[error("Failed to persist to GitHub {0}")]
    GitHubStorage(GitHubError),
    #[error("Failed to determine action")]
    InvalidAction,
    #[error("Failed to generate list of pull requests {0}")]
    PullRequestLookupFailed(GitHubError),
    #[error("Failed to create remote RFD to process {0}")]
    RfdCreate(#[from] FetchRemoteRfdError),
    #[error("Failed to persist remote RFD {0}")]
    PersistRemote(#[from] RemoteRfdError),
    #[error("Failed to update RFD {0}")]
    RfdUpdate(RfdError),
    #[error("Newly persisted RFD has an internal id that does match the expected existing id")]
    RfdIdMismatch((TypedUuid<RfdId>, TypedUuid<RfdId>)),
}

trait Validate {
    fn is_valid(&self) -> bool;
}

impl Validate for GitHubRfdUpdate {
    fn is_valid(&self) -> bool {
        // a RFD update is only valid in one of two cases:
        //  `1. The update is occurring on the default branch. In this case it does not matter what
        //      RFD is being updated, the update is always considered valid
        //   2. The update is occurring on a RFD branch with a name of the pattern \d\d\d\d . In
        //      this case, the update is only valid if the number of the RFD being updated matches
        //      the branch the update is occurring on.
        self.location.branch == self.location.default_branch
            || self.location.branch == self.number.as_number_string()
    }
}

impl TryFrom<&str> for BoxedAction {
    type Error = RfdUpdaterError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "CopyImagesToStorage" => Ok(Box::new(CopyImagesToStorage)),
            "UpdateSearch" => Ok(Box::new(UpdateSearch)),
            "UpdatePdfs" => Ok(Box::new(UpdatePdfs)),
            "CreatePullRequest" => Ok(Box::new(CreatePullRequest)),
            "UpdatePullRequest" => Ok(Box::new(UpdatePullRequest)),
            "UpdateDiscussionUrl" => Ok(Box::new(UpdateDiscussionUrl)),
            "ProcessIncludes" => Ok(Box::new(ProcessIncludes)),
            "EnsureRfdWithPullRequestIsInValidState" => {
                Ok(Box::new(EnsureRfdWithPullRequestIsInValidState))
            }
            "EnsureRfdOnDefaultIsInValidState" => Ok(Box::new(EnsureRfdOnDefaultIsInValidState)),
            _ => Err(RfdUpdaterError::InvalidAction),
        }
    }
}

pub struct RfdUpdater<'a> {
    actions: &'a [BoxedAction],
    mode: RfdUpdateMode,
}

impl<'a> RfdUpdater<'a> {
    pub fn new(actions: &'a [BoxedAction], mode: RfdUpdateMode) -> Self {
        tracing::info!(?actions, "Configuring new updater");

        Self { actions, mode }
    }

    #[instrument(skip(self, ctx, updates), name = "Run update batch", fields(size = updates.len()))]
    pub async fn handle(
        &self,
        ctx: &Context,
        updates: &[GitHubRfdUpdate],
    ) -> Result<(), RfdUpdaterError> {
        // Loop through the updates that were provided and process them individually. We also throw
        // out any updates that attempt to update a mismatched RFD
        for update in updates {
            // Skip any updates that fail validation
            if update.is_valid() {
                // If this branch does not actually exist in GitHub, then we drop the update
                if update.location.exists_in_remote(&ctx.github.client).await {
                    tracing::trace!("Located remote branch");

                    if let Err(err) = self.run_update(&ctx, update).await {
                        tracing::warn!(?update, ?err, "Failed to run update for RFD to completion",);
                    }
                } else {
                    tracing::info!(
                        ?update,
                        "Dropping RFD update as the remote branch has gone missing"
                    );
                }
            } else {
                tracing::warn!(
                    ?update,
                    "Encountered invalid RFD update (it will not be run)"
                );
            }
        }

        Ok(())
    }

    #[instrument(skip(self, ctx))]
    async fn run_update(
        &self,
        ctx: &Context,
        update: &GitHubRfdUpdate,
    ) -> Result<(), RfdUpdaterError> {
        // We have a README file that changed, let's parse the RFD and update it
        // in our database.
        tracing::info!("Updating RFD");

        // Fetch the latest RFD information from GitHub
        let remote = RemoteRfd::new_from_update(&ctx.github.client, update).await?;

        tracing::trace!(?remote.number, ?remote.commit, ?remote.commit_date, "Created remote RFD");
        tracing::info!("Generated RFD from branch on GitHub");

        // Before persisting the new revision, fetch the most recent existing revision. This is
        // provided to further actions for inspecting changes between the two revisions.
        let existing = PersistedRfd::load(remote.number, &ctx.db.storage)
            .await
            .map_err(|err| RfdUpdaterError::ExistingLookup(err))?;

        if let Some(existing) = &existing {
            tracing::info!(id = ?existing.rfd.id, number = ?update.number, "Found previous revision for RFD");
        } else {
            tracing::info!(number = ?update.number, "No previous revisions found for RFD");
        }

        // Update the RFD in the database.
        let mut persisted = remote.upsert(&ctx.db.storage).await?;

        tracing::info!(id = ?persisted.rfd.id, number = ?persisted.rfd.rfd_number, "Upserted RFD in to the database");

        // Ensure that the persisted revision wrote correctly and that it is referencing the same
        // RFD as the previous revision. If there is no existing revision (and RFD), then this is
        // check can be ignored
        if existing
            .as_ref()
            .map(|ex| ex.rfd.id != persisted.rfd.id)
            .unwrap_or(false)
        {
            // Able to unwrap here as the conditional above always fails in the None case
            return Err(RfdUpdaterError::RfdIdMismatch((
                existing.unwrap().rfd.id,
                persisted.rfd.id,
            )));
        }

        // The RFD has been stored internally, now trigger the update actions
        self.run_actions(ctx, update, existing.as_ref(), &mut persisted)
            .await?;

        // Perform a final update to capture and modifications made during update actions
        persisted
            .upsert(&ctx.db.storage)
            .await
            .map_err(RfdUpdaterError::RfdUpdate)?;

        tracing::info!(id = ?persisted.rfd.id, number = ?persisted.rfd.rfd_number, "Update for RFD completed");

        Ok(())
    }

    #[instrument(skip(self, ctx, previous, new, update), fields(id = ?new.rfd.id, revision = ?new.revision.id))]
    async fn run_actions(
        &self,
        ctx: &Context,
        update: &GitHubRfdUpdate,
        previous: Option<&PersistedRfd>,
        new: &mut PersistedRfd,
    ) -> Result<(), RfdUpdaterError> {
        let pull_requests = update
            .location
            .find_pull_requests(&ctx.github.client)
            .await
            .map_err(RfdUpdaterError::PullRequestLookupFailed)?
            .into_iter()
            .map(|pr| pr.into())
            .collect();

        let mut ctx = RfdUpdateActionContext {
            ctx,
            pull_requests,
            update,
            previous,
        };

        tracing::info!("Generated action context");

        let mut responses = vec![];

        for action in self.actions {
            match action.run(&mut ctx, new, self.mode).await {
                Ok(response) => responses.push(response),
                Err(err) => match err {
                    RfdUpdateActionErr::Continue(action_err) => {
                        tracing::warn!(?action_err, "Updating RFD failed with non-fatal error");
                    }
                    RfdUpdateActionErr::Stop(action_err) => {
                        tracing::warn!(action_err, "Updating RFD failed with fatal error");
                        return Err(action_err)?;
                    }
                },
            }
        }

        let response: RfdUpdateActionResponse = responses.into();

        tracing::info!(?response, "Computed follow up action for update");

        if response.requires_source_commit && self.mode == RfdUpdateMode::Write {
            // Update the file in GitHub.
            update
                .location
                .upsert(
                    &new.number,
                    new.content()
                        .map_err(|err| RfdUpdaterError::RfdUpdate(RfdError::Asciidoc(err)))?
                        .raw()
                        .as_bytes(),
                    "RFD processor update",
                )
                .await
                .map_err(RfdUpdaterError::GitHubStorage)?;
        }

        Ok(())
    }
}

pub struct RfdUpdateActionContext<'a, 'd, 'f> {
    pub ctx: &'a Context,
    pub pull_requests: Vec<PullRequestRef>,
    pub update: &'d GitHubRfdUpdate,
    pub previous: Option<&'f PersistedRfd>,
}

#[derive(Debug)]
pub struct PullRequestRef {
    pub number: i64,
    pub title: String,
    pub state: String,
    pub html_url: String,
    pub labels: Vec<LabelsData>,
}

impl From<PullRequestSimple> for PullRequestRef {
    fn from(value: PullRequestSimple) -> Self {
        Self {
            number: value.number,
            title: value.title,
            state: value.state,
            html_url: value.html_url,
            labels: value.labels,
        }
    }
}

impl From<PullRequestData> for PullRequestRef {
    fn from(value: PullRequestData) -> Self {
        Self {
            number: value.number,
            title: value.title,
            state: value.state.to_string(),
            html_url: value.html_url,
            labels: value.labels,
        }
    }
}

#[async_trait]
pub trait RfdUpdateAction: Debug {
    async fn run(
        &self,
        ctx: &mut RfdUpdateActionContext,
        new: &mut PersistedRfd,
        mode: RfdUpdateMode,
    ) -> Result<RfdUpdateActionResponse, RfdUpdateActionErr>;
}

pub type BoxedAction = Box<dyn RfdUpdateAction + Send + Sync>;

#[derive(Copy, Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum RfdUpdateMode {
    Read,
    Write,
}

#[derive(Debug, Default)]
pub struct RfdUpdateActionResponse {
    pub requires_source_commit: bool,
}

impl From<Vec<RfdUpdateActionResponse>> for RfdUpdateActionResponse {
    fn from(responses: Vec<RfdUpdateActionResponse>) -> Self {
        responses
            .iter()
            .fold(RfdUpdateActionResponse::default(), |acc, response| {
                RfdUpdateActionResponse {
                    requires_source_commit: acc.requires_source_commit
                        || response.requires_source_commit,
                }
            })
    }
}

#[derive(Debug)]
pub enum RfdUpdateActionErr {
    Continue(Box<dyn std::error::Error + Send>),
    Stop(Box<dyn std::error::Error + Send>),
}
