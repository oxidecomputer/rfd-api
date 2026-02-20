// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::{collections::HashMap, sync::OnceLock};

use chrono::Utc;
use dropshot::{
    endpoint, ClientErrorStatusCode, HttpError, HttpResponseCreated, RequestContext, TypedBody,
};
use newtype_uuid::{GenericUuid, TypedUuid};
use rfd_model::{storage::InitializationStore, InitializationModel};
use schemars::JsonSchema;
use secrecy::ExposeSecret;
use serde::{Deserialize, Serialize};
use trace_request::trace_request;
use tracing::instrument;
use uuid::Uuid;
use v_api::{
    authn::key::RawKey,
    permissions::VPermission,
    response::{client_error, to_internal_error},
    ApiContext,
};
use v_model::{permissions::Caller, OAuthClientId, UserId};

use crate::{context::RfdContext, permissions::RfdPermission};

#[derive(Debug, Deserialize, JsonSchema)]
pub struct InitRequestBody {
    pub redirect_uris: Vec<String>,
}

#[derive(Debug, Serialize, JsonSchema)]
pub struct InitResponse {
    pub client_id: TypedUuid<OAuthClientId>,
    pub secret: String,
    pub redirect_uris: Vec<String>,
}

/// Initialize the system with an initial OAuth client.
///
/// This endpoint is unauthenticated and can only be called once. If the system
/// has already been initialized, this endpoint will return a 409 Conflict error.
///
/// To re-initialize the system, an administrator must manually delete the
/// initialization record from the database.
#[trace_request]
#[endpoint {
    method = POST,
    path = "/init",
    tags = ["hidden"],
}]
#[instrument(skip(rqctx), fields(request_id = rqctx.request_id), err(Debug))]
pub async fn init(
    rqctx: RequestContext<RfdContext>,
    body: TypedBody<InitRequestBody>,
) -> Result<HttpResponseCreated<InitResponse>, HttpError> {
    let ctx = rqctx.context();
    let body = body.into_inner();
    init_op(ctx, body).await
}

/// Hardcoded caller ID for the init endpoint. This is used for logging purposes
/// to identify that the operation was performed by the init endpoint.
const INIT_CALLER_ID: Uuid = Uuid::from_u128(0x00000000_0000_0000_0000_000000000001);

/// Static caller instance for the init endpoint, initialized once on first use.
static INIT_CALLER: OnceLock<Caller<RfdPermission>> = OnceLock::new();

/// Internal operation for system initialization, separated for testability.
#[instrument(skip(ctx), err(Debug))]
pub async fn init_op(
    ctx: &RfdContext,
    body: InitRequestBody,
) -> Result<HttpResponseCreated<InitResponse>, HttpError> {
    // Step 1: Check if initialization record already exists
    let existing = InitializationStore::get(&*ctx.storage)
        .await
        .map_err(to_internal_error)?;

    if existing.is_some() {
        return Err(client_error(
            ClientErrorStatusCode::CONFLICT,
            "System already initialized",
        ));
    }

    // Step 2: Get the singleton caller with full OAuth permissions for initialization.
    // This is safe because we've already verified this is the first initialization via the
    // InitializationStore check above.
    let caller = INIT_CALLER.get_or_init(|| Caller {
        id: TypedUuid::<UserId>::from_untyped_uuid(INIT_CALLER_ID),
        permissions: vec![
            RfdPermission::from(VPermission::CreateOAuthClient),
            RfdPermission::from(VPermission::ManageOAuthClientsAll),
        ]
        .into(),
        extensions: HashMap::new(),
    });

    // Step 3: Create the OAuth client
    let client = ctx
        .v_ctx()
        .oauth
        .create_oauth_client(&caller)
        .await
        .map_err(|e| {
            tracing::error!(?e, "Failed to create OAuth client");
            to_internal_error(e)
        })?;

    // Step 4: Create a secret for the client
    let secret_id = TypedUuid::new_v4();
    let secret = RawKey::generate::<24>(secret_id.as_untyped_uuid())
        .sign(ctx.v_ctx().signer())
        .await
        .map_err(|e| {
            tracing::error!(?e, "Failed to sign OAuth client secret");
            to_internal_error(e)
        })?;

    ctx.v_ctx()
        .oauth
        .add_oauth_secret(&caller, &secret_id, &client.id, &secret.signature().to_string())
        .await
        .map_err(|e| {
            tracing::error!(?e, "Failed to store OAuth client secret");
            to_internal_error(e)
        })?;

    // Step 5: Add all redirect URIs
    for redirect_uri in &body.redirect_uris {
        ctx.v_ctx()
            .oauth
            .add_oauth_redirect_uri(&caller, &client.id, redirect_uri)
            .await
            .map_err(|e| {
                tracing::error!(?e, ?redirect_uri, "Failed to add redirect URI");
                to_internal_error(e)
            })?;
    }

    // Step 6: Write the initialization record
    let init_record = InitializationModel {
        id: Uuid::new_v4(),
        initialized_at: Utc::now(),
        oauth_client_id: client.id.into_untyped_uuid(),
    };

    InitializationStore::insert(&*ctx.storage, init_record)
        .await
        .map_err(|e| {
            tracing::error!(?e, "Failed to insert initialization record");
            to_internal_error(e)
        })?;

    tracing::info!(
        client_id = %client.id,
        "System initialized successfully"
    );

    Ok(HttpResponseCreated(InitResponse {
        client_id: client.id,
        secret: secret.key().expose_secret().to_string(),
        redirect_uris: body.redirect_uris,
    }))
}

