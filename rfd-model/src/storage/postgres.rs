use async_bb8_diesel::{AsyncRunQueryDsl, ConnectionError, ConnectionManager, OptionalExtension};
use async_trait::async_trait;
use bb8::Pool;
use chrono::Utc;
use diesel::{
    insert_into,
    pg::PgConnection,
    query_dsl::QueryDsl,
    update,
    upsert::{excluded, on_constraint},
    ExpressionMethods, PgArrayExpressionMethods,
};
use std::{
    collections::{BTreeMap, BTreeSet},
    time::Duration,
};
use uuid::Uuid;

use crate::{
    db::{
        AccessGroupModel, ApiKeyModel, ApiUserAccessTokenModel, ApiUserModel, ApiUserProviderModel,
        JobModel, LoginAttemptModel, MapperModel, OAuthClientModel, OAuthClientRedirectUriModel,
        OAuthClientSecretModel, RfdModel, RfdPdfModel, RfdRevisionModel,
    },
    permissions::{Permission, Permissions},
    schema::{
        access_groups, api_key, api_user, api_user_access_token, api_user_provider, job,
        login_attempt, mapper, oauth_client, oauth_client_redirect_uri, oauth_client_secret, rfd,
        rfd_pdf, rfd_revision,
    },
    storage::StoreError,
    AccessGroup, AccessToken, ApiKey, ApiUser, ApiUserProvider, Job, LoginAttempt, Mapper,
    NewAccessGroup, NewAccessToken, NewApiKey, NewApiUser, NewApiUserProvider, NewJob,
    NewLoginAttempt, NewMapper, NewOAuthClient, NewOAuthClientRedirectUri, NewOAuthClientSecret,
    NewRfd, NewRfdPdf, NewRfdRevision, OAuthClient, OAuthClientRedirectUri, OAuthClientSecret, Rfd,
    RfdPdf, RfdRevision,
};

use super::{
    AccessGroupFilter, AccessGroupStore, AccessTokenFilter, AccessTokenStore, ApiKeyFilter,
    ApiKeyStore, ApiUserFilter, ApiUserProviderFilter, ApiUserProviderStore, ApiUserStore,
    JobFilter, JobStore, ListPagination, LoginAttemptFilter, LoginAttemptStore, MapperFilter,
    MapperStore, OAuthClientFilter, OAuthClientRedirectUriStore, OAuthClientSecretStore,
    OAuthClientStore, RfdFilter, RfdPdfFilter, RfdPdfStore, RfdRevisionFilter, RfdRevisionStore,
    RfdStore,
};

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

pub struct PostgresStore {
    conn: DbPool,
}

#[derive(Debug)]
pub enum PostgresError {
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
            conn: Pool::builder()
                .connection_timeout(Duration::from_secs(5))
                .build(manager)
                .await?,
        })
    }
}

#[async_trait]
impl RfdStore for PostgresStore {
    async fn get(&self, id: &Uuid, deleted: bool) -> Result<Option<Rfd>, StoreError> {
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
            deleted,
        } = filter;

        if let Some(id) = id {
            query = query.filter(rfd::id.eq_any(id));
        }

        if let Some(rfd_number) = rfd_number {
            query = query.filter(rfd::rfd_number.eq_any(rfd_number));
        }

        if !deleted {
            query = query.filter(rfd::deleted_at.is_null());
        }

        let results = query
            .offset(pagination.offset)
            .limit(pagination.limit)
            .order(rfd::rfd_number.desc())
            .get_results_async::<RfdModel>(&self.conn)
            .await?;

        tracing::trace!(count = ?results.len(), "Found RFDs");

        Ok(results.into_iter().map(|rfd| rfd.into()).collect())
    }

    async fn upsert(&self, new_rfd: NewRfd) -> Result<Rfd, StoreError> {
        let rfd: RfdModel = insert_into(rfd::dsl::rfd)
            .values((
                rfd::id.eq(new_rfd.id),
                rfd::rfd_number.eq(new_rfd.rfd_number.clone()),
                rfd::link.eq(new_rfd.link.clone()),
                // rfd::relevant_components.eq(new_rfd.relevant_components.clone()),
                // rfd::milestones.eq(new_rfd.milestones.clone()),
            ))
            .on_conflict(rfd::id)
            .do_update()
            .set((
                rfd::rfd_number.eq(excluded(rfd::rfd_number)),
                rfd::link.eq(excluded(rfd::link)),
                // rfd::relevant_components.eq(excluded(rfd::relevant_components)),
                // rfd::milestones.eq(excluded(rfd::milestones)),
                rfd::updated_at.eq(Utc::now()),
            ))
            .get_result_async(&self.conn)
            .await?;

        Ok(rfd.into())
    }

    async fn delete(&self, id: &Uuid) -> Result<Option<Rfd>, StoreError> {
        let _ = update(rfd::dsl::rfd)
            .filter(rfd::id.eq(*id))
            .set(rfd::deleted_at.eq(Utc::now()))
            .execute_async(&self.conn)
            .await?;

        RfdStore::get(self, id, true).await
    }
}

