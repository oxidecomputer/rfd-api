use dropshot::{endpoint, RequestContext, Path, HttpResponseOk, HttpError};
use rfd_model::ApiUser;
use trace_request::trace_request;
use tracing::instrument;

use crate::{context::ApiContext, permissions::ApiPermission};

use super::api_user::ApiUserPath;

/// Create a new link token for linking this provider to a different api user
#[trace_request]
#[endpoint {
    method = POST,
    path = "/api-user/{identifier}/link-token",
}]
#[instrument(skip(rqctx), fields(request_id = rqctx.request_id), err(Debug))]
pub async fn create_link_token(
    rqctx: RequestContext<ApiContext>,
    path: Path<ApiUserPath>,
) -> Result<HttpResponseOk<ApiUser<ApiPermission>>, HttpError> {
    unimplemented!()
}
