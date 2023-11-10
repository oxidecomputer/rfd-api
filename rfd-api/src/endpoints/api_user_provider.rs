use dropshot::{endpoint, HttpError, HttpResponseOk, Path, RequestContext, TypedBody};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use trace_request::trace_request;
use tracing::instrument;
use uuid::Uuid;

use crate::{
    context::ApiContext, error::ApiError, permissions::ApiPermission, secrets::OpenApiSecretString,
    util::response::forbidden,
};

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ApiUserProviderPath {
    identifier: Uuid,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ApiUserLinkRequestPayload {
    user_identifier: Uuid,
}

#[derive(Debug, Serialize, JsonSchema)]
pub struct ApiUserLinkRequestResponse {
    token: OpenApiSecretString,
}

/// Create a new link token for linking this provider to a different api user
#[trace_request]
#[endpoint {
    method = POST,
    path = "/api-user-provider/{identifier}/link-token",
}]
#[instrument(skip(rqctx), fields(request_id = rqctx.request_id), err(Debug))]
pub async fn create_link_token(
    rqctx: RequestContext<ApiContext>,
    path: Path<ApiUserProviderPath>,
    body: TypedBody<ApiUserLinkRequestPayload>,
) -> Result<HttpResponseOk<ApiUserLinkRequestResponse>, HttpError> {
    let ctx = rqctx.context();
    let auth = ctx.authn_token(&rqctx).await?;
    let caller = ctx.get_caller(auth.as_ref()).await?;
    let path = path.into_inner();
    let body = body.into_inner();

    let provider = ctx
        .get_api_user_provider(&path.identifier)
        .await
        .map_err(ApiError::Storage)?;

    if let Some(provider) = provider {
        if provider.api_user_id == caller.id
            && caller.can(&ApiPermission::CreateUserApiProviderLinkToken)
        {
            let token = ctx
                .create_link_request_token(&path.identifier, &caller.id, &body.user_identifier)
                .await
                .map_err(ApiError::Storage)?;

            Ok(HttpResponseOk(ApiUserLinkRequestResponse {
                token: token.key().into(),
            }))
        } else {
            tracing::info!(caller = ?caller.id, provider = ?provider.id, provider_user = ?provider.api_user_id, "User does not have permission to modify this provider");
            Err(forbidden())
        }
    } else {
        tracing::debug!(id = ?path.identifier, "Failed to find requested provider");
        Err(forbidden())
    }
}