#[async_trait]
impl RfdRevisionStore for PostgresStore {
    async fn get(&self, id: &Uuid, deleted: bool) -> Result<Option<RfdRevision>, StoreError> {
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
            query = query.filter(rfd_revision::id.eq_any(id));
        }

        if let Some(rfd) = rfd {
            query = query.filter(rfd_revision::rfd_id.eq_any(rfd));
        }

        if let Some(sha) = sha {
            query = query.filter(rfd_revision::sha.eq_any(sha));
        }

        if !deleted {
            query = query.filter(rfd_revision::deleted_at.is_null());
        }

        let results = query
            .offset(pagination.offset)
            .limit(pagination.limit)
            .order(rfd_revision::created_at.desc())
            .get_results_async::<RfdRevisionModel>(&self.conn)
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

        tracing::trace!(?filter, "Lookup unique RFD revisions");

        let RfdRevisionFilter {
            id,
            rfd,
            sha,
            deleted,
        } = filter;

        if let Some(id) = id {
            query = query.filter(rfd_revision::id.eq_any(id));
        }

        if let Some(rfd) = rfd {
            query = query.filter(rfd_revision::rfd_id.eq_any(rfd));
        }

        if let Some(sha) = sha {
            query = query.filter(rfd_revision::sha.eq_any(sha));
        }

        if !deleted {
            query = query.filter(rfd_revision::deleted_at.is_null());
        }

        let results = query
            .offset(pagination.offset)
            .limit(pagination.limit)
            .order(rfd_revision::rfd_id.asc())
            .get_results_async::<RfdRevisionModel>(&self.conn)
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
                rfd_revision::id.eq(new_revision.id),
                rfd_revision::rfd_id.eq(new_revision.rfd_id.clone()),
                rfd_revision::title.eq(new_revision.title.clone()),
                rfd_revision::state.eq(new_revision.state.clone()),
                rfd_revision::discussion.eq(new_revision.discussion.clone()),
                rfd_revision::authors.eq(new_revision.authors.clone()),
                rfd_revision::content.eq(new_revision.content.clone()),
                rfd_revision::content_format.eq(new_revision.content_format.clone()),
                rfd_revision::sha.eq(new_revision.sha.clone()),
                rfd_revision::commit_sha.eq(new_revision.commit_sha.clone()),
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
                rfd_revision::content.eq(excluded(rfd_revision::content)),
                rfd_revision::content_format.eq(excluded(rfd_revision::content_format)),
                rfd_revision::sha.eq(excluded(rfd_revision::sha)),
                rfd_revision::commit_sha.eq(rfd_revision::commit_sha),
                rfd_revision::committed_at.eq(excluded(rfd_revision::committed_at)),
                rfd_revision::updated_at.eq(Utc::now()),
            ))
            .get_result_async(&self.conn)
            .await?;

        Ok(rfd.into())
    }

    async fn delete(&self, id: &Uuid) -> Result<Option<RfdRevision>, StoreError> {
        let _ = update(rfd_revision::dsl::rfd_revision)
            .filter(rfd_revision::id.eq(*id))
            .set(rfd_revision::deleted_at.eq(Utc::now()))
            .execute_async(&self.conn)
            .await?;

        RfdRevisionStore::get(self, id, true).await
    }
}

#[async_trait]
impl RfdPdfStore for PostgresStore {
    async fn get(&self, id: &Uuid, deleted: bool) -> Result<Option<RfdPdf>, StoreError> {
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
        } = filter;

        if let Some(id) = id {
            query = query.filter(rfd_pdf::id.eq_any(id));
        }

        if let Some(rfd_revision) = rfd_revision {
            query = query.filter(rfd_pdf::rfd_revision_id.eq_any(rfd_revision));
        }

        if let Some(source) = source {
            query = query.filter(rfd_pdf::source.eq_any(source));
        }

        if !deleted {
            query = query.filter(rfd_pdf::deleted_at.is_null());
        }

