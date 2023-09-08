use dropshot::{endpoint, RequestContext, HttpError, HttpResponseOk, Path, TypedBody};
use rfd_model::{OAuthClient, OAuthClientSecret};
use schemars::JsonSchema;
use serde::Deserialize;
use tracing::instrument;
use uuid::Uuid;

use crate::{context::ApiContext, ApiCaller};

/// Create a new OAuth Client
#[endpoint {
    method = POST,
    path = "/oauth/client"
}]
#[instrument(skip(rqctx), fields(request_id = rqctx.request_id), err(Debug))]
pub async fn create_oauth_client(
    rqctx: RequestContext<ApiContext>,
) -> Result<HttpResponseOk<OAuthClient>, HttpError> {
    let ctx = rqctx.context();
    let auth = ctx.authn_token(&rqctx).await?;
    let caller = ctx.get_caller(&auth).await?;
    create_oauth_client_op(ctx, &caller).await
}

#[instrument(skip(ctx, caller), fields(caller = ?caller.id), err(Debug))]
async fn create_oauth_client_op(
    ctx: &ApiContext,
    caller: &ApiCaller,
) -> Result<HttpResponseOk<OAuthClient>, HttpError> {
    unimplemented!()
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct DeleteOAuthClientPath {
    pub client_id: Uuid,
}

/// Delete a OAuth Client
#[endpoint {
    method = DELETE,
    path = "/oauth/client/{client_id}"
}]
#[instrument(skip(rqctx), fields(request_id = rqctx.request_id), err(Debug))]
pub async fn delete_oauth_client(
    rqctx: RequestContext<ApiContext>,
    path: Path<DeleteOAuthClientPath>
) -> Result<HttpResponseOk<OAuthClient>, HttpError> {
    let ctx = rqctx.context();
    let auth = ctx.authn_token(&rqctx).await?;
    let caller = ctx.get_caller(&auth).await?;
    delete_oauth_client_op(ctx, &caller, &path.into_inner()).await
}

#[instrument(skip(ctx, caller), fields(caller = ?caller.id), err(Debug))]
async fn delete_oauth_client_op(
    ctx: &ApiContext,
    caller: &ApiCaller,
    path: &DeleteOAuthClientPath,
) -> Result<HttpResponseOk<OAuthClient>, HttpError> {
    unimplemented!()
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct AddOAuthClientSecretPath {
    pub client_id: Uuid,
}

/// Add an OAuth client secret
#[endpoint {
    method = POST,
    path = "/oauth/client/{client_id}/secret"
}]
#[instrument(skip(rqctx), fields(request_id = rqctx.request_id), err(Debug))]
pub async fn create_oauth_client_secret(
    rqctx: RequestContext<ApiContext>,
    path: Path<AddOAuthClientSecretPath>
) -> Result<HttpResponseOk<OAuthClientSecret>, HttpError> {
    let ctx = rqctx.context();
    let auth = ctx.authn_token(&rqctx).await?;
    let caller = ctx.get_caller(&auth).await?;
    create_oauth_client_secret_op(ctx, &caller, &path.into_inner()).await
}

#[instrument(skip(ctx, caller), fields(caller = ?caller.id), err(Debug))]
async fn create_oauth_client_secret_op(
    ctx: &ApiContext,
    caller: &ApiCaller,
    path: &AddOAuthClientSecretPath,
) -> Result<HttpResponseOk<OAuthClientSecret>, HttpError> {
    unimplemented!()
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct DeleteOAuthClientSecretPath {
    pub client_id: Uuid,
    pub secret_id: Uuid,
}

/// Delete an OAuth client secret
#[endpoint {
    method = POST,
    path = "/oauth/client/{client_id}/secret/{secret_id}"
}]
#[instrument(skip(rqctx), fields(request_id = rqctx.request_id), err(Debug))]
pub async fn delete_oauth_client_secret(
    rqctx: RequestContext<ApiContext>,
    path: Path<DeleteOAuthClientSecretPath>
) -> Result<HttpResponseOk<OAuthClientSecret>, HttpError> {
    let ctx = rqctx.context();
    let auth = ctx.authn_token(&rqctx).await?;
    let caller = ctx.get_caller(&auth).await?;
    delete_oauth_client_secret_op(ctx, &caller, &path.into_inner()).await
}

#[instrument(skip(ctx, caller), fields(caller = ?caller.id), err(Debug))]
async fn delete_oauth_client_secret_op(
    ctx: &ApiContext,
    caller: &ApiCaller,
    path: &DeleteOAuthClientSecretPath,
) -> Result<HttpResponseOk<OAuthClientSecret>, HttpError> {
    unimplemented!()
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct AddOAuthClientRedirectPath {
    pub client_id: Uuid,
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct AddOAuthClientRedirectBody {
    pub redirect_uri: String,
}

/// Add an OAuth client redirect uri
#[endpoint {
    method = POST,
    path = "/oauth/client/{client_id}/redirect_uri"
}]
#[instrument(skip(rqctx), fields(request_id = rqctx.request_id), err(Debug))]
pub async fn create_oauth_client_redirect_uri(
    rqctx: RequestContext<ApiContext>,
    path: Path<AddOAuthClientRedirectPath>,
    body: TypedBody<AddOAuthClientRedirectBody>,
) -> Result<HttpResponseOk<OAuthClientSecret>, HttpError> {
    let ctx = rqctx.context();
    let auth = ctx.authn_token(&rqctx).await?;
    let caller = ctx.get_caller(&auth).await?;
    create_oauth_client_redirect_uri_op(ctx, &caller, &path.into_inner()).await
}

#[instrument(skip(ctx, caller), fields(caller = ?caller.id), err(Debug))]
async fn create_oauth_client_redirect_uri_op(
    ctx: &ApiContext,
    caller: &ApiCaller,
    path: &AddOAuthClientRedirectPath,
) -> Result<HttpResponseOk<OAuthClientSecret>, HttpError> {
    unimplemented!()
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct DeleteOAuthClientRediretPath {
    pub client_id: Uuid,
    pub redirect_uri_id: Uuid,
}

/// Delete an OAuth client secret
#[endpoint {
    method = POST,
    path = "/oauth/client/{client_id}/redirect_uri/{redirect_uri_id}"
}]
#[instrument(skip(rqctx), fields(request_id = rqctx.request_id), err(Debug))]
pub async fn delete_oauth_client_redirect_uri(
    rqctx: RequestContext<ApiContext>,
    path: Path<DeleteOAuthClientSecretPath>
) -> Result<HttpResponseOk<OAuthClientSecret>, HttpError> {
    let ctx = rqctx.context();
    let auth = ctx.authn_token(&rqctx).await?;
    let caller = ctx.get_caller(&auth).await?;
    delete_oauth_client_redirect_uri_op(ctx, &caller, &path.into_inner()).await
}

#[instrument(skip(ctx, caller), fields(caller = ?caller.id), err(Debug))]
async fn delete_oauth_client_redirect_uri_op(
    ctx: &ApiContext,
    caller: &ApiCaller,
    path: &DeleteOAuthClientSecretPath,
) -> Result<HttpResponseOk<OAuthClientSecret>, HttpError> {
    unimplemented!()
}

