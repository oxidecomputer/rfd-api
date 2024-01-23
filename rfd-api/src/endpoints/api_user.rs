// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::collections::BTreeSet;

use chrono::{DateTime, Utc};
use dropshot::{
    endpoint, HttpError, HttpResponseCreated, HttpResponseOk, HttpResponseUpdatedNoContent, Path,
    RequestContext, TypedBody,
};
use partial_struct::partial;
use rfd_model::{
    storage::{ApiUserProviderFilter, ListPagination},
    ApiUser, ApiUserProvider, NewApiKey, NewApiUser,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tap::TapFallible;
use trace_request::trace_request;
use tracing::instrument;
use uuid::Uuid;
use w_api_permissions::Permissions;

use crate::{
    authn::key::RawApiKey,
    context::ApiContext,
    error::ApiError,
    permissions::ApiPermissionResponse,
    secrets::OpenApiSecretString,
    util::response::{bad_request, not_found, to_internal_error, unauthorized},
    ApiCaller, ApiPermissions, User,
};

pub type UserResponse = ApiUser<ApiPermissionResponse>;

fn into_user_response(user: User) -> UserResponse {
    ApiUser {
        id: user.id,
        permissions: user
            .permissions
            .into_iter()
            .map(|p| p.into())
            .collect::<Permissions<ApiPermissionResponse>>(),
        groups: user.groups,
        created_at: user.created_at,
        updated_at: user.updated_at,
        deleted_at: user.deleted_at,
    }
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct GetUserResponse {
    info: ApiUser<ApiPermissionResponse>,
    providers: Vec<ApiUserProvider>,
}

impl GetUserResponse {
    pub fn new(user: User, providers: Vec<ApiUserProvider>) -> Self {
        Self {
            info: into_user_response(user),
            providers,
        }
    }
}

/// Retrieve the user information of the calling user
#[trace_request]
#[endpoint {
    method = GET,
    path = "/self",
}]
#[instrument(skip(rqctx), fields(request_id = rqctx.request_id), err(Debug))]
pub async fn get_self(
    rqctx: RequestContext<ApiContext>,
) -> Result<HttpResponseOk<GetUserResponse>, HttpError> {
    let ctx = rqctx.context();
    let auth = ctx.authn_token(&rqctx).await?;
    let caller = ctx.get_caller(auth.as_ref()).await?;
    get_api_user_op(ctx, &caller, &caller.id).await
}

/// Get user information for a given user id
#[trace_request]
#[endpoint {
    method = GET,
    path = "/api-user/{identifier}",
}]
#[instrument(skip(rqctx), fields(request_id = rqctx.request_id), err(Debug))]
pub async fn get_api_user(
    rqctx: RequestContext<ApiContext>,
    path: Path<ApiUserPath>,
) -> Result<HttpResponseOk<GetUserResponse>, HttpError> {
    let ctx = rqctx.context();
    let auth = ctx.authn_token(&rqctx).await?;
    let caller = ctx.get_caller(auth.as_ref()).await?;
    get_api_user_op(ctx, &caller, &path.into_inner().identifier).await
}

#[instrument(skip(ctx, caller), fields(caller = ?caller.id), err(Debug))]
async fn get_api_user_op(
    ctx: &ApiContext,
    caller: &ApiCaller,
    user_id: &Uuid,
) -> Result<HttpResponseOk<GetUserResponse>, HttpError> {
    let user = ctx.get_api_user(&caller, &caller.id).await?;

    let mut filter = ApiUserProviderFilter::default();
    filter.api_user_id = Some(vec![caller.id]);
    let providers = ctx
        .list_api_user_provider(caller, filter, &ListPagination::default().limit(10))
        .await?;

    tracing::trace!(user = ?serde_json::to_string(&user), "Found user");
    Ok(HttpResponseOk(GetUserResponse::new(user, providers)))
}

#[derive(Debug, Clone, PartialEq, Deserialize, JsonSchema)]
pub struct ApiUserUpdateParams {
    permissions: ApiPermissions,
    groups: BTreeSet<Uuid>,
}

/// Create a new user with a given set of permissions
#[trace_request]
#[endpoint {
    method = POST,
    path = "/api-user",
}]
#[instrument(skip(rqctx), fields(request_id = rqctx.request_id), err(Debug))]
pub async fn create_api_user(
    rqctx: RequestContext<ApiContext>,
    body: TypedBody<ApiUserUpdateParams>,
) -> Result<HttpResponseCreated<UserResponse>, HttpError> {
    let ctx = rqctx.context();
    let auth = ctx.authn_token(&rqctx).await?;
    let caller = ctx.get_caller(auth.as_ref()).await?;
    create_api_user_op(ctx, &caller, body.into_inner()).await
}

