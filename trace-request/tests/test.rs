use dropshot::{endpoint, HttpError, HttpResponseOk, RequestContext};
use trace_request::trace_request;

#[trace_request]
#[endpoint {
    method = GET,
    path = "/test"
}]
async fn _trace_entry_exit(rqctx: RequestContext<()>) -> Result<HttpResponseOk<()>, HttpError> {
    Ok(HttpResponseOk(()))
}
