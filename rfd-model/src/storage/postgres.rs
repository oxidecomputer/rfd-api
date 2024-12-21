// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use async_bb8_diesel::AsyncRunQueryDsl;
use async_trait::async_trait;
use chrono::Utc;
use diesel::{
    debug_query, insert_into,
    pg::Pg,
    query_dsl::QueryDsl,
    sql_types::Bool,
    update,
    upsert::{excluded, on_constraint},
    BoolExpressionMethods, BoxableExpression, ExpressionMethods, SelectableHelper,
};
use newtype_uuid::{GenericUuid, TypedUuid};
use uuid::Uuid;
use v_model::storage::postgres::PostgresStore;

use crate::{
    db::{JobModel, RfdModel, RfdPdfModel, RfdRevisionMetaModel, RfdRevisionModel},
    schema::{job, rfd, rfd_pdf, rfd_revision},
    schema_ext::Visibility,
    storage::StoreError,
    Job, NewJob, NewRfd, NewRfdPdf, NewRfdRevision, Rfd, RfdId, RfdMeta, RfdPdf, RfdPdfId,
    RfdRevision, RfdRevisionId, RfdRevisionMeta,
};

use super::{
    JobFilter, JobStore, ListPagination, RfdFilter, RfdMetaStore, RfdPdfFilter, RfdPdfStore,
    RfdRevisionFilter, RfdRevisionMetaStore, RfdRevisionStore, RfdStore,
};

#[async_trait]
impl RfdStore for PostgresStore {
    async fn get(
        &self,
        id: &TypedUuid<RfdId>,
        revision: Option<TypedUuid<RfdRevisionId>>,
        deleted: bool,
    ) -> Result<Option<Rfd>, StoreError> {
        let rfd = RfdStore::list(
            self,
            vec![RfdFilter::default()
                .id(Some(vec![*id]))
                .revision(revision.map(|rev| vec![rev]))
                .deleted(deleted)],
            &ListPagination::default().limit(1),
        )
        .await?;
        Ok(rfd.into_iter().nth(0))
    }

    async fn list(
        &self,
        filters: Vec<RfdFilter>,
        pagination: &ListPagination,
    ) -> Result<Vec<Rfd>, StoreError> {
        let mut query = rfd::table
            .inner_join(rfd_revision::table)
            .distinct_on(rfd::id)
            .into_boxed();

        tracing::trace!(?filters, "Lookup RFDs");

        let filter_predicates = filters
            .into_iter()
            .map(|filter| {
                let mut predicates: Vec<Box<dyn BoxableExpression<_, Pg, SqlType = Bool>>> = vec![];
                let RfdFilter {
                    id,
                    revision,
                    rfd_number,
                    commit_sha,
                    public,
                    deleted,
                } = filter;

                if let Some(id) = id {
                    predicates.push(Box::new(
                        rfd::id.eq_any(id.into_iter().map(GenericUuid::into_untyped_uuid)),
                    ));
                }

                if let Some(revision) = revision {
                    predicates
                        .push(Box::new(rfd_revision::id.eq_any(
                            revision.into_iter().map(GenericUuid::into_untyped_uuid),
                        )));
                }

                if let Some(rfd_number) = rfd_number {
                    predicates.push(Box::new(rfd::rfd_number.eq_any(rfd_number)));
                }

                if let Some(commit_sha) = commit_sha {
                    predicates.push(Box::new(
                        rfd_revision::commit_sha.eq_any(commit_sha.into_iter().map(|sha| sha.0)),
                    ));
                }

                if let Some(public) = public {
                    predicates.push(Box::new(
                        rfd::visibility.eq(public
                            .then(|| Visibility::Public)
                            .unwrap_or(Visibility::Private)),
                    ));
                }

                if !deleted {
                    predicates.push(Box::new(rfd::deleted_at.is_null()));
                    predicates.push(Box::new(rfd_revision::deleted_at.is_null()));
                }

                predicates
            })
            .collect::<Vec<_>>();

        if let Some(predicate) = flatten_predicates(filter_predicates) {
            query = query.filter(predicate);
        }

        let results = query
            .offset(pagination.offset)
            .limit(pagination.limit)
            .order((
                rfd_revision::rfd_id.asc(),
                rfd_revision::committed_at.desc(),
            ))
            .get_results_async::<(RfdModel, RfdRevisionModel)>(&*self.pool.get().await?)
            .await?;

        tracing::trace!(count = ?results.len(), "Found RFDs");

        Ok(results.into_iter().map(|rfd| rfd.into()).collect())
    }