        let results = query
            .offset(pagination.offset)
            .limit(pagination.limit)
            .order(rfd_pdf::created_at.desc())
            .get_results_async::<RfdPdfModel>(&self.conn)
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
                rfd_pdf::rfd_revision_id.eq(new_pdf.rfd_revision_id.clone()),
                rfd_pdf::source.eq(new_pdf.source.clone()),
                rfd_pdf::link.eq(new_pdf.link.clone()),
            ))
            .on_conflict(on_constraint("revision_links_unique"))
            .do_nothing()
            .get_result_async(&self.conn)
            .await?;

        Ok(rfd.into())
    }

    async fn delete(&self, id: &Uuid) -> Result<Option<RfdPdf>, StoreError> {
        let _ = update(rfd_pdf::dsl::rfd_pdf)
            .filter(rfd_pdf::id.eq(*id))
            .set(rfd_pdf::deleted_at.eq(Utc::now()))
            .execute_async(&self.conn)
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

        let JobFilter { id, sha, processed } = filter;

        if let Some(id) = id {
            query = query.filter(job::id.eq_any(id));
        }

        if let Some(sha) = sha {
            query = query.filter(job::sha.eq_any(sha));
        }

        if let Some(processed) = processed {
            query = query.filter(job::processed.eq(processed));
        }

        let results = query
            .offset(pagination.offset)
            .limit(pagination.limit)
            .order(job::processed.asc())
            .order(job::committed_at.asc())
            .order(job::created_at.asc())
            .get_results_async::<JobModel>(&self.conn)
            .await?;

        Ok(results.into_iter().map(|job| job.into()).collect())
    }

    async fn upsert(&self, new_job: NewJob) -> Result<Job, StoreError> {
        let rfd: JobModel = insert_into(job::dsl::job)
            .values((
                job::owner.eq(new_job.owner.clone()),
                job::repository.eq(new_job.repository.clone()),
                job::branch.eq(new_job.branch.clone()),
                job::sha.eq(new_job.sha.clone()),
                job::rfd.eq(new_job.rfd.clone()),
                job::webhook_delivery_id.eq(new_job.webhook_delivery_id.clone()),
                job::processed.eq(false),
                job::committed_at.eq(new_job.committed_at.clone()),
            ))
            .get_result_async(&self.conn)
            .await?;

        Ok(rfd.into())
    }

    async fn complete(&self, id: i32) -> Result<Option<Job>, StoreError> {
        let _ = update(job::dsl::job)
            .filter(job::id.eq(id))
            .set(job::processed.eq(true))
            .execute_async(&self.conn)
            .await?;

        JobStore::get(self, id).await
    }
}

#[async_trait]
impl<T> ApiUserStore<T> for PostgresStore
where
    T: Permission + Ord,
{
    async fn get(&self, id: &Uuid, deleted: bool) -> Result<Option<ApiUser<T>>, StoreError> {
        let user = ApiUserStore::list(
            self,
            ApiUserFilter {
                id: Some(vec![*id]),
                email: None,
                groups: None,
                deleted,
            },
            &ListPagination::default().limit(1),
        )
        .await?;
        Ok(user.into_iter().nth(0))
    }

    async fn list(
        &self,
        filter: ApiUserFilter,
        pagination: &ListPagination,
    ) -> Result<Vec<ApiUser<T>>, StoreError> {
        let mut query = api_user::dsl::api_user
            .left_join(api_user_provider::dsl::api_user_provider)
            .into_boxed();

        let ApiUserFilter {
            id,
            email,
            groups,
            deleted,
        } = filter;

        if let Some(id) = id {
            query = query.filter(api_user::id.eq_any(id));
        }

        if let Some(email) = email {
            query = query.filter(api_user_provider::emails.contains(email));
        }

        if let Some(groups) = groups {
            query = query.filter(api_user::groups.overlaps_with(groups));
        }

        if !deleted {
            query = query.filter(api_user::deleted_at.is_null());
        }

        let results = query
            .offset(pagination.offset)
            .limit(pagination.limit)
            .order(api_user::created_at.asc())
            .get_results_async::<(ApiUserModel<T>, Option<ApiUserProviderModel>)>(&self.conn)
            .await?;

        Ok(results
            .into_iter()
            .map(|(user, _)| ApiUser {
                id: user.id,
                permissions: user.permissions,
                groups: user.groups.into_iter().filter_map(|g| g).collect(),
                created_at: user.created_at,
                updated_at: user.updated_at,
                deleted_at: user.deleted_at,
            })
            .collect())
    }

    async fn upsert(&self, user: NewApiUser<T>) -> Result<ApiUser<T>, StoreError> {
        tracing::info!(id = ?user.id, permissions = ?user.permissions, "Upserting user");

        let user_m: ApiUserModel<T> = insert_into(api_user::dsl::api_user)
            .values((
                api_user::id.eq(user.id),
                api_user::permissions.eq(user.permissions.clone()),
                api_user::groups.eq(user.groups.clone()),
            ))
            .on_conflict(api_user::id)
            .do_update()
            .set((
                api_user::permissions.eq(excluded(api_user::permissions)),
                api_user::groups.eq(excluded(api_user::groups)),
                api_user::updated_at.eq(Utc::now()),
            ))
            .get_result_async(&self.conn)
            .await?;

        Ok(ApiUser {
            id: user_m.id,
            permissions: user_m.permissions,
            groups: user_m.groups.into_iter().filter_map(|g| g).collect(),
            created_at: user_m.created_at,
            updated_at: user_m.updated_at,
            deleted_at: user_m.deleted_at,
        })
    }

    async fn delete(&self, id: &Uuid) -> Result<Option<ApiUser<T>>, StoreError> {
        let _ = update(api_user::dsl::api_user)
            .filter(api_user::id.eq(*id))
            .set(api_user::deleted_at.eq(Utc::now()))
            .execute_async(&self.conn)
            .await?;

        ApiUserStore::get(self, id, true).await
    }
}