#[instrument(skip(ctx, caller, body), fields(caller = ?caller.id), err(Debug))]
async fn create_api_user_op(
    ctx: &ApiContext,
    caller: &ApiCaller,
    body: ApiUserUpdateParams,
) -> Result<HttpResponseCreated<UserResponse>, HttpError> {
    let user = ctx
        .create_api_user(caller, body.permissions, body.groups)
        .await?;

    Ok(HttpResponseCreated(into_user_response(user)))
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ApiUserPath {
    identifier: Uuid,
}

/// Update the permissions assigned to a given user
#[trace_request]
#[endpoint {
    method = POST,
    path = "/api-user/{identifier}",
}]
#[instrument(skip(rqctx), fields(request_id = rqctx.request_id), err(Debug))]
pub async fn update_api_user(
    rqctx: RequestContext<ApiContext>,
    path: Path<ApiUserPath>,
    body: TypedBody<ApiUserUpdateParams>,
) -> Result<HttpResponseOk<UserResponse>, HttpError> {
    let ctx = rqctx.context();
    let auth = ctx.authn_token(&rqctx).await?;
    let caller = ctx.get_caller(auth.as_ref()).await?;
    update_api_user_op(ctx, &caller, &path.into_inner(), body.into_inner()).await
}

#[instrument(skip(ctx, caller, body), fields(caller = ?caller.id), err(Debug))]
async fn update_api_user_op(
    ctx: &ApiContext,
    caller: &ApiCaller,
    path: &ApiUserPath,
    body: ApiUserUpdateParams,
) -> Result<HttpResponseOk<UserResponse>, HttpError> {
    let user = ctx
        .update_api_user(
            caller,
            NewApiUser {
                id: path.identifier,
                permissions: body.permissions,
                groups: body.groups,
            },
        )
        .await?;

    Ok(HttpResponseOk(into_user_response(user)))
}

/// List the active and expired API tokens for a given user
#[trace_request]
#[endpoint {
    method = GET,
    path = "/api-user/{identifier}/token",
}]
#[instrument(skip(rqctx), fields(request_id = rqctx.request_id), err(Debug))]
pub async fn list_api_user_tokens(
    rqctx: RequestContext<ApiContext>,
    path: Path<ApiUserPath>,
) -> Result<HttpResponseOk<Vec<ApiKeyResponse>>, HttpError> {
    let ctx = rqctx.context();
    let auth = ctx.authn_token(&rqctx).await?;
    let caller = ctx.get_caller(auth.as_ref()).await?;
    list_api_user_tokens_op(ctx, &caller, &path.into_inner()).await
}

#[instrument(skip(ctx, caller), fields(caller = ?caller.id), err(Debug))]
async fn list_api_user_tokens_op(
    ctx: &ApiContext,
    caller: &ApiCaller,
    path: &ApiUserPath,
) -> Result<HttpResponseOk<Vec<ApiKeyResponse>>, HttpError> {
    tracing::info!("Fetch token list");

    let tokens = ctx
        .get_api_user_tokens(caller, &path.identifier, &ListPagination::default())
        .await?;

    tracing::info!(count = ?tokens.len(), "Retrieved token list");

    Ok(HttpResponseOk(
        tokens
            .into_iter()
            .map(|token| ApiKeyResponse {
                id: token.id,
                permissions: into_permissions_response(token.permissions),
                created_at: token.created_at,
            })
            .collect(),
    ))
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct ApiKeyCreateParams {
    permissions: Option<Permissions<ApiPermissionResponse>>,
    expires_at: DateTime<Utc>,
}

#[partial(ApiKeyResponse)]
#[derive(Debug, Serialize, JsonSchema)]
pub struct InitialApiKeyResponse {
    pub id: Uuid,
    #[partial(ApiKeyResponse(skip))]
    pub key: OpenApiSecretString,
    pub permissions: Option<Permissions<ApiPermissionResponse>>,
    pub created_at: DateTime<Utc>,
}

// Create a new API token for a given user with a specific set of permissions and expiration. This
// is the only time that the returned token will be accessible
#[trace_request]
#[endpoint {
    method = POST,
    path = "/api-user/{identifier}/token",
}]
#[instrument(skip(rqctx), fields(request_id = rqctx.request_id), err(Debug))]
pub async fn create_api_user_token(
    rqctx: RequestContext<ApiContext>,
    path: Path<ApiUserPath>,
    body: TypedBody<ApiKeyCreateParams>,
) -> Result<HttpResponseCreated<InitialApiKeyResponse>, HttpError> {
    let ctx = rqctx.context();
    let auth = ctx.authn_token(&rqctx).await?;
    let caller = ctx.get_caller(auth.as_ref()).await?;
    create_api_user_token_op(ctx, &caller, &path.into_inner(), body.into_inner()).await
}

