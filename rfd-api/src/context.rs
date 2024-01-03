// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use chrono::{DateTime, Duration, Utc};
use dropshot::{HttpError, RequestContext};
use http::StatusCode;
use hyper::{client::HttpConnector, Body, Client};
use hyper_rustls::{HttpsConnector, HttpsConnectorBuilder};
use jsonwebtoken::jwk::JwkSet;
use oauth2::CsrfToken;
use partial_struct::partial;
use rfd_model::{
    permissions::{Caller, Permissions},
    schema_ext::{LoginAttemptState, Visibility},
    storage::{
        AccessGroupFilter, AccessGroupStore, AccessTokenStore, ApiKeyFilter, ApiKeyStore,
        ApiUserFilter, ApiUserProviderFilter, ApiUserProviderStore, ApiUserStore, JobStore,
        LinkRequestStore, ListPagination, LoginAttemptFilter, LoginAttemptStore, MapperFilter,
        MapperStore, OAuthClientFilter, OAuthClientRedirectUriStore, OAuthClientSecretStore,
        OAuthClientStore, RfdFilter, RfdPdfFilter, RfdPdfStore, RfdRevisionFilter,
        RfdRevisionStore, RfdStore, StoreError,
    },
    AccessGroup, AccessToken, ApiUser, ApiUserProvider, InvalidValueError, Job, LinkRequest,
    LoginAttempt, Mapper, NewAccessGroup, NewAccessToken, NewApiKey, NewApiUser,
    NewApiUserProvider, NewJob, NewLinkRequest, NewLoginAttempt, NewMapper, NewOAuthClient,
    NewOAuthClientRedirectUri, NewOAuthClientSecret, OAuthClient, OAuthClientRedirectUri,
    OAuthClientSecret,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeSet, HashMap},
    ops::Add,
    sync::Arc,
};
use tap::TapFallible;
use thiserror::Error;
use tracing::{info_span, instrument, Instrument};
use uuid::Uuid;

use crate::{
    authn::{
        jwt::{Claims, JwtSigner, JwtSignerError},
        key::{RawApiKey, SignedApiKey},
        AuthError, AuthToken, Signer,
    },
    config::{AsymmetricKey, JwtConfig, SearchConfig},
    endpoints::login::{
        oauth::{
            ClientType, OAuthProvider, OAuthProviderError, OAuthProviderFn, OAuthProviderName,
        },
        UserInfo,
    },
    error::{ApiError, AppError},
    mapper::{MapperRule, Mapping},
    permissions::{ApiPermission, ApiPermissionError, PermissionStorage},
    search::SearchClient,
    util::response::{bad_request, client_error, internal_error},
    ApiCaller, ApiPermissions, User, UserToken,
};

static UNLIMITED: i64 = 9999999;

pub trait Storage:
    RfdStore
    + RfdRevisionStore
    + RfdPdfStore
    + JobStore
    + ApiUserStore<ApiPermission>
    + ApiKeyStore<ApiPermission>
    + ApiUserProviderStore
    + AccessTokenStore
    + LoginAttemptStore
    + OAuthClientStore
    + OAuthClientSecretStore
    + OAuthClientRedirectUriStore
    + AccessGroupStore<ApiPermission>
    + MapperStore
    + LinkRequestStore
    + Send
    + Sync
    + 'static
{
}
impl<T> Storage for T where
    T: RfdStore
        + RfdRevisionStore
        + RfdPdfStore
        + JobStore
        + ApiUserStore<ApiPermission>
        + ApiKeyStore<ApiPermission>
        + ApiUserProviderStore
        + AccessTokenStore
        + LoginAttemptStore
        + OAuthClientStore
        + OAuthClientSecretStore
        + OAuthClientRedirectUriStore
        + AccessGroupStore<ApiPermission>
        + MapperStore
        + LinkRequestStore
        + Send
        + Sync
        + 'static
{
}

pub struct ApiContext {
    pub https_client: Client<HttpsConnector<HttpConnector>, Body>,
    pub public_url: String,
    pub storage: Arc<dyn Storage>,
    unauthenticated_caller: ApiCaller,
    registration_caller: ApiCaller,
    pub jwt: JwtContext,
    pub secrets: SecretContext,
    pub oauth_providers: HashMap<OAuthProviderName, Box<dyn OAuthProviderFn>>,
    pub search: SearchContext,
    pub system_caller: Caller<ApiPermission>,
}

pub struct JwtContext {
    pub default_expiration: i64,
    pub signers: Vec<JwtSigner>,
    pub jwks: JwkSet,
}

pub struct SecretContext {
    pub signer: Arc<dyn Signer>,
}

pub struct SearchContext {
    pub client: SearchClient,
}

pub struct RegisteredAccessToken {
    pub access_token: AccessToken,
    pub signed_token: String,
    pub expires_at: DateTime<Utc>,
}

