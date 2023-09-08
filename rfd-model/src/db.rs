use chrono::{DateTime, Utc};
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    permissions::Permissions,
    schema::{
        api_key, api_user, api_user_access_token, api_user_provider, job, login_attempt, rfd,
        rfd_pdf, rfd_revision,
    },
    schema_ext::{ContentFormat, LoginAttemptState, PdfSource},
};

#[derive(Debug, Deserialize, Serialize, Queryable, Insertable)]
#[diesel(table_name = rfd)]
pub struct RfdModel {
    pub id: Uuid,
    pub rfd_number: i32,
    pub link: Option<String>,
    // pub relevant_components: Vec<Option<String>>,
    // pub milestones: Vec<Option<String>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, Serialize, Queryable, Insertable)]
#[diesel(table_name = rfd_revision)]
pub struct RfdRevisionModel {
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
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, Serialize, Queryable, Insertable)]
#[diesel(table_name = rfd_pdf)]
pub struct RfdPdfModel {
    pub id: Uuid,
    pub rfd_revision_id: Uuid,
    pub source: PdfSource,
    pub link: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, Serialize, Queryable, Insertable)]
#[diesel(table_name = job)]
pub struct JobModel {
    pub id: i32,
    pub owner: String,
    pub repository: String,
    pub branch: String,
    pub sha: String,
    pub rfd: i32,
    pub webhook_delivery_id: Option<Uuid>,
    pub committed_at: DateTime<Utc>,
    pub processed: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize, Queryable, Insertable)]
#[diesel(table_name = api_user)]
pub struct ApiUserModel<T> {
    pub id: Uuid,
    pub permissions: Permissions<T>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, Serialize, Queryable, Insertable)]
#[diesel(table_name = api_key)]
pub struct ApiKeyModel<T> {
    pub id: Uuid,
    pub api_user_id: Uuid,
    pub key: String,
    pub permissions: Permissions<T>,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, Serialize, Queryable, Insertable)]
#[diesel(table_name = api_user_provider)]
pub struct ApiUserProviderModel {
    pub id: Uuid,
    pub api_user_id: Uuid,
    pub provider: String,
    pub provider_id: String,
    pub emails: Vec<Option<String>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, Serialize, Queryable, Insertable)]
#[diesel(table_name = api_user_access_token)]
pub struct ApiUserAccessTokenModel {
    pub id: Uuid,
    pub api_user_id: Uuid,
    pub revoked_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize, Queryable, Insertable)]
#[diesel(table_name = login_attempt)]
pub struct LoginAttemptModel {
    pub id: Uuid,
    pub attempt_state: LoginAttemptState,
    pub client_id: Uuid,
    pub redirect_uri: String,
    pub state: Option<String>,
    pub pkce_challenge: Option<String>,
    pub pkce_challenge_method: Option<String>,
    pub authz_code: Option<String>,
    pub expires_at: Option<DateTime<Utc>>,
    pub provider: String,
    pub provider_state: String,
    pub provider_pkce_verifier: String,
    pub provider_authz_code: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