#[async_trait]
impl<T> ApiKeyStore<T> for PostgresStore
where
    T: Permission + Ord,
{
    async fn get(&self, id: &Uuid, deleted: bool) -> Result<Option<ApiKey<T>>, StoreError> {
        let mut query = api_key::dsl::api_key
            .into_boxed()
            .filter(api_key::id.eq(*id));

        if !deleted {
            query = query.filter(api_key::deleted_at.is_null());
        }

        let result = query
            .get_result_async::<ApiKeyModel<T>>(&self.conn)
            .await
            .optional()?;

        Ok(result.map(|key| ApiKey {
            id: key.id,
            api_user_id: key.api_user_id,
            key_signature: key.key_signature,
            permissions: key.permissions,
            expires_at: key.expires_at,
            created_at: key.created_at,
            updated_at: key.updated_at,
            deleted_at: key.deleted_at,
        }))
    }

    async fn list(
        &self,
        filter: ApiKeyFilter,
        pagination: &ListPagination,
    ) -> Result<Vec<ApiKey<T>>, StoreError> {
        let mut query = api_key::dsl::api_key.into_boxed();

        let ApiKeyFilter {
            id,
            api_user_id,
            key_signature,
            expired,
            deleted,
        } = filter;

        if let Some(id) = id {
            query = query.filter(api_key::id.eq_any(id));
        }

        if let Some(api_user_id) = api_user_id {
            query = query.filter(api_key::api_user_id.eq_any(api_user_id));
        }

        if let Some(key_signature) = key_signature {
            query = query.filter(api_key::key_signature.eq_any(key_signature));
        }

        if !expired {
            query = query.filter(api_key::expires_at.gt(Utc::now()));
        }

        if !deleted {
            query = query.filter(api_key::deleted_at.is_null());
        }

        let results = query
            .offset(pagination.offset)
            .limit(pagination.limit)
            .order(api_key::created_at.desc())
            .get_results_async::<ApiKeyModel<T>>(&self.conn)
            .await?;

        Ok(results
            .into_iter()
            .map(|token| ApiKey {
                id: token.id,
                api_user_id: token.api_user_id,
                key_signature: token.key_signature,
                permissions: token.permissions,
                expires_at: token.expires_at,
                created_at: token.created_at,
                updated_at: token.updated_at,
                deleted_at: token.deleted_at,
            })
            .collect())
    }

    async fn upsert(
        &self,
        key: NewApiKey<T>,
        api_user: &ApiUser<T>,
    ) -> Result<ApiKey<T>, StoreError> {
        // Validate the the token permissions are a subset of the users permissions
        let permissions: Permissions<T> = key
            .permissions
            .iter()
            .filter(|permission| {
                let can = api_user.permissions.can(permission);

                if !can {
                    tracing::info!(user = ?api_user.id, ?permission, "Attempted to create API token with excess permissions");
                }

                can
            })
            .cloned()
            .collect::<BTreeSet<T>>()
            .into();

        let key_m: ApiKeyModel<T> = insert_into(api_key::dsl::api_key)
            .values((
                api_key::id.eq(key.id),
                api_key::api_user_id.eq(key.api_user_id),
                api_key::key_signature.eq(key.key_signature.clone()),
                api_key::expires_at.eq(key.expires_at),
                api_key::permissions.eq(permissions),
            ))
            .get_result_async(&self.conn)
            .await?;

        Ok(ApiKey {
            id: key_m.id,
            api_user_id: key_m.api_user_id,
            key_signature: key_m.key_signature,
            permissions: key_m.permissions,
            expires_at: key_m.expires_at,
            created_at: key_m.created_at,
            updated_at: key_m.updated_at,
            deleted_at: key_m.deleted_at,
        })
    }

    async fn delete(&self, id: &Uuid) -> Result<Option<ApiKey<T>>, StoreError> {
        let _ = update(api_key::dsl::api_key)
            .filter(api_key::id.eq(*id))
            .set(api_key::deleted_at.eq(Utc::now()))
            .execute_async(&self.conn)
            .await?;

        ApiKeyStore::get(self, id, true).await
    }
}

#[async_trait]
impl ApiUserProviderStore for PostgresStore {
    async fn get(&self, id: &Uuid, deleted: bool) -> Result<Option<ApiUserProvider>, StoreError> {
        let user = ApiUserProviderStore::list(
            self,
            ApiUserProviderFilter {
                id: Some(vec![*id]),
                api_user_id: None,
                provider: None,
                provider_id: None,
                email: None,
                deleted,
            },
            &ListPagination::default().limit(1),
        )
        .await?;
        Ok(user.into_iter().nth(0))
    }

    async fn list(
        &self,
        filter: ApiUserProviderFilter,
        pagination: &ListPagination,
    ) -> Result<Vec<ApiUserProvider>, StoreError> {
        let mut query = api_user_provider::dsl::api_user_provider.into_boxed();

        let ApiUserProviderFilter {
            id,
            api_user_id,
            provider,
            provider_id,
            email,
            deleted,
        } = filter;

        if let Some(id) = id {
            query = query.filter(api_user_provider::id.eq_any(id));
        }

        if let Some(api_user_id) = api_user_id {
            query = query.filter(api_user_provider::api_user_id.eq_any(api_user_id));
        }

        if let Some(provider) = provider {
            query = query.filter(api_user_provider::provider.eq_any(provider));
        }

        if let Some(provider_id) = provider_id {
            query = query.filter(api_user_provider::provider_id.eq_any(provider_id));
        }

        if let Some(email) = email {
            query = query.filter(api_user_provider::emails.contains(email));
        }

        if !deleted {
            query = query.filter(api_user_provider::deleted_at.is_null());
        }

        let results = query
            .offset(pagination.offset)
            .limit(pagination.limit)
            .order(api_user_provider::created_at.desc())
            .get_results_async::<ApiUserProviderModel>(&self.conn)
            .await?;

        Ok(results
            .into_iter()
            .map(|provider| ApiUserProvider {
                id: provider.id,
                api_user_id: provider.api_user_id,
                provider: provider.provider,
                provider_id: provider.provider_id,
                emails: provider.emails.into_iter().filter_map(|e| e).collect(),
                created_at: provider.created_at,
                updated_at: provider.updated_at,
                deleted_at: provider.deleted_at,
            })
            .collect())
    }

