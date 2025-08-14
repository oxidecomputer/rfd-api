// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::sync::Mutex;

use chrono::{DateTime, Utc};
use newtype_uuid::TypedUuid;
use octorust::{Client, ClientError};
use rfd_data::{content::RfdDocument, RfdNumber};
use rfd_github::{GitHubError, GitHubRfdReadme, GitHubRfdUpdate};
use rfd_model::{
    schema_ext::{ContentFormat, Visibility},
    storage::{
        RfdFilter, RfdPdfFilter, RfdPdfStore, RfdRevisionFilter, RfdRevisionStore, RfdStore,
    },
    CommitSha, FileSha, NewRfd, NewRfdRevision, Rfd, RfdRevision,
};
use thiserror::Error;
use v_model::storage::{ListPagination, StoreError};

use crate::content::{RenderableRfd, RenderableRfdError};

#[derive(Debug)]
pub struct PersistedRfd {
    pub number: RfdNumber,
    pub rfd: Rfd,
    pub revision: RfdRevision,
    pub pdf_external_id: Option<String>,
    needs_update: Mutex<bool>,
}

impl PersistedRfd {
    pub fn new(
        number: RfdNumber,
        rfd: Rfd,
        revision: RfdRevision,
        pdf_external_id: Option<String>,
    ) -> Self {
        Self {
            number,
            rfd,
            revision,
            pdf_external_id,
            needs_update: Mutex::new(false),
        }
    }

    pub fn name(&self) -> String {
        format!("RFD {} {}", self.rfd.rfd_number, self.revision.title)
    }

