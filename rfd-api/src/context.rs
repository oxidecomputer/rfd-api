use chrono::{DateTime, Duration, Utc};
use dropshot::{HttpError, RequestContext};
use http::StatusCode;
use hyper::{client::HttpConnector, Body, Client};
use hyper_tls::HttpsConnector;
use jsonwebtoken::jwk::JwkSet;
use meilisearch_sdk::Client as SearchClient;
use oauth2::CsrfToken;
use partial_struct::partial;
use rfd_model::{
    permissions::{Caller, Permissions},
    schema_ext::LoginAttemptState,
    storage::{
        AccessTokenStore, ApiKeyFilter, ApiKeyStore, ApiUserFilter, ApiUserProviderFilter,
        ApiUserProviderStore, ApiUserStore, JobStore, ListPagination, LoginAttemptFilter,
        LoginAttemptStore, OAuthClientFilter, OAuthClientRedirectUriStore, OAuthClientSecretStore,
        OAuthClientStore, RfdFilter, RfdPdfFilter, RfdPdfStore, RfdRevisionFilter,
        RfdRevisionStore, RfdStore, StoreError,
    },
    AccessToken, ApiUser, ApiUserProvider, InvalidValueError, Job, LoginAttempt, NewAccessToken,
    NewApiKey, NewApiUser, NewApiUserProvider, NewJob, NewLoginAttempt, NewOAuthClient,
    NewOAuthClientRedirectUri, NewOAuthClientSecret, OAuthClient, OAuthClientRedirectUri,
    OAuthClientSecret,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};
use tap::TapFallible;
use thiserror::Error;
use tracing::{info_span, instrument, Instrument};
use uuid::Uuid;

