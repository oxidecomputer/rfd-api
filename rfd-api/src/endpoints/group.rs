// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use dropshot::{
    endpoint, HttpError, HttpResponseCreated, HttpResponseOk, Path, RequestContext, TypedBody,
};
use rfd_model::{AccessGroup, NewAccessGroup};
use schemars::JsonSchema;
use serde::Deserialize;
use trace_request::trace_request;
use tracing::instrument;
use uuid::Uuid;
use w_api_permissions::Permissions;

use crate::{context::ApiContext, permissions::ApiPermissionResponse, ApiPermissions, Group};

pub type GroupResponse = AccessGroup<ApiPermissionResponse>;

fn into_group_response(group: Group) -> GroupResponse {
    AccessGroup {
        id: group.id,
        name: group.name,
        permissions: group
            .permissions
            .into_iter()
            .map(|p| p.into())
            .collect::<Permissions<ApiPermissionResponse>>(),
        created_at: group.created_at,
        updated_at: group.updated_at,
        deleted_at: group.deleted_at,
    }
}

#[trace_request]
#[endpoint {
    method = GET,
    path = "/group",    
}]
#[instrument(skip(rqctx), fields(request_id = rqctx.request_id), err(Debug))]
pub async fn get_groups(
    rqctx: RequestContext<ApiContext>,
) -> Result<HttpResponseOk<Vec<GroupResponse>>, HttpError> {
    let ctx = rqctx.context();
    let auth = ctx.authn_token(&rqctx).await?;
    let caller = ctx.get_caller(auth.as_ref()).await?;

    Ok(HttpResponseOk(
        ctx.get_groups(&caller)
            .await?
            .into_iter()
            .map(into_group_response)
            .collect(),
    ))
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
) -> Result<HttpResponseCreated<GroupResponse>, HttpError> {
    let ctx = rqctx.context();
    let auth = ctx.authn_token(&rqctx).await?;
    let caller = ctx.get_caller(auth.as_ref()).await?;
    let body = body.into_inner();

    Ok(HttpResponseCreated(
        ctx.create_group(
            &caller,
            NewAccessGroup {
                id: Uuid::new_v4(),
                name: body.name,
                permissions: body.permissions,
            },
        )
        .await
        .map(into_group_response)?,
    ))
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
) -> Result<HttpResponseOk<GroupResponse>, HttpError> {
    let ctx = rqctx.context();
    let auth = ctx.authn_token(&rqctx).await?;
    let caller = ctx.get_caller(auth.as_ref()).await?;
    let path = path.into_inner();
    let body = body.into_inner();

    Ok(HttpResponseOk(
        ctx.update_group(
            &caller,
            NewAccessGroup {
                id: path.group_id,
                name: body.name,
                permissions: body.permissions,
            },
        )
        .await
        .map(into_group_response)?,
    ))
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
) -> Result<HttpResponseOk<GroupResponse>, HttpError> {
    let ctx = rqctx.context();
    let auth = ctx.authn_token(&rqctx).await?;
    let caller = ctx.get_caller(auth.as_ref()).await?;
    let path = path.into_inner();

    Ok(HttpResponseOk(
        ctx.delete_group(&caller, &path.group_id)
            .await
            .map(into_group_response)?,
    ))
}
