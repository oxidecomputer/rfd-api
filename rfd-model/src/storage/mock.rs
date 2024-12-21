use async_trait::async_trait;
use newtype_uuid::TypedUuid;
use std::sync::Arc;
use v_model::storage::StoreError;

use crate::{
    Job, NewJob, NewRfd, NewRfdPdf, NewRfdRevision, Rfd, RfdId, RfdMeta, RfdPdf, RfdPdfId,
    RfdRevision, RfdRevisionId, RfdRevisionMeta,
};

use super::{
    JobFilter, JobStore, ListPagination, MockJobStore, MockRfdMetaStore, MockRfdPdfStore,
    MockRfdRevisionMetaStore, MockRfdRevisionStore, MockRfdStore, RfdFilter, RfdMetaStore,
    RfdPdfFilter, RfdPdfStore, RfdRevisionFilter, RfdRevisionMetaStore, RfdRevisionStore, RfdStore,
};

pub struct MockStorage {
    pub rfd_store: Option<Arc<MockRfdStore>>,
    pub rfd_meta_store: Option<Arc<MockRfdMetaStore>>,
    pub rfd_revision_store: Option<Arc<MockRfdRevisionStore>>,
    pub rfd_revision_meta_store: Option<Arc<MockRfdRevisionMetaStore>>,
    pub rfd_pdf_store: Option<Arc<MockRfdPdfStore>>,
    pub job_store: Option<Arc<MockJobStore>>,
}

impl MockStorage {
    pub fn new() -> Self {
        Self {
            rfd_store: None,
            rfd_meta_store: None,
            rfd_revision_store: None,
            rfd_revision_meta_store: None,
            rfd_pdf_store: None,
            job_store: None,
        }
    }
}

#[async_trait]
impl RfdStore for MockStorage {
    async fn get(
        &self,
        id: &TypedUuid<RfdId>,
        revision: Option<TypedUuid<RfdRevisionId>>,
        deleted: bool,
    ) -> Result<Option<Rfd>, StoreError> {
        self.rfd_store
            .as_ref()
            .unwrap()
            .get(id, revision, deleted)
            .await
    }

    async fn list(
        &self,
        filter: RfdFilter,
        pagination: &ListPagination,
    ) -> Result<Vec<Rfd>, StoreError> {
        self.rfd_store
            .as_ref()
            .unwrap()
            .list(filter, pagination)
            .await
    }

    async fn upsert(&self, new_rfd: NewRfd) -> Result<Rfd, StoreError> {
        self.rfd_store.as_ref().unwrap().upsert(new_rfd).await
    }

    async fn delete(&self, id: &TypedUuid<RfdId>) -> Result<Option<Rfd>, StoreError> {
        self.rfd_store.as_ref().unwrap().delete(id).await
    }
}

#[async_trait]
impl RfdMetaStore for MockStorage {
    async fn get(
        &self,
        id: TypedUuid<RfdId>,
        revision: Option<TypedUuid<RfdRevisionId>>,
        deleted: bool,
    ) -> Result<Option<RfdMeta>, StoreError> {
        self.rfd_meta_store
            .as_ref()
            .unwrap()
            .get(id, revision, deleted)
            .await
    }

    async fn list(
        &self,
        filter: RfdFilter,
        pagination: &ListPagination,
    ) -> Result<Vec<RfdMeta>, StoreError> {
        self.rfd_meta_store
            .as_ref()
            .unwrap()
            .list(filter, pagination)
            .await
    }
}

#[async_trait]
impl RfdRevisionStore for MockStorage {
    async fn get(
        &self,
        id: &TypedUuid<RfdRevisionId>,
        deleted: bool,
    ) -> Result<Option<RfdRevision>, StoreError> {
        self.rfd_revision_store
            .as_ref()
            .unwrap()
            .get(id, deleted)
            .await
    }

