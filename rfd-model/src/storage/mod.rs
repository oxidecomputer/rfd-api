// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

pub use async_bb8_diesel::{ConnectionError, PoolError};
use async_trait::async_trait;
pub use diesel::result::Error as DbError;
#[cfg(feature = "mock")]
use mockall::automock;
use newtype_uuid::TypedUuid;
use std::fmt::Debug;
use v_model::storage::{ListPagination, StoreError};

use crate::{
    schema_ext::PdfSource, Job, NewJob, NewRfd, NewRfdPdf, NewRfdRevision, Rfd, RfdId, RfdPdf,
    RfdPdfId, RfdRevision, RfdRevisionId, RfdRevisionMeta,
};

pub mod postgres;

#[derive(Debug, Default)]
pub struct RfdFilter {
    pub id: Option<Vec<TypedUuid<RfdId>>>,
    pub rfd_number: Option<Vec<i32>>,
    pub public: Option<bool>,
    pub deleted: bool,
}

impl RfdFilter {
    pub fn id(mut self, id: Option<Vec<TypedUuid<RfdId>>>) -> Self {
        self.id = id;
        self
    }

    pub fn rfd_number(mut self, rfd_number: Option<Vec<i32>>) -> Self {
        self.rfd_number = rfd_number;
        self
    }

    pub fn public(mut self, public: Option<bool>) -> Self {
        self.public = public;
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
    async fn get(&self, id: &TypedUuid<RfdId>, deleted: bool) -> Result<Option<Rfd>, StoreError>;
    async fn list(
        &self,
        filter: RfdFilter,
        pagination: &ListPagination,
    ) -> Result<Vec<Rfd>, StoreError>;
    async fn upsert(&self, new_rfd: NewRfd) -> Result<Rfd, StoreError>;
    async fn delete(&self, id: &TypedUuid<RfdId>) -> Result<Option<Rfd>, StoreError>;
}

// TODO: Make the revision store generic over a revision type. We want to be able to have a metadata
// only version of the revision model so that we do not need to always load content from the db

#[derive(Debug, Default)]
pub struct RfdRevisionFilter {
    pub id: Option<Vec<TypedUuid<RfdRevisionId>>>,
    pub rfd: Option<Vec<TypedUuid<RfdId>>>,
    pub sha: Option<Vec<String>>,
    pub deleted: bool,
}

impl RfdRevisionFilter {
    pub fn id(mut self, id: Option<Vec<TypedUuid<RfdRevisionId>>>) -> Self {
        self.id = id;
        self
    }

    pub fn rfd(mut self, rfd: Option<Vec<TypedUuid<RfdId>>>) -> Self {
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

#[derive(Debug, Default)]
pub enum RfdRevisionGroupBy {
    Id,
    #[default]
    None,
}

#[cfg_attr(feature = "mock", automock)]
#[async_trait]
pub trait RfdRevisionStore {
    async fn get(
        &self,
        id: &TypedUuid<RfdRevisionId>,
        deleted: bool,
    ) -> Result<Option<RfdRevision>, StoreError>;
    async fn list(
        &self,
        filter: RfdRevisionFilter,
        pagination: &ListPagination,
    ) -> Result<Vec<RfdRevision>, StoreError>;
    async fn list_unique_rfd(
        &self,
        filter: RfdRevisionFilter,
        pagination: &ListPagination,
    ) -> Result<Vec<RfdRevision>, StoreError>;
    async fn upsert(&self, new_revision: NewRfdRevision) -> Result<RfdRevision, StoreError>;
    async fn delete(
        &self,
        id: &TypedUuid<RfdRevisionId>,
    ) -> Result<Option<RfdRevision>, StoreError>;
}

#[cfg_attr(feature = "mock", automock)]
#[async_trait]
pub trait RfdRevisionMetaStore {
    async fn get(
        &self,
        id: &TypedUuid<RfdRevisionId>,
        deleted: bool,
    ) -> Result<Option<RfdRevisionMeta>, StoreError>;
    async fn list(
        &self,
        filter: RfdRevisionFilter,
        pagination: &ListPagination,
    ) -> Result<Vec<RfdRevisionMeta>, StoreError>;
    async fn list_unique_rfd(
        &self,
        filter: RfdRevisionFilter,
        pagination: &ListPagination,
    ) -> Result<Vec<RfdRevisionMeta>, StoreError>;
}

#[derive(Debug, Default)]
pub struct RfdPdfFilter {
    pub id: Option<Vec<TypedUuid<RfdPdfId>>>,
    pub rfd_revision: Option<Vec<TypedUuid<RfdRevisionId>>>,
    pub source: Option<Vec<PdfSource>>,
    pub deleted: bool,
    pub rfd: Option<Vec<TypedUuid<RfdId>>>,
    pub external_id: Option<Vec<String>>,
}

impl RfdPdfFilter {
    pub fn id(mut self, id: Option<Vec<TypedUuid<RfdPdfId>>>) -> Self {
        self.id = id;
        self
    }

    pub fn source(mut self, source: Option<Vec<PdfSource>>) -> Self {
        self.source = source;
        self
    }

    pub fn rfd_revision(mut self, rfd_revision: Option<Vec<TypedUuid<RfdRevisionId>>>) -> Self {
        self.rfd_revision = rfd_revision;
        self
    }

    pub fn deleted(mut self, deleted: bool) -> Self {
        self.deleted = deleted;
        self
    }

    pub fn rfd(mut self, rfd: Option<Vec<TypedUuid<RfdId>>>) -> Self {
        self.rfd = rfd;
        self
    }

    pub fn external_id(mut self, external_id: Option<Vec<String>>) -> Self {
        self.external_id = external_id;
        self
    }
}

#[cfg_attr(feature = "mock", automock)]
#[async_trait]
pub trait RfdPdfStore {
    async fn get(
        &self,
        id: &TypedUuid<RfdPdfId>,
        deleted: bool,
    ) -> Result<Option<RfdPdf>, StoreError>;
    async fn list(
        &self,
        filter: RfdPdfFilter,
        pagination: &ListPagination,
    ) -> Result<Vec<RfdPdf>, StoreError>;
    async fn upsert(&self, new_revision: NewRfdPdf) -> Result<RfdPdf, StoreError>;
    async fn delete(&self, id: &TypedUuid<RfdPdfId>) -> Result<Option<RfdPdf>, StoreError>;
}

#[derive(Debug, Default)]
pub struct JobFilter {
    pub id: Option<Vec<i32>>,
    pub sha: Option<Vec<String>>,
    pub processed: Option<bool>,
    pub started: Option<bool>,
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

    pub fn started(mut self, started: Option<bool>) -> Self {
        self.started = started;
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
    async fn start(&self, id: i32) -> Result<Option<Job>, StoreError>;
    async fn complete(&self, id: i32) -> Result<Option<Job>, StoreError>;
}