    async fn upsert(&self, provider: NewApiUserProvider) -> Result<ApiUserProvider, StoreError> {
        tracing::info!(id = ?provider.id, api_user_id = ?provider.api_user_id, provider = ?provider, "Upserting user provider");

        let provider_m: ApiUserProviderModel =
            insert_into(api_user_provider::dsl::api_user_provider)
                .values((
                    api_user_provider::id.eq(provider.id),
                    api_user_provider::api_user_id.eq(provider.api_user_id),
                    api_user_provider::provider.eq(provider.provider),
                    api_user_provider::provider_id.eq(provider.provider_id),
                    api_user_provider::emails.eq(provider.emails),
                ))
                .on_conflict(api_user_provider::id)
                .do_update()
                .set((
                    api_user_provider::api_user_id.eq(excluded(api_user_provider::api_user_id)),
                    api_user_provider::updated_at.eq(Utc::now()),
                ))
                .get_result_async(&self.conn)
                .await?;

        Ok(ApiUserProvider {
            id: provider_m.id,
            api_user_id: provider_m.api_user_id,
            provider: provider_m.provider,
            provider_id: provider_m.provider_id,
            emails: provider_m.emails.into_iter().filter_map(|e| e).collect(),
            created_at: provider_m.created_at,
            updated_at: provider_m.updated_at,
            deleted_at: provider_m.deleted_at,
        })
    }

    async fn delete(&self, id: &Uuid) -> Result<Option<ApiUserProvider>, StoreError> {
        let _ = update(api_user::dsl::api_user)
            .filter(api_user::id.eq(*id))
            .set(api_user::deleted_at.eq(Utc::now()))
            .execute_async(&self.conn)
            .await?;

        ApiUserProviderStore::get(self, id, true).await
    }
}

#[async_trait]
impl AccessTokenStore for PostgresStore {
    async fn get(&self, id: &Uuid, revoked: bool) -> Result<Option<AccessToken>, StoreError> {
        let mut query = api_user_access_token::dsl::api_user_access_token
            .into_boxed()
            .filter(api_user_access_token::id.eq(*id));

        if !revoked {
            query = query.filter(api_user_access_token::revoked_at.is_null());
        }

        let result = query
            .get_result_async::<ApiUserAccessTokenModel>(&self.conn)
            .await
            .optional()?;

        Ok(result.map(|token| AccessToken {
            id: token.id,
            api_user_id: token.api_user_id,
            revoked_at: token.revoked_at,
            created_at: token.created_at,
            updated_at: token.updated_at,
        }))
    }

    async fn list(
        &self,
        filter: AccessTokenFilter,
        pagination: &ListPagination,
    ) -> Result<Vec<AccessToken>, StoreError> {
        let mut query = api_user_access_token::dsl::api_user_access_token.into_boxed();

        let AccessTokenFilter {
            id,
            api_user_id,
            revoked,
        } = filter;

        if let Some(id) = id {
            query = query.filter(api_user_access_token::id.eq_any(id));
        }

        if let Some(api_user_id) = api_user_id {
            query = query.filter(api_user_access_token::api_user_id.eq_any(api_user_id));
        }

        if !revoked {
            query = query.filter(api_user_access_token::revoked_at.gt(Utc::now()));
        }

        let results = query
            .offset(pagination.offset)
            .limit(pagination.limit)
            .order(api_user_access_token::created_at.desc())
            .get_results_async::<ApiUserAccessTokenModel>(&self.conn)
            .await?;

        Ok(results
            .into_iter()
            .map(|token| AccessToken {
                id: token.id,
                api_user_id: token.api_user_id,
                revoked_at: token.revoked_at,
                created_at: token.created_at,
                updated_at: token.updated_at,
            })
            .collect())
    }

    async fn upsert(&self, token: NewAccessToken) -> Result<AccessToken, StoreError> {
        let token_m: ApiUserAccessTokenModel =
            insert_into(api_user_access_token::dsl::api_user_access_token)
                .values((
                    api_user_access_token::id.eq(token.id),
                    api_user_access_token::api_user_id.eq(token.api_user_id),
                    api_user_access_token::revoked_at.eq(token.revoked_at),
                ))
                .on_conflict(api_user_access_token::id)
                .do_update()
                .set((api_user_access_token::revoked_at
                    .eq(excluded(api_user_access_token::revoked_at)),))
                .get_result_async(&self.conn)
                .await?;

        Ok(AccessToken {
            id: token_m.id,
            api_user_id: token_m.api_user_id,
            revoked_at: token_m.revoked_at,
            created_at: token_m.created_at,
            updated_at: token_m.updated_at,
        })
    }
}

