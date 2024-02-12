// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use dropshot::{
    endpoint, HttpError, HttpResponseCreated, HttpResponseOk, Path, Query, RequestContext,
    TypedBody,
};
use rfd_model::{Mapper, NewMapper};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use trace_request::trace_request;
use tracing::instrument;
use uuid::Uuid;

use crate::{
    context::ApiContext,
    mapper::MappingRules,
    util::{
        is_uniqueness_error,
        response::{conflict, ResourceError},
    },
};

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ListMappersQuery {
    /// Include depleted mappers in the returned results
    include_depleted: Option<bool>,
}

#[trace_request]
#[endpoint {
    method = GET,
    path = "/mapper",
}]
#[instrument(skip(rqctx), fields(request_id = rqctx.request_id), err(Debug))]
pub async fn get_mappers(
    rqctx: RequestContext<ApiContext>,
    query: Query<ListMappersQuery>,
) -> Result<HttpResponseOk<Vec<Mapper>>, HttpError> {
    let ctx = rqctx.context();
    let auth = ctx.authn_token(&rqctx).await?;
    let caller = ctx.get_caller(auth.as_ref()).await?;

    Ok(HttpResponseOk(
        ctx.get_mappers(
            &caller,
            query.into_inner().include_depleted.unwrap_or(false),
        )
        .await?,
    ))
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct CreateMapper {
    name: String,
    rule: MappingRules,
    max_activations: Option<i32>,
}

#[trace_request]
#[endpoint {
    method = POST,
    path = "/mapper",
}]
#[instrument(skip(rqctx), fields(request_id = rqctx.request_id), err(Debug))]
pub async fn create_mapper(
    rqctx: RequestContext<ApiContext>,
    body: TypedBody<CreateMapper>,
) -> Result<HttpResponseCreated<Mapper>, HttpError> {
    let ctx = rqctx.context();
    let auth = ctx.authn_token(&rqctx).await?;
    let caller = ctx.get_caller(auth.as_ref()).await?;
    let body = body.into_inner();

    let res = ctx
        .add_mapper(
            &caller,
            &NewMapper {
                id: Uuid::new_v4(),
                name: body.name,
                // This was just unserialized from json, so it can be serialized back to a value
                rule: serde_json::to_value(body.rule).unwrap(),
                activations: body.max_activations.map(|_| 0),
                max_activations: body.max_activations,
            },
        )
        .await;

    res.map(HttpResponseCreated).map_err(|err| {
        if let ResourceError::InternalError(err) = &err {
            if is_uniqueness_error(&err) {
                return conflict();
            }
        }

        HttpError::from(err)
    })
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct MapperPath {
    identifier: Uuid,
}

#[trace_request]
#[endpoint {
    method = DELETE,
    path = "/mapper/{identifier}",
}]
#[instrument(skip(rqctx), fields(request_id = rqctx.request_id), err(Debug))]
pub async fn delete_mapper(
    rqctx: RequestContext<ApiContext>,
    path: Path<MapperPath>,
) -> Result<HttpResponseOk<Mapper>, HttpError> {
    let ctx = rqctx.context();
    let auth = ctx.authn_token(&rqctx).await?;
    let caller = ctx.get_caller(auth.as_ref()).await?;
    let path = path.into_inner();

    Ok(HttpResponseOk(
        ctx.remove_mapper(&caller, &path.identifier).await?,
    ))
}
