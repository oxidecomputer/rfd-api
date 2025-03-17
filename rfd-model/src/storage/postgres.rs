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
    sql_query,
    sql_types::Bool,
    update,
    upsert::{excluded, on_constraint},
    BoolExpressionMethods, BoxableExpression, ExpressionMethods, NullableExpressionMethods,
};
use newtype_uuid::{GenericUuid, TypedUuid};
use std::collections::{btree_map::Entry, BTreeMap};
use uuid::Uuid;
use v_model::storage::postgres::PostgresStore;

use crate::{
    db::{
        JobModel, RfdMetaJoinRow, RfdModel, RfdPdfJoinRow, RfdPdfModel, RfdRevisionMetaModel,
        RfdRevisionModel, RfdRevisionPdfModel,
    },
    schema::{job, rfd, rfd_pdf, rfd_revision},
    schema_ext::Visibility,
    storage::StoreError,
    Job, NewJob, NewRfd, NewRfdPdf, NewRfdRevision, Rfd, RfdId, RfdMeta, RfdPdf, RfdPdfId, RfdPdfs,
    RfdRevision, RfdRevisionId, RfdRevisionMeta, RfdRevisionPdf,
};

use super::{
    JobFilter, JobStore, ListPagination, RfdFilter, RfdMetaStore, RfdPdfFilter, RfdPdfStore,
    RfdPdfsStore, RfdRevisionFilter, RfdRevisionMetaStore, RfdRevisionPdfStore, RfdRevisionStore,
    RfdStore,
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
            .left_join(rfd_revision::table)
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
                    commit,
                    public,
                    deleted,
                } = filter;

                if let Some(id) = id {
                    predicates.push(Box::new(
                        rfd::id.eq_any(id.into_iter().map(GenericUuid::into_untyped_uuid)),
                    ));
                }

                if let Some(revision) = revision {
                    predicates.push(Box::new(
                        rfd_revision::id
                            .assume_not_null()
                            .eq_any(revision.into_iter().map(GenericUuid::into_untyped_uuid)),
                    ));
                }

                if let Some(rfd_number) = rfd_number {
                    predicates.push(Box::new(rfd::rfd_number.eq_any(rfd_number)));
                }

                if let Some(commit) = commit {
                    predicates.push(Box::new(
                        rfd_revision::commit_sha
                            .assume_not_null()
                            .eq_any(commit.into_iter().map(|sha| sha.0)),
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
                    predicates.push(Box::new(
                        rfd_revision::deleted_at.assume_not_null().is_null(),
                    ));
                }

                predicates
            })
            .collect::<Vec<_>>();

        if let Some(predicate) = flatten_predicates(filter_predicates) {
            query = query.filter(predicate);
        }

        query = query
            .offset(pagination.offset)
            .limit(pagination.limit)
            .order((rfd::id.asc(), rfd_revision::committed_at.desc()));

        tracing::trace!(query = ?debug_query(&query), "List RFDs query");

        let results = query
            .get_results_async::<(RfdModel, Option<RfdRevisionModel>)>(&*self.pool.get().await?)
            .await?;

        tracing::trace!(count = ?results.len(), "Found RFDs");

        Ok(results
            .into_iter()
            .map(|(rfd, revision)| match revision {
                Some(revision) => (rfd, revision).into(),
                None => rfd.into(),
            })
            .collect())
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
        tracing::trace!(?filters, "Lookup RFDs");

        let mut clauses = vec![];
        let mut bind_count = 1;

        for filter in &filters {
            let mut filter_clause = "1=1".to_string();

            let RfdFilter {
                id,
                revision,
                rfd_number,
                commit,
                public,
                deleted,
            } = filter;

            if let Some(ids) = &id {
                let id_binds = ids
                    .iter()
                    .enumerate()
                    .map(|(i, _)| format!("${}", bind_count + i))
                    .collect::<Vec<_>>();
                bind_count = bind_count + id_binds.len();
                filter_clause = filter_clause + &format!(" AND rfd.id IN ({})", id_binds.join(","));
            }

            if let Some(revisions) = &revision {
                let revision_binds = revisions
                    .iter()
                    .enumerate()
                    .map(|(i, _)| format!("${}", bind_count + i))
                    .collect::<Vec<_>>();
                bind_count = bind_count + revision_binds.len();
                filter_clause = filter_clause
                    + &format!(" AND rfd_revision.id IN ({})", revision_binds.join(","));
            }

            if let Some(rfd_numbers) = &rfd_number {
                let rfd_number_binds = rfd_numbers
                    .iter()
                    .enumerate()
                    .map(|(i, _)| format!("${}", bind_count + i))
                    .collect::<Vec<_>>();
                bind_count = bind_count + rfd_number_binds.len();
                filter_clause = filter_clause
                    + &format!(" AND rfd.rfd_number IN ({})", rfd_number_binds.join(","));
            }

            if let Some(commit_shas) = &commit {
                let commit_sha_binds = commit_shas
                    .iter()
                    .enumerate()
                    .map(|(i, _)| format!("${}", bind_count + i))
                    .collect::<Vec<_>>();
                bind_count = bind_count + commit_sha_binds.len();
                filter_clause = filter_clause
                    + &format!(
                        " AND rfd_revision.commit_sha IN ({})",
                        commit_sha_binds.join(",")
                    );
            }

            if let Some(_) = &public {
                bind_count = bind_count + 1;
                filter_clause = filter_clause + " AND rfd.public = {}";
            }

            if !deleted {
                filter_clause = filter_clause
                    + " AND rfd.deleted_at IS NULL AND rfd_revision.deleted_at IS NULL";
            }

            clauses.push(format!("({})", filter_clause));
        }

        let where_clause = if clauses.len() > 0 {
            format!("({})", clauses.join(" OR "))
        } else {
            format!("1=1")
        };

        let raw_query = format!(
            r#"SELECT
            rfd.id as id,
            rfd.rfd_number as rfd_number,
            rfd.link as link,
            rfd.created_at as created_at,
            rfd.updated_at as updated_at,
            rfd.deleted_at as deleted_at,
            rfd.visibility as visibility,
            rfd_revision.id AS revision_id,
            rfd_revision.rfd_id as revision_rfd_id,
            rfd_revision.title as revision_title,
            rfd_revision.state as revision_state,
            rfd_revision.discussion as revision_discussion,
            rfd_revision.authors as revision_authors,
            rfd_revision.content_format as revision_content_format,
            rfd_revision.sha as revision_sha,
            rfd_revision.commit_sha as revision_commit_sha,
            rfd_revision.committed_at as revision_committed_at,
            rfd_revision.created_at as revision_created_at,
            rfd_revision.updated_at as revision_updated_at,
            rfd_revision.deleted_at as revision_deleted_at,
            rfd_revision.labels as revision_labels
        FROM
            rfd
        INNER JOIN
            rfd_revision
            ON rfd_revision.rfd_id = rfd.id
        WHERE {} AND
            rfd_revision.id = (
                SELECT rfd_revision.id
                FROM rfd_revision
                WHERE rfd_revision.rfd_id = rfd.id
                ORDER BY rfd_revision.committed_at DESC
                LIMIT 1
            )
        ORDER BY
            rfd_revision.rfd_id ASC,
            rfd_revision.committed_at DESC
        LIMIT ${} OFFSET ${}"#,
            where_clause,
            bind_count,
            bind_count + 1,
        );

        let mut query = sql_query(raw_query).into_boxed::<Pg>();

        for filter in &filters {
            let RfdFilter {
                id,
                revision,
                rfd_number,
                commit,
                public,
                ..
            } = filter;

            if let Some(ids) = &id {
                for id in ids {
                    tracing::trace!(?id, "Binding id parameter");
                    query = query.bind::<diesel::sql_types::Uuid, _>(id.into_untyped_uuid());
                }
            }

            if let Some(revisions) = &revision {
                for revision in revisions {
                    tracing::trace!(?revision, "Binding revision parameter");
                    query = query.bind::<diesel::sql_types::Uuid, _>(revision.into_untyped_uuid());
                }
            }

            if let Some(rfd_numbers) = &rfd_number {
                for rfd_number in rfd_numbers {
                    tracing::trace!(?rfd_number, "Binding rfd_number parameter");
                    query = query.bind::<diesel::sql_types::Integer, _>(*rfd_number);
                }
            }

            if let Some(commits) = &commit {
                for commit in commits {
                    tracing::trace!(?commit, "Binding commit parameter");
                    query = query.bind::<diesel::sql_types::VarChar, _>(commit.to_string());
                }
            }

            if let Some(public) = &public {
                tracing::trace!(?public, "Binding public parameter");
                query = query.bind::<diesel::sql_types::Bool, _>(*public);
            }
        }

        query = query
            .bind::<diesel::sql_types::Integer, _>(pagination.limit as i32)
            .bind::<diesel::sql_types::Integer, _>(pagination.offset as i32);

        tracing::trace!(query = ?debug_query(&query), "List RFDs query");

        let rows = query
            .get_results_async::<RfdMetaJoinRow>(&*self.pool.get().await?)
            .await?;
        let results = rows
            .into_iter()
            .map(|row| <(RfdModel, RfdRevisionMetaModel)>::from(row).into())
            .collect::<Vec<_>>();

        tracing::trace!(count = ?results.len(), "Found RFDs");

        Ok(results)
    }
}