#[async_trait]
impl LoginAttemptStore for PostgresStore {
    async fn get(&self, id: &Uuid) -> Result<Option<LoginAttempt>, StoreError> {
        let query = login_attempt::dsl::login_attempt
            .into_boxed()
            .filter(login_attempt::id.eq(*id));

        let result = query
            .get_result_async::<LoginAttemptModel>(&self.conn)
            .await
            .optional()?;

        Ok(result.map(|attempt| attempt.into()))
    }

    async fn list(
        &self,
        filter: LoginAttemptFilter,
        pagination: &ListPagination,
    ) -> Result<Vec<LoginAttempt>, StoreError> {
        let mut query = login_attempt::dsl::login_attempt.into_boxed();

        let LoginAttemptFilter {
            id,
            client_id,
            attempt_state,
            authz_code,
        } = filter;

        if let Some(id) = id {
            query = query.filter(login_attempt::id.eq_any(id));
        }

        if let Some(client_id) = client_id {
            query = query.filter(login_attempt::client_id.eq_any(client_id));
        }

        if let Some(attempt_state) = attempt_state {
            query = query.filter(login_attempt::attempt_state.eq_any(attempt_state));
        }

        if let Some(authz_code) = authz_code {
            query = query.filter(login_attempt::authz_code.eq_any(authz_code));
        }

        let results = query
            .offset(pagination.offset)
            .limit(pagination.limit)
            .order(login_attempt::created_at.desc())
            .get_results_async::<LoginAttemptModel>(&self.conn)
            .await?;

        Ok(results.into_iter().map(|model| model.into()).collect())
    }

    async fn upsert(&self, attempt: NewLoginAttempt) -> Result<LoginAttempt, StoreError> {
        let attempt_m: LoginAttemptModel = insert_into(login_attempt::dsl::login_attempt)
            .values((
                login_attempt::id.eq(attempt.id),
                login_attempt::attempt_state.eq(attempt.attempt_state),
                login_attempt::client_id.eq(attempt.client_id),
                login_attempt::redirect_uri.eq(attempt.redirect_uri),
                login_attempt::state.eq(attempt.state),
                login_attempt::pkce_challenge.eq(attempt.pkce_challenge),
                login_attempt::pkce_challenge_method.eq(attempt.pkce_challenge_method),
                login_attempt::authz_code.eq(attempt.authz_code),
                login_attempt::expires_at.eq(attempt.expires_at),
                login_attempt::provider.eq(attempt.provider),
                login_attempt::provider_pkce_verifier.eq(attempt.provider_pkce_verifier),
                login_attempt::provider_authz_code.eq(attempt.provider_authz_code),
            ))
            .on_conflict(login_attempt::id)
            .do_update()
            .set((
                login_attempt::attempt_state.eq(excluded(login_attempt::attempt_state)),
                login_attempt::authz_code.eq(excluded(login_attempt::authz_code)),
                login_attempt::expires_at.eq(excluded(login_attempt::expires_at)),
                login_attempt::provider_authz_code.eq(excluded(login_attempt::provider_authz_code)),
                login_attempt::updated_at.eq(Utc::now()),
            ))
            .get_result_async(&self.conn)
            .await?;

        Ok(attempt_m.into())
    }
}

#[async_trait]
impl OAuthClientStore for PostgresStore {
    async fn get(&self, id: &Uuid, deleted: bool) -> Result<Option<OAuthClient>, StoreError> {
        let client = OAuthClientStore::list(
            self,
            OAuthClientFilter {
                id: Some(vec![*id]),
                deleted,
            },
            &ListPagination::default().limit(1),
        )
        .await?;

        Ok(client.into_iter().nth(0))
    }

    async fn list(
        &self,
        filter: OAuthClientFilter,
        pagination: &ListPagination,
    ) -> Result<Vec<OAuthClient>, StoreError> {
        let mut query = oauth_client::dsl::oauth_client
            .left_join(oauth_client_secret::table)
            .left_join(oauth_client_redirect_uri::table)
            .into_boxed();

        let OAuthClientFilter { id, deleted } = filter;

        if let Some(id) = id {
            query = query.filter(oauth_client::id.eq_any(id));
        }

        if !deleted {
            query = query.filter(oauth_client::deleted_at.is_null());
        }

        let clients = query
            .offset(pagination.offset)
            .limit(pagination.limit)
            .order(oauth_client::created_at.desc())
            .load_async::<(
                OAuthClientModel,
                Option<OAuthClientSecretModel>,
                Option<OAuthClientRedirectUriModel>,
            )>(&self.conn)
            .await?
            .into_iter()
            .fold(
                BTreeMap::new(),
                |mut clients, (client, secret, redirect)| {
                    let value = clients.entry(client.id).or_insert((
                        client,
                        BTreeSet::<OAuthClientSecret>::new(),
                        BTreeSet::<OAuthClientRedirectUri>::new(),
                    ));

                    if let Some(secret) = secret {
                        value.1.insert(secret.into());
                    }

                    if let Some(redirect) = redirect {
                        value.2.insert(redirect.into());
                    }

                    clients
                },
            )
            .into_iter()
            .map(|(_, (client, secrets, redirect_uris))| OAuthClient {
                id: client.id,
                secrets: secrets.into_iter().collect::<Vec<_>>(),
                redirect_uris: redirect_uris.into_iter().collect::<Vec<_>>(),
                created_at: client.created_at,
                deleted_at: client.deleted_at,
            })
            .collect::<Vec<_>>();

        Ok(clients)
    }