    async fn list(
        &self,
        filter: RfdRevisionFilter,
        pagination: &ListPagination,
    ) -> Result<Vec<RfdRevision>, StoreError> {
        self.rfd_revision_store
            .as_ref()
            .unwrap()
            .list(filter, pagination)
            .await
    }

    async fn list_unique_rfd(
        &self,
        filter: RfdRevisionFilter,
        pagination: &ListPagination,
    ) -> Result<Vec<RfdRevision>, StoreError> {
        self.rfd_revision_store
            .as_ref()
            .unwrap()
            .list(filter, pagination)
            .await
    }

    async fn upsert(&self, new_revision: NewRfdRevision) -> Result<RfdRevision, StoreError> {
        self.rfd_revision_store
            .as_ref()
            .unwrap()
            .upsert(new_revision)
            .await
    }

    async fn delete(
        &self,
        id: &TypedUuid<RfdRevisionId>,
    ) -> Result<Option<RfdRevision>, StoreError> {
        self.rfd_revision_store.as_ref().unwrap().delete(id).await
    }
}

#[async_trait]
impl RfdRevisionMetaStore for MockStorage {
    async fn get(
        &self,
        id: &TypedUuid<RfdRevisionId>,
        deleted: bool,
    ) -> Result<Option<RfdRevisionMeta>, StoreError> {
        self.rfd_revision_meta_store
            .as_ref()
            .unwrap()
            .get(id, deleted)
            .await
    }

    async fn list(
        &self,
        filter: RfdRevisionFilter,
        pagination: &ListPagination,
    ) -> Result<Vec<RfdRevisionMeta>, StoreError> {
        self.rfd_revision_meta_store
            .as_ref()
            .unwrap()
            .list(filter, pagination)
            .await
    }

    async fn list_unique_rfd(
        &self,
        filter: RfdRevisionFilter,
        pagination: &ListPagination,
    ) -> Result<Vec<RfdRevisionMeta>, StoreError> {
        self.rfd_revision_meta_store
            .as_ref()
            .unwrap()
            .list(filter, pagination)
            .await
    }
}

#[async_trait]
impl RfdPdfStore for MockStorage {
    async fn get(
        &self,
        id: &TypedUuid<RfdPdfId>,
        deleted: bool,
    ) -> Result<Option<RfdPdf>, StoreError> {
        self.rfd_pdf_store.as_ref().unwrap().get(id, deleted).await
    }

    async fn list(
        &self,
        filter: RfdPdfFilter,
        pagination: &ListPagination,
    ) -> Result<Vec<RfdPdf>, StoreError> {
        self.rfd_pdf_store
            .as_ref()
            .unwrap()
            .list(filter, pagination)
            .await
    }

    async fn upsert(&self, new_pdf: NewRfdPdf) -> Result<RfdPdf, StoreError> {
        self.rfd_pdf_store.as_ref().unwrap().upsert(new_pdf).await
    }

    async fn delete(&self, id: &TypedUuid<RfdPdfId>) -> Result<Option<RfdPdf>, StoreError> {
        self.rfd_pdf_store.as_ref().unwrap().delete(id).await
    }
}

#[async_trait]
impl JobStore for MockStorage {
    async fn get(&self, id: i32) -> Result<Option<Job>, StoreError> {
        self.job_store.as_ref().unwrap().get(id).await
    }

    async fn list(
        &self,
        filter: JobFilter,
        pagination: &ListPagination,
    ) -> Result<Vec<Job>, StoreError> {
        self.job_store
            .as_ref()
            .unwrap()
            .list(filter, pagination)
            .await
    }

    async fn upsert(&self, new_job: NewJob) -> Result<Job, StoreError> {
        self.job_store.as_ref().unwrap().upsert(new_job).await
    }

    async fn start(&self, id: i32) -> Result<Option<Job>, StoreError> {
        self.job_store.as_ref().unwrap().start(id).await
    }

    async fn complete(&self, id: i32) -> Result<Option<Job>, StoreError> {
        self.job_store.as_ref().unwrap().complete(id).await
    }
}
