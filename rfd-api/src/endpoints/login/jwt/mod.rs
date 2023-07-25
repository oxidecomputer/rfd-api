use async_trait::async_trait;
use chrono::{DateTime, Utc};
use dropshot::{endpoint, HttpError, HttpResponseCreated, Path, RequestContext, TypedBody};
use http::StatusCode;
use jsonwebtoken::jwk::JwkSet;
use rfd_model::permissions::Permissions;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use thiserror::Error;
use trace_request::trace_request;
use tracing::instrument;

use super::{LoginPermissions, UserInfo};
use crate::{context::ApiContext, util::response::client_error};

pub mod google;

#[derive(Debug, Error)]
pub enum JwtError {
    #[error("Failed to fetch configuration for Open ID provider")]
    FailedToFindOIDC,
    #[error("Failed to retrieve user information")]
    FailedToGetUserInfo,
    #[error("Failed to verify token")]
    FailedToVerifyToken,
    #[error("Token is malformed")]
    InvalidJWT,
    #[error("Issuer found does not match the configured issuer")]
    IssuerMismatch,
}

impl From<jsonwebtoken::errors::Error> for JwtError {
    fn from(_: jsonwebtoken::errors::Error) -> Self {
        Self::InvalidJWT
    }
}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Serialize, JsonSchema)]
pub struct JwtOpenIdConfiguration {
    pub issuer: String,
    pub jwks_uri: String,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Serialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum JwtProviderName {
    Google,
}

impl Display for JwtProviderName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JwtProviderName::Google => write!(f, "google"),
        }
    }
}

#[async_trait]
pub trait JwtProvider: Send + Sync {
    fn name(&self) -> JwtProviderName;
    async fn get_user_info(&self) -> Result<UserInfo, JwtError>;
}

#[async_trait]
pub trait JwksProvider: Send + Sync {
    fn issuer(&self) -> &str;
    async fn keys(&self) -> Result<JwkSet, JwtError>;
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct JwtProviderNameParam {
    provider: JwtProviderName,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct JwtProviderLogin {
    token: String,
    permissions: LoginPermissions,
    expiration: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct LoginTokenResponse {
    token: String,
}

#[trace_request]
#[endpoint {
    method = POST,
    path = "/login/jwt/{provider}",
}]
#[instrument(skip(rqctx, body), fields(request_id = rqctx.request_id), err(Debug))]
pub async fn jwt_login(
    rqctx: RequestContext<ApiContext>,
    path: Path<JwtProviderNameParam>,
    body: TypedBody<JwtProviderLogin>,
) -> Result<HttpResponseCreated<LoginTokenResponse>, HttpError> {
    let ctx = rqctx.context();
    let path = path.into_inner();
    let body = body.into_inner();
    jwt_login_op(ctx, path, body).await
}

#[instrument(skip(ctx, path, body), err(Debug))]
async fn jwt_login_op(
    ctx: &ApiContext,
    path: JwtProviderNameParam,
    body: JwtProviderLogin,
) -> Result<HttpResponseCreated<LoginTokenResponse>, HttpError> {
    let provider = ctx.get_jwt_identity(&path.provider, body.token).await?;

    tracing::debug!(provider = ?provider.name(), "Acquired provider for JWT");

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
            tracing::info!(provider = ?path.provider, ?err, "Failed to authenticate external user via JWT");
            let mut err = client_error(StatusCode::UNAUTHORIZED, "Unauthenticated");
            err.error_code = Some("INVALID_TOKEN".to_string());

            Err(err)
        }
    }
}
