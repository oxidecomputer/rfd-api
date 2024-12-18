// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use async_bb8_diesel::{AsyncRunQueryDsl, ConnectionError, ConnectionManager};
use async_trait::async_trait;
use bb8::Pool;
use chrono::Utc;
use diesel::{
    debug_query, insert_into,
    pg::PgConnection,
    query_dsl::QueryDsl,
    update,
    upsert::{excluded, on_constraint},
    ExpressionMethods,
};
use newtype_uuid::{GenericUuid, TypedUuid};
use std::time::Duration;
use thiserror::Error;
use uuid::Uuid;

use crate::{
    db::{JobModel, RfdModel, RfdPdfModel, RfdRevisionMetaModel, RfdRevisionModel},
    schema::{job, rfd, rfd_pdf, rfd_revision},
    schema_ext::Visibility,
    storage::StoreError,
    Job, NewJob, NewRfd, NewRfdPdf, NewRfdRevision, Rfd, RfdId, RfdPdf, RfdPdfId, RfdRevision,
    RfdRevisionId, RfdRevisionMeta,
};

use super::{
    JobFilter, JobStore, ListPagination, RfdFilter, RfdPdfFilter, RfdPdfStore, RfdRevisionFilter,
    RfdRevisionMetaStore, RfdRevisionStore, RfdStore,
};

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

pub struct PostgresStore {
    pool: DbPool,
}

#[derive(Debug, Error)]
pub enum PostgresError {
    #[error("Failed to connect to database")]
    Connection(ConnectionError),
}

impl From<ConnectionError> for PostgresError {
    fn from(error: ConnectionError) -> Self {
        PostgresError::Connection(error)
    }
}

impl PostgresStore {
    pub async fn new(url: &str) -> Result<Self, PostgresError> {
        let manager = ConnectionManager::<PgConnection>::new(url);

        Ok(Self {
            pool: Pool::builder()
                .connection_timeout(Duration::from_secs(30))
                .build(manager)
                .await?,
        })
    }
}

#[async_trait]
impl RfdStore for PostgresStore {
    async fn get(&self, id: &TypedUuid<RfdId>, deleted: bool) -> Result<Option<Rfd>, StoreError> {
        let rfd = RfdStore::list(
            self,
            RfdFilter::default().id(Some(vec![*id])).deleted(deleted),
            &ListPagination::default().limit(1),
        )
        .await?;
        Ok(rfd.into_iter().nth(0))
    }

    async fn list(
        &self,
        filter: RfdFilter,
        pagination: &ListPagination,
    ) -> Result<Vec<Rfd>, StoreError> {
        let mut query = rfd::dsl::rfd.into_boxed();

        tracing::trace!(?filter, "Lookup RFDs");

        let RfdFilter {
            id,
            rfd_number,
            public,
            deleted,
        } = filter;

        if let Some(id) = id {
            query =
                query.filter(rfd::id.eq_any(id.into_iter().map(GenericUuid::into_untyped_uuid)));
        }

        if let Some(rfd_number) = rfd_number {
            query = query.filter(rfd::rfd_number.eq_any(rfd_number));
        }

        if let Some(public) = public {
            query = query.filter(
                rfd::visibility.eq(public
                    .then(|| Visibility::Public)
                    .unwrap_or(Visibility::Private)),
            );
        }

        if !deleted {
            query = query.filter(rfd::deleted_at.is_null());
        }

        let results = query
            .offset(pagination.offset)
            .limit(pagination.limit)
            .order(rfd::rfd_number.desc())
            .get_results_async::<RfdModel>(&*self.pool.get().await?)
            .await?;

        tracing::trace!(count = ?results.len(), "Found RFDs");

        Ok(results.into_iter().map(|rfd| rfd.into()).collect())
    }

    async fn upsert(&self, new_rfd: NewRfd) -> Result<Rfd, StoreError> {
        let rfd: RfdModel = insert_into(rfd::dsl::rfd)
            .values((
                rfd::id.eq(new_rfd.id.into_untyped_uuid()),
                rfd::rfd_number.eq(new_rfd.rfd_number.clone()),
                rfd::link.eq(new_rfd.link.clone()),
                rfd::visibility.eq(new_rfd.visibility.clone()),
            ))
            .on_conflict(rfd::id)
            .do_update()
            .set((
                rfd::rfd_number.eq(excluded(rfd::rfd_number)),
                rfd::link.eq(excluded(rfd::link)),
                rfd::updated_at.eq(Utc::now()),
                rfd::visibility.eq(excluded(rfd::visibility)),
            ))
            .get_result_async(&*self.pool.get().await?)
            .await?;

        Ok(rfd.into())
    }

