use dropshot::{endpoint, HttpError, HttpResponseOk, Path, RequestContext, TypedBody};
use rfd_model::{AccessGroup, NewAccessGroup};
use schemars::JsonSchema;
use serde::Deserialize;
use trace_request::trace_request;
use tracing::instrument;
use uuid::Uuid;

use crate::{
    context::ApiContext, error::ApiError, permissions::ApiPermission, util::response::unauthorized,
    ApiPermissions,
};

#[trace_request]
#[endpoint {
    method = GET,
    path = "/group",    
}]
#[instrument(skip(rqctx), fields(request_id = rqctx.request_id), err(Debug))]
pub async fn get_groups(
    rqctx: RequestContext<ApiContext>,
) -> Result<HttpResponseOk<Vec<AccessGroup<ApiPermission>>>, HttpError> {
    let ctx = rqctx.context();
    let auth = ctx.authn_token(&rqctx).await;
    let caller = ctx.get_caller(&auth?).await?;

    if caller.can(&ApiPermission::ListGroups) {
        Ok(HttpResponseOk(
            ctx.get_groups().await.map_err(ApiError::Storage)?,
        ))
    } else {
        Err(unauthorized())
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, JsonSchema)]
pub struct AccessGroupUpdateParams {
    name: String,
    permissions: ApiPermissions,
}

#[trace_request]
#[endpoint {
    method = POST,
    path = "/group",    
}]
#[instrument(skip(rqctx), fields(request_id = rqctx.request_id), err(Debug))]
pub async fn create_group(
    rqctx: RequestContext<ApiContext>,
    body: TypedBody<AccessGroupUpdateParams>,
) -> Result<HttpResponseOk<AccessGroup<ApiPermission>>, HttpError> {
    let ctx = rqctx.context();
    let auth = ctx.authn_token(&rqctx).await;
    let caller = ctx.get_caller(&auth?).await?;

    if caller.can(&ApiPermission::CreateGroup) {
        let body = body.into_inner();
        Ok(HttpResponseOk(
            ctx.create_group(NewAccessGroup {
                id: Uuid::new_v4(),
                name: body.name,
                permissions: body.permissions,
            })
            .await
            .map_err(ApiError::Storage)?,
        ))
    } else {
        Err(unauthorized())
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, JsonSchema)]
pub struct AccessGroupPath {
    group_id: Uuid,
}

#[trace_request]
#[endpoint {
    method = PUT,
    path = "/group/{group_id}",    
}]
#[instrument(skip(rqctx), fields(request_id = rqctx.request_id), err(Debug))]
pub async fn update_group(
    rqctx: RequestContext<ApiContext>,
    path: Path<AccessGroupPath>,
    body: TypedBody<AccessGroupUpdateParams>,
) -> Result<HttpResponseOk<AccessGroup<ApiPermission>>, HttpError> {
    let ctx = rqctx.context();
    let auth = ctx.authn_token(&rqctx).await;
    let caller = ctx.get_caller(&auth?).await?;
    let path = path.into_inner();

    if caller.can(&ApiPermission::UpdateGroup(path.group_id)) {
        let body = body.into_inner();
        Ok(HttpResponseOk(
            ctx.update_group(NewAccessGroup {
                id: path.group_id,
                name: body.name,
                permissions: body.permissions,
            })
            .await
            .map_err(ApiError::Storage)?,
        ))
    } else {
        Err(unauthorized())
    }
}

#[trace_request]
#[endpoint {
    method = DELETE,
    path = "/group/{group_id}",    
}]
#[instrument(skip(rqctx), fields(request_id = rqctx.request_id), err(Debug))]
pub async fn delete_group(
    rqctx: RequestContext<ApiContext>,
    path: Path<AccessGroupPath>,
) -> Result<HttpResponseOk<Option<AccessGroup<ApiPermission>>>, HttpError> {
    let ctx = rqctx.context();
    let auth = ctx.authn_token(&rqctx).await;
    let caller = ctx.get_caller(&auth?).await?;
    let path = path.into_inner();

    if caller.can(&ApiPermission::DeleteGroup(path.group_id)) {
        Ok(HttpResponseOk(
            ctx.delete_group(&path.group_id)
                .await
                .map_err(ApiError::Storage)?,
        ))
    } else {
        Err(unauthorized())
    }
}