use crate::{
    authn::{
        jwt::{Claims, JwtSigner, JwtSignerError},
        AuthError, AuthToken, Signer,
    },
    config::{AsymmetricKey, JwtConfig, PermissionsConfig, SearchConfig},
    email_validator::EmailValidator,
    endpoints::login::{
        oauth::{OAuthProvider, OAuthProviderError, OAuthProviderFn, OAuthProviderName},
        LoginError, UserInfo,
    },
    error::{ApiError, AppError},
    permissions::{ApiPermission, PermissionStorage},
    util::response::{client_error, internal_error},
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
        + Send
        + Sync
        + 'static
{
}

pub struct ApiContext {
    pub email_validator: Arc<dyn EmailValidator + Send + Sync>,
    pub https_client: Client<HttpsConnector<HttpConnector>, Body>,
    pub public_url: String,
    pub storage: Arc<dyn Storage>,
    pub permissions: PermissionsContext,
    pub jwt: JwtContext,
    pub secrets: SecretContext,
    pub oauth_providers: HashMap<OAuthProviderName, Box<dyn OAuthProviderFn>>,
    pub search: SearchContext,
}

pub struct PermissionsContext {
    pub default: ApiPermissions,
}

pub struct JwtContext {
    pub default_expiration: i64,
    pub max_expiration: i64,
    // pub signers: Vec<Box<dyn JwtSigner<Claims = Claims>>>,
    pub signers: Vec<JwtSigner>,
    pub jwks: JwkSet,
}

pub struct SecretContext {
    pub signer: Arc<dyn Signer>,
}

pub struct SearchContext {
    pub client: SearchClient,
    pub index: String,
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
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct FullRfdPdfEntry {
    pub source: String,
    pub link: String,
}

impl ApiContext {
    pub async fn new(
        email_validator: Arc<dyn EmailValidator + Send + Sync>,
        public_url: String,
        storage: Arc<dyn Storage>,
        permissions: PermissionsConfig,
        jwt: JwtConfig,
        keys: Vec<AsymmetricKey>,
        search: SearchConfig,
    ) -> Result<Self, AppError> {
        let mut jwt_signers = vec![];

        for key in &keys {
            jwt_signers.push(JwtSigner::new(&key).await.unwrap())
        }

        Ok(Self {
            email_validator,
            https_client: hyper::Client::builder().build(HttpsConnector::new()),
            public_url,
            storage,
            permissions: PermissionsContext {
                default: permissions.default.into(),
            },

            jwt: JwtContext {
                default_expiration: jwt.default_expiration,
                max_expiration: jwt.max_expiration,
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
                client: SearchClient::new(search.host, search.key),
                index: search.index,
            },
        })
    }

    pub fn set_storage(&mut self, storage: Arc<dyn Storage>) {
        self.storage = storage;
    }

    pub async fn authn_token(&self, rqctx: &RequestContext<Self>) -> Result<AuthToken, AuthError> {
        AuthToken::extract(rqctx).await
    }

    pub fn default_jwt_expiration(&self) -> i64 {
        self.jwt.default_expiration
    }

    pub fn max_jwt_expiration(&self) -> i64 {
        self.jwt.max_expiration
    }

    pub async fn jwks(&self) -> &JwkSet {
        &self.jwt.jwks
    }

    pub async fn sign_jwt(&self, claims: &Claims) -> Result<String, JwtSignerError> {
        let signer = self.jwt.signers.first().unwrap();
        signer.sign(claims).await
    }

    #[instrument(skip(self, auth))]
    pub async fn get_caller(&self, auth: &AuthToken) -> Result<ApiCaller, CallerError> {
        let (api_user_id, permissions) = match auth {
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

                    // TODO: Verify found signature

                    if let Some(key) = key.pop() {
                        tracing::debug!("Verified caller key");

                        Ok((key.api_user_id, key.permissions))
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
                Ok((jwt.claims.aud, jwt.claims.prm.clone()))
            }
        }?;

        // The permissions for the caller is the intersection of the user's permissions and the tokens permissions
        if let Some(user) = self.get_api_user(&api_user_id).await? {
            let caller = Caller {
                id: api_user_id,
                permissions: permissions.expand(&user).intersect(&user.permissions),
                user,
            };

            tracing::info!(?caller, "Resolved caller");

            Ok(caller)
        } else {
            tracing::error!("User for verified token does not exist");
            Err(CallerError::FailedToAuthenticate)
        }
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
        caller: &Caller<ApiPermission>,
        filter: Option<RfdFilter>,
    ) -> Result<Vec<ListRfd>, StoreError> {
        let mut filter = filter.unwrap_or_default();

        if !caller.can(&ApiPermission::GetAllRfds) {
            let numbers = caller
                .permissions
                .iter()
                .filter_map(|p| match p {
                    ApiPermission::GetRfd(number) => Some(*number),
                    _ => None,
                })
                .collect::<Vec<_>>();

            filter = filter.rfd_number(Some(numbers));
        }

        let mut rfds = RfdStore::list(
            &*self.storage,
            filter,
            &ListPagination::default().limit(UNLIMITED),
        )
        .await
        .tap_err(|err| tracing::error!(?err, "Failed to lookup RFDs"))?;

        let mut rfd_revisions = RfdRevisionStore::list_unique_rfd(
            &*self.storage,
            RfdRevisionFilter::default().rfd(Some(rfds.iter().map(|rfd| rfd.id).collect())),
            &ListPagination::default().limit(UNLIMITED),
        )
        .await
        .tap_err(|err| tracing::error!(?err, "Failed to lookup RFD revisions"))?;

        rfds.sort_by(|a, b| a.id.cmp(&b.id));
        rfd_revisions.sort_by(|a, b| a.rfd_id.cmp(&b.rfd_id));

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
            })
            .collect::<Vec<_>>();

        rfd_list.sort_by(|a, b| b.rfd_number.cmp(&a.rfd_number));

        Ok(rfd_list)
    }

    pub async fn get_rfd(
        &self,
        rfd_number: i32,
        sha: Option<String>,
    ) -> Result<Option<FullRfd>, StoreError> {
        let rfds = RfdStore::list(
            &*self.storage,
            RfdFilter::default().rfd_number(Some(vec![rfd_number])),
            &ListPagination::default().limit(1),
        )
        .await?;

        if let Some(rfd) = rfds.into_iter().nth(0) {
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
                }))
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }

    // Webhook Operations

    pub async fn register_job(&self, new_job: NewJob) -> Result<Job, StoreError> {
        JobStore::upsert(&*self.storage, new_job).await
    }

    // Login Operations

    #[instrument(skip(self, info), fields(info.external_id))]
    pub async fn register_api_user(&self, info: UserInfo) -> Result<User, ApiError> {
        // Check if we have seen this identity before
        let mut filter = ApiUserProviderFilter::default();
        filter.provider = Some(vec![info.external_id.provider().to_string()]);
        filter.provider_id = Some(vec![info.external_id.id().to_string()]);

        tracing::info!("Check for existing users matching the requested external id");

        let api_user_providers = self
            .list_api_user_provider(filter, &ListPagination::latest())
            .await?;

        match api_user_providers.len() {
            0 => {
                tracing::info!("Did not find any existing users. Registering a new user.");

                let user = self.ensure_api_user(Uuid::new_v4()).await?;
                self.update_api_user_provider(NewApiUserProvider {
                    id: Uuid::new_v4(),
                    api_user_id: user.id,
                    emails: info.verified_emails,
                    provider: info.external_id.provider().to_string(),
                    provider_id: info.external_id.id().to_string(),
                })
                .await?;

                Ok(user)
            }
            1 => {
                tracing::info!("Found an existing user. Attaching provider.");

                // This branch ensures that there is a 0th indexed item
                let provider = api_user_providers.into_iter().nth(0).unwrap();
                Ok(self.ensure_api_user(provider.api_user_id).await?)
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

    async fn ensure_api_user(&self, api_user_id: Uuid) -> Result<User, ApiError> {
        match self.get_api_user(&api_user_id).await? {
            Some(api_user) => Ok(api_user),
            None => self
                .update_api_user(NewApiUser {
                    id: api_user_id,
                    permissions: self.permissions.default.clone(),
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
        requested_permissions: &Permissions<ApiPermission>,
        expires_at: Option<DateTime<Utc>>,
    ) -> Result<RegisteredAccessToken, ApiError> {
        let expires_at = expires_at
            .unwrap_or_else(|| Utc::now() + Duration::seconds(self.default_jwt_expiration()));

        if expires_at > Utc::now() + Duration::seconds(self.max_jwt_expiration()) {
            return Err(ApiError::Login(LoginError::ExcessTokenExpiration));
        }

        // Take the intersection of the api user permissions and the requested permissions. Tokens
        // should never have permissions that are wider than the user's permissions
        let permissions = requested_permissions.intersect(&api_user.permissions);

        // Ensure that the token is within the configured limits
        let claims = Claims {
            aud: api_user.id,
            prm: permissions,
            exp: expires_at.timestamp(),
            nbf: Utc::now().timestamp(),
            jti: Uuid::new_v4(),
        };

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
                user.permissions = user.permissions.expand(&user);
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

    pub async fn update_api_user(
        &self,
        mut api_user: NewApiUser<ApiPermission>,
    ) -> Result<User, StoreError> {
        api_user.permissions = api_user.permissions.contract(&api_user);
        ApiUserStore::upsert(&*self.storage, api_user).await
    }

    pub async fn add_permissions_to_user(
        &self,
        api_user: &ApiUser<ApiPermission>,
        new_permissions: Permissions<ApiPermission>,
    ) -> Result<User, StoreError> {
        let mut user_update: NewApiUser<ApiPermission> = api_user.clone().into();
        for permission in new_permissions.into_iter() {
            tracing::info!(id = ?api_user.id, ?permission, "Adding permission to user");
            user_update.permissions.insert(permission);
        }

        self.update_api_user(user_update).await
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
}

#[cfg(test)]
pub(crate) mod tests {
    use async_trait::async_trait;
    use rfd_model::{
        permissions::Caller,
        storage::{
            AccessTokenStore, ApiKeyStore, ApiUserProviderStore, ApiUserStore, JobStore,
            ListPagination, LoginAttemptStore, MockAccessTokenStore, MockApiKeyStore,
            MockApiUserProviderStore, MockApiUserStore, MockJobStore, MockLoginAttemptStore,
            MockOAuthClientRedirectUriStore, MockOAuthClientSecretStore, MockOAuthClientStore,
            MockRfdPdfStore, MockRfdRevisionStore, MockRfdStore, OAuthClientRedirectUriStore,
            OAuthClientSecretStore, OAuthClientStore, RfdPdfStore, RfdRevisionStore, RfdStore,
        },
        ApiKey, ApiUserProvider, NewAccessToken, NewApiKey, NewApiUser, NewApiUserProvider, NewJob,
        NewLoginAttempt, NewRfd, NewRfdPdf, NewRfdRevision,
    };

    use std::sync::Arc;

    use crate::{
        config::{JwtConfig, PermissionsConfig, SearchConfig},
        endpoints::login::oauth::{google::GoogleOAuthProvider, OAuthProviderName},
        permissions::ApiPermission,
        util::tests::{mock_key, AnyEmailValidator},
    };

    use super::ApiContext;

    // Construct a mock context that can be used in tests
    pub async fn mock_context(storage: MockStorage) -> ApiContext {
        let mut ctx = ApiContext::new(
            Arc::new(AnyEmailValidator),
            "".to_string(),
            Arc::new(storage),
            PermissionsConfig::default(),
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
                    "google_device_client_secret".to_string(),
                    "google_web_client_id".to_string(),
                    "google_web_client_secret".to_string(),
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
}