    async fn delete(&self, id: &TypedUuid<RfdId>) -> Result<Option<Rfd>, StoreError> {
        let _ = update(rfd::dsl::rfd)
            .filter(rfd::id.eq(id.into_untyped_uuid()))
            .set(rfd::deleted_at.eq(Utc::now()))
            .execute_async(&*self.pool.get().await?)
            .await?;

        RfdStore::get(self, id, true).await
    }
}

#[async_trait]
impl RfdRevisionStore for PostgresStore {
    async fn get(
        &self,
        id: &TypedUuid<RfdRevisionId>,
        deleted: bool,
    ) -> Result<Option<RfdRevision>, StoreError> {
        let user = RfdRevisionStore::list(
            self,
            RfdRevisionFilter::default()
                .id(Some(vec![*id]))
                .deleted(deleted),
            &ListPagination::default().limit(1),
        )
        .await?;
        Ok(user.into_iter().nth(0))
    }

    async fn list(
        &self,
        filter: RfdRevisionFilter,
        pagination: &ListPagination,
    ) -> Result<Vec<RfdRevision>, StoreError> {
        let mut query = rfd_revision::dsl::rfd_revision.into_boxed();

        tracing::trace!(?filter, "Lookup RFD revisions");

        let RfdRevisionFilter {
            id,
            rfd,
            sha,
            deleted,
        } = filter;

        if let Some(id) = id {
            query = query.filter(
                rfd_revision::id.eq_any(id.into_iter().map(GenericUuid::into_untyped_uuid)),
            );
        }

        if let Some(rfd) = rfd {
            query = query.filter(
                rfd_revision::rfd_id.eq_any(rfd.into_iter().map(GenericUuid::into_untyped_uuid)),
            );
        }

        if let Some(sha) = sha {
            query = query.filter(rfd_revision::sha.eq_any(sha));
        }

        if !deleted {
            query = query.filter(rfd_revision::deleted_at.is_null());
        }

        let query = query
            .offset(pagination.offset)
            .limit(pagination.limit)
            .order(rfd_revision::committed_at.desc());

        tracing::info!(query = ?debug_query(&query), "Run list rfds");

        let results = query
            .get_results_async::<RfdRevisionModel>(&*self.pool.get().await?)
            .await?;

        Ok(results
            .into_iter()
            .map(|revision| revision.into())
            .collect())
    }

    // TODO: Refactor into a group by arg in list. Diesel types here are a pain
    async fn list_unique_rfd(
        &self,
        filter: RfdRevisionFilter,
        pagination: &ListPagination,
    ) -> Result<Vec<RfdRevision>, StoreError> {
        let mut query = rfd_revision::dsl::rfd_revision
            .distinct_on(rfd_revision::rfd_id)
            .into_boxed();

        tracing::trace!(rfd_ids = ?filter.rfd.as_ref().map(|list| list.len()), "Lookup unique RFD revisions");

        let RfdRevisionFilter {
            id,
            rfd,
            sha,
            deleted,
        } = filter;

        if let Some(id) = id {
            query = query.filter(
                rfd_revision::id.eq_any(id.into_iter().map(GenericUuid::into_untyped_uuid)),
            );
        }

        if let Some(rfd) = rfd {
            query = query.filter(
                rfd_revision::rfd_id.eq_any(rfd.into_iter().map(GenericUuid::into_untyped_uuid)),
            );
        }

        if let Some(sha) = sha {
            query = query.filter(rfd_revision::sha.eq_any(sha));
        }

        if !deleted {
            query = query.filter(rfd_revision::deleted_at.is_null());
        }

        let query = query
            .offset(pagination.offset)
            .limit(pagination.limit)
            .order((
                rfd_revision::rfd_id.asc(),
                rfd_revision::committed_at.desc(),
            ));

        tracing::info!(query = ?debug_query(&query), "Run list unique rfds");

        let results = query
            .get_results_async::<RfdRevisionModel>(&*self.pool.get().await?)
            .await?;

        tracing::trace!(count = ?results.len(), "Found unique RFD revisions");

        Ok(results
            .into_iter()
            .map(|revision| revision.into())
            .collect())
    }