    async fn upsert(&self, client: NewOAuthClient) -> Result<OAuthClient, StoreError> {
        let client_m: OAuthClientModel = insert_into(oauth_client::dsl::oauth_client)
            .values(oauth_client::id.eq(client.id))
            .get_result_async(&self.conn)
            .await?;

        Ok(OAuthClient {
            id: client_m.id,
            secrets: vec![],
            redirect_uris: vec![],
            created_at: client_m.created_at,
            deleted_at: client_m.deleted_at,
        })
    }

    async fn delete(&self, id: &Uuid) -> Result<Option<OAuthClient>, StoreError> {
        let _ = update(oauth_client::dsl::oauth_client)
            .filter(oauth_client::id.eq(*id))
            .set(oauth_client::deleted_at.eq(Utc::now()))
            .execute_async(&self.conn)
            .await?;

        OAuthClientStore::get(self, id, true).await
    }
}

#[async_trait]
impl OAuthClientSecretStore for PostgresStore {
    async fn upsert(&self, secret: NewOAuthClientSecret) -> Result<OAuthClientSecret, StoreError> {
        let secret_m: OAuthClientSecretModel =
            insert_into(oauth_client_secret::dsl::oauth_client_secret)
                .values((
                    oauth_client_secret::id.eq(secret.id),
                    oauth_client_secret::oauth_client_id.eq(secret.oauth_client_id),
                    oauth_client_secret::secret_signature.eq(secret.secret_signature),
                ))
                .get_result_async(&self.conn)
                .await?;

        Ok(OAuthClientSecret {
            id: secret_m.id,
            oauth_client_id: secret_m.oauth_client_id,
            secret_signature: secret_m.secret_signature,
            created_at: secret_m.created_at,
            deleted_at: secret_m.deleted_at,
        })
    }

    async fn delete(&self, id: &Uuid) -> Result<Option<OAuthClientSecret>, StoreError> {
        let _ = update(oauth_client_secret::dsl::oauth_client_secret)
            .filter(oauth_client_secret::id.eq(*id))
            .set(oauth_client_secret::deleted_at.eq(Utc::now()))
            .execute_async(&self.conn)
            .await?;

        let query = oauth_client_secret::dsl::oauth_client_secret
            .into_boxed()
            .filter(oauth_client_secret::id.eq(*id));

        let result = query
            .get_result_async::<OAuthClientSecretModel>(&self.conn)
            .await
            .optional()?;

        Ok(result.map(|secret| secret.into()))
    }
}

#[async_trait]
impl OAuthClientRedirectUriStore for PostgresStore {
    async fn upsert(
        &self,
        redirect_uri: NewOAuthClientRedirectUri,
    ) -> Result<OAuthClientRedirectUri, StoreError> {
        let redirect_uri_m: OAuthClientRedirectUriModel =
            insert_into(oauth_client_redirect_uri::dsl::oauth_client_redirect_uri)
                .values((
                    oauth_client_redirect_uri::id.eq(redirect_uri.id),
                    oauth_client_redirect_uri::oauth_client_id.eq(redirect_uri.oauth_client_id),
                    oauth_client_redirect_uri::redirect_uri.eq(redirect_uri.redirect_uri),
                ))
                .get_result_async(&self.conn)
                .await?;

        Ok(OAuthClientRedirectUri {
            id: redirect_uri_m.id,
            oauth_client_id: redirect_uri_m.oauth_client_id,
            redirect_uri: redirect_uri_m.redirect_uri,
            created_at: redirect_uri_m.created_at,
            deleted_at: redirect_uri_m.deleted_at,
        })
    }

    async fn delete(&self, id: &Uuid) -> Result<Option<OAuthClientRedirectUri>, StoreError> {
        let _ = update(oauth_client_redirect_uri::dsl::oauth_client_redirect_uri)
            .filter(oauth_client_redirect_uri::id.eq(*id))
            .set(oauth_client_redirect_uri::deleted_at.eq(Utc::now()))
            .execute_async(&self.conn)
            .await?;

        let query = oauth_client_redirect_uri::dsl::oauth_client_redirect_uri
            .into_boxed()
            .filter(oauth_client_redirect_uri::id.eq(*id));

        let result = query
            .get_result_async::<OAuthClientRedirectUriModel>(&self.conn)
            .await
            .optional()?;

        Ok(result.map(|redirect| redirect.into()))
    }
}