#[derive(Debug, Error)]
pub enum CallerError {
    #[error("Failed to authenticate caller")]
    FailedToAuthenticate,
    #[error("Supplied API key is invalid")]
    InvalidKey,
    #[error("Invalid scope: {0}")]
    Scope(#[from] ApiPermissionError),
    #[error("Inner storage failure: {0}")]
    Storage(#[from] StoreError),
}

impl From<CallerError> for HttpError {
    fn from(error: CallerError) -> Self {
        tracing::info!(?error, "Failed to authenticate caller");

        match error {
            CallerError::FailedToAuthenticate => {
                client_error(StatusCode::UNAUTHORIZED, "Failed to authenticate")
            }
            CallerError::InvalidKey => {
                client_error(StatusCode::UNAUTHORIZED, "Failed to authenticate")
            }
            CallerError::Scope(_) => bad_request("Invalid scope"),
            CallerError::Storage(_) => internal_error("Internal storage failed"),
        }
    }
}

#[derive(Debug, Error)]
pub enum LoginAttemptError {
    #[error(transparent)]
    FailedToCreate(#[from] InvalidValueError),
    #[error(transparent)]
    Storage(#[from] StoreError),
}

#[partial(ListRfd)]
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct FullRfd {
    pub id: Uuid,
    pub rfd_number: i32,
    pub link: Option<String>,
    pub discussion: Option<String>,
    pub title: String,
    pub state: Option<String>,
    pub authors: Option<String>,
    #[partial(ListRfd(skip))]
    pub content: String,
    pub sha: String,
    pub commit: String,
    pub committed_at: DateTime<Utc>,
    #[partial(ListRfd(skip))]
    pub pdfs: Vec<FullRfdPdfEntry>,
    pub visibility: Visibility,
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct FullRfdPdfEntry {
    pub source: String,
    pub link: String,
}

impl ApiContext {
    pub async fn new(
        public_url: String,
        storage: Arc<dyn Storage>,
        jwt: JwtConfig,
        keys: Vec<AsymmetricKey>,
        search: SearchConfig,
    ) -> Result<Self, AppError> {
        let mut jwt_signers = vec![];

        for key in &keys {
            jwt_signers.push(JwtSigner::new(&key).await.unwrap())
        }

        Ok(Self {
            https_client: hyper::Client::builder().build(
                HttpsConnectorBuilder::new()
                    .with_native_roots()
                    .https_only()
                    .enable_http1()
                    .build(),
            ),
            public_url,
            storage,

            unauthenticated_caller: ApiCaller {
                id: Uuid::parse_str("00000000-0000-4000-8000-000000000000").unwrap(),
                permissions: vec![ApiPermission::SearchRfds].into(),
            },
            registration_caller: ApiCaller {
                id: Uuid::parse_str("00000000-0000-4000-8000-000000000001").unwrap(),
                permissions: vec![
                    ApiPermission::CreateApiUser,
                    ApiPermission::CreateGroup,
                    ApiPermission::ListMappers,
                ]
                .into(),
            },
            jwt: JwtContext {
                default_expiration: jwt.default_expiration,
                jwks: JwkSet {
                    keys: jwt_signers.iter().map(|k| k.jwk()).cloned().collect(),
                },
                signers: jwt_signers,
            },
            secrets: SecretContext {
                signer: keys[0].as_signer().await?,
            },
            oauth_providers: HashMap::new(),
            search: SearchContext {
                client: SearchClient::new(search.host, search.index, search.key),
            },
            system_caller: Caller {
                id: Uuid::new_v4(),
                permissions: vec![ApiPermission::GetGroupsAll].into(),
            },
        })
    }

    pub fn device_client(&self) -> ClientType {
        ClientType::Device
    }

    pub fn web_client(&self) -> ClientType {
        ClientType::Web {
            prefix: self.public_url.to_string(),
        }
    }

    pub fn set_storage(&mut self, storage: Arc<dyn Storage>) {
        self.storage = storage;
    }

    pub async fn authn_token(
        &self,
        rqctx: &RequestContext<Self>,
    ) -> Result<Option<AuthToken>, AuthError> {
        match AuthToken::extract(rqctx).await {
            Ok(token) => Ok(Some(token)),
            Err(err) => match err {
                AuthError::NoToken => Ok(None),
                other => Err(other),
            },
        }
    }

    pub fn default_jwt_expiration(&self) -> i64 {
        self.jwt.default_expiration
    }

    pub async fn jwks(&self) -> &JwkSet {
        &self.jwt.jwks
    }

    pub async fn sign_jwt(&self, claims: &Claims) -> Result<String, JwtSignerError> {
        let signer = self.jwt.signers.first().unwrap();
        signer.sign(claims).await
    }

    #[instrument(skip(self, auth))]
    pub async fn get_caller(&self, auth: Option<&AuthToken>) -> Result<ApiCaller, CallerError> {
        match auth {
            Some(token) => {
                let (api_user_id, permissions) = self.get_base_permissions(&token).await?;

                // The permissions for the caller is the intersection of the user's permissions and the tokens permissions
                if let Some(user) = self.get_api_user(&api_user_id).await? {
                    let user_permissions = self.get_user_permissions(&user).await?;
                    let token_permissions = permissions.expand(&user.id, Some(&user_permissions));

                    let combined_permissions = token_permissions.intersect(&user_permissions);

                    tracing::trace!(token = ?token_permissions, user = ?user_permissions, combined = ?combined_permissions, "Computed caller permissions");

                    let caller = Caller {
                        id: api_user_id,
                        permissions: combined_permissions,
                    };

                    tracing::info!(?caller.id, "Resolved caller");
                    tracing::debug!(?caller.permissions, "Caller permissions");

                    Ok(caller)
                } else {
                    tracing::error!("User for verified token does not exist");
                    Err(CallerError::FailedToAuthenticate)
                }
            }
            None => Ok(self.builtin_unauthenticated_caller().clone()),
        }
    }

    pub fn builtin_unauthenticated_caller(&self) -> &ApiCaller {
        &self.unauthenticated_caller
    }

    pub fn builtin_registration_user(&self) -> &ApiCaller {
        &self.registration_caller
    }

    async fn get_base_permissions(
        &self,
        auth: &AuthToken,
    ) -> Result<(Uuid, Permissions<ApiPermission>), CallerError> {
        Ok(match auth {
            AuthToken::ApiKey(api_key) => {
                async {
                    tracing::debug!("Attempt to authenticate");

                    let id = Uuid::from_slice(api_key.id()).map_err(|err| {
                    tracing::info!(?err, slice = ?api_key.id(), "Failed to parse id from API key");
                    CallerError::InvalidKey
                })?;

                    let mut key = ApiKeyStore::list(
                        &*self.storage,
                        ApiKeyFilter {
                            id: Some(vec![id]),
                            expired: false,
                            deleted: false,
                            ..Default::default()
                        },
                        &ListPagination {
                            offset: 0,
                            limit: 1,
                        },
                    )
                    .await?;

                    if let Some(key) = key.pop() {
                        if let Err(err) =
                            api_key.verify(&*self.secrets.signer, key.key_signature.as_bytes())
                        {
                            tracing::debug!(?err, "Failed to verify api key");
                            Err(CallerError::FailedToAuthenticate)
                        } else {
                            tracing::debug!("Verified caller key");
                            Ok((key.api_user_id, key.permissions))
                        }
                    } else {
                        tracing::debug!("Failed to find matching key");
                        Err(CallerError::FailedToAuthenticate)
                    }
                }
                .instrument(info_span!("Test api key"))
                .await
            }
            AuthToken::Jwt(jwt) => {
                // AuthnToken::Jwt can only be generated from a verified JWT
                let permissions = ApiPermission::from_scope(jwt.claims.scp.iter())?;
                Ok((jwt.claims.sub, permissions))
            }
        }?)
    }

    #[instrument(skip(self), fields(user_id = ?user.id, groups = ?user.groups))]
    async fn get_user_permissions(
        &self,
        user: &ApiUser<ApiPermission>,
    ) -> Result<ApiPermissions, StoreError> {
        let mut group_permissions = self.get_user_group_permissions(&user).await?;
        let mut permissions = user.permissions.clone();
        permissions.append(&mut group_permissions);

        Ok(permissions)
    }

    async fn get_user_group_permissions(
        &self,
        user: &ApiUser<ApiPermission>,
    ) -> Result<Permissions<ApiPermission>, StoreError> {
        tracing::trace!("Expanding groups into permissions");

        let groups = AccessGroupStore::list(
            &*self.storage,
            AccessGroupFilter {
                id: Some(user.groups.iter().copied().collect()),
                ..Default::default()
            },
            &ListPagination::default().limit(UNLIMITED),
        )
        .await?;

        tracing::trace!(?groups, "Found groups to map to permissions");

        let permissions = groups
            .into_iter()
            .fold(ApiPermissions::new(), |mut aggregate, group| {
                let mut expanded = group.permissions.expand(&user.id, Some(&user.permissions));

                tracing::trace!(group_id = ?group.id, group_name = ?group.name, permissions = ?expanded, "Transformed group into permission set");
                aggregate.append(&mut expanded);

                aggregate
            });

        Ok(permissions)
    }

    pub async fn is_empty(&self) -> Result<bool, StoreError> {
        let mut user_filter = ApiUserFilter::default();
        user_filter.deleted = true;

        let users =
            ApiUserStore::list(&*self.storage, user_filter, &ListPagination::latest()).await?;

        let mut token_filter = ApiKeyFilter::default();
        token_filter.deleted = true;

        let tokens =
            ApiKeyStore::list(&*self.storage, token_filter, &ListPagination::latest()).await?;

        Ok(users.len() == 0 && tokens.len() == 0)
    }

    pub fn insert_oauth_provider(
        &mut self,
        name: OAuthProviderName,
        provider_fn: Box<dyn OAuthProviderFn>,
    ) {
        self.oauth_providers.insert(name, provider_fn);
    }

    pub async fn get_oauth_provider(
        &self,
        provider: &OAuthProviderName,
    ) -> Result<Box<dyn OAuthProvider + Send + Sync>, OAuthProviderError> {
        self.oauth_providers
            .get(provider)
            .map(|factory| (*factory)())
            .ok_or(OAuthProviderError::FailToCreateInvalidProvider)
    }

    // RFD Operations

    pub async fn list_rfds(
        &self,
        caller: &ApiCaller,
        filter: Option<RfdFilter>,
    ) -> Result<Vec<ListRfd>, StoreError> {
        // List all of the RFDs first and then perform filter. This should be be improved once
        // filters can be combined to support OR expressions. Effectively we need to be able to
        // express "Has access to OR is public" with a filter
        let mut rfds = RfdStore::list(
            &*self.storage,
            filter.unwrap_or_default(),
            &ListPagination::default().limit(UNLIMITED),
        )
        .await
        .tap_err(|err| tracing::error!(?err, "Failed to lookup RFDs"))?;

        // Determine the list of RFDs the caller has direct access to
        let direct_access_rfds = caller
            .permissions
            .iter()
            .filter_map(|p| match p {
                ApiPermission::GetRfd(number) => Some(*number),
                _ => None,
            })
            .collect::<BTreeSet<_>>();

        // Filter the list of RFDs down to only those that the caller is allowed to access
        rfds.retain_mut(|rfd| {
            caller.can(&ApiPermission::GetRfdsAll)
                || rfd.visibility == Visibility::Public
                || direct_access_rfds.contains(&rfd.rfd_number)
        });

        // Fetch the latest revision for each of the RFDs that is to be returned
        let mut rfd_revisions = RfdRevisionStore::list_unique_rfd(
            &*self.storage,
            RfdRevisionFilter::default().rfd(Some(rfds.iter().map(|rfd| rfd.id).collect())),
            &ListPagination::default().limit(UNLIMITED),
        )
        .await
        .tap_err(|err| tracing::error!(?err, "Failed to lookup RFD revisions"))?;

        // Sort both the RFDs and revisions based on their RFD id to ensure they line up
        rfds.sort_by(|a, b| a.id.cmp(&b.id));
        rfd_revisions.sort_by(|a, b| a.rfd_id.cmp(&b.rfd_id));

        // Zip together the RFDs with their associated revision
        let mut rfd_list = rfds
            .into_iter()
            .zip(rfd_revisions)
            .map(|(rfd, revision)| ListRfd {
                id: rfd.id,
                rfd_number: rfd.rfd_number,
                link: rfd.link,
                discussion: revision.discussion,
                title: revision.title,
                state: revision.state,
                authors: revision.authors,
                sha: revision.sha,
                commit: revision.commit_sha,
                committed_at: revision.committed_at,
                visibility: rfd.visibility,
            })
            .collect::<Vec<_>>();

        // Finally sort the RFD list by RFD number
        rfd_list.sort_by(|a, b| b.rfd_number.cmp(&a.rfd_number));

        Ok(rfd_list)
    }

    #[instrument(skip(self, caller))]
    pub async fn get_rfd(
        &self,
        caller: &ApiCaller,
        rfd_number: i32,
        sha: Option<String>,
    ) -> Result<Option<FullRfd>, StoreError> {
        // list_rfds performs authorization checks, if the caller does not have access to the
        // requested RFD any empty Vec will be returned
        let rfds = self
            .list_rfds(
                caller,
                Some(RfdFilter::default().rfd_number(Some(vec![rfd_number]))),
            )
            .await?;

        if let Some(rfd) = rfds.into_iter().nth(0) {
            // If list_rfds returned an RFD, then the caller is allowed to access that RFD and we
            // can return the full RFD revision. This is sub-optimal as we are required to execute
            // the revision lookup twice
            let latest_revision = RfdRevisionStore::list(
                &*self.storage,
                RfdRevisionFilter::default()
                    .rfd(Some(vec![rfd.id]))
                    .sha(sha.map(|sha| vec![sha])),
                &ListPagination::default().limit(1),
            )
            .await?;

            if let Some(revision) = latest_revision.into_iter().nth(0) {
                let pdfs = RfdPdfStore::list(
                    &*self.storage,
                    RfdPdfFilter::default().rfd_revision(Some(vec![revision.id])),
                    &ListPagination::default(),
                )
                .await?;

                Ok(Some(FullRfd {
                    id: rfd.id,
                    rfd_number: rfd.rfd_number,
                    link: rfd.link,
                    discussion: revision.discussion,
                    title: revision.title,
                    state: revision.state,
                    authors: revision.authors,
                    content: revision.content,
                    sha: revision.sha,
                    commit: revision.commit_sha,
                    committed_at: revision.committed_at,
                    pdfs: pdfs
                        .into_iter()
                        .map(|pdf| FullRfdPdfEntry {
                            source: pdf.source.to_string(),
                            link: pdf.link,
                        })
                        .collect(),
                    visibility: rfd.visibility,
                }))
            } else {
                // It should not be possible to reach this branch. If we have then the database
                // has entered an inconsistent state
                tracing::error!("Looking up revision for RFD returned no results");
                Ok(None)
            }
        } else {
            // Either the RFD does not exist, or the caller is not allowed to access it
            Ok(None)
        }
    }

    // Webhook Operations

    pub async fn register_job(&self, new_job: NewJob) -> Result<Job, StoreError> {
        JobStore::upsert(&*self.storage, new_job).await
    }

    // Login Operations

    #[instrument(skip(self, info), fields(info.external_id))]
    pub async fn register_api_user(
        &self,
        caller: &ApiCaller,
        info: UserInfo,
    ) -> Result<(User, ApiUserProvider), ApiError> {
        // Check if we have seen this identity before
        let mut filter = ApiUserProviderFilter::default();
        filter.provider = Some(vec![info.external_id.provider().to_string()]);
        filter.provider_id = Some(vec![info.external_id.id().to_string()]);

        tracing::info!("Check for existing users matching the requested external id");

        let api_user_providers = self
            .list_api_user_provider(filter, &ListPagination::latest())
            .await?;

        let (mapped_permissions, mapped_groups) = self.get_mapped_fields(caller, &info).await?;

        match api_user_providers.len() {
            0 => {
                tracing::info!(
                    ?mapped_permissions,
                    ?mapped_groups,
                    "Did not find any existing users. Registering a new user."
                );

                let user = self
                    .ensure_api_user(Uuid::new_v4(), mapped_permissions, mapped_groups)
                    .await?;
                let user_provider = self
                    .update_api_user_provider(NewApiUserProvider {
                        id: Uuid::new_v4(),
                        api_user_id: user.id,
                        emails: info.verified_emails,
                        provider: info.external_id.provider().to_string(),
                        provider_id: info.external_id.id().to_string(),
                    })
                    .await?;

                Ok((user, user_provider))
            }
            1 => {
                tracing::info!("Found an existing user. Attaching provider.");

                // This branch ensures that there is a 0th indexed item
                let provider = api_user_providers.into_iter().nth(0).unwrap();
                Ok((
                    self.ensure_api_user(provider.api_user_id, mapped_permissions, mapped_groups)
                        .await?,
                    provider,
                ))
            }
            _ => {
                // If we found more than one provider, then we have encountered an inconsistency in
                // our database.
                tracing::error!(
                    count = api_user_providers.len(),
                    "Found multiple providers for external id"
                );

                Err(StoreError::InvariantFailed(
                    "Multiple providers for external id found".to_string(),
                )
                .into())
            }
        }
    }

    async fn get_mapped_fields(
        &self,
        caller: &Caller<ApiPermission>,
        info: &UserInfo,
    ) -> Result<(ApiPermissions, BTreeSet<Uuid>), StoreError> {
        let mut mapped_permissions = ApiPermissions::new();
        let mut mapped_groups = BTreeSet::new();

        // We optimistically load mappers here. We do not want to take a lock on the mappers and
        // instead handle mappers that become depleted before we can evaluate them at evaluation
        // time.
        for mapping in self.get_mappings(caller).await? {
            let (permissions, groups) = (
                mapping.rule.permissions_for(&self, &info).await?,
                mapping.rule.groups_for(&self, &info).await?,
            );

            // If a rule is set to apply a permission or group to a user, then the rule needs to be
            // checked for usage. If it does not have an activation limit then nothing is needed.
            // If it does have a limit then we need to attempt to consume an activation. If the
            // consumption works then we add the permissions. If they fail then we do not, but we
            // do not fail the entire mapping process
            let apply = if !permissions.is_empty() || !groups.is_empty() {
                if mapping.max_activations.is_some() {
                    match self.consume_mapping_activation(&mapping).await {
                        Ok(_) => true,
                        Err(err) => {
                            // TODO: Inspect the error. We expect to see a conflict error, and
                            // should is expected to be seen. Other errors are problematic.
                            tracing::warn!(?err, "Login may have attempted to use depleted mapper. This may be ok if it is an isolated occurrence, but should occur repeatedly.");
                            false
                        }
                    }
                } else {
                    true
                }
            } else {
                false
            };

            if apply {
                mapped_permissions.append(&mut mapping.rule.permissions_for(&self, &info).await?);
                mapped_groups.append(&mut mapping.rule.groups_for(&self, &info).await?);
            }
        }

        Ok((mapped_permissions, mapped_groups))
    }

    #[instrument(skip(self), err(Debug))]
    async fn ensure_api_user(
        &self,
        api_user_id: Uuid,
        mut mapped_permissions: ApiPermissions,
        mut mapped_groups: BTreeSet<Uuid>,
    ) -> Result<User, ApiError> {
        match self.get_api_user(&api_user_id).await? {
            Some(api_user) => {
                // Ensure that the existing user has "at least" the mapped permissions
                let mut update: NewApiUser<ApiPermission> = api_user.into();
                update.permissions.append(&mut mapped_permissions);
                update.groups.append(&mut mapped_groups);

                Ok(ApiUserStore::upsert(&*self.storage, update).await?)
            }
            None => self
                .update_api_user(NewApiUser {
                    id: api_user_id,
                    permissions: mapped_permissions,
                    groups: mapped_groups,
                })
                .await
                .map_err(ApiError::Storage)
                .tap_err(|err| {
                    tracing::error!(
                        ?err,
                        "Failed to create new api user for OAuth authenticated user"
                    )
                }),
        }
    }

    pub async fn register_access_token(
        &self,
        api_user: &ApiUser<ApiPermission>,
        api_user_provider: &ApiUserProvider,
        scope: Vec<String>,
    ) -> Result<RegisteredAccessToken, ApiError> {
        let expires_at = Utc::now() + Duration::seconds(self.default_jwt_expiration());

        let claims = Claims::new(self, &api_user, &api_user_provider, scope, expires_at);
        let token = self
            .create_access_token(NewAccessToken {
                id: claims.jti,
                api_user_id: api_user.id,
                revoked_at: None,
            })
            .await?;

        let signed = self.sign_jwt(&claims).await?;

        Ok(RegisteredAccessToken {
            access_token: token,
            signed_token: signed,
            expires_at,
        })
    }

    // API User Operations

    pub async fn get_api_user(&self, id: &Uuid) -> Result<Option<User>, StoreError> {
        let user = ApiUserStore::get(&*self.storage, id, false)
            .await?
            .map(|mut user| {
                user.permissions = user.permissions.expand(&user.id, None);
                user
            });

        Ok(user)
    }

    pub async fn list_api_user(
        &self,
        filter: ApiUserFilter,
        pagination: &ListPagination,
    ) -> Result<Vec<User>, StoreError> {
        ApiUserStore::list(&*self.storage, filter, pagination).await
    }

    #[instrument(skip(self))]
    pub async fn update_api_user(
        &self,
        mut api_user: NewApiUser<ApiPermission>,
    ) -> Result<User, StoreError> {
        api_user.permissions = api_user.permissions.contract(&api_user.id);
        ApiUserStore::upsert(&*self.storage, api_user).await
    }

    pub async fn add_permissions_to_user(
        &self,
        user_id: &Uuid,
        new_permissions: Permissions<ApiPermission>,
    ) -> Result<Option<User>, StoreError> {
        Ok(match self.get_api_user(user_id).await? {
            Some(user) => {
                let mut user_update: NewApiUser<ApiPermission> = user.into();
                for permission in new_permissions.into_iter() {
                    tracing::info!(id = ?user_id, ?permission, "Adding permission to user");
                    user_update.permissions.insert(permission);
                }

                Some(self.update_api_user(user_update).await?)
            }
            None => None,
        })
    }

    pub async fn remove_permissions_from_user(
        &self,
        api_user: &ApiUser<ApiPermission>,
        new_permissions: Permissions<ApiPermission>,
    ) -> Result<User, StoreError> {
        let mut user_update: NewApiUser<ApiPermission> = api_user.clone().into();
        for permission in new_permissions.into_iter() {
            tracing::info!(id = ?api_user.id, ?permission, "Removing permission from user");
            user_update.permissions.remove(&permission);
        }

        self.update_api_user(user_update).await
    }

    pub async fn create_api_user_token(
        &self,
        token: NewApiKey<ApiPermission>,
        api_user: &ApiUser<ApiPermission>,
    ) -> Result<UserToken, StoreError> {
        ApiKeyStore::upsert(&*self.storage, token, api_user).await
    }

    pub async fn get_api_user_token(&self, id: &Uuid) -> Result<Option<UserToken>, StoreError> {
        ApiKeyStore::get(&*self.storage, id, false).await
    }

    pub async fn get_api_user_tokens(
        &self,
        api_user_id: &Uuid,
        pagination: &ListPagination,
    ) -> Result<Vec<UserToken>, StoreError> {
        ApiKeyStore::list(
            &*self.storage,
            ApiKeyFilter {
                api_user_id: Some(vec![*api_user_id]),
                expired: true,
                deleted: false,
                ..Default::default()
            },
            pagination,
        )
        .await
    }

    pub async fn get_api_user_provider(
        &self,
        id: &Uuid,
    ) -> Result<Option<ApiUserProvider>, StoreError> {
        ApiUserProviderStore::get(&*self.storage, id, false).await
    }

    pub async fn list_api_user_provider(
        &self,
        filter: ApiUserProviderFilter,
        pagination: &ListPagination,
    ) -> Result<Vec<ApiUserProvider>, StoreError> {
        ApiUserProviderStore::list(&*self.storage, filter, pagination).await
    }

    pub async fn update_api_user_provider(
        &self,
        api_user: NewApiUserProvider,
    ) -> Result<ApiUserProvider, StoreError> {
        ApiUserProviderStore::upsert(&*self.storage, api_user).await
    }

    pub async fn delete_api_user_token(&self, id: &Uuid) -> Result<Option<UserToken>, StoreError> {
        ApiKeyStore::delete(&*self.storage, id).await
    }

    pub async fn create_access_token(
        &self,
        access_token: NewAccessToken,
    ) -> Result<AccessToken, StoreError> {
        AccessTokenStore::upsert(&*self.storage, access_token).await
    }

    // Login Attempt Operations

    pub async fn create_login_attempt(
        &self,
        attempt: NewLoginAttempt,
    ) -> Result<LoginAttempt, StoreError> {
        LoginAttemptStore::upsert(&*self.storage, attempt).await
    }

    pub async fn set_login_provider_authz_code(
        &self,
        attempt: LoginAttempt,
        code: String,
    ) -> Result<LoginAttempt, StoreError> {
        let mut attempt: NewLoginAttempt = attempt.into();
        attempt.provider_authz_code = Some(code);

        // TODO: Internal state changes to the struct
        attempt.attempt_state = LoginAttemptState::RemoteAuthenticated;
        attempt.authz_code = Some(CsrfToken::new_random().secret().to_string());

        LoginAttemptStore::upsert(&*self.storage, attempt).await
    }

    pub async fn get_login_attempt(&self, id: &Uuid) -> Result<Option<LoginAttempt>, StoreError> {
        LoginAttemptStore::get(&*self.storage, id).await
    }

    pub async fn get_login_attempt_for_code(
        &self,
        code: &str,
    ) -> Result<Option<LoginAttempt>, StoreError> {
        let filter = LoginAttemptFilter {
            attempt_state: Some(vec![LoginAttemptState::RemoteAuthenticated]),
            authz_code: Some(vec![code.to_string()]),
            ..Default::default()
        };

        let mut attempts = LoginAttemptStore::list(
            &*self.storage,
            filter,
            &ListPagination {
                offset: 0,
                limit: 1,
            },
        )
        .await?;

        Ok(attempts.pop())
    }

    pub async fn fail_login_attempt(
        &self,
        attempt: LoginAttempt,
        error: Option<&str>,
        provider_error: Option<&str>,
    ) -> Result<LoginAttempt, StoreError> {
        let mut attempt: NewLoginAttempt = attempt.into();
        attempt.attempt_state = LoginAttemptState::Failed;
        attempt.error = error.map(|s| s.to_string());
        attempt.provider_error = provider_error.map(|s| s.to_string());
        LoginAttemptStore::upsert(&*self.storage, attempt).await
    }

    // OAuth Client Operations

    pub async fn create_oauth_client(&self) -> Result<OAuthClient, StoreError> {
        OAuthClientStore::upsert(&*self.storage, NewOAuthClient { id: Uuid::new_v4() }).await
    }

    pub async fn get_oauth_client(&self, id: &Uuid) -> Result<Option<OAuthClient>, StoreError> {
        OAuthClientStore::get(&*self.storage, id, false).await
    }

    pub async fn list_oauth_clients(&self) -> Result<Vec<OAuthClient>, StoreError> {
        OAuthClientStore::list(
            &*self.storage,
            OAuthClientFilter {
                id: None,
                deleted: false,
            },
            &ListPagination::default(),
        )
        .await
    }

    pub async fn add_oauth_secret(
        &self,
        id: &Uuid,
        client_id: &Uuid,
        secret: &str,
    ) -> Result<OAuthClientSecret, StoreError> {
        OAuthClientSecretStore::upsert(
            &*self.storage,
            NewOAuthClientSecret {
                id: *id,
                oauth_client_id: *client_id,
                secret_signature: secret.to_string(),
            },
        )
        .await
    }

    pub async fn delete_oauth_secret(
        &self,
        id: &Uuid,
    ) -> Result<Option<OAuthClientSecret>, StoreError> {
        OAuthClientSecretStore::delete(&*self.storage, id).await
    }

    pub async fn add_oauth_redirect_uri(
        &self,
        client_id: &Uuid,
        uri: &str,
    ) -> Result<OAuthClientRedirectUri, StoreError> {
        OAuthClientRedirectUriStore::upsert(
            &*self.storage,
            NewOAuthClientRedirectUri {
                id: Uuid::new_v4(),
                oauth_client_id: *client_id,
                redirect_uri: uri.to_string(),
            },
        )
        .await
    }

    pub async fn delete_oauth_redirect_uri(
        &self,
        id: &Uuid,
    ) -> Result<Option<OAuthClientRedirectUri>, StoreError> {
        OAuthClientRedirectUriStore::delete(&*self.storage, id).await
    }

    // Group Operations
    pub async fn get_groups(
        &self,
        caller: &Caller<ApiPermission>,
    ) -> Result<Vec<AccessGroup<ApiPermission>>, StoreError> {
        // Callers will fall in to one of three permission groups:
        //   - Has GetGroupsAll
        //   - Has GetGroupsJoined
        //   - No permissions
        //
        // Based on this hierarchy we can create a filter that includes only the groups they have
        // access to.
        let mut filter = AccessGroupFilter {
            id: None,
            name: None,
            deleted: false,
        };

        if caller.can(&ApiPermission::GetGroupsAll) {
            // Nothing we need to do, the filter is already setup for this
        } else if caller.can(&ApiPermission::GetGroupsJoined) {
            // If a caller can only view the groups they are a member of then we need to fetch the
            // callers user record to determine what those are
            let user = self.get_api_user(&caller.id).await?;
            filter.id = Some(
                user.map(|user| user.groups.into_iter().collect::<Vec<_>>())
                    .unwrap_or_default(),
            );
        } else {
            // The caller does not have any permissions to view groups
            filter.id = Some(vec![])
        };

        Ok(AccessGroupStore::list(
            &*self.storage,
            filter,
            &ListPagination::default().limit(UNLIMITED),
        )
        .await?)
    }

    pub async fn create_group(
        &self,
        caller: &ApiCaller,
        group: NewAccessGroup<ApiPermission>,
    ) -> Result<Option<AccessGroup<ApiPermission>>, StoreError> {
        if caller.can(&ApiPermission::CreateGroup) {
            AccessGroupStore::upsert(&*self.storage, &group)
                .await
                .map(Option::Some)
        } else {
            Ok(None)
        }
    }

    pub async fn update_group(
        &self,
        caller: &ApiCaller,
        group: NewAccessGroup<ApiPermission>,
    ) -> Result<Option<AccessGroup<ApiPermission>>, StoreError> {
        if caller.can(&ApiPermission::UpdateGroup(group.id)) {
            AccessGroupStore::upsert(&*self.storage, &group)
                .await
                .map(Option::Some)
        } else {
            Ok(None)
        }
    }

    pub async fn delete_group(
        &self,
        caller: &ApiCaller,
        group_id: &Uuid,
    ) -> Result<Option<AccessGroup<ApiPermission>>, StoreError> {
        if caller.can(&ApiPermission::DeleteGroup(*group_id)) {
            AccessGroupStore::delete(&*self.storage, &group_id).await
        } else {
            Ok(None)
        }
    }

    pub async fn add_api_user_to_group(
        &self,
        api_user_id: &Uuid,
        group_id: &Uuid,
    ) -> Result<Option<ApiUser<ApiPermission>>, StoreError> {
        // TODO: This needs to be wrapped in a transaction. That requires reworking the way the
        // store traits are handled. Ideally we could have an API that still abstracts away the
        // underlying connection management while allowing for transactions. Possibly something
        // that takes a closure and passes in a connection that implements all of the expected
        // data store traits
        let user = ApiUserStore::get(&*self.storage, api_user_id, false).await?;

        Ok(if let Some(user) = user {
            let mut update: NewApiUser<ApiPermission> = user.into();
            update.groups.insert(*group_id);

            Some(ApiUserStore::upsert(&*self.storage, update).await?)
        } else {
            None
        })
    }

    pub async fn remove_api_user_from_group(
        &self,
        api_user_id: &Uuid,
        group_id: &Uuid,
    ) -> Result<Option<ApiUser<ApiPermission>>, StoreError> {
        // TODO: This needs to be wrapped in a transaction. That requires reworking the way the
        // store traits are handled. Ideally we could have an API that still abstracts away the
        // underlying connection management while allowing for transactions. Possibly something
        // that takes a closure and passes in a connection that implements all of the expected
        // data store traits
        let user = ApiUserStore::get(&*self.storage, api_user_id, false).await?;

        Ok(if let Some(user) = user {
            let mut update: NewApiUser<ApiPermission> = user.into();
            update.groups.retain(|id| id != group_id);

            Some(ApiUserStore::upsert(&*self.storage, update).await?)
        } else {
            None
        })
    }

    // Mapper Operations

    async fn get_mappings(
        &self,
        caller: &Caller<ApiPermission>,
    ) -> Result<Vec<Mapping>, StoreError> {
        let mappers = self
            .get_mappers(caller, false)
            .await?
            .into_iter()
            .filter_map(|mapper| mapper.try_into().ok())
            .collect::<Vec<_>>();

        tracing::trace!(?mappers, "Fetched list of mappers to test");

        Ok(mappers)
    }

    pub async fn get_mappers(
        &self,
        caller: &Caller<ApiPermission>,
        included_depleted: bool,
    ) -> Result<Vec<Mapper>, StoreError> {
        if caller.can(&ApiPermission::ListMappers) {
            Ok(MapperStore::list(
                &*self.storage,
                MapperFilter::default().depleted(included_depleted),
                &ListPagination::default().limit(UNLIMITED),
            )
            .await?)
        } else {
            Ok(Vec::new())
        }
    }

    pub async fn add_mapper(
        &self,
        caller: &ApiCaller,
        new_mapper: &NewMapper,
    ) -> Result<Option<Mapper>, StoreError> {
        if caller.can(&ApiPermission::CreateMapper) {
            Ok(Some(MapperStore::upsert(&*self.storage, new_mapper).await?))
        } else {
            Ok(None)
        }
    }

    pub async fn remove_mapper(
        &self,
        caller: &ApiCaller,
        id: &Uuid,
    ) -> Result<Option<Mapper>, StoreError> {
        if caller.any(&[
            &ApiPermission::DeleteMapper(*id).into(),
            &ApiPermission::ManageMapper(*id).into(),
            &ApiPermission::ManageMappersAll.into(),
        ]) {
            Ok(MapperStore::delete(&*self.storage, id).await?)
        } else {
            Ok(None)
        }
    }

    async fn consume_mapping_activation(&self, mapping: &Mapping) -> Result<(), StoreError> {
        // Activations are only incremented if the rule actually has a max activation value
        let activations = mapping
            .max_activations
            .map(|_| mapping.activations.unwrap_or(0) + 1);

        Ok(MapperStore::upsert(
            &*self.storage,
            &NewMapper {
                id: mapping.id,
                name: mapping.name.clone(),
                // If a rule fails to serialize, then something critical has gone wrong. Rules should
                // never be modified after they are created, and rules must be persisted before they
                // can be used for an activation. So if a rule fails to serialize, then the stored rule
                // has become corrupted or something in the application has manipulated the rule.
                rule: serde_json::to_value(&mapping.rule)
                    .expect("Store rules must be able to be re-serialized"),
                activations: activations,
                max_activations: mapping.max_activations,
            },
        )
        .await
        .map(|_| ())?)
    }

    pub async fn get_link_request(&self, id: &Uuid) -> Result<Option<LinkRequest>, StoreError> {
        Ok(LinkRequestStore::get(&*self.storage, id, false, false).await?)
    }

    pub async fn create_link_request_token(
        &self,
        source_provider: &Uuid,
        source_user: &Uuid,
        target: &Uuid,
    ) -> Result<SignedApiKey, StoreError> {
        let link_id = Uuid::new_v4();
        let secret = RawApiKey::generate::<8>(&link_id);
        let signed = secret.sign(&*self.secrets.signer).await.unwrap();

        Ok(LinkRequestStore::upsert(
            &*self.storage,
            &NewLinkRequest {
                id: link_id,
                source_provider_id: *source_provider,
                source_api_user_id: *source_user,
                target_api_user_id: *target,
                secret_signature: signed.signature().to_string(),
                expires_at: Utc::now().add(Duration::minutes(15)),
                completed_at: None,
            },
        )
        .await
        .map(|_| signed)?)
    }

    pub async fn complete_link_request(
        &self,
        link_request: LinkRequest,
    ) -> Result<Option<ApiUserProvider>, StoreError> {
        if let Some(mut provider) = self
            .get_api_user_provider(&link_request.source_provider_id)
            .await?
        {
            provider.api_user_id = link_request.target_api_user_id;

            tracing::info!(?provider, "Created provider update");

            let source_api_user_id = link_request.source_api_user_id;
            let mut update_request: NewLinkRequest = link_request.into();
            update_request.completed_at = Some(Utc::now());
            LinkRequestStore::upsert(&*self.storage, &update_request).await?;

            Ok(Some(
                ApiUserProviderStore::transfer(&*self.storage, provider.into(), source_api_user_id)
                    .await?,
            ))
        } else {
            tracing::warn!(?link_request, "Expected to find a provider that was assigned to a link request, but it looks to have gone missing");
            Ok(None)
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::{Duration, Utc};
    use mockall::predicate::eq;
    use rfd_model::{
        storage::{AccessGroupFilter, ListPagination, MockAccessGroupStore, MockApiUserStore},
        AccessGroup, ApiUser, ApiUserProvider,
    };
    use std::{collections::BTreeSet, ops::Add, sync::Arc};
    use uuid::Uuid;

    use crate::{
        authn::{
            jwt::{Claims, Jwt},
            AuthToken,
        },
        context::UNLIMITED,
        permissions::ApiPermission,
        ApiPermissions, User,
    };

    use super::{
        test_mocks::{mock_context, MockStorage},
        ApiContext,
    };

    async fn create_token(ctx: &ApiContext, user_id: Uuid, scope: Vec<String>) -> AuthToken {
        let user = User {
            id: user_id,
            permissions: vec![].into(),
            groups: BTreeSet::new(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            deleted_at: None,
        };

        let provider = ApiUserProvider {
            id: Uuid::new_v4(),
            api_user_id: user_id,
            provider: "test".to_string(),
            provider_id: "test_id".to_string(),
            emails: vec![],
            created_at: Utc::now(),
            updated_at: Utc::now(),
            deleted_at: None,
        };

        let user_token = ctx.jwt.signers[0]
            .sign(&Claims::new(
                ctx,
                &user,
                &provider,
                scope,
                Utc::now().add(Duration::seconds(300)),
            ))
            .await
            .unwrap();

        let jwt = AuthToken::Jwt(Jwt::new(&ctx, &user_token).await.unwrap());

        jwt
    }

    #[tokio::test]
    async fn test_jwt_permissions() {
        let mut storage = MockStorage::new();

        let group_id = Uuid::new_v4();
        let group_permissions: ApiPermissions = vec![ApiPermission::GetRfd(10)].into();
        let group = AccessGroup {
            id: group_id,
            name: "TestGroup".to_string(),
            permissions: group_permissions.clone(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            deleted_at: None,
        };
        let pagination = ListPagination::default().limit(UNLIMITED);
        let mut group_store = MockAccessGroupStore::new();
        group_store
            .expect_list()
            .with(
                eq(AccessGroupFilter {
                    id: Some(vec![group_id]),
                    ..Default::default()
                }),
                eq(pagination),
            )
            .returning(move |_, _| Ok(vec![group.clone()]));

        let user_id = Uuid::new_v4();
        let user_permissions: ApiPermissions = vec![ApiPermission::GetRfd(5)].into();
        let mut groups = BTreeSet::new();
        groups.insert(group_id);
        let user = ApiUser {
            id: user_id,
            permissions: user_permissions.clone(),
            groups,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            deleted_at: None,
        };

        let mut user_store = MockApiUserStore::new();
        user_store
            .expect_get()
            .with(eq(user.id), eq(false))
            .returning(move |_, _| Ok(Some(user.clone())));

        storage.access_group_store = Some(Arc::new(group_store));
        storage.api_user_store = Some(Arc::new(user_store));
        let ctx = mock_context(storage).await;

        let token_with_no_scope = create_token(&ctx, user_id, vec![]).await;
        let permissions = ctx.get_caller(Some(&token_with_no_scope)).await.unwrap();
        assert_eq!(ApiPermissions::new(), permissions.permissions);

        let token_with_rfd_read_scope =
            create_token(&ctx, user_id, vec!["rfd:content:r".to_string()]).await;
        let permissions = ctx
            .get_caller(Some(&token_with_rfd_read_scope))
            .await
            .unwrap();
        assert_eq!(
            ApiPermissions::from(vec![ApiPermission::GetRfd(5), ApiPermission::GetRfd(10)]),
            permissions.permissions
        );
    }
}

#[cfg(test)]
pub(crate) mod test_mocks {
    use async_trait::async_trait;
    use rfd_model::{
        permissions::Caller,
        storage::{
            AccessGroupStore, AccessTokenStore, ApiKeyStore, ApiUserProviderStore, ApiUserStore,
            JobStore, LinkRequestStore, ListPagination, LoginAttemptStore, MapperStore,
            MockAccessGroupStore, MockAccessTokenStore, MockApiKeyStore, MockApiUserProviderStore,
            MockApiUserStore, MockJobStore, MockLinkRequestStore, MockLoginAttemptStore,
            MockMapperStore, MockOAuthClientRedirectUriStore, MockOAuthClientSecretStore,
            MockOAuthClientStore, MockRfdPdfStore, MockRfdRevisionStore, MockRfdStore,
            OAuthClientRedirectUriStore, OAuthClientSecretStore, OAuthClientStore, RfdPdfStore,
            RfdRevisionStore, RfdStore,
        },
        ApiKey, ApiUserProvider, NewAccessGroup, NewAccessToken, NewApiKey, NewApiUser,
        NewApiUserProvider, NewJob, NewLoginAttempt, NewMapper, NewRfd, NewRfdPdf, NewRfdRevision,
    };

    use std::sync::Arc;

    use crate::{
        config::{JwtConfig, SearchConfig},
        endpoints::login::oauth::{google::GoogleOAuthProvider, OAuthProviderName},
        permissions::ApiPermission,
        util::tests::mock_key,
    };

    use super::ApiContext;

    // Construct a mock context that can be used in tests
    pub async fn mock_context(storage: MockStorage) -> ApiContext {
        let mut ctx = ApiContext::new(
            "".to_string(),
            Arc::new(storage),
            JwtConfig::default(),
            vec![
                // We are in the context of a test and do not care about the key leaking
                mock_key(),
            ],
            SearchConfig::default(),
        )
        .await
        .unwrap();

        ctx.insert_oauth_provider(
            OAuthProviderName::Google,
            Box::new(move || {
                Box::new(GoogleOAuthProvider::new(
                    "google_device_client_id".to_string(),
                    "google_device_client_secret".to_string().into(),
                    "google_web_client_id".to_string(),
                    "google_web_client_secret".to_string().into(),
                ))
            }),
        );

        ctx
    }

    // Construct a mock storage engine that can be wrapped in an ApiContext for testing
    pub struct MockStorage {
        pub caller: Option<Caller<ApiPermission>>,
        pub rfd_store: Option<Arc<MockRfdStore>>,
        pub rfd_revision_store: Option<Arc<MockRfdRevisionStore>>,
        pub rfd_pdf_store: Option<Arc<MockRfdPdfStore>>,
        pub job_store: Option<Arc<MockJobStore>>,
        pub api_user_store: Option<Arc<MockApiUserStore<ApiPermission>>>,
        pub api_user_token_store: Option<Arc<MockApiKeyStore<ApiPermission>>>,
        pub api_user_provider_store: Option<Arc<MockApiUserProviderStore>>,
        pub device_token_store: Option<Arc<MockAccessTokenStore>>,
        pub login_attempt_store: Option<Arc<MockLoginAttemptStore>>,
        pub oauth_client_store: Option<Arc<MockOAuthClientStore>>,
        pub oauth_client_secret_store: Option<Arc<MockOAuthClientSecretStore>>,
        pub oauth_client_redirect_uri_store: Option<Arc<MockOAuthClientRedirectUriStore>>,
        pub access_group_store: Option<Arc<MockAccessGroupStore<ApiPermission>>>,
        pub mapper_store: Option<Arc<MockMapperStore>>,
        pub link_request_store: Option<Arc<MockLinkRequestStore>>,
    }

    impl MockStorage {
        pub fn new() -> Self {
            Self {
                caller: None,
                rfd_store: None,
                rfd_revision_store: None,
                rfd_pdf_store: None,
                job_store: None,
                api_user_store: None,
                api_user_token_store: None,
                api_user_provider_store: None,
                device_token_store: None,
                login_attempt_store: None,
                oauth_client_store: None,
                oauth_client_secret_store: None,
                oauth_client_redirect_uri_store: None,
                access_group_store: None,
                mapper_store: None,
                link_request_store: None,
            }
        }
    }

    #[async_trait]
    impl RfdStore for MockStorage {
        async fn get(
            &self,
            id: &uuid::Uuid,
            deleted: bool,
        ) -> Result<Option<rfd_model::Rfd>, rfd_model::storage::StoreError> {
            self.rfd_store.as_ref().unwrap().get(id, deleted).await
        }

        async fn list(
            &self,
            filter: rfd_model::storage::RfdFilter,
            pagination: &ListPagination,
        ) -> Result<Vec<rfd_model::Rfd>, rfd_model::storage::StoreError> {
            self.rfd_store
                .as_ref()
                .unwrap()
                .list(filter, pagination)
                .await
        }

        async fn upsert(
            &self,
            new_rfd: NewRfd,
        ) -> Result<rfd_model::Rfd, rfd_model::storage::StoreError> {
            self.rfd_store.as_ref().unwrap().upsert(new_rfd).await
        }

        async fn delete(
            &self,
            id: &uuid::Uuid,
        ) -> Result<Option<rfd_model::Rfd>, rfd_model::storage::StoreError> {
            self.rfd_store.as_ref().unwrap().delete(id).await
        }
    }

    #[async_trait]
    impl RfdRevisionStore for MockStorage {
        async fn get(
            &self,
            id: &uuid::Uuid,
            deleted: bool,
        ) -> Result<Option<rfd_model::RfdRevision>, rfd_model::storage::StoreError> {
            self.rfd_revision_store
                .as_ref()
                .unwrap()
                .get(id, deleted)
                .await
        }

        async fn list(
            &self,
            filter: rfd_model::storage::RfdRevisionFilter,
            pagination: &ListPagination,
        ) -> Result<Vec<rfd_model::RfdRevision>, rfd_model::storage::StoreError> {
            self.rfd_revision_store
                .as_ref()
                .unwrap()
                .list(filter, pagination)
                .await
        }

        async fn list_unique_rfd(
            &self,
            filter: rfd_model::storage::RfdRevisionFilter,
            pagination: &ListPagination,
        ) -> Result<Vec<rfd_model::RfdRevision>, rfd_model::storage::StoreError> {
            self.rfd_revision_store
                .as_ref()
                .unwrap()
                .list(filter, pagination)
                .await
        }

        async fn upsert(
            &self,
            new_revision: NewRfdRevision,
        ) -> Result<rfd_model::RfdRevision, rfd_model::storage::StoreError> {
            self.rfd_revision_store
                .as_ref()
                .unwrap()
                .upsert(new_revision)
                .await
        }

        async fn delete(
            &self,
            id: &uuid::Uuid,
        ) -> Result<Option<rfd_model::RfdRevision>, rfd_model::storage::StoreError> {
            self.rfd_revision_store.as_ref().unwrap().delete(id).await
        }
    }

    #[async_trait]
    impl RfdPdfStore for MockStorage {
        async fn get(
            &self,
            id: &uuid::Uuid,
            deleted: bool,
        ) -> Result<Option<rfd_model::RfdPdf>, rfd_model::storage::StoreError> {
            self.rfd_pdf_store.as_ref().unwrap().get(id, deleted).await
        }

        async fn list(
            &self,
            filter: rfd_model::storage::RfdPdfFilter,
            pagination: &ListPagination,
        ) -> Result<Vec<rfd_model::RfdPdf>, rfd_model::storage::StoreError> {
            self.rfd_pdf_store
                .as_ref()
                .unwrap()
                .list(filter, pagination)
                .await
        }

        async fn upsert(
            &self,
            new_pdf: NewRfdPdf,
        ) -> Result<rfd_model::RfdPdf, rfd_model::storage::StoreError> {
            self.rfd_pdf_store.as_ref().unwrap().upsert(new_pdf).await
        }

        async fn delete(
            &self,
            id: &uuid::Uuid,
        ) -> Result<Option<rfd_model::RfdPdf>, rfd_model::storage::StoreError> {
            self.rfd_pdf_store.as_ref().unwrap().delete(id).await
        }
    }

    #[async_trait]
    impl JobStore for MockStorage {
        async fn get(
            &self,
            id: i32,
        ) -> Result<Option<rfd_model::Job>, rfd_model::storage::StoreError> {
            self.job_store.as_ref().unwrap().get(id).await
        }

        async fn list(
            &self,
            filter: rfd_model::storage::JobFilter,
            pagination: &ListPagination,
        ) -> Result<Vec<rfd_model::Job>, rfd_model::storage::StoreError> {
            self.job_store
                .as_ref()
                .unwrap()
                .list(filter, pagination)
                .await
        }

        async fn upsert(
            &self,
            new_job: NewJob,
        ) -> Result<rfd_model::Job, rfd_model::storage::StoreError> {
            self.job_store.as_ref().unwrap().upsert(new_job).await
        }

        async fn start(
            &self,
            id: i32,
        ) -> Result<Option<rfd_model::Job>, rfd_model::storage::StoreError> {
            self.job_store.as_ref().unwrap().start(id).await
        }

        async fn complete(
            &self,
            id: i32,
        ) -> Result<Option<rfd_model::Job>, rfd_model::storage::StoreError> {
            self.job_store.as_ref().unwrap().complete(id).await
        }
    }

    #[async_trait]
    impl ApiUserStore<ApiPermission> for MockStorage {
        async fn get(
            &self,
            id: &uuid::Uuid,
            deleted: bool,
        ) -> Result<Option<rfd_model::ApiUser<ApiPermission>>, rfd_model::storage::StoreError>
        {
            self.api_user_store.as_ref().unwrap().get(id, deleted).await
        }

        async fn list(
            &self,
            filter: rfd_model::storage::ApiUserFilter,
            pagination: &ListPagination,
        ) -> Result<Vec<rfd_model::ApiUser<ApiPermission>>, rfd_model::storage::StoreError>
        {
            self.api_user_store
                .as_ref()
                .unwrap()
                .list(filter, pagination)
                .await
        }

        async fn upsert(
            &self,
            api_user: NewApiUser<ApiPermission>,
        ) -> Result<rfd_model::ApiUser<ApiPermission>, rfd_model::storage::StoreError> {
            self.api_user_store.as_ref().unwrap().upsert(api_user).await
        }

        async fn delete(
            &self,
            id: &uuid::Uuid,
        ) -> Result<Option<rfd_model::ApiUser<ApiPermission>>, rfd_model::storage::StoreError>
        {
            self.api_user_store.as_ref().unwrap().delete(id).await
        }
    }

    #[async_trait]
    impl ApiKeyStore<ApiPermission> for MockStorage {
        async fn get(
            &self,
            id: &uuid::Uuid,
            deleted: bool,
        ) -> Result<Option<ApiKey<ApiPermission>>, rfd_model::storage::StoreError> {
            self.api_user_token_store
                .as_ref()
                .unwrap()
                .get(id, deleted)
                .await
        }

        async fn list(
            &self,
            filter: rfd_model::storage::ApiKeyFilter,
            pagination: &ListPagination,
        ) -> Result<Vec<ApiKey<ApiPermission>>, rfd_model::storage::StoreError> {
            self.api_user_token_store
                .as_ref()
                .unwrap()
                .list(filter, pagination)
                .await
        }

        async fn upsert(
            &self,
            token: NewApiKey<ApiPermission>,
            api_user: &rfd_model::ApiUser<ApiPermission>,
        ) -> Result<ApiKey<ApiPermission>, rfd_model::storage::StoreError> {
            self.api_user_token_store
                .as_ref()
                .unwrap()
                .upsert(token, api_user)
                .await
        }

        async fn delete(
            &self,
            id: &uuid::Uuid,
        ) -> Result<Option<ApiKey<ApiPermission>>, rfd_model::storage::StoreError> {
            self.api_user_token_store.as_ref().unwrap().delete(id).await
        }
    }

    #[async_trait]
    impl ApiUserProviderStore for MockStorage {
        async fn get(
            &self,
            id: &uuid::Uuid,
            deleted: bool,
        ) -> Result<Option<ApiUserProvider>, rfd_model::storage::StoreError> {
            self.api_user_provider_store
                .as_ref()
                .unwrap()
                .get(id, deleted)
                .await
        }

        async fn list(
            &self,
            filter: rfd_model::storage::ApiUserProviderFilter,
            pagination: &ListPagination,
        ) -> Result<Vec<ApiUserProvider>, rfd_model::storage::StoreError> {
            self.api_user_provider_store
                .as_ref()
                .unwrap()
                .list(filter, pagination)
                .await
        }

        async fn upsert(
            &self,
            provider: NewApiUserProvider,
        ) -> Result<ApiUserProvider, rfd_model::storage::StoreError> {
            self.api_user_provider_store
                .as_ref()
                .unwrap()
                .upsert(provider)
                .await
        }

        async fn transfer(
            &self,
            provider: NewApiUserProvider,
            current_api_user_id: uuid::Uuid,
        ) -> Result<ApiUserProvider, rfd_model::storage::StoreError> {
            self.api_user_provider_store
                .as_ref()
                .unwrap()
                .transfer(provider, current_api_user_id)
                .await
        }

        async fn delete(
            &self,
            id: &uuid::Uuid,
        ) -> Result<Option<ApiUserProvider>, rfd_model::storage::StoreError> {
            self.api_user_provider_store
                .as_ref()
                .unwrap()
                .delete(id)
                .await
        }
    }

    #[async_trait]
    impl AccessTokenStore for MockStorage {
        async fn get(
            &self,
            id: &uuid::Uuid,
            revoked: bool,
        ) -> Result<Option<rfd_model::AccessToken>, rfd_model::storage::StoreError> {
            self.device_token_store
                .as_ref()
                .unwrap()
                .get(id, revoked)
                .await
        }

        async fn list(
            &self,
            filter: rfd_model::storage::AccessTokenFilter,
            pagination: &ListPagination,
        ) -> Result<Vec<rfd_model::AccessToken>, rfd_model::storage::StoreError> {
            self.device_token_store
                .as_ref()
                .unwrap()
                .list(filter, pagination)
                .await
        }

        async fn upsert(
            &self,
            token: NewAccessToken,
        ) -> Result<rfd_model::AccessToken, rfd_model::storage::StoreError> {
            self.device_token_store
                .as_ref()
                .unwrap()
                .upsert(token)
                .await
        }
    }

    #[async_trait]
    impl LoginAttemptStore for MockStorage {
        async fn get(
            &self,
            id: &uuid::Uuid,
        ) -> Result<Option<rfd_model::LoginAttempt>, rfd_model::storage::StoreError> {
            self.login_attempt_store.as_ref().unwrap().get(id).await
        }

        async fn list(
            &self,
            filter: rfd_model::storage::LoginAttemptFilter,
            pagination: &ListPagination,
        ) -> Result<Vec<rfd_model::LoginAttempt>, rfd_model::storage::StoreError> {
            self.login_attempt_store
                .as_ref()
                .unwrap()
                .list(filter, pagination)
                .await
        }

        async fn upsert(
            &self,
            attempt: NewLoginAttempt,
        ) -> Result<rfd_model::LoginAttempt, rfd_model::storage::StoreError> {
            self.login_attempt_store
                .as_ref()
                .unwrap()
                .upsert(attempt)
                .await
        }
    }

    #[async_trait]
    impl OAuthClientStore for MockStorage {
        async fn get(
            &self,
            id: &uuid::Uuid,
            deleted: bool,
        ) -> Result<Option<rfd_model::OAuthClient>, rfd_model::storage::StoreError> {
            self.oauth_client_store
                .as_ref()
                .unwrap()
                .get(id, deleted)
                .await
        }

        async fn list(
            &self,
            filter: rfd_model::storage::OAuthClientFilter,
            pagination: &ListPagination,
        ) -> Result<Vec<rfd_model::OAuthClient>, rfd_model::storage::StoreError> {
            self.oauth_client_store
                .as_ref()
                .unwrap()
                .list(filter, pagination)
                .await
        }

        async fn upsert(
            &self,
            client: rfd_model::NewOAuthClient,
        ) -> Result<rfd_model::OAuthClient, rfd_model::storage::StoreError> {
            self.oauth_client_store
                .as_ref()
                .unwrap()
                .upsert(client)
                .await
        }

        async fn delete(
            &self,
            id: &uuid::Uuid,
        ) -> Result<Option<rfd_model::OAuthClient>, rfd_model::storage::StoreError> {
            self.oauth_client_store.as_ref().unwrap().delete(id).await
        }
    }

    #[async_trait]
    impl OAuthClientSecretStore for MockStorage {
        async fn upsert(
            &self,
            secret: rfd_model::NewOAuthClientSecret,
        ) -> Result<rfd_model::OAuthClientSecret, rfd_model::storage::StoreError> {
            self.oauth_client_secret_store
                .as_ref()
                .unwrap()
                .upsert(secret)
                .await
        }

        async fn delete(
            &self,
            id: &uuid::Uuid,
        ) -> Result<Option<rfd_model::OAuthClientSecret>, rfd_model::storage::StoreError> {
            self.oauth_client_secret_store
                .as_ref()
                .unwrap()
                .delete(id)
                .await
        }
    }

    #[async_trait]
    impl OAuthClientRedirectUriStore for MockStorage {
        async fn upsert(
            &self,
            redirect_uri: rfd_model::NewOAuthClientRedirectUri,
        ) -> Result<rfd_model::OAuthClientRedirectUri, rfd_model::storage::StoreError> {
            self.oauth_client_redirect_uri_store
                .as_ref()
                .unwrap()
                .upsert(redirect_uri)
                .await
        }

        async fn delete(
            &self,
            id: &uuid::Uuid,
        ) -> Result<Option<rfd_model::OAuthClientRedirectUri>, rfd_model::storage::StoreError>
        {
            self.oauth_client_redirect_uri_store
                .as_ref()
                .unwrap()
                .delete(id)
                .await
        }
    }

    #[async_trait]
    impl AccessGroupStore<ApiPermission> for MockStorage {
        async fn get(
            &self,
            id: &uuid::Uuid,
            deleted: bool,
        ) -> Result<Option<rfd_model::AccessGroup<ApiPermission>>, rfd_model::storage::StoreError>
        {
            self.access_group_store
                .as_ref()
                .unwrap()
                .get(id, deleted)
                .await
        }

        async fn list(
            &self,
            filter: rfd_model::storage::AccessGroupFilter,
            pagination: &ListPagination,
        ) -> Result<Vec<rfd_model::AccessGroup<ApiPermission>>, rfd_model::storage::StoreError>
        {
            self.access_group_store
                .as_ref()
                .unwrap()
                .list(filter, pagination)
                .await
        }

        async fn upsert(
            &self,
            group: &NewAccessGroup<ApiPermission>,
        ) -> Result<rfd_model::AccessGroup<ApiPermission>, rfd_model::storage::StoreError> {
            self.access_group_store
                .as_ref()
                .unwrap()
                .upsert(group)
                .await
        }

        async fn delete(
            &self,
            id: &uuid::Uuid,
        ) -> Result<Option<rfd_model::AccessGroup<ApiPermission>>, rfd_model::storage::StoreError>
        {
            self.access_group_store.as_ref().unwrap().delete(id).await
        }
    }

    #[async_trait]
    impl MapperStore for MockStorage {
        async fn get(
            &self,
            id: &uuid::Uuid,
            used: bool,
            deleted: bool,
        ) -> Result<Option<rfd_model::Mapper>, rfd_model::storage::StoreError> {
            self.mapper_store
                .as_ref()
                .unwrap()
                .get(id, used, deleted)
                .await
        }

        async fn list(
            &self,
            filter: rfd_model::storage::MapperFilter,
            pagination: &ListPagination,
        ) -> Result<Vec<rfd_model::Mapper>, rfd_model::storage::StoreError> {
            self.mapper_store
                .as_ref()
                .unwrap()
                .list(filter, pagination)
                .await
        }

        async fn upsert(
            &self,
            new_mapper: &NewMapper,
        ) -> Result<rfd_model::Mapper, rfd_model::storage::StoreError> {
            self.mapper_store.as_ref().unwrap().upsert(new_mapper).await
        }

        async fn delete(
            &self,
            id: &uuid::Uuid,
        ) -> Result<Option<rfd_model::Mapper>, rfd_model::storage::StoreError> {
            self.mapper_store.as_ref().unwrap().delete(id).await
        }
    }

    #[async_trait]
    impl LinkRequestStore for MockStorage {
        async fn get(
            &self,
            id: &uuid::Uuid,
            expired: bool,
            completed: bool,
        ) -> Result<Option<rfd_model::LinkRequest>, rfd_model::storage::StoreError> {
            self.link_request_store
                .as_ref()
                .unwrap()
                .get(id, expired, completed)
                .await
        }

        async fn list(
            &self,
            filter: rfd_model::storage::LinkRequestFilter,
            pagination: &ListPagination,
        ) -> Result<Vec<rfd_model::LinkRequest>, rfd_model::storage::StoreError> {
            self.link_request_store
                .as_ref()
                .unwrap()
                .list(filter, pagination)
                .await
        }

        async fn upsert(
            &self,
            request: &rfd_model::NewLinkRequest,
        ) -> Result<rfd_model::LinkRequest, rfd_model::storage::StoreError> {
            self.link_request_store
                .as_ref()
                .unwrap()
                .upsert(request)
                .await
        }
    }
}