    async fn upsert(&self, new_revision: NewRfdRevision) -> Result<RfdRevision, StoreError> {
        let rfd: RfdRevisionModel = insert_into(rfd_revision::dsl::rfd_revision)
            .values((
                rfd_revision::id.eq(new_revision.id.into_untyped_uuid()),
                rfd_revision::rfd_id.eq(new_revision.rfd_id.into_untyped_uuid()),
                rfd_revision::title.eq(new_revision.title.clone()),
                rfd_revision::state.eq(new_revision.state.clone()),
                rfd_revision::discussion.eq(new_revision.discussion.clone()),
                rfd_revision::authors.eq(new_revision.authors.clone()),
                rfd_revision::labels.eq(new_revision.labels.clone()),
                rfd_revision::content.eq(new_revision.content.clone()),
                rfd_revision::content_format.eq(new_revision.content_format.clone()),
                rfd_revision::sha.eq(String::from(new_revision.sha)),
                rfd_revision::commit_sha.eq(String::from(new_revision.commit)),
                rfd_revision::committed_at.eq(new_revision.committed_at.clone()),
            ))
            .on_conflict(rfd_revision::id)
            .do_update()
            .set((
                rfd_revision::rfd_id.eq(excluded(rfd_revision::rfd_id)),
                rfd_revision::title.eq(excluded(rfd_revision::title)),
                rfd_revision::state.eq(excluded(rfd_revision::state)),
                rfd_revision::discussion.eq(excluded(rfd_revision::discussion)),
                rfd_revision::authors.eq(excluded(rfd_revision::authors)),
                rfd_revision::labels.eq(excluded(rfd_revision::labels)),
                rfd_revision::content.eq(excluded(rfd_revision::content)),
                rfd_revision::content_format.eq(excluded(rfd_revision::content_format)),
                rfd_revision::sha.eq(excluded(rfd_revision::sha)),
                rfd_revision::commit_sha.eq(rfd_revision::commit_sha),
                rfd_revision::committed_at.eq(excluded(rfd_revision::committed_at)),
                rfd_revision::updated_at.eq(Utc::now()),
            ))
            .get_result_async(&*self.pool.get().await?)
            .await?;

        Ok(rfd.into())
    }

    async fn delete(
        &self,
        id: &TypedUuid<RfdRevisionId>,
    ) -> Result<Option<RfdRevision>, StoreError> {
        let _ = update(rfd_revision::dsl::rfd_revision)
            .filter(rfd_revision::id.eq(id.into_untyped_uuid()))
            .set(rfd_revision::deleted_at.eq(Utc::now()))
            .execute_async(&*self.pool.get().await?)
            .await?;

        RfdRevisionStore::get(self, id, true).await
    }
}

#[async_trait]
impl RfdRevisionMetaStore for PostgresStore {
    async fn get(
        &self,
        id: &TypedUuid<RfdRevisionId>,
        deleted: bool,
    ) -> Result<Option<RfdRevisionMeta>, StoreError> {
        let user = RfdRevisionMetaStore::list(
            self,
            RfdRevisionFilter::default()
                .id(Some(vec![*id]))
                .deleted(deleted),
            &ListPagination::default().limit(1),
        )
        .await?;
        Ok(user.into_iter().nth(0))
    }

    async fn list(
        &self,
        filter: RfdRevisionFilter,
        pagination: &ListPagination,
    ) -> Result<Vec<RfdRevisionMeta>, StoreError> {
        let mut query = rfd_revision::dsl::rfd_revision
            .select((
                rfd_revision::id,
                rfd_revision::rfd_id,
                rfd_revision::title,
                rfd_revision::state,
                rfd_revision::discussion,
                rfd_revision::authors,
                rfd_revision::content_format,
                rfd_revision::sha,
                rfd_revision::commit_sha,
                rfd_revision::committed_at,
                rfd_revision::created_at,
                rfd_revision::updated_at,
                rfd_revision::deleted_at,
                rfd_revision::labels,
            ))
            .into_boxed();

        tracing::trace!(?filter, "Lookup RFD revision metadata");

        let RfdRevisionFilter {
            id,
            rfd,
            sha,
            deleted,
        } = filter;

        if let Some(id) = id {
            query = query.filter(
                rfd_revision::id.eq_any(id.into_iter().map(GenericUuid::into_untyped_uuid)),
            );
        }

        if let Some(rfd) = rfd {
            query = query.filter(
                rfd_revision::rfd_id.eq_any(rfd.into_iter().map(GenericUuid::into_untyped_uuid)),
            );
        }

        if let Some(sha) = sha {
            query = query.filter(rfd_revision::sha.eq_any(sha));
        }

        if !deleted {
            query = query.filter(rfd_revision::deleted_at.is_null());
        }

        let query = query
            .offset(pagination.offset)
            .limit(pagination.limit)
            .order(rfd_revision::committed_at.desc());

        tracing::info!(query = ?debug_query(&query), "Run list rfd metadata");

        let results = query
            .get_results_async::<RfdRevisionMetaModel>(&*self.pool.get().await?)
            .await?;

        tracing::trace!(count = ?results.len(), "Found RFD revision metadata");

        Ok(results
            .into_iter()
            .map(|revision| revision.into())
            .collect())
    }

