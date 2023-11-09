use chrono::{DateTime, Utc};
use dropshot::{endpoint, HttpError, HttpResponseOk, Path, RequestContext, TypedBody};
use http::StatusCode;
use rfd_model::{OAuthClient, OAuthClientRedirectUri, OAuthClientSecret};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use trace_request::trace_request;
use tracing::instrument;
use uuid::Uuid;

use crate::{
    authn::key::RawApiKey,
    context::ApiContext,
    error::ApiError,
    permissions::ApiPermission,
    util::response::{client_error, to_internal_error},
    ApiCaller, secrets::OpenApiSecretString,
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
    Ok(HttpResponseOk(
        ctx.list_oauth_clients()
            .await
            .map_err(ApiError::Storage)?
            .into_iter()
            .filter(|client| caller.can(&ApiPermission::GetOAuthClient(client.id)))
            .collect(),
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
    let caller = ctx.get_caller(auth.as_ref()).await?;
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
        ctx.add_permissions_to_user(
            &caller.id,
            vec![
                ApiPermission::GetOAuthClient(client.id),
                ApiPermission::UpdateOAuthClient(client.id),
                ApiPermission::DeleteOAuthClient(client.id),
            ]
            .into(),
        )
        .await
        .map_err(ApiError::Storage)?;

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
    let caller = ctx.get_caller(auth.as_ref()).await?;
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
    if caller.can(&ApiPermission::UpdateOAuthClient(client_id)) {
        let id = Uuid::new_v4();
        let secret = RawApiKey::generate::<24>(&id)
            .sign(&*ctx.secrets.signer)
            .await
            .map_err(to_internal_error)?;
        let client_secret = ctx
            .add_oauth_secret(&id, &client_id, secret.signature())
            .await
            .map_err(ApiError::Storage)?;

        Ok(HttpResponseOk(InitialOAuthClientSecretResponse {
            id: client_secret.id,
            key: secret.key().into(),
            created_at: client_secret.created_at,
        }))
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
    let caller = ctx.get_caller(auth.as_ref()).await?;
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
    let caller = ctx.get_caller(auth.as_ref()).await?;
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
        ApiUser {
            id: Uuid::new_v4(),
            permissions: vec![ApiPermission::CreateOAuthClient].into(),
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
