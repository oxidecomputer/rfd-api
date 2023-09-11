use dropshot::{endpoint, HttpError, HttpResponseOk, Path, RequestContext, TypedBody};
use http::StatusCode;
use rfd_model::{OAuthClient, OAuthClientRedirectUri, OAuthClientSecret};
use schemars::JsonSchema;
use serde::Deserialize;
use tracing::instrument;
use uuid::Uuid;

use crate::{
    authn::key::RawApiKey,
    context::ApiContext,
    error::ApiError,
    permissions::ApiPermission,
    util::response::{client_error, to_internal_error},
    ApiCaller,
};

/// List OAuth clients
#[endpoint {
    method = GET,
    path = "/oauth/client"
}]
#[instrument(skip(rqctx), fields(request_id = rqctx.request_id), err(Debug))]
pub async fn list_oauth_clients(
    rqctx: RequestContext<ApiContext>,
) -> Result<HttpResponseOk<Vec<OAuthClient>>, HttpError> {
    let ctx = rqctx.context();
    let auth = ctx.authn_token(&rqctx).await?;
    let caller = ctx.get_caller(&auth).await?;
    list_oauth_clients_op(ctx, &caller).await
}

#[instrument(skip(ctx, caller), fields(caller = ?caller.id), err(Debug))]
async fn list_oauth_clients_op(
    ctx: &ApiContext,
    caller: &ApiCaller,
) -> Result<HttpResponseOk<Vec<OAuthClient>>, HttpError> {
    if caller.can(&ApiPermission::GetOAuthClientAll) {
        Ok(HttpResponseOk(
            ctx.list_oauth_clients().await.map_err(ApiError::Storage)?,
        ))
    } else {
        Err(client_error(StatusCode::FORBIDDEN, "Unauthorized"))
    }
}

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
    if caller.can(&ApiPermission::CreateOAuthClient) {
        Ok(HttpResponseOk(
            ctx.create_oauth_client().await.map_err(ApiError::Storage)?,
        ))
    } else {
        Err(client_error(StatusCode::FORBIDDEN, "Unauthorized"))
    }
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct GetOAuthClientPath {
    pub client_id: Uuid,
}

/// Get an new OAuth Client
#[endpoint {
    method = GET,
    path = "/oauth/client/{client_id}"
}]
#[instrument(skip(rqctx), fields(request_id = rqctx.request_id), err(Debug))]
pub async fn get_oauth_client(
    rqctx: RequestContext<ApiContext>,
    path: Path<GetOAuthClientPath>,
) -> Result<HttpResponseOk<Option<OAuthClient>>, HttpError> {
    let ctx = rqctx.context();
    let auth = ctx.authn_token(&rqctx).await?;
    let caller = ctx.get_caller(&auth).await?;
    get_oauth_client_op(ctx, &caller, &path.into_inner()).await
}