    // TODO: Refactor into a group by arg in list. Diesel types here are a pain
    async fn list_unique_rfd(
        &self,
        filter: RfdRevisionFilter,
        pagination: &ListPagination,
    ) -> Result<Vec<RfdRevisionMeta>, StoreError> {
        let mut query = rfd_revision::dsl::rfd_revision
            .select((
                rfd_revision::id,
                rfd_revision::rfd_id,
                rfd_revision::title,
                rfd_revision::state,
                rfd_revision::discussion,
                rfd_revision::authors,
                rfd_revision::content_format,
                rfd_revision::sha,
                rfd_revision::commit_sha,
                rfd_revision::committed_at,
                rfd_revision::created_at,
                rfd_revision::updated_at,
                rfd_revision::deleted_at,
                rfd_revision::labels,
            ))
            .distinct_on(rfd_revision::rfd_id)
            .into_boxed();

        tracing::trace!(rfd_ids = ?filter.rfd.as_ref().map(|list| list.len()), "Lookup unique RFD revision metadata");

        let RfdRevisionFilter {
            id,
            rfd,
            sha,
            deleted,
        } = filter;

        if let Some(id) = id {
            query = query.filter(
                rfd_revision::id.eq_any(id.into_iter().map(GenericUuid::into_untyped_uuid)),
            );
        }

        if let Some(rfd) = rfd {
            query = query.filter(
                rfd_revision::rfd_id.eq_any(rfd.into_iter().map(GenericUuid::into_untyped_uuid)),
            );
        }

        if let Some(sha) = sha {
            query = query.filter(rfd_revision::sha.eq_any(sha));
        }

        if !deleted {
            query = query.filter(rfd_revision::deleted_at.is_null());
        }

        let query = query
            .offset(pagination.offset)
            .limit(pagination.limit)
            .order((
                rfd_revision::rfd_id.asc(),
                rfd_revision::committed_at.desc(),
            ));

        tracing::info!(query = ?debug_query(&query), "Run list unique rfd metadata");

        let results = query
            .get_results_async::<RfdRevisionMetaModel>(&*self.pool.get().await?)
            .await?;

        tracing::trace!(count = ?results.len(), "Found unique RFD revision metadata");

        Ok(results
            .into_iter()
            .map(|revision| revision.into())
            .collect())
    }
}

#[async_trait]
impl RfdPdfStore for PostgresStore {
    async fn get(
        &self,
        id: &TypedUuid<RfdPdfId>,
        deleted: bool,
    ) -> Result<Option<RfdPdf>, StoreError> {
        let user = RfdPdfStore::list(
            self,
            RfdPdfFilter::default().id(Some(vec![*id])).deleted(deleted),
            &ListPagination::default().limit(1),
        )
        .await?;
        Ok(user.into_iter().nth(0))
    }

    async fn list(
        &self,
        filter: super::RfdPdfFilter,
        pagination: &ListPagination,
    ) -> Result<Vec<RfdPdf>, StoreError> {
        let mut query = rfd_pdf::dsl::rfd_pdf.into_boxed();

        tracing::trace!(?filter, "Lookup RFD pdfs");

        let RfdPdfFilter {
            id,
            rfd_revision,
            source,
            deleted,
            rfd,
            external_id,
        } = filter;

        if let Some(id) = id {
            query = query
                .filter(rfd_pdf::id.eq_any(id.into_iter().map(GenericUuid::into_untyped_uuid)));
        }

        if let Some(rfd_revision) = rfd_revision {
            query = query.filter(
                rfd_pdf::rfd_revision_id
                    .eq_any(rfd_revision.into_iter().map(GenericUuid::into_untyped_uuid)),
            );
        }

        if let Some(source) = source {
            query = query.filter(rfd_pdf::source.eq_any(source));
        }

        if let Some(rfd) = rfd {
            query = query.filter(
                rfd_pdf::rfd_id.eq_any(rfd.into_iter().map(GenericUuid::into_untyped_uuid)),
            );
        }

        if let Some(external_id) = external_id {
            query = query.filter(rfd_pdf::external_id.eq_any(external_id));
        }

        if !deleted {
            query = query.filter(rfd_pdf::deleted_at.is_null());
        }

        let results = query
            .offset(pagination.offset)
            .limit(pagination.limit)
            .order(rfd_pdf::created_at.desc())
            .get_results_async::<RfdPdfModel>(&*self.pool.get().await?)
            .await?;

        Ok(results
            .into_iter()
            .map(|revision| revision.into())
            .collect())
    }

