use std::{collections::BTreeMap, fmt::Display};

use chrono::{DateTime, Utc};
use db::{
    JobModel, LoginAttemptModel, OAuthClientRedirectUriModel, OAuthClientSecretModel, RfdModel,
    RfdPdfModel, RfdRevisionModel,
};
use partial_struct::partial;
use permissions::Permissions;
use schema_ext::{ContentFormat, LoginAttemptState, PdfSource};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use thiserror::Error;
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

#[partial(NewApiKey)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct ApiKey<T> {
    pub id: Uuid,
    pub api_user_id: Uuid,
    pub key: String,
    pub permissions: Permissions<T>,
    pub expires_at: DateTime<Utc>,
    #[partial(NewApiKey(skip))]
    pub created_at: DateTime<Utc>,
    #[partial(NewApiKey(skip))]
    pub updated_at: DateTime<Utc>,
    #[partial(NewApiKey(skip))]
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

#[partial(NewLoginAttempt)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct LoginAttempt {
    pub id: Uuid,
    pub attempt_state: LoginAttemptState,
    pub client_id: Uuid,
    pub redirect_uri: String,
    pub state: Option<String>,
    pub pkce_challenge: Option<String>,
    pub pkce_challenge_method: Option<String>,
    pub authz_code: Option<String>,
    pub expires_at: Option<DateTime<Utc>>,
    pub error: Option<String>,
    pub provider: String,
    pub provider_state: String,
    pub provider_pkce_verifier: Option<String>,
    pub provider_authz_code: Option<String>,
    pub provider_error: Option<String>,
    #[partial(NewLoginAttempt(skip))]
    pub created_at: DateTime<Utc>,
    #[partial(NewLoginAttempt(skip))]
    pub updated_at: DateTime<Utc>,
}

impl LoginAttempt {
    pub fn callback_url(&self) -> String {
        let mut params = BTreeMap::new();

        if let Some(state) = &self.state {
            params.insert("state", state);
        }

        if let Some(error) = &self.error {
            params.insert("error", error);
        } else {
            if let Some(authz_code) = &self.authz_code {
                params.insert("code", authz_code);
            }
        }

        let query_string = params
            .into_iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<_>>()
            .join("&");

        [self.redirect_uri.as_str(), query_string.as_str()].join("?")
    }
}

#[derive(Debug, Error)]
pub struct InvalidValueError {
    pub field: String,
    pub error: String,
}

impl Display for InvalidValueError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} has an invalid value: {}", self.field, self.error)
    }
}

impl NewLoginAttempt {
    pub fn new(
        client_id: Uuid,
        redirect_uri: String,
        provider: String,
        provider_state: String,
    ) -> Result<Self, InvalidValueError> {
        Ok(Self {
            id: Uuid::new_v4(),
            attempt_state: LoginAttemptState::New,
            client_id,
            redirect_uri,
            state: None,
            pkce_challenge: None,
            pkce_challenge_method: None,
            authz_code: None,
            expires_at: None,
            error: None,
            provider,
            provider_state,
            provider_pkce_verifier: None,
            provider_authz_code: None,
            provider_error: None,
        })
    }
}

impl From<LoginAttemptModel> for LoginAttempt {
    fn from(value: LoginAttemptModel) -> Self {
        Self {
            id: value.id,
            attempt_state: value.attempt_state,
            client_id: value.client_id,
            redirect_uri: value.redirect_uri,
            state: value.state,
            pkce_challenge: value.pkce_challenge,
            pkce_challenge_method: value.pkce_challenge_method,
            authz_code: value.authz_code,
            expires_at: value.expires_at,
            error: None,
            provider: value.provider,
            provider_state: value.provider_state,
            provider_pkce_verifier: value.provider_pkce_verifier,
            provider_authz_code: value.provider_authz_code,
            provider_error: value.provider_error,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

#[partial(NewOAuthClient)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct OAuthClient {
    pub id: Uuid,
    #[partial(NewOAuthClient(skip))]
    pub secrets: Vec<OAuthClientSecret>,
    #[partial(NewOAuthClient(skip))]
    pub redirect_uris: Vec<OAuthClientRedirectUri>,
    #[partial(NewOAuthClient(skip))]
    pub created_at: DateTime<Utc>,
    #[partial(NewOAuthClient(skip))]
    pub deleted_at: Option<DateTime<Utc>>,
}

#[partial(NewOAuthClientSecret)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, Eq, PartialOrd, Ord)]
pub struct OAuthClientSecret {
    pub id: Uuid,
    pub oauth_client_id: Uuid,
    pub secret: String,
    #[partial(NewOAuthClientSecret(skip))]
    pub created_at: DateTime<Utc>,
    #[partial(NewOAuthClientSecret(skip))]
    pub deleted_at: Option<DateTime<Utc>>,
}

impl From<OAuthClientSecretModel> for OAuthClientSecret {
    fn from(value: OAuthClientSecretModel) -> Self {
        OAuthClientSecret {
            id: value.id,
            oauth_client_id: value.oauth_client_id,
            secret: value.secret,
            created_at: value.created_at,
            deleted_at: value.deleted_at,
        }
    }
}

#[partial(NewOAuthClientRedirectUri)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, Eq, PartialOrd, Ord)]
pub struct OAuthClientRedirectUri {
    pub id: Uuid,
    pub oauth_client_id: Uuid,
    pub redirect_uri: String,
    #[partial(NewOAuthClientRedirectUri(skip))]
    pub created_at: DateTime<Utc>,
    #[partial(NewOAuthClientRedirectUri(skip))]
    pub deleted_at: Option<DateTime<Utc>>,
}

impl From<OAuthClientRedirectUriModel> for OAuthClientRedirectUri {
    fn from(value: OAuthClientRedirectUriModel) -> Self {
        OAuthClientRedirectUri {
            id: value.id,
            oauth_client_id: value.oauth_client_id,
            redirect_uri: value.redirect_uri,
            created_at: value.created_at,
            deleted_at: value.deleted_at,
        }
    }
}

impl OAuthClient {
    pub fn is_secret_valid(&self, secret: &str) -> bool {
        self.secrets.iter().any(|s| s.secret == secret)
    }

    pub fn is_redirect_uri_valid(&self, redirect_uri: &str) -> bool {
        self.redirect_uris
            .iter()
            .any(|r| r.redirect_uri == redirect_uri)
    }
}
