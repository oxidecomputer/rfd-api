// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use chrono::{DateTime, Utc};
use dropshot::{
    endpoint, HttpError, HttpResponseCreated, HttpResponseOk, Path, RequestContext, TypedBody,
};
use rfd_model::{OAuthClient, OAuthClientRedirectUri, OAuthClientSecret};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use trace_request::trace_request;
use tracing::instrument;
use uuid::Uuid;

use crate::{
    authn::key::RawApiKey, context::ApiContext, permissions::ApiPermission,
    secrets::OpenApiSecretString, util::response::to_internal_error, ApiCaller,
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
    let caller = ctx.get_caller(auth.as_ref()).await?;
    list_oauth_clients_op(ctx, &caller).await
}

#[instrument(skip(ctx, caller), fields(caller = ?caller.id), err(Debug))]
async fn list_oauth_clients_op(
    ctx: &ApiContext,
    caller: &ApiCaller,
) -> Result<HttpResponseOk<Vec<OAuthClient>>, HttpError> {
    Ok(HttpResponseOk(ctx.list_oauth_clients(caller).await?))
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
) -> Result<HttpResponseCreated<OAuthClient>, HttpError> {
    let ctx = rqctx.context();
    let auth = ctx.authn_token(&rqctx).await?;
    let caller = ctx.get_caller(auth.as_ref()).await?;
    create_oauth_client_op(ctx, &caller).await
}

#[instrument(skip(ctx, caller), fields(caller = ?caller.id), err(Debug))]
async fn create_oauth_client_op(
    ctx: &ApiContext,
    caller: &ApiCaller,
) -> Result<HttpResponseCreated<OAuthClient>, HttpError> {
    // Create the new client
    let client = ctx.create_oauth_client(caller).await?;

    // Give the caller permission to perform actions on the client
    ctx.add_permissions_to_user(
        caller,
        &caller.id,
        vec![
            ApiPermission::GetOAuthClient(client.id),
            ApiPermission::UpdateOAuthClient(client.id),
            ApiPermission::DeleteOAuthClient(client.id),
        ]
        .into(),
    )
    .await?;

    Ok(HttpResponseCreated(client))
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
) -> Result<HttpResponseOk<OAuthClient>, HttpError> {
    let ctx = rqctx.context();
    let auth = ctx.authn_token(&rqctx).await?;
    let caller = ctx.get_caller(auth.as_ref()).await?;
    get_oauth_client_op(ctx, &caller, &path.into_inner()).await
}