    async fn upsert(&self, new_pdf: NewRfdPdf) -> Result<RfdPdf, StoreError> {
        let rfd: RfdPdfModel = insert_into(rfd_pdf::dsl::rfd_pdf)
            .values((
                rfd_pdf::id.eq(Uuid::new_v4()),
                rfd_pdf::rfd_revision_id.eq(new_pdf.rfd_revision_id.into_untyped_uuid()),
                rfd_pdf::source.eq(new_pdf.source.clone()),
                rfd_pdf::link.eq(new_pdf.link.clone()),
                rfd_pdf::rfd_id.eq(new_pdf.rfd_id.into_untyped_uuid()),
                rfd_pdf::external_id.eq(new_pdf.external_id.clone()),
            ))
            .on_conflict(on_constraint("revision_links_unique"))
            .do_nothing()
            .get_result_async(&*self.pool.get().await?)
            .await?;

        Ok(rfd.into())
    }

    async fn delete(&self, id: &TypedUuid<RfdPdfId>) -> Result<Option<RfdPdf>, StoreError> {
        let _ = update(rfd_pdf::dsl::rfd_pdf)
            .filter(rfd_pdf::id.eq(id.into_untyped_uuid()))
            .set(rfd_pdf::deleted_at.eq(Utc::now()))
            .execute_async(&*self.pool.get().await?)
            .await?;

        RfdPdfStore::get(self, id, true).await
    }
}

#[async_trait]
impl JobStore for PostgresStore {
    async fn get(&self, id: i32) -> Result<Option<Job>, StoreError> {
        let user = JobStore::list(
            self,
            JobFilter::default().id(Some(vec![id])),
            &ListPagination::default().limit(1),
        )
        .await?;
        Ok(user.into_iter().nth(0))
    }

    async fn list(
        &self,
        filter: super::JobFilter,
        pagination: &ListPagination,
    ) -> Result<Vec<Job>, StoreError> {
        let mut query = job::dsl::job.into_boxed();

        let JobFilter {
            id,
            sha,
            processed,
            started,
        } = filter;

        if let Some(id) = id {
            query = query.filter(job::id.eq_any(id));
        }

        if let Some(sha) = sha {
            query = query.filter(job::sha.eq_any(sha));
        }

        if let Some(processed) = processed {
            query = query.filter(job::processed.eq(processed));
        }

        if let Some(started) = started {
            if started {
                query = query.filter(job::started_at.is_not_null());
            } else {
                query = query.filter(job::started_at.is_null());
            }
        }

        let results = query
            .offset(pagination.offset)
            .limit(pagination.limit)
            .order(job::processed.asc())
            .order(job::committed_at.asc())
            .order(job::created_at.asc())
            .get_results_async::<JobModel>(&*self.pool.get().await?)
            .await?;

        Ok(results.into_iter().map(|job| job.into()).collect())
    }

    async fn upsert(&self, new_job: NewJob) -> Result<Job, StoreError> {
        let rfd: JobModel = insert_into(job::dsl::job)
            .values((
                job::owner.eq(new_job.owner.clone()),
                job::repository.eq(new_job.repository.clone()),
                job::branch.eq(new_job.branch.clone()),
                job::sha.eq(String::from(new_job.sha)),
                job::rfd.eq(new_job.rfd.clone()),
                job::webhook_delivery_id
                    .eq(new_job.webhook_delivery_id.map(|id| id.into_untyped_uuid())),
                job::processed.eq(false),
                job::committed_at.eq(new_job.committed_at.clone()),
            ))
            .get_result_async(&*self.pool.get().await?)
            .await?;

        Ok(rfd.into())
    }

    async fn start(&self, id: i32) -> Result<Option<Job>, StoreError> {
        let _ = update(job::dsl::job)
            .filter(job::id.eq(id))
            .filter(job::started_at.is_null())
            .set(job::started_at.eq(Utc::now()))
            .execute_async(&*self.pool.get().await?)
            .await?;

        JobStore::get(self, id).await
    }

    async fn complete(&self, id: i32) -> Result<Option<Job>, StoreError> {
        let _ = update(job::dsl::job)
            .filter(job::id.eq(id))
            .set(job::processed.eq(true))
            .execute_async(&*self.pool.get().await?)
            .await?;

        JobStore::get(self, id).await
    }
}