#[instrument(skip(ctx, caller, body), fields(caller = ?caller.id), err(Debug))]
async fn create_api_user_token_op(
    ctx: &ApiContext,
    caller: &ApiCaller,
    path: &ApiUserPath,
    body: ApiKeyCreateParams,
) -> Result<HttpResponseCreated<InitialApiKeyResponse>, HttpError> {
    let api_user = ctx.get_api_user(caller, &path.identifier).await?;

    let key_id = Uuid::new_v4();
    let key = RawApiKey::generate::<24>(&key_id)
        .sign(&*ctx.secrets.signer)
        .await
        .map_err(to_internal_error)?;

    let user_key = ctx
        .create_api_user_token(
            caller,
            NewApiKey {
                id: key_id,
                api_user_id: path.identifier,
                key_signature: key.signature().to_string(),
                permissions: into_permissions(body.permissions),
                expires_at: body.expires_at,
            },
            &api_user.id,
        )
        .await?;

    // Returning an api token will return the hashed version, but we need to return the
    // plaintext token as we do not store a copy
    Ok(HttpResponseCreated(InitialApiKeyResponse {
        id: user_key.id,
        key: key.key().into(),
        permissions: into_permissions_response(user_key.permissions),
        created_at: user_key.created_at,
    }))
}

// The identifier field is currently unused
#[allow(dead_code)]
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ApiUserTokenPath {
    identifier: Uuid,
    token_identifier: Uuid,
}

// Get details for a specific API token
#[trace_request]
#[endpoint {
    method = GET,
    path = "/api-user/{identifier}/token/{token_identifier}",
}]
#[instrument(skip(rqctx), fields(request_id = rqctx.request_id), err(Debug))]
pub async fn get_api_user_token(
    rqctx: RequestContext<ApiContext>,
    path: Path<ApiUserTokenPath>,
) -> Result<HttpResponseOk<ApiKeyResponse>, HttpError> {
    let ctx = rqctx.context();
    let auth = ctx.authn_token(&rqctx).await?;
    let caller = ctx.get_caller(auth.as_ref()).await?;
    get_api_user_token_op(ctx, &caller, &path.into_inner()).await
}

#[instrument(skip(ctx, caller), fields(caller = ?caller.id), err(Debug))]
async fn get_api_user_token_op(
    ctx: &ApiContext,
    caller: &ApiCaller,
    path: &ApiUserTokenPath,
) -> Result<HttpResponseOk<ApiKeyResponse>, HttpError> {
    let token = ctx
        .get_api_user_token(caller, &path.token_identifier)
        .await?;

    Ok(HttpResponseOk(ApiKeyResponse {
        id: token.id,
        permissions: into_permissions_response(token.permissions),
        created_at: token.created_at,
    }))
}

// Revoke a specific API token so it can no longer be used
#[trace_request]
#[endpoint {
    method = DELETE,
    path = "/api-user/{identifier}/token/{token_identifier}",
}]
#[instrument(skip(rqctx), fields(request_id = rqctx.request_id), err(Debug))]
pub async fn delete_api_user_token(
    rqctx: RequestContext<ApiContext>,
    path: Path<ApiUserTokenPath>,
) -> Result<HttpResponseOk<ApiKeyResponse>, HttpError> {
    let ctx = rqctx.context();
    let auth = ctx.authn_token(&rqctx).await?;
    let caller = ctx.get_caller(auth.as_ref()).await?;
    delete_api_user_token_op(ctx, &caller, &path.into_inner()).await
}

#[instrument(skip(ctx, caller), fields(caller = ?caller.id), err(Debug))]
async fn delete_api_user_token_op(
    ctx: &ApiContext,
    caller: &ApiCaller,
    path: &ApiUserTokenPath,
) -> Result<HttpResponseOk<ApiKeyResponse>, HttpError> {
    let token = ctx
        .delete_api_user_token(caller, &path.token_identifier)
        .await?;

    Ok(HttpResponseOk(ApiKeyResponse {
        id: token.id,
        permissions: into_permissions_response(token.permissions),
        created_at: token.created_at,
    }))
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct AddGroupBody {
    group_id: Uuid,
}

#[trace_request]
#[endpoint {
    method = POST,
    path = "/api-user/{identifier}/group",
}]
#[instrument(skip(rqctx), fields(request_id = rqctx.request_id), err(Debug))]
pub async fn add_api_user_to_group(
    rqctx: RequestContext<ApiContext>,
    path: Path<ApiUserPath>,
    body: TypedBody<AddGroupBody>,
) -> Result<HttpResponseOk<UserResponse>, HttpError> {
    let ctx = rqctx.context();
    let auth = ctx.authn_token(&rqctx).await?;
    let caller = ctx.get_caller(auth.as_ref()).await?;
    let path = path.into_inner();
    let body = body.into_inner();

    let user = ctx
        .add_api_user_to_group(&caller, &path.identifier, &body.group_id)
        .await?;

    Ok(HttpResponseOk(into_user_response(user)))
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ApiUserRemoveGroupPath {
    identifier: Uuid,
    group_id: Uuid,
}

#[trace_request]
#[endpoint {
    method = DELETE,
    path = "/api-user/{identifier}/group/{group_id}",
}]
#[instrument(skip(rqctx), fields(request_id = rqctx.request_id), err(Debug))]
pub async fn remove_api_user_from_group(
    rqctx: RequestContext<ApiContext>,
    path: Path<ApiUserRemoveGroupPath>,
) -> Result<HttpResponseOk<UserResponse>, HttpError> {
    let ctx = rqctx.context();
    let auth = ctx.authn_token(&rqctx).await?;
    let caller = ctx.get_caller(auth.as_ref()).await?;
    let path = path.into_inner();

    let user = ctx
        .remove_api_user_from_group(&caller, &path.identifier, &path.group_id)
        .await?;

    Ok(HttpResponseOk(into_user_response(user)))
}