#[async_trait]
impl RfdPdfsStore for PostgresStore {
    async fn get(
        &self,
        id: TypedUuid<RfdId>,
        revision: Option<TypedUuid<RfdRevisionId>>,
        deleted: bool,
    ) -> Result<Option<RfdPdfs>, StoreError> {
        let rfd = RfdPdfsStore::list(
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
    ) -> Result<Vec<RfdPdfs>, StoreError> {
        tracing::trace!(?filters, "Lookup RFDs");

        let mut clauses = vec![];
        let mut bind_count = 1;

        for filter in &filters {
            let mut filter_clause = "1=1".to_string();

            let RfdFilter {
                id,
                revision,
                rfd_number,
                commit,
                public,
                deleted,
            } = filter;

            if let Some(ids) = &id {
                let id_binds = ids
                    .iter()
                    .enumerate()
                    .map(|(i, _)| format!("${}", bind_count + i))
                    .collect::<Vec<_>>();
                bind_count = bind_count + id_binds.len();
                filter_clause = filter_clause + &format!(" AND rfd.id IN ({})", id_binds.join(","));
            }

            if let Some(revisions) = &revision {
                let revision_binds = revisions
                    .iter()
                    .enumerate()
                    .map(|(i, _)| format!("${}", bind_count + i))
                    .collect::<Vec<_>>();
                bind_count = bind_count + revision_binds.len();
                filter_clause = filter_clause
                    + &format!(" AND rfd_revision.id IN ({})", revision_binds.join(","));
            }

            if let Some(rfd_numbers) = &rfd_number {
                let rfd_number_binds = rfd_numbers
                    .iter()
                    .enumerate()
                    .map(|(i, _)| format!("${}", bind_count + i))
                    .collect::<Vec<_>>();
                bind_count = bind_count + rfd_number_binds.len();
                filter_clause = filter_clause
                    + &format!(" AND rfd.rfd_number IN ({})", rfd_number_binds.join(","));
            }

            if let Some(commit_shas) = &commit {
                let commit_sha_binds = commit_shas
                    .iter()
                    .enumerate()
                    .map(|(i, _)| format!("${}", bind_count + i))
                    .collect::<Vec<_>>();
                bind_count = bind_count + commit_sha_binds.len();
                filter_clause = filter_clause
                    + &format!(
                        " AND rfd_revision.commit_sha IN ({})",
                        commit_sha_binds.join(",")
                    );
            }

            if let Some(_) = &public {
                bind_count = bind_count + 1;
                filter_clause = filter_clause + " AND rfd.public = {}";
            }

            if !deleted {
                filter_clause = filter_clause +" AND rfd.deleted_at IS NULL AND rfd_revision.deleted_at IS NULL AND rfd_pdf.deleted_at IS NULL";
            }

            clauses.push(format!("({})", filter_clause));
        }

        let where_clause = if clauses.len() > 0 {
            format!("({})", clauses.join(" OR "))
        } else {
            format!("1=1")
        };

        let raw_query = format!(
            r#"SELECT
            rfd.id as id,
            rfd.rfd_number as rfd_number,
            rfd.link as link,
            rfd.created_at as created_at,
            rfd.updated_at as updated_at,
            rfd.deleted_at as deleted_at,
            rfd.visibility as visibility,
            rfd_revision.id AS revision_id,
            rfd_revision.rfd_id as revision_rfd_id,
            rfd_revision.title as revision_title,
            rfd_revision.state as revision_state,
            rfd_revision.discussion as revision_discussion,
            rfd_revision.authors as revision_authors,
            rfd_pdf.id as pdf_id,
            rfd_pdf.rfd_revision_id as pdf_rfd_revision_id,
            rfd_pdf.source as pfd_source,
            rfd_pdf.link as pdf_link,
            rfd_pdf.created_at as pdf_created_at,
            rfd_pdf.updated_at as pdf_updated_at,
            rfd_pdf.deleted_at as pdf_deleted_at,
            rfd_pdf.rfd_id as pdf_rfd_id,
            rfd_pdf.external_id as pdf_external_id,
            rfd_revision.content_format as revision_content_format,
            rfd_revision.sha as revision_sha,
            rfd_revision.commit_sha as revision_commit_sha,
            rfd_revision.committed_at as revision_committed_at,
            rfd_revision.created_at as revision_created_at,
            rfd_revision.updated_at as revision_updated_at,
            rfd_revision.deleted_at as revision_deleted_at,
            rfd_revision.labels as revision_labels
        FROM
            rfd
        INNER JOIN
            rfd_revision
            ON rfd_revision.rfd_id = rfd.id
        INNER JOIN
            rfd_pdf
            ON rfd_revision.id = rfd_pdf.rfd_revision_id
        WHERE {} AND
            rfd_revision.id = (
                SELECT rfd_revision.id
                FROM rfd_revision
                WHERE rfd_revision.rfd_id = rfd.id
                ORDER BY rfd_revision.committed_at DESC
                LIMIT 1
            )
        ORDER BY
            rfd_revision.rfd_id ASC,
            rfd_revision.committed_at DESC
        LIMIT ${} OFFSET ${}"#,
            where_clause,
            bind_count,
            bind_count + 1,
        );

        let mut query = sql_query(raw_query).into_boxed::<Pg>();

        for filter in &filters {
            let RfdFilter {
                id,
                revision,
                rfd_number,
                commit,
                public,
                ..
            } = filter;

            if let Some(ids) = &id {
                for id in ids {
                    tracing::trace!(?id, "Binding id parameter");
                    query = query.bind::<diesel::sql_types::Uuid, _>(id.into_untyped_uuid());
                }
            }

            if let Some(revisions) = &revision {
                for revision in revisions {
                    tracing::trace!(?revision, "Binding revision parameter");
                    query = query.bind::<diesel::sql_types::Uuid, _>(revision.into_untyped_uuid());
                }
            }

            if let Some(rfd_numbers) = &rfd_number {
                for rfd_number in rfd_numbers {
                    tracing::trace!(?rfd_number, "Binding rfd_number parameter");
                    query = query.bind::<diesel::sql_types::Integer, _>(*rfd_number);
                }
            }

            if let Some(commits) = &commit {
                for commit in commits {
                    tracing::trace!(?commit, "Binding commit parameter");
                    query = query.bind::<diesel::sql_types::VarChar, _>(commit.to_string());
                }
            }

            if let Some(public) = &public {
                tracing::trace!(?public, "Binding public parameter");
                query = query.bind::<diesel::sql_types::Bool, _>(*public);
            }
        }

        query = query
            .bind::<diesel::sql_types::Integer, _>(pagination.limit as i32)
            .bind::<diesel::sql_types::Integer, _>(pagination.offset as i32);

        tracing::trace!(query = ?debug_query(&query), "List RFDs query");

        let rows = query
            .get_results_async::<RfdPdfJoinRow>(&*self.pool.get().await?)
            .await?;
        let results = rows
            .into_iter()
            .map(|row| <(RfdModel, RfdRevisionPdfModel)>::from(row))
            .fold(
                BTreeMap::<TypedUuid<RfdId>, RfdPdfs>::default(),
                |mut map, (rfd_model, revision_model)| {
                    let entry = map.entry(TypedUuid::from_untyped_uuid(rfd_model.id));
                    match entry {
                        Entry::Occupied(mut existing) => {
                            existing
                                .get_mut()
                                .content
                                .as_mut()
                                .unwrap()
                                .content
                                .push(revision_model.content.into());
                        }
                        Entry::Vacant(empty) => {
                            empty.insert((rfd_model, revision_model).into());
                        }
                    };
                    map
                },
            );

        tracing::trace!(count = ?results.len(), "Found RFDs");

        Ok(results.into_values().collect())
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
                    commit,
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

                if let Some(commit) = commit {
                    predicates.push(Box::new(
                        rfd_revision::commit_sha.eq_any(commit.into_iter().map(|sha| sha.0)),
                    ));
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
                    commit,
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

                if let Some(commit) = commit {
                    predicates.push(Box::new(
                        rfd_revision::commit_sha.eq_any(commit.into_iter().map(|sha| sha.0)),
                    ));
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
impl RfdRevisionPdfStore for PostgresStore {
    async fn get(
        &self,
        id: &TypedUuid<RfdRevisionId>,
        deleted: bool,
    ) -> Result<Option<RfdRevisionPdf>, StoreError> {
        let user = RfdRevisionPdfStore::list(
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
    ) -> Result<Vec<RfdRevisionPdf>, StoreError> {
        let mut query = rfd_revision::table
            .inner_join(rfd_pdf::table)
            .select((
                rfd_revision::id,
                rfd_revision::rfd_id,
                rfd_revision::title,
                rfd_revision::state,
                rfd_revision::discussion,
                rfd_revision::authors,
                (
                    rfd_pdf::id,
                    rfd_pdf::rfd_revision_id,
                    rfd_pdf::source,
                    rfd_pdf::link,
                    rfd_pdf::created_at,
                    rfd_pdf::updated_at,
                    rfd_pdf::deleted_at,
                    rfd_pdf::rfd_id,
                    rfd_pdf::external_id,
                ),
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

        tracing::trace!(?filters, "Lookup RFD revision pdf");

        let filter_predicates = filters
            .into_iter()
            .map(|filter| {
                let mut predicates: Vec<Box<dyn BoxableExpression<_, Pg, SqlType = Bool>>> = vec![];
                let RfdRevisionFilter {
                    id,
                    rfd,
                    commit,
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

                if let Some(commit) = commit {
                    predicates.push(Box::new(
                        rfd_revision::commit_sha.eq_any(commit.into_iter().map(|sha| sha.0)),
                    ));
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

        tracing::info!(query = ?debug_query(&query), "Run list rfd pdf");

        let results = query
            .get_results_async::<RfdRevisionPdfModel>(&*self.pool.get().await?)
            .await?;
        let revisions = results.into_iter().fold(
            BTreeMap::<TypedUuid<RfdRevisionId>, RfdRevisionPdf>::default(),
            |mut map, revision| {
                let entry = map.entry(TypedUuid::from_untyped_uuid(revision.id));
                match entry {
                    Entry::Occupied(mut existing) => {
                        existing.get_mut().content.push(revision.content.into());
                    }
                    Entry::Vacant(empty) => {
                        empty.insert(revision.into());
                    }
                };
                map
            },
        );

        tracing::trace!(count = ?revisions.len(), "Found RFD revision metadata");

        Ok(revisions.into_values().collect::<Vec<_>>())
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
                    rfd,
                    processed,
                    started,
                } = filter;

                if let Some(id) = id {
                    predicates.push(Box::new(job::id.eq_any(id)));
                }

                if let Some(sha) = sha {
                    predicates.push(Box::new(job::sha.eq_any(sha)));
                }

                if let Some(rfd) = rfd {
                    predicates.push(Box::new(job::rfd.eq_any(rfd)));
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