#[async_trait]
impl<T> AccessGroupStore<T> for PostgresStore
where
    T: Permission + Ord,
{
    async fn get(&self, id: &Uuid, deleted: bool) -> Result<Option<AccessGroup<T>>, StoreError> {
        let client = AccessGroupStore::list(
            self,
            AccessGroupFilter {
                id: Some(vec![*id]),
                name: None,
                deleted,
            },
            &ListPagination::default().limit(1),
        )
        .await?;

        Ok(client.into_iter().nth(0))
    }

    async fn list(
        &self,
        filter: AccessGroupFilter,
        pagination: &ListPagination,
    ) -> Result<Vec<AccessGroup<T>>, StoreError> {
        let mut query = access_groups::dsl::access_groups.into_boxed();

        let AccessGroupFilter { id, name, deleted } = filter;

        if let Some(id) = id {
            query = query.filter(access_groups::id.eq_any(id));
        }

        if let Some(name) = name {
            query = query.filter(access_groups::name.eq_any(name));
        }

        if !deleted {
            query = query.filter(access_groups::deleted_at.is_null());
        }

        let results = query
            .offset(pagination.offset)
            .limit(pagination.limit)
            .order(access_groups::created_at.desc())
            .get_results_async::<AccessGroupModel<T>>(&self.conn)
            .await?;

        Ok(results.into_iter().map(|model| model.into()).collect())
    }

    async fn upsert(&self, group: &NewAccessGroup<T>) -> Result<AccessGroup<T>, StoreError> {
        let group_m: AccessGroupModel<T> = insert_into(access_groups::dsl::access_groups)
            .values((
                access_groups::id.eq(group.id),
                access_groups::name.eq(group.name.clone()),
                access_groups::permissions.eq(group.permissions.clone()),
            ))
            .on_conflict(access_groups::id)
            .do_update()
            .set((
                access_groups::name.eq(excluded(access_groups::name)),
                access_groups::permissions.eq(excluded(access_groups::permissions)),
                access_groups::updated_at.eq(Utc::now()),
            ))
            .get_result_async(&self.conn)
            .await?;

        Ok(group_m.into())
    }

    async fn delete(&self, id: &Uuid) -> Result<Option<AccessGroup<T>>, StoreError> {
        let _ = update(access_groups::dsl::access_groups)
            .filter(access_groups::id.eq(*id))
            .set(access_groups::deleted_at.eq(Utc::now()))
            .execute_async(&self.conn)
            .await?;

        AccessGroupStore::get(self, id, true).await
    }
}

#[async_trait]
impl MapperStore for PostgresStore {
    async fn get(&self, id: &Uuid, depleted: bool, deleted: bool) -> Result<Option<Mapper>, StoreError> {
        let client = MapperStore::list(
            self,
            MapperFilter {
                id: Some(vec![*id]),
                name: None,
                depleted,
                deleted,
            },
            &ListPagination::default().limit(1),
        )
        .await?;

        Ok(client.into_iter().nth(0))
    }

    async fn list(
        &self,
        filter: MapperFilter,
        pagination: &ListPagination,
    ) -> Result<Vec<Mapper>, StoreError> {
        let mut query = mapper::dsl::mapper.into_boxed();

        let MapperFilter { id, name, depleted, deleted } = filter;

        if let Some(id) = id {
            query = query.filter(mapper::id.eq_any(id));
        }

        if let Some(name) = name {
            query = query.filter(mapper::name.eq_any(name));
        }

        if !depleted {
            query = query.filter(mapper::depleted_at.is_null());
        }

        if !deleted {
            query = query.filter(mapper::deleted_at.is_null());
        }

        let results = query
            .offset(pagination.offset)
            .limit(pagination.limit)
            .order(mapper::created_at.desc())
            .get_results_async::<MapperModel>(&self.conn)
            .await?;

        Ok(results.into_iter().map(|model| model.into()).collect())
    }
    async fn upsert(&self, new_mapper: &NewMapper) -> Result<Mapper, StoreError> {
        let mapper_m: MapperModel = insert_into(mapper::dsl::mapper)
            .values((
                mapper::id.eq(new_mapper.id),
                mapper::name.eq(new_mapper.name.clone()),
                mapper::rule.eq(new_mapper.rule.clone()),
                mapper::activations.eq(new_mapper.activations),
                mapper::max_activations.eq(new_mapper.max_activations),
                mapper::depleted_at.eq(
                    if new_mapper.activations == new_mapper.max_activations {
                        Some(Utc::now())
                    } else {
                        None
                    }
                ),
            ))
            .on_conflict(mapper::id)
            .do_update()
            .set((
                mapper::activations.eq(excluded(mapper::activations)),
                mapper::depleted_at.eq(excluded(mapper::depleted_at)),
            ))
            .get_result_async(&self.conn)
            .await?;

        Ok(mapper_m.into())
    }
    async fn delete(&self, id: &Uuid) -> Result<Option<Mapper>, StoreError> {
        let _ = update(mapper::dsl::mapper)
            .filter(mapper::id.eq(*id))
            .set(mapper::deleted_at.eq(Utc::now()))
            .execute_async(&self.conn)
            .await?;

        MapperStore::get(self, id, false, true).await
    }
}
