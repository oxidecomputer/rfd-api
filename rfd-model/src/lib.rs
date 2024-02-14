// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::Display,
};

use chrono::{DateTime, Utc};
use db::{
    AccessGroupModel, JobModel, LinkRequestModel, LoginAttemptModel, MapperModel,
    OAuthClientRedirectUriModel, OAuthClientSecretModel, RfdModel, RfdPdfModel, RfdRevisionModel,
};
use partial_struct::partial;
use schema_ext::{ContentFormat, LoginAttemptState, PdfSource, Visibility};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use thiserror::Error;
use uuid::Uuid;
use w_api_permissions::Permissions;

pub mod db;
pub mod schema;
pub mod schema_ext;
pub mod storage;

#[partial(NewRfd)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct Rfd {
    pub id: Uuid,
    pub rfd_number: i32,
    pub link: Option<String>,
    #[partial(NewRfd(skip))]
    pub created_at: DateTime<Utc>,
    #[partial(NewRfd(skip))]
    pub updated_at: DateTime<Utc>,
    #[partial(NewRfd(skip))]
    pub deleted_at: Option<DateTime<Utc>>,
    pub visibility: Visibility,
}

impl From<RfdModel> for Rfd {
    fn from(value: RfdModel) -> Self {
        Self {
            id: value.id,
            rfd_number: value.rfd_number,
            link: value.link,
            created_at: value.created_at,
            updated_at: value.updated_at,
            deleted_at: value.deleted_at,
            visibility: value.visibility,
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
    pub rfd_id: Uuid,
    pub external_id: String,
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
            rfd_id: value.rfd_id,
            external_id: value.external_id,
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
    #[partial(NewJob(skip))]
    pub started_at: Option<DateTime<Utc>>,
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
            started_at: value.started_at,
        }
    }
}

#[partial(NewApiUser)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct ApiUser<T: Ord> {
    pub id: Uuid,
    pub permissions: Permissions<T>,
    pub groups: BTreeSet<Uuid>,
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
    pub display_names: Vec<String>,
    #[partial(NewApiUserProvider(skip))]
    pub created_at: DateTime<Utc>,
    #[partial(NewApiUserProvider(skip))]
    pub updated_at: DateTime<Utc>,
    #[partial(NewApiUserProvider(skip))]
    pub deleted_at: Option<DateTime<Utc>>,
}

#[partial(NewApiKey)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct ApiKey<T: Ord> {
    pub id: Uuid,
    pub api_user_id: Uuid,
    pub key_signature: String,
    pub permissions: Option<Permissions<T>>,
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
    pub provider_pkce_verifier: Option<String>,
    pub provider_authz_code: Option<String>,
    pub provider_error: Option<String>,
    #[partial(NewLoginAttempt(skip))]
    pub created_at: DateTime<Utc>,
    #[partial(NewLoginAttempt(skip))]
    pub updated_at: DateTime<Utc>,
    pub scope: String,
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
        provider: String,
        client_id: Uuid,
        redirect_uri: String,
        scope: String,
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
            provider_pkce_verifier: None,
            provider_authz_code: None,
            provider_error: None,
            scope,
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
            provider_pkce_verifier: value.provider_pkce_verifier,
            provider_authz_code: value.provider_authz_code,
            provider_error: value.provider_error,
            created_at: value.created_at,
            updated_at: value.updated_at,
            scope: value.scope,
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
    pub secret_signature: String,
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
            secret_signature: value.secret_signature,
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

#[partial(NewAccessGroup)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct AccessGroup<T: Ord> {
    pub id: Uuid,
    pub name: String,
    pub permissions: Permissions<T>,
    #[partial(NewAccessGroup(skip))]
    pub created_at: DateTime<Utc>,
    #[partial(NewAccessGroup(skip))]
    pub updated_at: DateTime<Utc>,
    #[partial(NewAccessGroup(skip))]
    pub deleted_at: Option<DateTime<Utc>>,
}

impl<T> From<AccessGroupModel<T>> for AccessGroup<T>
where
    T: Ord,
{
    fn from(value: AccessGroupModel<T>) -> Self {
        AccessGroup {
            id: value.id,
            name: value.name,
            permissions: value.permissions,
            created_at: value.created_at,
            updated_at: value.updated_at,
            deleted_at: value.deleted_at,
        }
    }
}

#[partial(NewMapper)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct Mapper {
    pub id: Uuid,
    pub name: String,
    pub rule: Value,
    pub activations: Option<i32>,
    pub max_activations: Option<i32>,
    #[partial(NewMapper(skip))]
    pub depleted_at: Option<DateTime<Utc>>,
    #[partial(NewMapper(skip))]
    pub created_at: DateTime<Utc>,
    #[partial(NewMapper(skip))]
    pub deleted_at: Option<DateTime<Utc>>,
}

impl From<MapperModel> for Mapper {
    fn from(value: MapperModel) -> Self {
        Mapper {
            id: value.id,
            name: value.name,
            rule: value.rule,
            activations: value.activations,
            max_activations: value.max_activations,
            depleted_at: value.depleted_at,
            created_at: value.created_at,
            deleted_at: value.deleted_at,
        }
    }
}

#[partial(NewLinkRequest)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct LinkRequest {
    pub id: Uuid,
    pub source_provider_id: Uuid,
    pub source_api_user_id: Uuid,
    pub target_api_user_id: Uuid,
    pub secret_signature: String,
    #[partial(NewLinkRequest(skip))]
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

impl From<LinkRequestModel> for LinkRequest {
    fn from(value: LinkRequestModel) -> Self {
        LinkRequest {
            id: value.id,
            source_provider_id: value.source_provider_id,
            source_api_user_id: value.source_api_user_id,
            target_api_user_id: value.target_api_user_id,
            secret_signature: value.secret_signature,
            created_at: value.created_at,
            expires_at: value.expires_at,
            completed_at: value.completed_at,
        }
    }
}
