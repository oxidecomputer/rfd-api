use chrono::{DateTime, Utc};
use db::{JobModel, RfdModel, RfdPdfModel, RfdRevisionModel};
use partial_struct::partial;
use permissions::Permissions;
use schema_ext::{ContentFormat, PdfSource};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod db;
pub mod permissions;
pub mod schema;
pub mod schema_ext;
pub mod storage;
mod utils;

#[partial(NewRfd)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct Rfd {
    pub id: Uuid,
    pub rfd_number: i32,
    pub link: Option<String>,
    // pub relevant_components: Vec<Option<String>>,
    // pub milestones: Vec<Option<String>>,
    #[partial(NewRfd(skip))]
    pub created_at: DateTime<Utc>,
    #[partial(NewRfd(skip))]
    pub updated_at: DateTime<Utc>,
    #[partial(NewRfd(skip))]
    pub deleted_at: Option<DateTime<Utc>>,
}

impl From<RfdModel> for Rfd {
    fn from(value: RfdModel) -> Self {
        Self {
            id: value.id,
            rfd_number: value.rfd_number,
            link: value.link,
            // relevant_components: value.relevant_components,
            // milestones: value.milestones,
            created_at: value.created_at,
            updated_at: value.updated_at,
            deleted_at: value.deleted_at,
        }
    }
}

#[partial(NewRfdRevision)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct RfdRevision {
    pub id: Uuid,
    pub rfd_id: Uuid,
    pub title: String,
    pub state: Option<String>,
    pub discussion: Option<String>,
    pub authors: Option<String>,
    pub content: String,
    pub content_format: ContentFormat,
    pub sha: String,
    pub commit_sha: String,
    pub committed_at: DateTime<Utc>,
    #[partial(NewRfdRevision(skip))]
    pub created_at: DateTime<Utc>,
    #[partial(NewRfdRevision(skip))]
    pub updated_at: DateTime<Utc>,
    #[partial(NewRfdRevision(skip))]
    pub deleted_at: Option<DateTime<Utc>>,
}

impl From<RfdRevisionModel> for RfdRevision {
    fn from(value: RfdRevisionModel) -> Self {
        Self {
            id: value.id,
            rfd_id: value.rfd_id,
            title: value.title,
            state: value.state,
            discussion: value.discussion,
            authors: value.authors,
            content: value.content,
            content_format: value.content_format,
            sha: value.sha,
            commit_sha: value.commit_sha,
            committed_at: value.committed_at,
            created_at: value.created_at,
            updated_at: value.updated_at,
            deleted_at: value.deleted_at,
        }
    }
}

#[partial(NewRfdPdf)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct RfdPdf {
    pub id: Uuid,
    pub rfd_revision_id: Uuid,
    pub source: PdfSource,
    pub link: String,
    #[partial(NewRfdPdf(skip))]
    pub created_at: DateTime<Utc>,
    #[partial(NewRfdPdf(skip))]
    pub updated_at: DateTime<Utc>,
    #[partial(NewRfdPdf(skip))]
    pub deleted_at: Option<DateTime<Utc>>,
}

impl From<RfdPdfModel> for RfdPdf {
    fn from(value: RfdPdfModel) -> Self {
        Self {
            id: value.id,
            rfd_revision_id: value.rfd_revision_id,
            source: value.source,
            link: value.link,
            created_at: value.created_at,
            updated_at: value.updated_at,
            deleted_at: value.deleted_at,
        }
    }
}

#[partial(NewJob)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct Job {
    #[partial(NewJob(skip))]
    pub id: i32,
    pub owner: String,
    pub repository: String,
    pub branch: String,
    pub sha: String,
    pub rfd: i32,
    pub webhook_delivery_id: Option<Uuid>,
    pub committed_at: DateTime<Utc>,
    #[partial(NewJob(skip))]
    pub processed: bool,
    #[partial(NewJob(skip))]
    pub created_at: DateTime<Utc>,
}

impl From<JobModel> for Job {
    fn from(value: JobModel) -> Self {
        Self {
            id: value.id,
            owner: value.owner,
            repository: value.repository,
            branch: value.branch,
            sha: value.sha,
            rfd: value.rfd,
            webhook_delivery_id: value.webhook_delivery_id,
            committed_at: value.committed_at,
            processed: value.processed,
            created_at: value.created_at,
        }
    }
}

#[partial(NewApiUser)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct ApiUser<T> {
    pub id: Uuid,
    pub permissions: Permissions<T>,
    #[partial(NewApiUser(skip))]
    pub created_at: DateTime<Utc>,
    #[partial(NewApiUser(skip))]
    pub updated_at: DateTime<Utc>,
    #[partial(NewApiUser(skip))]
    pub deleted_at: Option<DateTime<Utc>>,
}

#[partial(NewApiUserProvider)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct ApiUserProvider {
    pub id: Uuid,
    pub api_user_id: Uuid,
    pub provider: String,
    pub provider_id: String,
    pub emails: Vec<String>,
    #[partial(NewApiUserProvider(skip))]
    pub created_at: DateTime<Utc>,
    #[partial(NewApiUserProvider(skip))]
    pub updated_at: DateTime<Utc>,
    #[partial(NewApiUserProvider(skip))]
    pub deleted_at: Option<DateTime<Utc>>,
}

#[partial(NewApiUserToken)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct ApiUserToken<T> {
    pub id: Uuid,
    pub api_user_id: Uuid,
    pub token: String,
    pub permissions: Permissions<T>,
    pub expires_at: DateTime<Utc>,
    #[partial(NewApiUserToken(skip))]
    pub created_at: DateTime<Utc>,
    #[partial(NewApiUserToken(skip))]
    pub updated_at: DateTime<Utc>,
    #[partial(NewApiUserToken(skip))]
    pub deleted_at: Option<DateTime<Utc>>,
}

#[partial(NewAccessToken)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct AccessToken {
    pub id: Uuid,
    pub api_user_id: Uuid,
    pub revoked_at: Option<DateTime<Utc>>,
    #[partial(NewAccessToken(skip))]
    pub created_at: DateTime<Utc>,
    #[partial(NewAccessToken(skip))]
    pub updated_at: DateTime<Utc>,
}
