use std::sync::Mutex;

use chrono::{DateTime, Utc};
use octorust::{Client, ClientError};
use rfd_data::RfdNumber;
use rfd_model::{
    schema_ext::ContentFormat,
    storage::{
        ListPagination, RfdFilter, RfdRevisionFilter, RfdRevisionStore, RfdStore, StoreError,
    },
    NewRfd, NewRfdRevision, Rfd, RfdRevision,
};
use thiserror::Error;
use uuid::Uuid;

use crate::{
    content::{RfdAttributes, RfdContent},
    github::{GitHubError, GitHubRfdReadme, GitHubRfdUpdate},
};

#[derive(Debug)]
pub struct PersistedRfd {
    pub number: RfdNumber,
    pub rfd: Rfd,
    pub revision: RfdRevision,
    needs_update: Mutex<bool>,
}

impl PersistedRfd {
    pub fn new(number: RfdNumber, rfd: Rfd, revision: RfdRevision) -> Self {
        Self {
            number,
            rfd,
            revision,
            needs_update: Mutex::new(false),
        }
    }

    pub fn name(&self) -> String {
        format!("RFD {} {}", self.rfd.rfd_number, self.revision.title)
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

    pub fn content(&self) -> RfdContent {
        match self.revision.content_format {
            ContentFormat::Asciidoc => RfdContent::new_asciidoc(&self.revision.content),
            ContentFormat::Markdown => RfdContent::new_markdown(&self.revision.content),
        }
    }

    pub fn update_discussion(&mut self, new_discussion_url: impl ToString) -> Result<(), RfdError> {
        let new_discussion_url = new_discussion_url.to_string();

        let mut content = self.content();
        content.update_discussion(&new_discussion_url);

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

        let mut content = self.content();
        content.update_state(&new_state);

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
    Storage(#[from] StoreError),
}

pub struct RfdPayload {
    // Rfd
    pub number: i32,
    pub link: String,
    pub discussion: Option<String>,
    // Revision
    pub state: String,
    pub name: String,
    pub title: String,
    pub content: RfdContent<'static>,
    pub content_format: ContentFormat,
    pub authors: String,
    pub sha: String,
    pub commit_sha: String,
    pub commit_date: DateTime<Utc>,
}

impl RfdPayload {
    pub fn generate_name(number: i32, title: &str) -> String {
        format!("RFD {} {}", number, title)
    }
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
    pub commit_sha: String,
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
            commit_sha: update.location.commit.clone(),
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
        let name = RfdPayload::generate_name(self.number.into(), &title);
        let authors = self
            .readme
            .content
            .get_authors()
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
            name,
            title,
            content: self.readme.content,
            content_format,
            authors,
            sha: self.readme.sha,
            commit_sha: self.commit_sha,
            commit_date: self.commit_date,
        })
    }

    pub async fn upsert<S>(self, storage: &S) -> Result<PersistedRfd, RemoteRfdError>
    where
        S: RfdStore + RfdRevisionStore,
    {
        let number = self.number;
        let payload = self.into_payload()?;

        let id = RfdStore::list(
            storage,
            RfdFilter::default().rfd_number(Some(vec![payload.number.into()])),
            &ListPagination::latest(),
        )
        .await?
        .into_iter()
        .next()
        .map(|rfd| rfd.id)
        .unwrap_or_else(|| Uuid::new_v4());

        let rfd = RfdStore::upsert(
            storage,
            NewRfd {
                id,
                rfd_number: payload.number.into(),
                link: payload.link.into(),
                // relevant_components: vec![],
                // milestones: vec![],
            },
        )
        .await?;

        let id = RfdRevisionStore::list(
            storage,
            RfdRevisionFilter::default()
                .rfd(Some(vec![rfd.id]))
                .sha(Some(vec![payload.sha.clone()])),
            &ListPagination::latest(),
        )
        .await?
        .into_iter()
        .next()
        .map(|revision| revision.id)
        .unwrap_or_else(|| Uuid::new_v4());

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
                content: payload.content.raw().to_string(),
                content_format: payload.content_format,
                sha: payload.sha,
                commit_sha: payload.commit_sha,
                committed_at: payload.commit_date,
            },
        )
        .await?;

        Ok(PersistedRfd::new(number, rfd, revision))
    }
}
