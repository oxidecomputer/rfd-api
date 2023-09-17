use dropshot::{endpoint, HttpError, HttpResponseOk, Path, RequestContext};
use http::StatusCode;
use schemars::JsonSchema;
use serde::Deserialize;
use trace_request::trace_request;
use tracing::instrument;

use crate::{
    context::{ApiContext, FullRfd},
    permissions::ApiPermission,
    util::response::{client_error, internal_error, not_found},
    ApiCaller,
};

#[derive(Debug, Deserialize, JsonSchema)]
pub struct RfdPathParams {
    number: String,
}

// Get the latest known representation of an RFD
#[trace_request]
#[endpoint {
    method = GET,
    path = "/rfd/{number}",
}]
#[instrument(skip(rqctx), fields(request_id = rqctx.request_id), err(Debug))]
pub async fn get_rfd(
    rqctx: RequestContext<ApiContext>,
    path: Path<RfdPathParams>,
) -> Result<HttpResponseOk<FullRfd>, HttpError> {
    let ctx = rqctx.context();
    let auth = ctx.authn_token(&rqctx).await?;
    get_rfd_op(ctx, &ctx.get_caller(&auth).await?, path.into_inner().number).await
}

#[instrument(skip(ctx, caller), fields(caller = ?caller.id), err(Debug))]
async fn get_rfd_op(
    ctx: &ApiContext,
    caller: &ApiCaller,
    number: String,
) -> Result<HttpResponseOk<FullRfd>, HttpError> {
    if let Ok(rfd_number) = number.parse::<i32>() {
        if caller.any(&[
            &ApiPermission::GetRfd(rfd_number).into(),
            &ApiPermission::GetAllRfds.into(),
        ]) {
            match ctx.get_rfd(rfd_number, None).await {
                Ok(Some(rfd)) => Ok(HttpResponseOk(rfd)),
                Ok(None) => {
                    tracing::error!(?rfd_number, "Failed to find RFD");
                    Err(not_found("Failed to find RFD"))
                }
                Err(err) => {
                    tracing::error!(?rfd_number, ?err, "Looking up RFD failed");
                    Err(internal_error("Failed to lookup RFD"))
                }
            }
        } else {
            Err(client_error(StatusCode::FORBIDDEN, "Unauthorized"))
        }
    } else {
        Err(client_error(
            StatusCode::UNPROCESSABLE_ENTITY,
            "Malformed RFD number",
        ))
    }
}