#[instrument(skip(ctx, caller), fields(caller = ?caller.id), err(Debug))]
async fn get_oauth_client_op(
    ctx: &ApiContext,
    caller: &ApiCaller,
    path: &GetOAuthClientPath,
) -> Result<HttpResponseOk<OAuthClient>, HttpError> {
    Ok(HttpResponseOk(
        ctx.get_oauth_client(caller, &path.client_id).await?,
    ))
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct AddOAuthClientSecretPath {
    pub client_id: Uuid,
}

#[derive(Debug, Serialize, JsonSchema)]
pub struct InitialOAuthClientSecretResponse {
    pub id: Uuid,
    pub key: OpenApiSecretString,
    pub created_at: DateTime<Utc>,
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
) -> Result<HttpResponseOk<InitialOAuthClientSecretResponse>, HttpError> {
    let ctx = rqctx.context();
    let auth = ctx.authn_token(&rqctx).await?;
    let caller = ctx.get_caller(auth.as_ref()).await?;
    create_oauth_client_secret_op(ctx, &caller, path.into_inner().client_id).await
}

#[instrument(skip(ctx, caller), fields(caller = ?caller.id), err(Debug))]
async fn create_oauth_client_secret_op(
    ctx: &ApiContext,
    caller: &ApiCaller,
    client_id: Uuid,
) -> Result<HttpResponseOk<InitialOAuthClientSecretResponse>, HttpError> {
    let id = Uuid::new_v4();
    let secret = RawApiKey::generate::<24>(&id)
        .sign(&*ctx.secrets.signer)
        .await
        .map_err(to_internal_error)?;
    let client_secret = ctx
        .add_oauth_secret(caller, &id, &client_id, secret.signature())
        .await?;

    Ok(HttpResponseOk(InitialOAuthClientSecretResponse {
        id: client_secret.id,
        key: secret.key().into(),
        created_at: client_secret.created_at,
    }))
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
) -> Result<HttpResponseOk<OAuthClientSecret>, HttpError> {
    let ctx = rqctx.context();
    let auth = ctx.authn_token(&rqctx).await?;
    let caller = ctx.get_caller(auth.as_ref()).await?;
    delete_oauth_client_secret_op(ctx, &caller, &path.into_inner()).await
}

#[instrument(skip(ctx, caller), fields(caller = ?caller.id), err(Debug))]
async fn delete_oauth_client_secret_op(
    ctx: &ApiContext,
    caller: &ApiCaller,
    path: &DeleteOAuthClientSecretPath,
) -> Result<HttpResponseOk<OAuthClientSecret>, HttpError> {
    Ok(HttpResponseOk(
        ctx.delete_oauth_secret(caller, &path.secret_id, &path.client_id)
            .await?,
    ))
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
    let caller = ctx.get_caller(auth.as_ref()).await?;
    create_oauth_client_redirect_uri_op(ctx, &caller, &path.into_inner(), body.into_inner()).await
}

#[instrument(skip(ctx, caller), fields(caller = ?caller.id), err(Debug))]
async fn create_oauth_client_redirect_uri_op(
    ctx: &ApiContext,
    caller: &ApiCaller,
    path: &AddOAuthClientRedirectPath,
    body: AddOAuthClientRedirectBody,
) -> Result<HttpResponseOk<OAuthClientRedirectUri>, HttpError> {
    Ok(HttpResponseOk(
        ctx.add_oauth_redirect_uri(caller, &path.client_id, &body.redirect_uri)
            .await?,
    ))
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
) -> Result<HttpResponseOk<OAuthClientRedirectUri>, HttpError> {
    let ctx = rqctx.context();
    let auth = ctx.authn_token(&rqctx).await?;
    let caller = ctx.get_caller(auth.as_ref()).await?;
    delete_oauth_client_redirect_uri_op(ctx, &caller, &path.into_inner()).await
}

#[instrument(skip(ctx, caller), fields(caller = ?caller.id), err(Debug))]
async fn delete_oauth_client_redirect_uri_op(
    ctx: &ApiContext,
    caller: &ApiCaller,
    path: &DeleteOAuthClientRedirectPath,
) -> Result<HttpResponseOk<OAuthClientRedirectUri>, HttpError> {
    Ok(HttpResponseOk(
        ctx.delete_oauth_redirect_uri(caller, &path.redirect_uri_id, &path.client_id)
            .await?,
    ))
}

#[cfg(test)]
mod tests {
    use std::{
        collections::BTreeSet,
        sync::{Arc, Mutex},
    };

    use chrono::Utc;
    use mockall::predicate::eq;
    use rfd_model::{
        storage::{MockApiUserStore, MockOAuthClientSecretStore, MockOAuthClientStore},
        ApiUser, OAuthClient, OAuthClientSecret,
    };
    use uuid::Uuid;

    use crate::{
        authn::key::RawApiKey,
        context::test_mocks::{mock_context, MockStorage},
        endpoints::login::oauth::CheckOAuthClient,
        permissions::ApiPermission,
        ApiCaller,
    };

    use super::{create_oauth_client_op, create_oauth_client_secret_op};

    fn mock_user() -> ApiUser<ApiPermission> {
        let user_id = Uuid::new_v4();
        ApiUser {
            id: user_id,
            permissions: vec![
                ApiPermission::CreateOAuthClient,
                ApiPermission::GetApiUser(user_id),
                ApiPermission::UpdateApiUser(user_id),
            ]
            .into(),
            groups: BTreeSet::new(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            deleted_at: None,
        }
    }

    #[tokio::test]
    async fn test_create_client_with_secret() {
        let user = mock_user();
        let mut caller = ApiCaller {
            id: user.id,
            permissions: user.permissions.clone(),
        };

        let mut user_store = MockApiUserStore::new();
        user_store
            .expect_get()
            .with(eq(user.id), eq(false))
            .returning(move |_, _| Ok(Some(user.clone())));
        user_store.expect_upsert().returning(|user| {
            Ok(ApiUser {
                id: user.id,
                permissions: user.permissions,
                groups: user.groups,
                created_at: Utc::now(),
                updated_at: Utc::now(),
                deleted_at: None,
            })
        });

        let mut store = MockOAuthClientStore::new();
        store.expect_upsert().returning(|client| {
            Ok(OAuthClient {
                id: client.id,
                secrets: vec![],
                redirect_uris: vec![],
                created_at: Utc::now(),
                deleted_at: None,
            })
        });

        let last_stored_secret = Arc::new(Mutex::new(None));

        let mut secret_store = MockOAuthClientSecretStore::new();
        let extractor = last_stored_secret.clone();
        secret_store.expect_upsert().returning(move |secret| {
            let stored = OAuthClientSecret {
                id: secret.id,
                oauth_client_id: secret.oauth_client_id,
                secret_signature: secret.secret_signature,
                created_at: Utc::now(),
                deleted_at: None,
            };

            let mut extract = extractor.lock().unwrap();
            *extract = Some(stored.clone());
            drop(extract);

            Ok(stored)
        });

        let mut storage = MockStorage::new();
        storage.api_user_store = Some(Arc::new(user_store));
        storage.oauth_client_store = Some(Arc::new(store));
        storage.oauth_client_secret_store = Some(Arc::new(secret_store));

        let ctx = mock_context(storage).await;

        let mut client = create_oauth_client_op(&ctx, &caller).await.unwrap().0;
        caller
            .permissions
            .insert(ApiPermission::UpdateOAuthClient(client.id));

        let secret = create_oauth_client_secret_op(&ctx, &caller, client.id)
            .await
            .unwrap()
            .0;
        client
            .secrets
            .push(last_stored_secret.lock().unwrap().clone().unwrap());

        let key = RawApiKey::try_from(&secret.key.0).unwrap();

        assert!(client.is_secret_valid(&key, &*ctx.secrets.signer))
    }
}