// TODO: Needs to be implemented

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ApiUserProviderLinkPayload {
    token: String,
}

/// Link an existing login provider to this user
#[trace_request]
#[endpoint {
    method = POST,
    path = "/api-user/{identifier}/link",
}]
#[instrument(skip(rqctx), fields(request_id = rqctx.request_id), err(Debug))]
pub async fn link_provider(
    rqctx: RequestContext<ApiContext>,
    path: Path<ApiUserPath>,
    body: TypedBody<ApiUserProviderLinkPayload>,
) -> Result<HttpResponseUpdatedNoContent, HttpError> {
    let ctx = rqctx.context();
    let auth = ctx.authn_token(&rqctx).await?;
    let caller = ctx.get_caller(auth.as_ref()).await?;
    let path = path.into_inner();
    let body = body.into_inner();

    // TODO: This permission check indicates that the permission modeling for this functionality
    // is not correct. Need to rethink it
    //
    // This endpoint can only be called by the user themselves, it can not be performed on behalf
    // of a user
    if path.identifier == caller.id {
        let secret = RawApiKey::try_from(body.token.as_str()).map_err(|err| {
            tracing::debug!(?err, "Invalid link request token");
            bad_request("Malformed link request token")
        })?;
        let link_request_id = Uuid::from_slice(secret.id()).map_err(|err| {
            tracing::debug!(?err, "Failed to parse link request id from token");
            bad_request("Invalid link request token")
        })?;

        // TODO: We need an actual permission for reading a LinkRequest
        let link_request = ctx
            .get_link_request(&link_request_id)
            .await
            .map_err(ApiError::Storage)?
            .ok_or_else(|| not_found("Failed to find identifier"))?;

        // TODO: How can this check be lowered to the context (including a proper permission check)

        // Verify that the found link request is assigned to the user calling the endpoint and that
        // the token provided matches the stored signature
        if link_request.target_api_user_id == caller.id
            && secret
                .verify(
                    &*ctx.secrets.signer,
                    link_request.secret_signature.as_bytes(),
                )
                .is_ok()
        {
            let provider = ctx
                .complete_link_request(&caller, link_request)
                .await
                .tap_err(|err| tracing::error!(?err, "Failed to complete link request"))?;

            tracing::info!(?provider, "Completed link request");
            Ok(HttpResponseUpdatedNoContent())
        } else {
            Err(unauthorized())
        }
    } else {
        Err(unauthorized())
    }
}

fn into_permissions(
    permissions: Option<Permissions<ApiPermissionResponse>>,
) -> Option<ApiPermissions> {
    permissions.map(|permissions| {
        permissions
            .into_iter()
            .map(|p| p.into())
            .collect::<ApiPermissions>()
    })
}

fn into_permissions_response(
    permissions: Option<ApiPermissions>,
) -> Option<Permissions<ApiPermissionResponse>> {
    permissions.map(|permissions| {
        permissions
            .into_iter()
            .map(|p| p.into())
            .collect::<Permissions<ApiPermissionResponse>>()
    })
}

#[cfg(test)]
mod tests {
    use std::{collections::BTreeSet, sync::Arc};

    use chrono::{Duration, Utc};
    use http::StatusCode;
    use mockall::predicate::eq;
    use rfd_model::{
        storage::{ApiKeyFilter, ListPagination, MockApiKeyStore, MockApiUserStore, StoreError},
        ApiKey, ApiUser, NewApiUser,
    };
    use uuid::Uuid;

    use crate::{
        context::test_mocks::{mock_context, MockStorage},
        endpoints::api_user::{
            create_api_user_token_op, delete_api_user_token_op, get_api_user_token_op,
            into_permissions_response, list_api_user_tokens_op, update_api_user_op,
            ApiKeyCreateParams, ApiUserPath, ApiUserTokenPath,
        },
        permissions::ApiPermission,
        util::tests::get_status,
        ApiCaller,
    };

    use super::{create_api_user_op, ApiUserUpdateParams};