#[instrument(skip(ctx, caller), fields(caller = ?caller.id), err(Debug))]
async fn get_oauth_client_op(
    ctx: &ApiContext,
    caller: &ApiCaller,
    path: &GetOAuthClientPath,
) -> Result<HttpResponseOk<Option<OAuthClient>>, HttpError> {
    if caller.any(&[
        &ApiPermission::GetOAuthClientAll,
        &ApiPermission::GetOAuthClient(path.client_id),
    ]) {
        Ok(HttpResponseOk(
            ctx.get_oauth_client(&path.client_id)
                .await
                .map_err(ApiError::Storage)?,
        ))
    } else {
        Err(client_error(StatusCode::FORBIDDEN, "Unauthorized"))
    }
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
    path: Path<AddOAuthClientSecretPath>,
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
    if caller.can(&ApiPermission::CreateOAuthClientSecret(path.client_id)) {
        let secret = RawApiKey::generate::<24>();
        let mut client_secret = ctx
            .add_oauth_secret(
                &path.client_id,
                &secret
                    .encrypt(&*ctx.secrets.encryptor)
                    .await
                    .map_err(to_internal_error)?
                    .encrypted,
            )
            .await
            .map_err(ApiError::Storage)?;
        client_secret.secret = secret.consume();

        Ok(HttpResponseOk(client_secret))
    } else {
        Err(client_error(StatusCode::FORBIDDEN, "Unauthorized"))
    }
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct DeleteOAuthClientSecretPath {
    pub client_id: Uuid,
    pub secret_id: Uuid,
}

/// Delete an OAuth client secret
#[endpoint {
    method = DELETE,
    path = "/oauth/client/{client_id}/secret/{secret_id}"
}]
#[instrument(skip(rqctx), fields(request_id = rqctx.request_id), err(Debug))]
pub async fn delete_oauth_client_secret(
    rqctx: RequestContext<ApiContext>,
    path: Path<DeleteOAuthClientSecretPath>,
) -> Result<HttpResponseOk<Option<OAuthClientSecret>>, HttpError> {
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
) -> Result<HttpResponseOk<Option<OAuthClientSecret>>, HttpError> {
    if caller.can(&ApiPermission::DeleteOAuthClientSecret(path.secret_id)) {
        Ok(HttpResponseOk(
            ctx.delete_oauth_secret(&path.secret_id)
                .await
                .map_err(ApiError::Storage)?,
        ))
    } else {
        Err(client_error(StatusCode::FORBIDDEN, "Unauthorized"))
    }
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
) -> Result<HttpResponseOk<OAuthClientRedirectUri>, HttpError> {
    let ctx = rqctx.context();
    let auth = ctx.authn_token(&rqctx).await?;
    let caller = ctx.get_caller(&auth).await?;
    create_oauth_client_redirect_uri_op(ctx, &caller, &path.into_inner(), body.into_inner()).await
}

#[instrument(skip(ctx, caller), fields(caller = ?caller.id), err(Debug))]
async fn create_oauth_client_redirect_uri_op(
    ctx: &ApiContext,
    caller: &ApiCaller,
    path: &AddOAuthClientRedirectPath,
    body: AddOAuthClientRedirectBody,
) -> Result<HttpResponseOk<OAuthClientRedirectUri>, HttpError> {
    if caller.can(&ApiPermission::CreateOAuthClientRedirectUri(path.client_id)) {
        Ok(HttpResponseOk(
            ctx.add_oauth_redirect_uri(&path.client_id, &body.redirect_uri)
                .await
                .map_err(ApiError::Storage)?,
        ))
    } else {
        Err(client_error(StatusCode::FORBIDDEN, "Unauthorized"))
    }
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct DeleteOAuthClientRedirectPath {
    pub client_id: Uuid,
    pub redirect_uri_id: Uuid,
}

/// Delete an OAuth client redirect uri
#[endpoint {
    method = DELETE,
    path = "/oauth/client/{client_id}/redirect_uri/{redirect_uri_id}"
}]
#[instrument(skip(rqctx), fields(request_id = rqctx.request_id), err(Debug))]
pub async fn delete_oauth_client_redirect_uri(
    rqctx: RequestContext<ApiContext>,
    path: Path<DeleteOAuthClientRedirectPath>,
) -> Result<HttpResponseOk<Option<OAuthClientRedirectUri>>, HttpError> {
    let ctx = rqctx.context();
    let auth = ctx.authn_token(&rqctx).await?;
    let caller = ctx.get_caller(&auth).await?;
    delete_oauth_client_redirect_uri_op(ctx, &caller, &path.into_inner()).await
}

#[instrument(skip(ctx, caller), fields(caller = ?caller.id), err(Debug))]
async fn delete_oauth_client_redirect_uri_op(
    ctx: &ApiContext,
    caller: &ApiCaller,
    path: &DeleteOAuthClientRedirectPath,
) -> Result<HttpResponseOk<Option<OAuthClientRedirectUri>>, HttpError> {
    if caller.can(&ApiPermission::DeleteOAuthClientRedirectUri(path.client_id)) {
        Ok(HttpResponseOk(
            ctx.delete_oauth_redirect_uri(&path.redirect_uri_id)
                .await
                .map_err(ApiError::Storage)?,
        ))
    } else {
        Err(client_error(StatusCode::FORBIDDEN, "Unauthorized"))
    }
}