    async fn upsert(&self, new_rfd: NewRfd) -> Result<Rfd, StoreError> {
        let _: RfdModel = insert_into(rfd::dsl::rfd)
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

        // There is a race condition here than case a failure where a delete occurs between
        // the upsert and the get
        Ok(RfdStore::get(self, &new_rfd.id, None, false)
            .await?
            .unwrap())
    }

    async fn delete(&self, id: &TypedUuid<RfdId>) -> Result<Option<Rfd>, StoreError> {
        let _ = update(rfd::dsl::rfd)
            .filter(rfd::id.eq(id.into_untyped_uuid()))
            .set(rfd::deleted_at.eq(Utc::now()))
            .execute_async(&*self.pool.get().await?)
            .await?;

        RfdStore::get(self, id, None, true).await
    }
}

#[async_trait]
impl RfdMetaStore for PostgresStore {
    async fn get(
        &self,
        id: TypedUuid<RfdId>,
        revision: Option<TypedUuid<RfdRevisionId>>,
        deleted: bool,
    ) -> Result<Option<RfdMeta>, StoreError> {
        let rfd = RfdMetaStore::list(
            self,
            vec![RfdFilter::default()
                .id(Some(vec![id]))
                .revision(revision.map(|rev| vec![rev]))
                .deleted(deleted)],
            &ListPagination::default().limit(1),
        )
        .await?;
        Ok(rfd.into_iter().nth(0))
    }

