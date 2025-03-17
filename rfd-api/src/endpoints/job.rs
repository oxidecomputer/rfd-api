// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use dropshot::{endpoint, ClientErrorStatusCode, HttpError, HttpResponseOk, Query, RequestContext};
use rfd_model::{storage::JobFilter, Job};
use schemars::JsonSchema;
use serde::Deserialize;
use trace_request::trace_request;
use tracing::instrument;
use v_api::{response::client_error, ApiContext};
use v_model::permissions::Caller;

use crate::{context::RfdContext, permissions::RfdPermission};

// Read Endpoints

#[derive(Debug, Deserialize, JsonSchema)]
struct ListJobsQuery {
    rfd: String,
}

/// List all jobs for a RFD
#[trace_request]
#[endpoint {
    method = GET,
    path = "/job",
}]
#[instrument(skip(rqctx), fields(request_id = rqctx.request_id), err(Debug))]
pub async fn list_jobs(
    rqctx: RequestContext<RfdContext>,
    query: Query<ListJobsQuery>,
) -> Result<HttpResponseOk<Vec<Job>>, HttpError> {
    let ctx = rqctx.context();
    let caller = ctx.v_ctx().get_caller(&rqctx).await?;
    let query = query.into_inner();
    list_jobs_op(ctx, &caller, query).await
}

// Read operation

#[instrument(skip(ctx, caller), fields(caller = ?caller.id), err(Debug))]
async fn list_jobs_op(
    ctx: &RfdContext,
    caller: &Caller<RfdPermission>,
    query: ListJobsQuery,
) -> Result<HttpResponseOk<Vec<Job>>, HttpError> {
    if let Ok(rfd_number) = query.rfd.parse::<i32>() {
        let jobs = ctx
            .list_jobs(
                caller,
                Some(JobFilter::default().rfd(Some(vec![rfd_number]))),
            )
            .await?;
        Ok(HttpResponseOk(jobs))
    } else {
        Err(client_error(
            ClientErrorStatusCode::BAD_REQUEST,
            "Malformed RFD number",
        ))
    }
}