    pub async fn load<S>(number: RfdNumber, storage: &S) -> Result<Option<Self>, StoreError>
    where
        S: RfdStore + RfdRevisionStore + RfdPdfStore,
    {
        let existing_rfd = RfdStore::list(
            storage,
            vec![RfdFilter::default().rfd_number(Some(vec![number.into()]))],
            &ListPagination::latest(),
        )
        .await?
        .into_iter()
        .next();

        if let Some(rfd) = existing_rfd {
            let most_recent_revision = RfdRevisionStore::list(
                storage,
                vec![RfdRevisionFilter::default().rfd(Some(vec![rfd.id]))],
                &ListPagination::latest(),
            )
            .await?
            .into_iter()
            .next();

            let most_recent_pdf = RfdPdfStore::list(
                storage,
                vec![RfdPdfFilter::default().rfd(Some(vec![rfd.id]))],
                &ListPagination::latest(),
            )
            .await?
            .into_iter()
            .next();

            Ok(most_recent_revision.map(|revision| {
                PersistedRfd::new(
                    number.into(),
                    rfd,
                    revision,
                    most_recent_pdf.map(|pdf| pdf.external_id),
                )
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn upsert<S>(&self, storage: &S) -> Result<(), RfdError>
    where
        S: RfdStore + RfdRevisionStore,
    {
        let should_update = *self.needs_update.lock().unwrap();

        if should_update {
            tracing::info!("Persisted RFD has been modified. Storing to database");

            RfdStore::upsert(storage, self.rfd.clone().into()).await?;
            RfdRevisionStore::upsert(storage, self.revision.clone().into()).await?;

            tracing::info!("Updated persisted RFD and revision");

            *self.needs_update.lock().unwrap() = false;
        } else {
            tracing::info!("No changes detected for revision. Skipping update");
        }

        Ok(())
    }

    pub fn content(&self) -> Result<RenderableRfd, RenderableRfdError> {
        Ok(match self.revision.content_format {
            ContentFormat::Asciidoc => RenderableRfd::new_asciidoc(&self.revision.content)?,
            ContentFormat::Markdown => RenderableRfd::new_markdown(&self.revision.content),
        })
    }

    pub fn set_content(&mut self, format: ContentFormat, content: &str) {
        self.revision.content = content.to_string();
        self.revision.content_format = format;

        *self.needs_update.lock().unwrap() = true;
    }

    pub fn update_discussion(&mut self, new_discussion_url: impl ToString) -> Result<(), RfdError> {
        let new_discussion_url = new_discussion_url.to_string();

        let mut content = self.content()?;
        content.update_discussion(&new_discussion_url)?;

        self.revision.content = content.into_inner_content();
        self.revision.discussion = Some(new_discussion_url);

        *self.needs_update.lock().unwrap() = true;

        Ok(())
    }

    pub fn is_state(&self, state: &str) -> bool {
        self.revision
            .state
            .as_ref()
            .map(|s| s.as_str() == state)
            .unwrap_or(false)
    }

    pub fn update_state(&mut self, new_state: impl ToString) -> Result<(), RfdError> {
        let new_state = new_state.to_string();

        let mut content = self.content()?;
        content.update_state(&new_state)?;

        self.revision.content = content.into_inner_content();
        self.revision.state = Some(new_state);

        *self.needs_update.lock().unwrap() = true;

        Ok(())
    }

    pub fn get_pdf_filename(&self) -> String {
        let mut filename = format!("RFD {}", self.number.as_number_string());

        if !self.revision.title.trim().is_empty() {
            tracing::trace!(?filename, title = ?self.revision.title, "Add title to pdf filename");

            filename = filename
                + " "
                + self
                    .revision
                    .title
                    .replace('/', "-")
                    .replace('\'', "")
                    .replace(':', "")
                    .trim();
        } else {
            tracing::trace!(?filename, "Omitting RFD title from pdf filename");
        }

        filename = filename + ".pdf";

        filename
    }
}

#[derive(Debug, Error)]
pub enum RfdError {
    #[error(transparent)]
    Asciidoc(#[from] RenderableRfdError),
    #[error(transparent)]
    Storage(#[from] StoreError),
}

pub struct RfdPayload {
    // Rfd
    pub number: i32,
    pub link: String,
    pub discussion: Option<String>,
    // Revision
    pub state: String,
    pub title: String,
    pub content: RenderableRfd<'static>,
    pub content_format: ContentFormat,
    pub authors: String,
    pub labels: String,
    pub sha: FileSha,
    pub commit_sha: CommitSha,
    pub commit_date: DateTime<Utc>,
}

#[derive(Debug, Error)]
pub enum RemoteRfdError {
    #[error("Remote RFD does not have canonical link")]
    MissingLink,
    #[error("Remote RFD does not have a state defined")]
    MissingState,
    #[error("Remote RFD does not have a title defined")]
    MissingTitle,
    #[error(transparent)]
    Storage(#[from] StoreError),
}

#[derive(Debug, Error)]
pub enum FetchRemoteRfdError {
    #[error(transparent)]
    Client(#[from] ClientError),
    #[error("Failed to find readme specified by update {0}")]
    FailedToFindReadme(GitHubError),
    #[error("Failed to determine a most recent commit date")]
    NoCommitDate(GitHubError),
}

#[derive(Debug)]
pub struct RemoteRfd {
    pub number: RfdNumber,
    pub readme: GitHubRfdReadme<'static>,
    pub commit: CommitSha,
    pub commit_date: DateTime<Utc>,
}

impl RemoteRfd {
    pub async fn new_from_update(
        client: &Client,
        update: &GitHubRfdUpdate,
    ) -> Result<Self, FetchRemoteRfdError> {
        // A README file must be findable for the given update
        let readme = update
            .location
            .get_readme_contents(client, &update.number)
            .await
            .map_err(FetchRemoteRfdError::FailedToFindReadme)?;
        let commit_date = update
            .location
            .get_commit_date(client, &update.number)
            .await
            .map_err(FetchRemoteRfdError::NoCommitDate)?;

        Ok(RemoteRfd {
            number: update.number,
            readme,
            commit: update.location.commit.clone(),
            commit_date: commit_date,
        })
    }

    fn into_payload(self) -> Result<RfdPayload, RemoteRfdError> {
        let title = self
            .readme
            .content
            .get_title()
            .ok_or(RemoteRfdError::MissingTitle)?
            .to_string();
        let authors = self
            .readme
            .content
            .get_authors()
            .unwrap_or_default()
            .to_string();
        let labels = self
            .readme
            .content
            .get_labels()
            .unwrap_or_default()
            .to_string();
        let discussion = self.readme.content.get_discussion();
        let state = self
            .readme
            .content
            .get_state()
            .ok_or(RemoteRfdError::MissingState)?
            .to_string();
        let content_format = self.readme.content.format();

        Ok(RfdPayload {
            // Rfd
            number: self.number.into(),
            link: self
                .readme
                .location
                .tree_link
                .ok_or(RemoteRfdError::MissingLink)?,
            discussion: discussion.map(|s| s.to_string()),

            // Revision
            state,
            title,
            content: RenderableRfd::new(self.readme.content),
            content_format,
            authors,
            labels,
            sha: self.readme.sha.into(),
            commit_sha: self.commit,
            commit_date: self.commit_date,
        })
    }

    pub async fn upsert<S>(self, storage: &S) -> Result<PersistedRfd, RemoteRfdError>
    where
        S: RfdStore + RfdRevisionStore + RfdPdfStore,
    {
        let number = self.number;
        let payload = self.into_payload()?;

        let (id, visibility) = RfdStore::list(
            storage,
            vec![RfdFilter::default().rfd_number(Some(vec![payload.number.into()]))],
            &ListPagination::latest(),
        )
        .await?
        .into_iter()
        .next()
        .map(|rfd| (rfd.id, rfd.visibility))
        .unwrap_or_else(|| (TypedUuid::new_v4(), Visibility::Private));

        let rfd = RfdStore::upsert(
            storage,
            NewRfd {
                id,
                rfd_number: payload.number.into(),
                link: payload.link.into(),
                visibility,
            },
        )
        .await?;

        let id = RfdRevisionStore::list(
            storage,
            vec![RfdRevisionFilter::default()
                .rfd(Some(vec![rfd.id]))
                .commit(Some(vec![payload.commit_sha.clone()]))],
            &ListPagination::latest(),
        )
        .await?
        .into_iter()
        .next()
        .map(|revision| {
            tracing::info!("Found existing RFD revision for this commit. Updating the revision.");
            revision.id
        })
        .unwrap_or_else(|| {
            tracing::info!("No existing revisions exist for this commit. Creating a new revision.");
            TypedUuid::new_v4()
        });

        let revision = RfdRevisionStore::upsert(
            storage,
            NewRfdRevision {
                id,
                rfd_id: rfd.id,
                title: payload.title,
                state: if payload.state.is_empty() {
                    None
                } else {
                    Some(payload.state)
                },
                discussion: payload.discussion,
                authors: if payload.authors.is_empty() {
                    None
                } else {
                    Some(payload.authors)
                },
                labels: if payload.labels.is_empty() {
                    None
                } else {
                    Some(payload.labels)
                },
                content: payload.content.raw().to_string(),
                content_format: payload.content_format,
                sha: payload.sha,
                commit: payload.commit_sha,
                committed_at: payload.commit_date,
                major_change: false,
            },
        )
        .await?;

        let mut existing_pdf = RfdPdfStore::list(
            storage,
            vec![RfdPdfFilter::default().rfd(Some(vec![rfd.id]))],
            &ListPagination::latest(),
        )
        .await?;

        Ok(PersistedRfd::new(
            number,
            rfd,
            revision,
            existing_pdf.pop().map(|pdf| pdf.external_id),
        ))
    }
}
