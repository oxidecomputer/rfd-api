use dropshot::{
    endpoint, http_response_temporary_redirect, HttpError, HttpResponseTemporaryRedirect, Path,
    RequestContext,
};
use rfd_data::RfdNumber;
use schemars::JsonSchema;
use serde::Deserialize;
use thiserror::Error;

use crate::context::Context;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct RfdPathParams {
    number: String,
}

#[endpoint {
    method = GET,
    path = "/{number}",
}]
pub async fn redirect_path(
    rqctx: RequestContext<Context>,
    path: Path<RfdPathParams>,
) -> Result<HttpResponseTemporaryRedirect, HttpError> {
    tracing::trace!("Attempt path redirect");

    let rfd_number = path
        .into_inner()
        .number
        .parse::<u16>()
        .ok()
        .filter(|rfd_number| rqctx.context().is_rfd_number_valid(*rfd_number));

    if let Some(rfd_number) = rfd_number {
        let location = rqctx.context().github_template.replace(
            "{rfd_number}",
            &RfdNumber::from(rfd_number as i32).as_number_string(),
        );
        tracing::trace!(?rfd_number, ?location, "Redirect to RFD via path");
        http_response_temporary_redirect(location)
    } else {
        Err(HttpError::for_not_found(
            None,
            format!("Invalid RFD number requested"),
        ))
    }
}

#[endpoint {
    method = GET,
    path = "/",
}]
pub async fn redirect_host(
    rqctx: RequestContext<Context>,
) -> Result<HttpResponseTemporaryRedirect, HttpError> {
    tracing::trace!("Attempt host redirect");

    if let Ok(host) = get_host_header(&rqctx) {
        if let Some(captures) = rqctx.context().host_regex.captures(&host) {
            if let Some(rfd_number) = captures
                .get(1)
                .and_then(|rfd_number| rfd_number.as_str().parse::<u16>().ok())
            {
                if rqctx.context().is_rfd_number_valid(rfd_number) {
                    return http_response_temporary_redirect(
                        rqctx.context().github_template.replace(
                            "{rfd_number}",
                            &RfdNumber::from(rfd_number as i32).as_number_string(),
                        ),
                    );
                }
            }
        }

        tracing::trace!(?host, "Dropping invalid host");
    }

    Err(HttpError::for_not_found(
        None,
        "Invalid host name received".to_string(),
    ))
}

#[derive(Debug, Error)]
enum HostError {
    #[error("Failed to extract a host header from the request")]
    FailedToExtractHost,
}

fn get_host_header(rqctx: &RequestContext<Context>) -> Result<String, HostError> {
    let header = rqctx.request.headers().get("Host");
    header
        .and_then(|header| header.to_str().ok())
        .map(|s| s.to_string())
        .ok_or(HostError::FailedToExtractHost)
}
