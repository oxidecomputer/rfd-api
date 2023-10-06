use dropshot::{endpoint, HttpError, HttpResponseOk, Path, Query, RequestContext};
use http::StatusCode;
use rfd_model::{schema_ext::Visibility, storage::RfdFilter};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use trace_request::trace_request;
use tracing::instrument;

use crate::{
    context::{ApiContext, FullRfd, ListRfd},
    error::ApiError,
    permissions::ApiPermission,
    util::response::{client_error, internal_error, not_found, unauthorized},
    ApiCaller,
};

#[derive(Debug, Deserialize, JsonSchema)]
pub struct RfdPathParams {
    number: String,
}

/// List all available RFDs
#[trace_request]
#[endpoint {
    method = GET,
    path = "/rfd",
}]
#[instrument(skip(rqctx), fields(request_id = rqctx.request_id), err(Debug))]
pub async fn get_rfds(
    rqctx: RequestContext<ApiContext>,
) -> Result<HttpResponseOk<Vec<ListRfd>>, HttpError> {
    let ctx = rqctx.context();
    let auth = ctx.authn_token(&rqctx).await?;
    get_rfds_op(ctx, &ctx.get_caller(&auth).await?).await
}

#[instrument(skip(ctx, caller), fields(caller = ?caller.id), err(Debug))]
async fn get_rfds_op(
    ctx: &ApiContext,
    caller: &ApiCaller,
) -> Result<HttpResponseOk<Vec<ListRfd>>, HttpError> {
    let rfds = ctx
        .list_rfds(caller, None)
        .await
        .map_err(ApiError::Storage)?;
    Ok(HttpResponseOk(rfds))
}

/// Get the latest representation of an RFD
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
        // Lookup occurs before permission checks, as visibility controls are stored on the RFD
        // itself. This would be better if we could perform all authz checks prior to retrieving
        // the RFD. As-is we leak the how many RFDs exist
        match ctx.get_rfd(rfd_number, None).await {
            Ok(result) => match result {
                Some(rfd) => {
                    let visible = rfd.visibility == Visibility::Public
                        || caller.any(&[
                            &ApiPermission::GetRfd(rfd_number).into(),
                            &ApiPermission::GetRfdsAll.into(),
                        ]);

                    if visible {
                        Ok(HttpResponseOk(rfd))
                    } else {
                        Err(client_error(StatusCode::FORBIDDEN, "Unauthorized"))
                    }
                }
                None => {
                    tracing::error!(?rfd_number, "Failed to find RFD");
                    Err(not_found("Failed to find RFD"))
                }
            },
            Err(err) => {
                tracing::error!(?rfd_number, ?err, "Looking up RFD failed");
                Err(internal_error("Failed to lookup RFD"))
            }
        }
    } else {
        Err(client_error(
            StatusCode::BAD_REQUEST,
            "Malformed RFD number",
        ))
    }
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct RfdSearchQuery {
    q: String,
}

/// Search the RFD index and get a list of results
#[trace_request]
#[endpoint {
    method = GET,
    path = "/rfd-search",
}]
#[instrument(skip(rqctx), fields(request_id = rqctx.request_id), err(Debug))]
pub async fn search_rfds(
    rqctx: RequestContext<ApiContext>,
    query: Query<RfdSearchQuery>,
) -> Result<HttpResponseOk<Vec<ListRfd>>, HttpError> {
    let ctx = rqctx.context();
    let auth = ctx.authn_token(&rqctx).await?;
    search_rfds_op(ctx, &ctx.get_caller(&auth).await?, query.into_inner()).await
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
struct MinimalSearchResult {
    rfd_number: i32,
}

#[instrument(skip(ctx, caller), fields(caller = ?caller.id), err(Debug))]
async fn search_rfds_op(
    ctx: &ApiContext,
    caller: &ApiCaller,
    query: RfdSearchQuery,
) -> Result<HttpResponseOk<Vec<ListRfd>>, HttpError> {
    if caller.can(&ApiPermission::SearchRfds) {
        let results = ctx
            .search
            .client
            .index(&ctx.search.index)
            .search()
            .with_query(&query.q)
            .with_limit(999999)
            .execute::<MinimalSearchResult>()
            .await;
        tracing::trace!(?results, "Fetched search results from remote");

        match results {
            Ok(results) => {
                let rfds = results
                    .hits
                    .into_iter()
                    .map(|result| result.result.rfd_number)
                    .collect::<Vec<_>>();

                let found_rfds = ctx
                    .list_rfds(caller, Some(RfdFilter::default().rfd_number(Some(rfds))))
                    .await
                    .map_err(ApiError::Storage)?;

                Ok(HttpResponseOk(found_rfds))
            }
            Err(err) => {
                tracing::error!(?err, "Search request failed");
                Err(internal_error("Search failed".to_string()))
            }
        }
    } else {
        Err(unauthorized())
    }
}