    fn mock_user() -> ApiUser<ApiPermission> {
        ApiUser {
            id: Uuid::new_v4(),
            permissions: vec![].into(),
            groups: BTreeSet::new(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            deleted_at: None,
        }
    }

    #[tokio::test]
    async fn test_create_api_user_permissions() {
        let successful_update = ApiUserUpdateParams {
            permissions: vec![ApiPermission::CreateApiUser.into()].into(),
            groups: BTreeSet::new(),
        };

        let failure_update = ApiUserUpdateParams {
            permissions: vec![ApiPermission::GetApiUserAll.into()].into(),
            groups: BTreeSet::new(),
        };

        let mut store = MockApiUserStore::new();
        store
            .expect_upsert()
            .withf(|x: &NewApiUser<ApiPermission>| {
                x.permissions.can(&ApiPermission::CreateApiUser.into())
            })
            .returning(|user| {
                Ok(ApiUser {
                    id: user.id,
                    permissions: user.permissions,
                    groups: BTreeSet::new(),
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                    deleted_at: None,
                })
            });
        store
            .expect_upsert()
            .withf(|x: &NewApiUser<ApiPermission>| {
                x.permissions.can(&ApiPermission::GetApiUserAll.into())
            })
            .returning(|_| Err(StoreError::Unknown));

        let mut storage = MockStorage::new();
        storage.api_user_store = Some(Arc::new(store));

        let ctx = mock_context(storage).await;

        let user1 = mock_user();

        // 1. Fail to create due to lack of permissions
        let no_permissions = ApiCaller {
            id: user1.id,
            permissions: Vec::new().into(),
        };

        let resp = create_api_user_op(&ctx, &no_permissions, successful_update.clone()).await;

        assert!(resp.is_err());
        assert_eq!(get_status(&resp), StatusCode::FORBIDDEN);

        let user2 = mock_user();

        // 2. Succeed in creating new api user
        let with_permissions = ApiCaller {
            id: user2.id,
            permissions: vec![ApiPermission::CreateApiUser.into()].into(),
        };

        let resp = create_api_user_op(&ctx, &with_permissions, successful_update.clone()).await;

        assert!(resp.is_ok());
        assert_eq!(get_status(&resp), StatusCode::CREATED);

        // 3. Handle storage failure and return error
        let resp = create_api_user_op(&ctx, &with_permissions, failure_update.clone()).await;

        assert!(resp.is_err());
        assert_eq!(get_status(&resp), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_update_api_user_permissions() {
        let success_id = Uuid::new_v4();
        let successful_update = ApiUserUpdateParams {
            permissions: Vec::new().into(),
            groups: BTreeSet::new(),
        };

        let failure_id = Uuid::new_v4();
        let failure_update = ApiUserUpdateParams {
            permissions: Vec::new().into(),
            groups: BTreeSet::new(),
        };

        let mut store = MockApiUserStore::new();
        store
            .expect_upsert()
            .withf(move |x: &NewApiUser<ApiPermission>| &x.id == &success_id)
            .returning(|user| {
                Ok(ApiUser {
                    id: user.id,
                    permissions: user.permissions,
                    groups: BTreeSet::new(),
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                    deleted_at: None,
                })
            });
        store
            .expect_upsert()
            .withf(move |x: &NewApiUser<ApiPermission>| &x.id == &failure_id)
            .returning(|_| Err(StoreError::Unknown));

        let mut storage = MockStorage::new();
        storage.api_user_store = Some(Arc::new(store));

        let ctx = mock_context(storage).await;

        let success_path = ApiUserPath {
            identifier: success_id,
        };
        let failure_path = ApiUserPath {
            identifier: failure_id,
        };

        let user1 = mock_user();

        // 1. Fail to create due to lack of permissions
        let no_permissions = ApiCaller {
            id: user1.id,
            permissions: Vec::new().into(),
        };

        let resp = update_api_user_op(
            &ctx,
            &no_permissions,
            &success_path,
            successful_update.clone(),
        )
        .await;

        assert!(resp.is_err());
        assert_eq!(get_status(&resp), StatusCode::FORBIDDEN);

        let user2 = mock_user();

        // 2. Succeed in updating api user with direct permission
        let with_specific_permissions = ApiCaller {
            id: user2.id,
            permissions: vec![ApiPermission::UpdateApiUser(success_path.identifier).into()].into(),
        };

        let resp = update_api_user_op(
            &ctx,
            &with_specific_permissions,
            &success_path,
            successful_update.clone(),
        )
        .await;

        assert!(resp.is_ok());
        assert_eq!(get_status(&resp), StatusCode::OK);

        let user3 = mock_user();

        // 3. Succeed in updating api user with general permission
        let with_general_permissions = ApiCaller {
            id: user3.id,
            permissions: vec![ApiPermission::UpdateApiUserAll.into()].into(),
        };

        let resp = update_api_user_op(
            &ctx,
            &with_general_permissions,
            &success_path,
            successful_update.clone(),
        )
        .await;

        assert!(resp.is_ok());
        assert_eq!(get_status(&resp), StatusCode::OK);

        // 4. Handle storage failure and return error
        let resp = update_api_user_op(
            &ctx,
            &with_general_permissions,
            &failure_path,
            failure_update.clone(),
        )
        .await;

        assert!(resp.is_err());
        assert_eq!(get_status(&resp), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_list_api_user_token_permissions() {
        let success_id = Uuid::new_v4();
        let failure_id = Uuid::new_v4();

        let mut store = MockApiKeyStore::new();
        store
            .expect_list()
            .withf(move |x: &ApiKeyFilter, _: &ListPagination| {
                x.api_user_id
                    .as_ref()
                    .map(|id| id.contains(&success_id))
                    .unwrap_or(false)
            })
            .returning(|_, _| Ok(vec![]));
        store
            .expect_list()
            .withf(move |x: &ApiKeyFilter, _: &ListPagination| {
                x.api_user_id
                    .as_ref()
                    .map(|id| id.contains(&failure_id))
                    .unwrap_or(false)
            })
            .returning(|_, _| Err(StoreError::Unknown));

        let mut storage = MockStorage::new();
        storage.api_user_token_store = Some(Arc::new(store));

        let ctx = mock_context(storage).await;

        let user1 = mock_user();

        // 1. Fail to list due to lack of permissions
        let no_permissions = ApiCaller {
            id: user1.id,
            permissions: Vec::new().into(),
        };

        let resp = list_api_user_tokens_op(
            &ctx,
            &no_permissions,
            &ApiUserPath {
                identifier: success_id,
            },
        )
        .await;

        assert!(resp.is_ok());
        assert_eq!(resp.unwrap().0.len(), 0);

        let user2 = mock_user();

        // 2. Fail to list due to incorrect permissions
        let incorrect_permissions = ApiCaller {
            id: user2.id,
            permissions: vec![ApiPermission::GetApiUserToken(Uuid::new_v4()).into()].into(),
        };

        let resp = list_api_user_tokens_op(
            &ctx,
            &incorrect_permissions,
            &ApiUserPath {
                identifier: success_id,
            },
        )
        .await;

        assert!(resp.is_ok());
        assert_eq!(resp.unwrap().0.len(), 0);

        let user3 = mock_user();

        // 3. Succeed in list tokens
        let success_permissions = ApiCaller {
            id: user3.id,
            permissions: vec![ApiPermission::GetApiUserToken(success_id).into()].into(),
        };

        let resp = list_api_user_tokens_op(
            &ctx,
            &success_permissions,
            &ApiUserPath {
                identifier: success_id,
            },
        )
        .await;

        assert!(resp.is_ok());
        assert_eq!(get_status(&resp), StatusCode::OK);

        let user4 = mock_user();

        // 4. Handle storage failure and return error
        let failure_permissions = ApiCaller {
            id: user4.id,
            permissions: vec![ApiPermission::GetApiUserToken(failure_id).into()].into(),
        };

        let resp = list_api_user_tokens_op(
            &ctx,
            &failure_permissions,
            &ApiUserPath {
                identifier: failure_id,
            },
        )
        .await;

        assert!(resp.is_err());
        assert_eq!(get_status(&resp), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_create_api_user_token_permissions() {
        let api_user_id = Uuid::new_v4();

        let api_user = ApiUser {
            id: api_user_id,
            permissions: vec![ApiPermission::GetApiUserToken(api_user_id).into()].into(),
            groups: BTreeSet::new(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            deleted_at: None,
        };

        let api_user_path = ApiUserPath {
            identifier: api_user.id,
        };

        let failure_api_user_path = ApiUserPath {
            identifier: Uuid::new_v4(),
        };

        let unknown_api_user_path = ApiUserPath {
            identifier: Uuid::new_v4(),
        };

        let new_token = ApiKeyCreateParams {
            permissions: into_permissions_response(None),
            expires_at: Utc::now() + Duration::seconds(5 * 60),
        };

        let mut api_user_store = MockApiUserStore::new();
        api_user_store
            .expect_get()
            .with(eq(api_user_path.identifier), eq(false))
            .returning(move |_, _| Ok(Some(api_user.clone())));
        api_user_store
            .expect_get()
            .with(eq(failure_api_user_path.identifier), eq(false))
            .returning(|_, _| Err(StoreError::Unknown));
        api_user_store
            .expect_get()
            .with(eq(unknown_api_user_path.identifier), eq(false))
            .returning(move |_, _| Ok(None));

        let mut token_store = MockApiKeyStore::new();
        token_store
            .expect_upsert()
            // .withf(move |_, user| user.id == api_user_id)
            .returning(move |key| {
                Ok(ApiKey {
                    id: Uuid::new_v4(),
                    api_user_id: api_user_id,
                    key_signature: key.key_signature,
                    permissions: key.permissions,
                    expires_at: key.expires_at,
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                    deleted_at: None,
                })
            });

        let mut storage = MockStorage::new();
        storage.api_user_store = Some(Arc::new(api_user_store));
        storage.api_user_token_store = Some(Arc::new(token_store));

        let ctx = mock_context(storage).await;

        let user1 = mock_user();

        // 1. Fail to create due to lack of permissions
        let no_permissions = ApiCaller {
            id: user1.id,
            permissions: Vec::new().into(),
        };

        let resp =
            create_api_user_token_op(&ctx, &no_permissions, &api_user_path, new_token.clone())
                .await;

        assert!(resp.is_err());
        assert_eq!(get_status(&resp), StatusCode::FORBIDDEN);

        let user2 = mock_user();

        // 2. Fail to create due to incorrect permissions
        let incorrect_permissions = ApiCaller {
            id: user2.id,
            permissions: vec![ApiPermission::CreateApiUserToken(Uuid::new_v4()).into()].into(),
        };

        let resp = create_api_user_token_op(
            &ctx,
            &incorrect_permissions,
            &api_user_path,
            new_token.clone(),
        )
        .await;

        assert!(resp.is_err());
        assert_eq!(get_status(&resp), StatusCode::FORBIDDEN);

        let user3 = mock_user();

        // 3. Fail to create due to unknown user
        let incorrect_permissions = ApiCaller {
            id: user3.id,
            permissions: vec![
                ApiPermission::GetApiUser(unknown_api_user_path.identifier).into(),
                ApiPermission::CreateApiUserToken(unknown_api_user_path.identifier).into(),
            ]
            .into(),
        };

        let resp = create_api_user_token_op(
            &ctx,
            &incorrect_permissions,
            &unknown_api_user_path,
            new_token.clone(),
        )
        .await;

        assert!(resp.is_err());
        assert_eq!(get_status(&resp), StatusCode::NOT_FOUND);

        let user4 = mock_user();

        // 4. Succeed in creating token
        let success_permissions = ApiCaller {
            id: user4.id,
            permissions: vec![
                ApiPermission::GetApiUser(api_user_path.identifier).into(),
                ApiPermission::CreateApiUserToken(api_user_path.identifier).into(),
            ]
            .into(),
        };

        let resp = create_api_user_token_op(
            &ctx,
            &success_permissions,
            &api_user_path,
            new_token.clone(),
        )
        .await;

        assert!(resp.is_ok());
        assert_eq!(get_status(&resp), StatusCode::CREATED);
        assert_eq!(resp.as_ref().unwrap().0.permissions, new_token.permissions);

        let user5 = mock_user();

        // 5. Handle storage failure and return error
        let failure_permissions = ApiCaller {
            id: user5.id,
            permissions: vec![
                ApiPermission::GetApiUser(failure_api_user_path.identifier).into(),
                ApiPermission::CreateApiUserToken(failure_api_user_path.identifier).into(),
            ]
            .into(),
        };

        let resp = create_api_user_token_op(
            &ctx,
            &failure_permissions,
            &failure_api_user_path,
            new_token,
        )
        .await;

        assert!(resp.is_err());
        assert_eq!(get_status(&resp), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_get_api_user_token_permissions() {
        let api_user_id = Uuid::new_v4();

        let token = ApiKey {
            id: Uuid::new_v4(),
            api_user_id: api_user_id,
            key_signature: "encrypted_key".to_string(),
            permissions: None,
            expires_at: Utc::now() + Duration::seconds(5 * 60),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            deleted_at: None,
        };

        let api_user_token_path = ApiUserTokenPath {
            identifier: api_user_id,
            token_identifier: token.id,
        };

        let failure_api_user_token_path = ApiUserTokenPath {
            identifier: api_user_id,
            token_identifier: Uuid::new_v4(),
        };

        let unknown_api_user_token_path = ApiUserTokenPath {
            identifier: api_user_id,
            token_identifier: Uuid::new_v4(),
        };

        let mut token_store = MockApiKeyStore::new();
        token_store
            .expect_get()
            .with(eq(api_user_token_path.token_identifier), eq(false))
            .returning(move |_, _| Ok(Some(token.clone())));
        token_store
            .expect_get()
            .with(eq(failure_api_user_token_path.token_identifier), eq(false))
            .returning(move |_, _| Err(StoreError::Unknown));
        token_store
            .expect_get()
            .with(eq(unknown_api_user_token_path.token_identifier), eq(false))
            .returning(move |_, _| Ok(None));

        let mut storage = MockStorage::new();
        storage.api_user_token_store = Some(Arc::new(token_store));

        let ctx = mock_context(storage).await;

        let user1 = mock_user();

        // 1. Fail to get due to lack of permissions
        let no_permissions = ApiCaller {
            id: user1.id,
            permissions: Vec::new().into(),
        };

        let resp = get_api_user_token_op(&ctx, &no_permissions, &api_user_token_path).await;

        assert!(resp.is_err());
        assert_eq!(get_status(&resp), StatusCode::FORBIDDEN);

        let user2 = mock_user();

        // 2. Fail to get due to incorrect permissions
        let incorrect_permissions = ApiCaller {
            id: user2.id,
            permissions: vec![ApiPermission::GetApiUserToken(Uuid::new_v4()).into()].into(),
        };

        let resp = get_api_user_token_op(&ctx, &incorrect_permissions, &api_user_token_path).await;

        assert!(resp.is_err());
        assert_eq!(get_status(&resp), StatusCode::FORBIDDEN);

        let user3 = mock_user();

        // 3. Fail to get due to unknown token id
        let incorrect_permissions = ApiCaller {
            id: user3.id,
            permissions: vec![ApiPermission::GetApiUserToken(
                unknown_api_user_token_path.token_identifier,
            )
            .into()]
            .into(),
        };

        let resp =
            get_api_user_token_op(&ctx, &incorrect_permissions, &unknown_api_user_token_path).await;

        assert!(resp.is_err());
        assert_eq!(get_status(&resp), StatusCode::NOT_FOUND);

        let user4 = mock_user();

        // 4. Succeed in getting token
        let success_permissions = ApiCaller {
            id: user4.id,
            permissions: vec![
                ApiPermission::GetApiUserToken(api_user_token_path.token_identifier).into(),
            ]
            .into(),
        };

        let resp = get_api_user_token_op(&ctx, &success_permissions, &api_user_token_path).await;

        assert!(resp.is_ok());
        assert_eq!(get_status(&resp), StatusCode::OK);

        let user5 = mock_user();

        // 5. Handle storage failure and return error
        let failure_permissions = ApiCaller {
            id: user5.id,
            permissions: vec![ApiPermission::GetApiUserToken(
                failure_api_user_token_path.token_identifier,
            )
            .into()]
            .into(),
        };

        let resp =
            get_api_user_token_op(&ctx, &failure_permissions, &failure_api_user_token_path).await;

        assert!(resp.is_err());
        assert_eq!(get_status(&resp), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_delete_api_user_token_permissions() {
        let api_user_id = Uuid::new_v4();

        let token = ApiKey {
            id: Uuid::new_v4(),
            api_user_id: api_user_id,
            key_signature: "encrypted_key".to_string(),
            permissions: None,
            expires_at: Utc::now() + Duration::seconds(5 * 60),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            deleted_at: None,
        };

        let api_user_token_path = ApiUserTokenPath {
            identifier: api_user_id,
            token_identifier: token.id,
        };

        let failure_api_user_token_path = ApiUserTokenPath {
            identifier: api_user_id,
            token_identifier: Uuid::new_v4(),
        };

        let unknown_api_user_token_path = ApiUserTokenPath {
            identifier: api_user_id,
            token_identifier: Uuid::new_v4(),
        };

        let mut token_store = MockApiKeyStore::new();
        token_store
            .expect_delete()
            .with(eq(api_user_token_path.token_identifier))
            .returning(move |_| Ok(Some(token.clone())));
        token_store
            .expect_delete()
            .with(eq(failure_api_user_token_path.token_identifier))
            .returning(move |_| Err(StoreError::Unknown));
        token_store
            .expect_delete()
            .with(eq(unknown_api_user_token_path.token_identifier))
            .returning(move |_| Ok(None));

        let mut storage = MockStorage::new();
        storage.api_user_token_store = Some(Arc::new(token_store));

        let ctx = mock_context(storage).await;

        let user1 = mock_user();

        // 1. Fail to get due to lack of permissions
        let no_permissions = ApiCaller {
            id: user1.id,
            permissions: Vec::new().into(),
        };

        let resp = delete_api_user_token_op(&ctx, &no_permissions, &api_user_token_path).await;

        assert!(resp.is_err());
        assert_eq!(get_status(&resp), StatusCode::FORBIDDEN);

        let user2 = mock_user();

        // 2. Fail to get due to incorrect permissions
        let incorrect_permissions = ApiCaller {
            id: user2.id,
            permissions: vec![ApiPermission::DeleteApiUserToken(Uuid::new_v4()).into()].into(),
        };

        let resp =
            delete_api_user_token_op(&ctx, &incorrect_permissions, &api_user_token_path).await;

        assert!(resp.is_err());
        assert_eq!(get_status(&resp), StatusCode::FORBIDDEN);

        let user3 = mock_user();

        // 3. Fail to get due to unknown token id
        let incorrect_permissions = ApiCaller {
            id: user3.id,
            permissions: vec![ApiPermission::DeleteApiUserToken(
                unknown_api_user_token_path.token_identifier,
            )
            .into()]
            .into(),
        };

        let resp =
            delete_api_user_token_op(&ctx, &incorrect_permissions, &unknown_api_user_token_path)
                .await;

        assert!(resp.is_err());
        assert_eq!(get_status(&resp), StatusCode::NOT_FOUND);

        let user4 = mock_user();

        // 4. Succeed in getting token
        let success_permissions = ApiCaller {
            id: user4.id,
            permissions: vec![ApiPermission::DeleteApiUserToken(
                api_user_token_path.token_identifier,
            )
            .into()]
            .into(),
        };

        let resp = delete_api_user_token_op(&ctx, &success_permissions, &api_user_token_path).await;

        assert!(resp.is_ok());
        assert_eq!(get_status(&resp), StatusCode::OK);

        let user5 = mock_user();

        // 5. Handle storage failure and return error
        let failure_permissions = ApiCaller {
            id: user5.id,
            permissions: vec![ApiPermission::DeleteApiUserToken(
                failure_api_user_token_path.token_identifier,
            )
            .into()]
            .into(),
        };

        let resp =
            delete_api_user_token_op(&ctx, &failure_permissions, &failure_api_user_token_path)
                .await;

        assert!(resp.is_err());
        assert_eq!(get_status(&resp), StatusCode::INTERNAL_SERVER_ERROR);
    }
}
