use dropshot::{endpoint, HttpError, HttpResponseOk, Path, RequestContext, TypedBody};
use http::StatusCode;
use rfd_model::{OAuthClient, OAuthClientRedirectUri, OAuthClientSecret};
use schemars::JsonSchema;
use serde::Deserialize;
use trace_request::trace_request;
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
#[trace_request]
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
    Ok(HttpResponseOk(
        ctx.list_oauth_clients().await.map_err(ApiError::Storage)?.into_iter().filter(|client| caller.can(&ApiPermission::GetOAuthClient(client.id))).collect(),
    ))
}

/// Create a new OAuth Client
#[trace_request]
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
        // Create the new client
        let client = ctx.create_oauth_client().await.map_err(ApiError::Storage)?;

        // Give the caller permission to perform actions on the client        
        ctx.add_permissions_to_user(&caller.user, vec![
            ApiPermission::GetOAuthClient(client.id),
            ApiPermission::UpdateOAuthClient(client.id),
            ApiPermission::DeleteOAuthClient(client.id),
        ].into()).await.map_err(ApiError::Storage)?;

        Ok(HttpResponseOk(client))
    } else {
        Err(client_error(StatusCode::FORBIDDEN, "Unauthorized"))
    }
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct GetOAuthClientPath {
    pub client_id: Uuid,
}

/// Get an new OAuth Client
#[trace_request]
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
    if caller.can(&ApiPermission::GetOAuthClient(path.client_id)) {
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
#[trace_request]
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
    if caller.can(&ApiPermission::UpdateOAuthClient(path.client_id)) {
        let id = Uuid::new_v4();
        let secret = RawApiKey::generate::<24>(&id)
            .sign(&*ctx.secrets.signer)
            .await
            .map_err(to_internal_error)?;
        let mut client_secret = ctx
            .add_oauth_secret(&id, &path.client_id, secret.signature())
            .await
            .map_err(ApiError::Storage)?;
        client_secret.secret_signature = secret.key();

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
#[trace_request]
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
    if caller.can(&ApiPermission::UpdateOAuthClient(path.secret_id)) {
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
#[trace_request]
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
    if caller.can(&ApiPermission::UpdateOAuthClient(path.client_id)) {
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
#[trace_request]
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
    if caller.can(&ApiPermission::UpdateOAuthClient(path.client_id)) {
        Ok(HttpResponseOk(
            ctx.delete_oauth_redirect_uri(&path.redirect_uri_id)
                .await
                .map_err(ApiError::Storage)?,
        ))
    } else {
        Err(client_error(StatusCode::FORBIDDEN, "Unauthorized"))
    }
}
