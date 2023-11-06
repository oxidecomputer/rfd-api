use dropshot::{endpoint, HttpError, HttpResponseOk, Path, RequestContext, TypedBody};
use rfd_model::{Mapper, NewMapper};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use trace_request::trace_request;
use tracing::instrument;
use uuid::Uuid;

use crate::{
    context::ApiContext,
    error::ApiError,
    mapper::MappingRules,
    permissions::ApiPermission,
    util::{
        is_uniqueness_error,
        response::{conflict, unauthorized},
    },
};

#[trace_request]
#[endpoint {
    method = GET,
    path = "/mapper",
}]
#[instrument(skip(rqctx), fields(request_id = rqctx.request_id), err(Debug))]
pub async fn get_mappers(
    rqctx: RequestContext<ApiContext>,
) -> Result<HttpResponseOk<Vec<Mapper>>, HttpError> {
    let ctx = rqctx.context();
    let auth = ctx.authn_token(&rqctx).await?;
    let caller = ctx.get_caller(auth.as_ref()).await?;

    if caller.can(&ApiPermission::ListMappers) {
        Ok(HttpResponseOk(
            ctx.get_mappers().await.map_err(ApiError::Storage)?,
        ))
    } else {
        Err(unauthorized())
    }
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
) -> Result<HttpResponseOk<Mapper>, HttpError> {
    let ctx = rqctx.context();
    let auth = ctx.authn_token(&rqctx).await?;
    let caller = ctx.get_caller(auth.as_ref()).await?;

    if caller.can(&ApiPermission::CreateMapper) {
        let body = body.into_inner();
        let res = ctx
            .add_mapper(&NewMapper {
                id: Uuid::new_v4(),
                name: body.name,
                // This was just unserialized from json, so it can be serialized back to a value
                rule: serde_json::to_value(body.rule).unwrap(),
                activations: body.max_activations.map(|_| 0),
                max_activations: body.max_activations,
            })
            .await;

        res.map(HttpResponseOk).map_err(|err| {
            if is_uniqueness_error(&err) {
                conflict()
            } else {
                ApiError::Storage(err).into()
            }
        })
    } else {
        Err(unauthorized())
    }
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
) -> Result<HttpResponseOk<Option<Mapper>>, HttpError> {
    let ctx = rqctx.context();
    let auth = ctx.authn_token(&rqctx).await?;
    let caller = ctx.get_caller(auth.as_ref()).await?;
    let path = path.into_inner();

    if caller.any(&[
        &ApiPermission::DeleteMapper(path.identifier).into(),
        &ApiPermission::ManageMapper(path.identifier).into(),
        &ApiPermission::ManageMappersAll.into(),
    ]) {
        Ok(HttpResponseOk(
            ctx.remove_mapper(&path.identifier)
                .await
                .map_err(ApiError::Storage)?,
        ))
    } else {
        Err(unauthorized())
    }
}
