use std::fmt::Debug;

pub use async_bb8_diesel::{ConnectionError, PoolError};
use async_trait::async_trait;
pub use diesel::result::Error as DbError;
#[cfg(feature = "mock")]
use mockall::automock;
use thiserror::Error;
use uuid::Uuid;

use crate::{
    permissions::Permission, schema_ext::PdfSource, AccessToken, ApiUser, ApiUserProvider,
    ApiUserToken, Job, NewAccessToken, NewApiUser, NewApiUserProvider, NewApiUserToken, NewJob,
    NewRfd, NewRfdPdf, NewRfdRevision, Rfd, RfdPdf, RfdRevision,
};

pub mod postgres;

#[derive(Debug, Error)]
pub enum StoreError {
    #[error("Connection failure: {0}")]
    Conn(#[from] ConnectionError),
    #[error("Database failure: {0}")]
    Db(#[from] DbError),
    #[error("Connection pool failure: {0}")]
    Pool(#[from] PoolError),
    #[error("Database invariant failed to hold")]
    InvariantFailed(String),
    #[error("Unknown error")]
    Unknown,
}

#[derive(Debug)]
pub struct ListPagination {
    pub offset: i64,
    pub limit: i64,
}

impl Default for ListPagination {
    fn default() -> Self {
        Self {
            offset: 0,
            limit: 10,
        }
    }
}

impl ListPagination {
    pub fn latest() -> Self {
        Self::default().limit(1)
    }

    pub fn offset(mut self, offset: i64) -> Self {
        self.offset = offset;
        self
    }

    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = limit;
        self
    }
}

#[derive(Debug, Default)]
pub struct RfdFilter {
    pub id: Option<Vec<Uuid>>,
    pub rfd_number: Option<Vec<i32>>,
    pub deleted: bool,
}

impl RfdFilter {
    pub fn id(mut self, id: Option<Vec<Uuid>>) -> Self {
        self.id = id;
        self
    }

    pub fn rfd_number(mut self, rfd_number: Option<Vec<i32>>) -> Self {
        self.rfd_number = rfd_number;
        self
    }

    pub fn deleted(mut self, deleted: bool) -> Self {
        self.deleted = deleted;
        self
    }
}

#[cfg_attr(feature = "mock", automock)]
#[async_trait]
pub trait RfdStore {
    async fn get(&self, id: &Uuid, deleted: bool) -> Result<Option<Rfd>, StoreError>;
    async fn list(
        &self,
        filter: RfdFilter,
        pagination: &ListPagination,
    ) -> Result<Vec<Rfd>, StoreError>;
    async fn upsert(&self, new_rfd: NewRfd) -> Result<Rfd, StoreError>;
    async fn delete(&self, id: &Uuid) -> Result<Option<Rfd>, StoreError>;
}

#[derive(Debug, Default)]
pub struct RfdRevisionFilter {
    pub id: Option<Vec<Uuid>>,
    pub rfd: Option<Vec<Uuid>>,
    pub sha: Option<Vec<String>>,
    pub deleted: bool,
}

impl RfdRevisionFilter {
    pub fn id(mut self, id: Option<Vec<Uuid>>) -> Self {
        self.id = id;
        self
    }

    pub fn rfd(mut self, rfd: Option<Vec<Uuid>>) -> Self {
        self.rfd = rfd;
        self
    }

    pub fn sha(mut self, sha: Option<Vec<String>>) -> Self {
        self.sha = sha;
        self
    }

    pub fn deleted(mut self, deleted: bool) -> Self {
        self.deleted = deleted;
        self
    }
}

#[cfg_attr(feature = "mock", automock)]
#[async_trait]
pub trait RfdRevisionStore {
    async fn get(&self, id: &Uuid, deleted: bool) -> Result<Option<RfdRevision>, StoreError>;
    async fn list(
        &self,
        filter: RfdRevisionFilter,
        pagination: &ListPagination,
    ) -> Result<Vec<RfdRevision>, StoreError>;
    async fn upsert(&self, new_revision: NewRfdRevision) -> Result<RfdRevision, StoreError>;
    async fn delete(&self, id: &Uuid) -> Result<Option<RfdRevision>, StoreError>;
}

#[derive(Debug, Default)]
pub struct RfdPdfFilter {
    pub id: Option<Vec<Uuid>>,
    pub rfd_revision: Option<Vec<Uuid>>,
    pub source: Option<Vec<PdfSource>>,
    pub deleted: bool,
}

impl RfdPdfFilter {
    pub fn id(mut self, id: Option<Vec<Uuid>>) -> Self {
        self.id = id;
        self
    }

    pub fn source(mut self, source: Option<Vec<PdfSource>>) -> Self {
        self.source = source;
        self
    }

    pub fn rfd_revision(mut self, rfd_revision: Option<Vec<Uuid>>) -> Self {
        self.rfd_revision = rfd_revision;
        self
    }

    pub fn deleted(mut self, deleted: bool) -> Self {
        self.deleted = deleted;
        self
    }
}

#[cfg_attr(feature = "mock", automock)]
#[async_trait]
pub trait RfdPdfStore {
    async fn get(&self, id: &Uuid, deleted: bool) -> Result<Option<RfdPdf>, StoreError>;
    async fn list(
        &self,
        filter: RfdPdfFilter,
        pagination: &ListPagination,
    ) -> Result<Vec<RfdPdf>, StoreError>;
    async fn upsert(&self, new_revision: NewRfdPdf) -> Result<RfdPdf, StoreError>;
    async fn delete(&self, id: &Uuid) -> Result<Option<RfdPdf>, StoreError>;
}

#[derive(Debug, Default)]
pub struct JobFilter {
    pub id: Option<Vec<i32>>,
    pub sha: Option<Vec<String>>,
    pub processed: Option<bool>,
}

impl JobFilter {
    pub fn id(mut self, id: Option<Vec<i32>>) -> Self {
        self.id = id;
        self
    }

    pub fn sha(mut self, sha: Option<Vec<String>>) -> Self {
        self.sha = sha;
        self
    }

    pub fn processed(mut self, processed: Option<bool>) -> Self {
        self.processed = processed;
        self
    }
}

#[cfg_attr(feature = "mock", automock)]
#[async_trait]
pub trait JobStore {
    async fn get(&self, id: i32) -> Result<Option<Job>, StoreError>;
    async fn list(
        &self,
        filter: JobFilter,
        pagination: &ListPagination,
    ) -> Result<Vec<Job>, StoreError>;
    async fn upsert(&self, new_job: NewJob) -> Result<Job, StoreError>;
    async fn complete(&self, id: i32) -> Result<Option<Job>, StoreError>;
}

#[derive(Debug, Default)]
pub struct ApiUserFilter {
    pub id: Option<Vec<Uuid>>,
    pub email: Option<Vec<String>>,
    pub deleted: bool,
}

#[cfg_attr(feature = "mock", automock)]
#[async_trait]
pub trait ApiUserStore<T: Permission> {
    async fn get(&self, id: &Uuid, deleted: bool) -> Result<Option<ApiUser<T>>, StoreError>;
    async fn list(
        &self,
        filter: ApiUserFilter,
        pagination: &ListPagination,
    ) -> Result<Vec<ApiUser<T>>, StoreError>;
    async fn upsert(&self, api_user: NewApiUser<T>) -> Result<ApiUser<T>, StoreError>;
    async fn delete(&self, id: &Uuid) -> Result<Option<ApiUser<T>>, StoreError>;
}

#[derive(Debug, Default)]
pub struct ApiUserTokenFilter {
    pub api_user_id: Option<Vec<Uuid>>,
    pub expired: bool,
    pub deleted: bool,
}

#[cfg_attr(feature = "mock", automock)]
#[async_trait]
pub trait ApiUserTokenStore<T: Permission> {
    async fn get(&self, id: &Uuid, deleted: bool) -> Result<Option<ApiUserToken<T>>, StoreError>;
    async fn list(
        &self,
        filter: ApiUserTokenFilter,
        pagination: &ListPagination,
    ) -> Result<Vec<ApiUserToken<T>>, StoreError>;
    async fn upsert(
        &self,
        token: NewApiUserToken<T>,
        api_user: &ApiUser<T>,
    ) -> Result<ApiUserToken<T>, StoreError>;
    async fn delete(&self, id: &Uuid) -> Result<Option<ApiUserToken<T>>, StoreError>;
}

#[derive(Debug, Default)]
pub struct ApiUserProviderFilter {
    pub id: Option<Vec<Uuid>>,
    pub api_user_id: Option<Vec<Uuid>>,
    pub provider: Option<Vec<String>>,
    pub provider_id: Option<Vec<String>>,
    pub email: Option<Vec<String>>,
    pub deleted: bool,
}

#[cfg_attr(feature = "mock", automock)]
#[async_trait]
pub trait ApiUserProviderStore {
    async fn get(&self, id: &Uuid, deleted: bool) -> Result<Option<ApiUserProvider>, StoreError>;
    async fn list(
        &self,
        filter: ApiUserProviderFilter,
        pagination: &ListPagination,
    ) -> Result<Vec<ApiUserProvider>, StoreError>;
    async fn upsert(&self, api_user: NewApiUserProvider) -> Result<ApiUserProvider, StoreError>;
    async fn delete(&self, id: &Uuid) -> Result<Option<ApiUserProvider>, StoreError>;
}

#[derive(Debug, Default)]
pub struct AccessTokenFilter {
    pub id: Option<Vec<Uuid>>,
    pub api_user_id: Option<Vec<Uuid>>,
    pub revoked: bool,
}

#[cfg_attr(feature = "mock", automock)]
#[async_trait]
pub trait AccessTokenStore {
    async fn get(&self, id: &Uuid, revoked: bool) -> Result<Option<AccessToken>, StoreError>;
    async fn list(
        &self,
        filter: AccessTokenFilter,
        pagination: &ListPagination,
    ) -> Result<Vec<AccessToken>, StoreError>;
    async fn upsert(&self, token: NewAccessToken) -> Result<AccessToken, StoreError>;
}