    async fn list(
        &self,
        filters: Vec<RfdFilter>,
        pagination: &ListPagination,
    ) -> Result<Vec<RfdMeta>, StoreError> {
        let mut query = rfd::table
            .inner_join(rfd_revision::table)
            .distinct_on(rfd::id)
            .select((RfdModel::as_select(), RfdRevisionMetaModel::as_select()))
            .into_boxed();

        tracing::trace!(?filters, "Lookup RFDs");

        let filter_predicates = filters
            .into_iter()
            .map(|filter| {
                let mut predicates: Vec<Box<dyn BoxableExpression<_, Pg, SqlType = Bool>>> = vec![];
                let RfdFilter {
                    id,
                    revision,
                    rfd_number,
                    commit_sha,
                    public,
                    deleted,
                } = filter;

                if let Some(id) = id {
                    predicates.push(Box::new(
                        rfd::id.eq_any(id.into_iter().map(GenericUuid::into_untyped_uuid)),
                    ));
                }

                if let Some(revision) = revision {
                    predicates
                        .push(Box::new(rfd_revision::id.eq_any(
                            revision.into_iter().map(GenericUuid::into_untyped_uuid),
                        )));
                }

                if let Some(rfd_number) = rfd_number {
                    predicates.push(Box::new(rfd::rfd_number.eq_any(rfd_number)));
                }

                if let Some(commit_sha) = commit_sha {
                    predicates.push(Box::new(
                        rfd_revision::commit_sha.eq_any(commit_sha.into_iter().map(|sha| sha.0)),
                    ));
                }

                if let Some(public) = public {
                    predicates.push(Box::new(
                        rfd::visibility.eq(public
                            .then(|| Visibility::Public)
                            .unwrap_or(Visibility::Private)),
                    ));
                }

                if !deleted {
                    predicates.push(Box::new(rfd::deleted_at.is_null()));
                    predicates.push(Box::new(rfd_revision::deleted_at.is_null()));
                }

                predicates
            })
            .collect::<Vec<_>>();

        if let Some(predicate) = flatten_predicates(filter_predicates) {
            query = query.filter(predicate);
        }

        let results = query
            .offset(pagination.offset)
            .limit(pagination.limit)
            .order((
                rfd_revision::rfd_id.asc(),
                rfd_revision::committed_at.desc(),
            ))
            .get_results_async::<(RfdModel, RfdRevisionMetaModel)>(&*self.pool.get().await?)
            .await?;

        tracing::trace!(count = ?results.len(), "Found RFDs");

        Ok(results.into_iter().map(|rfd| rfd.into()).collect())
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
            vec![RfdRevisionFilter::default()
                .id(Some(vec![*id]))
                .deleted(deleted)],
            &ListPagination::default().limit(1),
        )
        .await?;
        Ok(user.into_iter().nth(0))
    }

    async fn list(
        &self,
        filters: Vec<RfdRevisionFilter>,
        pagination: &ListPagination,
    ) -> Result<Vec<RfdRevision>, StoreError> {
        let mut query = rfd_revision::dsl::rfd_revision.into_boxed();

        tracing::trace!(?filters, "Lookup RFD revisions");

        let filter_predicates = filters
            .into_iter()
            .map(|filter| {
                let mut predicates: Vec<Box<dyn BoxableExpression<_, Pg, SqlType = Bool>>> = vec![];
                let RfdRevisionFilter {
                    id,
                    rfd,
                    sha,
                    deleted,
                } = filter;

                if let Some(id) = id {
                    predicates.push(Box::new(
                        rfd_revision::id.eq_any(id.into_iter().map(GenericUuid::into_untyped_uuid)),
                    ));
                }

                if let Some(rfd) = rfd {
                    predicates.push(Box::new(
                        rfd_revision::rfd_id
                            .eq_any(rfd.into_iter().map(GenericUuid::into_untyped_uuid)),
                    ));
                }

                if let Some(sha) = sha {
                    predicates.push(Box::new(rfd_revision::sha.eq_any(sha)));
                }

                if !deleted {
                    predicates.push(Box::new(rfd_revision::deleted_at.is_null()));
                }

                predicates
            })
            .collect::<Vec<_>>();

        if let Some(predicate) = flatten_predicates(filter_predicates) {
            query = query.filter(predicate);
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
            vec![RfdRevisionFilter::default()
                .id(Some(vec![*id]))
                .deleted(deleted)],
            &ListPagination::default().limit(1),
        )
        .await?;
        Ok(user.into_iter().nth(0))
    }

    async fn list(
        &self,
        filters: Vec<RfdRevisionFilter>,
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

        tracing::trace!(?filters, "Lookup RFD revision metadata");

        let filter_predicates = filters
            .into_iter()
            .map(|filter| {
                let mut predicates: Vec<Box<dyn BoxableExpression<_, Pg, SqlType = Bool>>> = vec![];
                let RfdRevisionFilter {
                    id,
                    rfd,
                    sha,
                    deleted,
                } = filter;

                if let Some(id) = id {
                    predicates.push(Box::new(
                        rfd_revision::id.eq_any(id.into_iter().map(GenericUuid::into_untyped_uuid)),
                    ));
                }

                if let Some(rfd) = rfd {
                    predicates.push(Box::new(
                        rfd_revision::rfd_id
                            .eq_any(rfd.into_iter().map(GenericUuid::into_untyped_uuid)),
                    ));
                }

                if let Some(sha) = sha {
                    predicates.push(Box::new(rfd_revision::sha.eq_any(sha)));
                }

                if !deleted {
                    predicates.push(Box::new(rfd_revision::deleted_at.is_null()));
                }

                predicates
            })
            .collect::<Vec<_>>();

        if let Some(predicate) = flatten_predicates(filter_predicates) {
            query = query.filter(predicate);
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
            vec![RfdPdfFilter::default().id(Some(vec![*id])).deleted(deleted)],
            &ListPagination::default().limit(1),
        )
        .await?;
        Ok(user.into_iter().nth(0))
    }

    async fn list(
        &self,
        filters: Vec<RfdPdfFilter>,
        pagination: &ListPagination,
    ) -> Result<Vec<RfdPdf>, StoreError> {
        let mut query = rfd_pdf::dsl::rfd_pdf.into_boxed();

        tracing::trace!(?filters, "Lookup RFD pdfs");

        let filter_predicates = filters
            .into_iter()
            .map(|filter| {
                let mut predicates: Vec<Box<dyn BoxableExpression<_, Pg, SqlType = Bool>>> = vec![];
                let RfdPdfFilter {
                    id,
                    rfd_revision,
                    source,
                    deleted,
                    rfd,
                    external_id,
                } = filter;

                if let Some(id) = id {
                    predicates.push(Box::new(
                        rfd_pdf::id.eq_any(id.into_iter().map(GenericUuid::into_untyped_uuid)),
                    ));
                }

                if let Some(rfd_revision) = rfd_revision {
                    predicates
                        .push(Box::new(rfd_pdf::rfd_revision_id.eq_any(
                            rfd_revision.into_iter().map(GenericUuid::into_untyped_uuid),
                        )));
                }

                if let Some(source) = source {
                    predicates.push(Box::new(rfd_pdf::source.eq_any(source)));
                }

                if let Some(rfd) = rfd {
                    predicates.push(Box::new(
                        rfd_pdf::rfd_id.eq_any(rfd.into_iter().map(GenericUuid::into_untyped_uuid)),
                    ));
                }

                if let Some(external_id) = external_id {
                    predicates.push(Box::new(rfd_pdf::external_id.eq_any(external_id)));
                }

                if !deleted {
                    predicates.push(Box::new(rfd_pdf::deleted_at.is_null()));
                }

                predicates
            })
            .collect::<Vec<_>>();

        if let Some(predicate) = flatten_predicates(filter_predicates) {
            query = query.filter(predicate);
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
            vec![JobFilter::default().id(Some(vec![id]))],
            &ListPagination::default().limit(1),
        )
        .await?;
        Ok(user.into_iter().nth(0))
    }

    async fn list(
        &self,
        filters: Vec<JobFilter>,
        pagination: &ListPagination,
    ) -> Result<Vec<Job>, StoreError> {
        let mut query = job::dsl::job.into_boxed();
        let filter_predicates = filters
            .into_iter()
            .map(|filter| {
                let mut predicates: Vec<Box<dyn BoxableExpression<_, Pg, SqlType = Bool>>> = vec![];
                let JobFilter {
                    id,
                    sha,
                    processed,
                    started,
                } = filter;

                if let Some(id) = id {
                    predicates.push(Box::new(job::id.eq_any(id)));
                }

                if let Some(sha) = sha {
                    predicates.push(Box::new(job::sha.eq_any(sha)));
                }

                if let Some(processed) = processed {
                    predicates.push(Box::new(job::processed.eq(processed)));
                }

                if let Some(started) = started {
                    if started {
                        predicates.push(Box::new(job::started_at.is_not_null()));
                    } else {
                        predicates.push(Box::new(job::started_at.is_null()));
                    }
                }

                predicates
            })
            .collect::<Vec<_>>();

        if let Some(predicate) = flatten_predicates(filter_predicates) {
            query = query.filter(predicate);
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

fn flatten_predicates<T>(
    predicates: Vec<Vec<Box<dyn BoxableExpression<T, Pg, SqlType = Bool>>>>,
) -> Option<Box<dyn BoxableExpression<T, Pg, SqlType = Bool>>>
where
    T: 'static,
{
    let mut filter_predicates = vec![];

    for p in predicates {
        let flat = p
            .into_iter()
            .reduce(|combined, entry| Box::new(combined.and(entry)));
        if let Some(flat) = flat {
            filter_predicates.push(flat);
        }
    }

    filter_predicates
        .into_iter()
        .reduce(|combined, entry| Box::new(combined.or(entry)))
}
