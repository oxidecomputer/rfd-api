use async_trait::async_trait;
use chrono::{DateTime, Utc};
use dropshot::{endpoint, HttpError, HttpResponseCreated, Path, RequestContext, TypedBody};
use http::StatusCode;
use rfd_model::permissions::Permissions;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use thiserror::Error;
use tracing::instrument;

use super::UserInfo;
use crate::{
    context::ApiContext,
    endpoints::login::LoginPermissions,
    util::response::{client_error, internal_error},
};

pub mod github;

#[derive(Debug, Error)]
pub enum AccessTokenError {
    #[error("Failed to construct client")]
    FailedToConstructClient,
    #[error("Failed to retrieve user information")]
    FailedToGetUserInfo,
}

#[async_trait]
pub trait AccessTokenProvider: Send + Sync {
    fn name(&self) -> AccessTokenProviderName;
    async fn get_user_info(&self) -> Result<UserInfo, AccessTokenError>;
}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Serialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum AccessTokenProviderName {
    GitHub,
}

impl Display for AccessTokenProviderName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AccessTokenProviderName::GitHub => write!(f, "github"),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct AccessTokenProviderNameParam {
    provider: AccessTokenProviderName,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct AccessTokenProviderLogin {
    token: String,
    permissions: LoginPermissions,
    expiration: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct LoginTokenResponse {
    token: String,
}

#[endpoint {
    method = POST,
    path = "/login/access-token/{provider}",
}]
#[instrument(skip(rqctx, body), fields(request_id = rqctx.request_id), err(Debug))]
pub async fn access_token_login(
    rqctx: RequestContext<ApiContext>,
    path: Path<AccessTokenProviderNameParam>,
    body: TypedBody<AccessTokenProviderLogin>,
) -> Result<HttpResponseCreated<LoginTokenResponse>, HttpError> {
    let ctx = rqctx.context();
    let path = path.into_inner();
    let body = body.into_inner();
    let provider = ctx
        .get_access_token_provider(&path.provider, body.token)
        .await?;

    tracing::debug!(provider = ?provider.name(), "Acquired access token provider for access token");

    match provider.get_user_info().await {
        Ok(user_info) => {
            let api_user = ctx.register_api_user(user_info).await?;
            let permissions = match body.permissions {
                LoginPermissions::Specific(specific_permissions) => {
                    Some(Permissions::from(specific_permissions))
                }
                _ => None,
            };

            let token = ctx
                .register_access_token(
                    &api_user,
                    permissions.as_ref().unwrap_or(&api_user.permissions),
                    body.expiration,
                )
                .await?;

            Ok(HttpResponseCreated(LoginTokenResponse {
                token: token.signed_token,
            }))
        }
        Err(err) => {
            tracing::info!(provider = ?path.provider, ?err, "Failed to authenticate external user via access token");

            match err {
                AccessTokenError::FailedToConstructClient => Err(internal_error("")),
                AccessTokenError::FailedToGetUserInfo => {
                    let mut err = client_error(StatusCode::UNAUTHORIZED, "Unauthenticated");
                    err.error_code = Some("INVALID_TOKEN".to_string());

                    Err(err)
                }
            }
        }
    }
}
