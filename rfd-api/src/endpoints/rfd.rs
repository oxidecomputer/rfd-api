// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use dropshot::{
    endpoint, HttpError, HttpResponseAccepted, HttpResponseOk, Path, Query, RequestContext,
    TypedBody,
};
use http::StatusCode;
use rfd_data::{
    content::{RfdAsciidoc, RfdContent, RfdDocument, RfdMarkdown},
    RfdState,
};
use rfd_model::{
    schema_ext::{ContentFormat, Visibility},
    Rfd,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use trace_request::trace_request;
use tracing::instrument;

use crate::{
    caller::CallerExt,
    context::{ApiContext, FullRfd, ListRfd},
    permissions::ApiPermission,
    search::{MeiliSearchResult, SearchRequest},
    util::response::{client_error, internal_error, unauthorized},
    ApiCaller,
};

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
    get_rfds_op(ctx, &ctx.get_caller(auth.as_ref()).await?).await
}

#[instrument(skip(ctx, caller), fields(caller = ?caller.id), err(Debug))]
async fn get_rfds_op(
    ctx: &ApiContext,
    caller: &ApiCaller,
) -> Result<HttpResponseOk<Vec<ListRfd>>, HttpError> {
    let rfds = ctx.list_rfds(caller, None).await?;
    Ok(HttpResponseOk(rfds))
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ReserveRfdBody {
    /// Title of the RFD
    pub title: String,
    /// Optional contents of the RFD
    pub content: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct ReserveRfdResponse {
    number: i32,
}

/// Create a new RFD
#[trace_request]
#[endpoint {
    method = POST,
    path = "/rfd",
}]
#[instrument(skip(rqctx), fields(request_id = rqctx.request_id), err(Debug))]
pub async fn reserve_rfd(
    rqctx: RequestContext<ApiContext>,
    body: TypedBody<ReserveRfdBody>,
) -> Result<HttpResponseAccepted<ReserveRfdResponse>, HttpError> {
    let ctx = rqctx.context();
    let auth = ctx.authn_token(&rqctx).await?;
    reserve_rfd_op(
        ctx,
        &ctx.get_caller(auth.as_ref()).await?,
        body.into_inner(),
    )
    .await
}

#[instrument(skip(ctx, caller), fields(caller = ?caller.id), err(Debug))]
async fn reserve_rfd_op(
    ctx: &ApiContext,
    caller: &ApiCaller,
    body: ReserveRfdBody,
) -> Result<HttpResponseAccepted<ReserveRfdResponse>, HttpError> {
    let number = ctx.create_rfd(caller, body.title, body.content).await?;
    Ok(HttpResponseAccepted(ReserveRfdResponse {
        number: number.into(),
    }))
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct RfdPathParams {
    /// The RFD number (examples: 1 or 123)
    number: String,
}

/// Get the latest representation of a RFD
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
    get_rfd_op(
        ctx,
        &ctx.get_caller(auth.as_ref()).await?,
        path.into_inner().number,
    )
    .await
}

#[instrument(skip(ctx, caller), fields(caller = ?caller.id), err(Debug))]
async fn get_rfd_op(
    ctx: &ApiContext,
    caller: &ApiCaller,
    number: String,
) -> Result<HttpResponseOk<FullRfd>, HttpError> {
    if let Ok(rfd_number) = number.parse::<i32>() {
        Ok(HttpResponseOk(ctx.get_rfd(caller, rfd_number, None).await?))
    } else {
        Err(client_error(
            StatusCode::BAD_REQUEST,
            "Malformed RFD number",
        ))
    }
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct RfdUpdateBody {
    /// Full Asciidoc document to store for this RFD
    document: String,
    /// Optional Git commit message to send with this update (recommended)
    message: Option<String>,
}

/// Replace the full document of a RFD
#[trace_request]
#[endpoint {
    method = POST,
    path = "/rfd/{number}",
}]
pub async fn set_rfd_document(
    rqctx: RequestContext<ApiContext>,
    path: Path<RfdPathParams>,
    body: TypedBody<RfdUpdateBody>,
) -> Result<HttpResponseAccepted<()>, HttpError> {
    let ctx = rqctx.context();
    let auth = ctx.authn_token(&rqctx).await?;
    set_rfd_document_op(
        ctx,
        &ctx.get_caller(auth.as_ref()).await?,
        path.into_inner().number,
        body.into_inner(),
    )
    .await
}

async fn set_rfd_document_op(
    ctx: &ApiContext,
    caller: &ApiCaller,
    number: String,
    body: RfdUpdateBody,
) -> Result<HttpResponseAccepted<()>, HttpError> {
    if let Ok(rfd_number) = number.parse::<i32>() {
        ctx.update_rfd_document(
            caller,
            rfd_number.into(),
            &body.document,
            body.message.as_deref(),
            None,
        )
        .await?;
        Ok(HttpResponseAccepted(()))
    } else {
        Err(client_error(
            StatusCode::BAD_REQUEST,
            "Malformed RFD number",
        ))
    }
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct RfdUpdateContentBody {
    /// Asciidoc content to store for this RFD
    content: String,
    /// Optional Git commit message to send with this update (recommended)
    message: Option<String>,
}

/// Replace the contents of a RFD
#[trace_request]
#[endpoint {
    method = POST,
    path = "/rfd/{number}/content",
}]
pub async fn set_rfd_content(
    rqctx: RequestContext<ApiContext>,
    path: Path<RfdPathParams>,
    body: TypedBody<RfdUpdateContentBody>,
) -> Result<HttpResponseAccepted<()>, HttpError> {
    let ctx = rqctx.context();
    let auth = ctx.authn_token(&rqctx).await?;
    set_rfd_content_op(
        ctx,
        &ctx.get_caller(auth.as_ref()).await?,
        path.into_inner().number,
        body.into_inner(),
    )
    .await
}

async fn set_rfd_content_op(
    ctx: &ApiContext,
    caller: &ApiCaller,
    number: String,
    body: RfdUpdateContentBody,
) -> Result<HttpResponseAccepted<()>, HttpError> {
    if let Ok(rfd_number) = number.parse::<i32>() {
        ctx.update_rfd_document(
            caller,
            rfd_number.into(),
            &body.content,
            body.message.as_deref(),
            None,
        )
        .await?;
        Ok(HttpResponseAccepted(()))
    } else {
        Err(client_error(
            StatusCode::BAD_REQUEST,
            "Malformed RFD number",
        ))
    }
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct RfdAttrPathParams {
    number: String,
    attr: RfdAttrName,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum RfdAttrName {
    Discussion,
    Labels,
    State,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum RfdAttr {
    Discussion(String),
    Labels(String),
    State(RfdState),
}

/// Get an attribute of a RFD
#[trace_request]
#[endpoint {
    method = GET,
    path = "/rfd/{number}/attr/{attr}",
}]
#[instrument(skip(rqctx), fields(request_id = rqctx.request_id), err(Debug))]
pub async fn get_rfd_attr(
    rqctx: RequestContext<ApiContext>,
    path: Path<RfdAttrPathParams>,
) -> Result<HttpResponseOk<RfdAttr>, HttpError> {
    let ctx = rqctx.context();
    let auth = ctx.authn_token(&rqctx).await?;
    let path = path.into_inner();
    get_rfd_attr_op(
        ctx,
        &ctx.get_caller(auth.as_ref()).await?,
        path.number,
        path.attr,
    )
    .await
}

#[instrument(skip(ctx, caller), fields(caller = ?caller.id), err(Debug))]
async fn get_rfd_attr_op(
    ctx: &ApiContext,
    caller: &ApiCaller,
    number: String,
    attr: RfdAttrName,
) -> Result<HttpResponseOk<RfdAttr>, HttpError> {
    if let Ok(rfd_number) = number.parse::<i32>() {
        let rfd = ctx.get_rfd(caller, rfd_number, None).await?;
        let content = match rfd.format {
            ContentFormat::Asciidoc => RfdContent::Asciidoc(RfdAsciidoc::new(rfd.content)),
            ContentFormat::Markdown => RfdContent::Markdown(RfdMarkdown::new(rfd.content)),
        };

        extract_attr(&attr, &content).map(HttpResponseOk)
    } else {
        Err(client_error(
            StatusCode::BAD_REQUEST,
            "Malformed RFD number",
        ))
    }
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct RfdAttrValue {
    /// Full value to set this attribute to in the existing RFD contents
    value: String,
    /// Optional Git commit message to send with this update (recommended)
    message: Option<String>,
}

/// Set an attribute of a RFD
#[trace_request]
#[endpoint {
    method = POST,
    path = "/rfd/{number}/attr/{attr}",
}]
#[instrument(skip(rqctx), fields(request_id = rqctx.request_id), err(Debug))]
pub async fn set_rfd_attr(
    rqctx: RequestContext<ApiContext>,
    path: Path<RfdAttrPathParams>,
    body: TypedBody<RfdAttrValue>,
) -> Result<HttpResponseAccepted<RfdAttr>, HttpError> {
    let ctx = rqctx.context();
    let auth = ctx.authn_token(&rqctx).await?;
    let path = path.into_inner();
    set_rfd_attr_op(
        ctx,
        &ctx.get_caller(auth.as_ref()).await?,
        path.number,
        path.attr,
        &body.into_inner(),
    )
    .await
}

#[instrument(skip(ctx, caller), fields(caller = ?caller.id), err(Debug))]
async fn set_rfd_attr_op(
    ctx: &ApiContext,
    caller: &ApiCaller,
    number: String,
    attr: RfdAttrName,
    body: &RfdAttrValue,
) -> Result<HttpResponseAccepted<RfdAttr>, HttpError> {
    if let Ok(rfd_number) = number.parse::<i32>() {
        // Get the latest revision
        let revision = ctx.get_rfd_revision(caller, rfd_number, None).await?;

        // TODO: Get rid of these clones
        let mut content = match revision.content_format {
            ContentFormat::Asciidoc => RfdContent::Asciidoc(RfdAsciidoc::new(revision.content)),
            ContentFormat::Markdown => RfdContent::Markdown(RfdMarkdown::new(revision.content)),
        };

        // Update the requested attribute
        match &attr {
            RfdAttrName::Discussion => content.update_discussion(&body.value),
            RfdAttrName::Labels => content.update_labels(&body.value),
            RfdAttrName::State => {
                let state: RfdState = body.value.as_str().try_into().map_err(|err| {
                    tracing::info!(?err, "Invalid state was supplied");
                    HttpError::for_bad_request(None, "Invalid RFD state".to_string())
                })?;
                content.update_state(&state.to_string());
            }
        };

        tracing::info!("Updated attribute in RFD document");

        // Persist the data back to GitHub. Note that we do not store this back to the database.
        // We rely on GitHub as the source of truth and revisions are required to tbe linked to
        // commits
        ctx.update_rfd_document(
            caller,
            rfd_number,
            content.raw(),
            body.message.as_deref(),
            None,
        )
        .await?;

        extract_attr(&attr, &content).map(HttpResponseAccepted)
    } else {
        Err(client_error(
            StatusCode::BAD_REQUEST,
            "Malformed RFD number",
        ))
    }
}

/// Open a RFD for discussion
#[trace_request]
#[endpoint {
    method = POST,
    path = "/rfd/{number}/discuss",
}]
#[instrument(skip(rqctx), fields(request_id = rqctx.request_id), err(Debug))]
pub async fn discuss_rfd(
    rqctx: RequestContext<ApiContext>,
    path: Path<RfdPathParams>,
) -> Result<HttpResponseAccepted<RfdAttr>, HttpError> {
    let ctx = rqctx.context();
    let auth = ctx.authn_token(&rqctx).await?;
    let path = path.into_inner();
    set_rfd_attr_op(
        ctx,
        &ctx.get_caller(auth.as_ref()).await?,
        path.number,
        RfdAttrName::State,
        &RfdAttrValue {
            value: RfdState::Discussion.to_string(),
            message: Some("Move to discussion".to_string()),
        },
    )
    .await
}

/// Publish a RFD
#[trace_request]
#[endpoint {
    method = POST,
    path = "/rfd/{number}/publish",
}]
#[instrument(skip(rqctx), fields(request_id = rqctx.request_id), err(Debug))]
pub async fn publish_rfd(
    rqctx: RequestContext<ApiContext>,
    path: Path<RfdPathParams>,
) -> Result<HttpResponseAccepted<RfdAttr>, HttpError> {
    let ctx = rqctx.context();
    let auth = ctx.authn_token(&rqctx).await?;
    let path = path.into_inner();
    set_rfd_attr_op(
        ctx,
        &ctx.get_caller(auth.as_ref()).await?,
        path.number,
        RfdAttrName::State,
        &RfdAttrValue {
            value: RfdState::Published.to_string(),
            message: Some("Publish".to_string()),
        },
    )
    .await
}

fn extract_attr(attr: &RfdAttrName, content: &RfdContent) -> Result<RfdAttr, HttpError> {
    match attr {
        RfdAttrName::Discussion => content
            .get_discussion()
            .ok_or_else(|| {
                HttpError::for_not_found(
                    None,
                    "RFD does not have the requested attribute".to_string(),
                )
            })
            .map(|value| RfdAttr::Discussion(value.to_string())),
        RfdAttrName::Labels => content
            .get_labels()
            .ok_or_else(|| {
                HttpError::for_not_found(
                    None,
                    "RFD does not have the requested attribute".to_string(),
                )
            })
            .map(|value| RfdAttr::Labels(value.to_string())),
        RfdAttrName::State => content
            .get_state()
            .ok_or_else(|| {
                HttpError::for_not_found(
                    None,
                    "RFD does not have the requested attribute".to_string(),
                )
            })
            .and_then(|value| match value.try_into() {
                Ok(rfd_state) => Ok(RfdAttr::State(rfd_state)),
                Err(err) => {
                    tracing::error!(?err, "RFD has an invalid state stored in its contents");
                    Err(HttpError::for_internal_error(
                        "Set attribute placed an invalid state".to_string(),
                    ))
                }
            }),
    }
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct RfdSearchQuery {
    pub q: String,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
    pub highlight_pre_tag: Option<String>,
    pub highlight_post_tag: Option<String>,
    pub attributes_to_crop: Option<String>,
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
) -> Result<HttpResponseOk<SearchResults>, HttpError> {
    let ctx = rqctx.context();
    let auth = ctx.authn_token(&rqctx).await?;
    let caller = ctx.get_caller(auth.as_ref()).await?;
    search_rfds_op(ctx, &caller, query.into_inner()).await
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct SearchResults {
    hits: Vec<SearchResultHit>,
    query: String,
    limit: Option<usize>,
    offset: Option<usize>,
}

// TODO: This should be a shared type across the api and processor, but it likely needs custom
// deserialization, serialization, and schema implementations
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct SearchResultHit {
    pub hierarchy: [Option<String>; 6],
    pub hierarchy_radio: [Option<String>; 6],
    pub content: String,
    pub object_id: String,
    pub rfd_number: u64,
    pub anchor: Option<String>,
    pub url: Option<String>,
    pub formatted: Option<FormattedSearchResultHit>,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct FormattedSearchResultHit {
    pub hierarchy: [Option<String>; 6],
    pub hierarchy_radio: [Option<String>; 6],
    pub content: Option<String>,
    pub object_id: String,
    pub rfd_number: u64,
    pub anchor: Option<String>,
    pub url: Option<String>,
}

#[instrument(skip(ctx, caller), fields(caller = ?caller.id), err(Debug))]
async fn search_rfds_op(
    ctx: &ApiContext,
    caller: &ApiCaller,
    query: RfdSearchQuery,
) -> Result<HttpResponseOk<SearchResults>, HttpError> {
    // TODO: Move all of this into a ctx

    // Ensure that the user has the search permission before searching
    if caller.can(&ApiPermission::SearchRfds) {
        tracing::debug!("Fetching from remote search API");

        // Transform the inbound query into a meilisearch request
        let mut search_request: SearchRequest = query.into();

        // Construct a meilisearch formatted filter. Either the caller has permission to search across
        // all RFDs or they access to some smaller set. If we need to filter down the RFD list we
        // construct a filter that will search across the RFDs the caller has direct access to as
        // well as any RFDs that are marked as publicly accessible.
        search_request.filter = if caller.can(&ApiPermission::GetRfdsAll) {
            None
        } else {
            let mut filter = "public = true".to_string();

            let allowed_rfds = caller
                .allow_rfds()
                .iter()
                .map(|num| num.to_string())
                .collect::<Vec<_>>()
                .join(", ");
            if allowed_rfds.len() > 0 {
                filter = filter + &format!("OR rfd_number in [{}]", allowed_rfds);
            }

            Some(filter)
        };

        // Pass the search request off to the meilisearch backend
        let results = ctx
            .search
            .client
            .search::<MeiliSearchResult>(&search_request)
            .await;

        tracing::debug!("Fetched results from remote search");

        match results {
            Ok(results) => {
                let results = SearchResults {
                    hits: results
                        .hits
                        .into_iter()
                        .map(|hit| hit.into())
                        .collect::<Vec<_>>(),
                    query: results.query,
                    limit: results.limit,
                    offset: results.offset,
                };

                tracing::debug!(count = ?results.hits.len(), "Transformed search results");

                Ok(HttpResponseOk(results))
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

#[derive(Debug, Deserialize, JsonSchema)]
pub struct RfdVisibility {
    ///
    pub visibility: Visibility,
}

/// Modify the visibility of a RFD
#[trace_request]
#[endpoint {
    method = POST,
    path = "/rfd/{number}/visibility",
}]
#[instrument(skip(rqctx), fields(request_id = rqctx.request_id), err(Debug))]
pub async fn update_rfd_visibility(
    rqctx: RequestContext<ApiContext>,
    path: Path<RfdPathParams>,
    body: TypedBody<RfdVisibility>,
) -> Result<HttpResponseOk<Rfd>, HttpError> {
    let ctx = rqctx.context();
    let auth = ctx.authn_token(&rqctx).await?;
    update_rfd_visibility_op(
        ctx,
        &ctx.get_caller(auth.as_ref()).await?,
        path.into_inner().number,
        body.into_inner(),
    )
    .await
}

#[instrument(skip(ctx, caller), fields(caller = ?caller.id), err(Debug))]
async fn update_rfd_visibility_op(
    ctx: &ApiContext,
    caller: &ApiCaller,
    number: String,
    body: RfdVisibility,
) -> Result<HttpResponseOk<Rfd>, HttpError> {
    if let Ok(rfd_number) = number.parse::<i32>() {
        Ok(HttpResponseOk(
            ctx.update_rfd_visibility(caller, rfd_number, body.visibility)
                .await?,
        ))
    } else {
        Err(client_error(
            StatusCode::BAD_REQUEST,
            "Malformed RFD number",
        ))
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use chrono::Utc;
    use dropshot::HttpResponseOk;
    use http::StatusCode;
    use rfd_model::{
        storage::{MockRfdPdfStore, MockRfdRevisionStore, MockRfdStore},
        Rfd, RfdRevision,
    };
    use uuid::Uuid;
    use w_api_permissions::Caller;

    use crate::{
        context::{
            test_mocks::{mock_context, MockStorage},
            ApiContext,
        },
        endpoints::rfd::get_rfd_op,
        permissions::ApiPermission,
    };

    use super::get_rfds_op;

    async fn ctx() -> ApiContext {
        let private_rfd_id_1 = Uuid::new_v4();
        let private_rfd_id_2 = Uuid::new_v4();
        let public_rfd_id = Uuid::new_v4();

        let mut rfd_store = MockRfdStore::new();
        rfd_store.expect_list().returning(move |filter, _| {
            let mut results = vec![
                Rfd {
                    id: private_rfd_id_1,
                    rfd_number: 123,
                    link: None,
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                    deleted_at: None,
                    visibility: rfd_model::schema_ext::Visibility::Private,
                },
                Rfd {
                    id: public_rfd_id,
                    rfd_number: 456,
                    link: None,
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                    deleted_at: None,
                    visibility: rfd_model::schema_ext::Visibility::Public,
                },
                Rfd {
                    id: private_rfd_id_2,
                    rfd_number: 789,
                    link: None,
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                    deleted_at: None,
                    visibility: rfd_model::schema_ext::Visibility::Private,
                },
            ];

            results.retain(|rfd| {
                filter.rfd_number.is_none()
                    || filter
                        .rfd_number
                        .as_ref()
                        .unwrap()
                        .contains(&rfd.rfd_number)
            });

            Ok(results)
        });

        let mut rfd_revision_store = MockRfdRevisionStore::new();
        rfd_revision_store
            .expect_list()
            .returning(move |filter, _| {
                let mut results = vec![
                    RfdRevision {
                        id: Uuid::new_v4(),
                        rfd_id: private_rfd_id_1,
                        title: "Private Test RFD 1".to_string(),
                        state: None,
                        discussion: None,
                        authors: None,
                        labels: None,
                        content: String::new(),
                        content_format: rfd_model::schema_ext::ContentFormat::Asciidoc,
                        sha: String::new(),
                        commit_sha: String::new(),
                        committed_at: Utc::now(),
                        created_at: Utc::now(),
                        updated_at: Utc::now(),
                        deleted_at: None,
                    },
                    RfdRevision {
                        id: Uuid::new_v4(),
                        rfd_id: public_rfd_id,
                        title: "Public Test RFD".to_string(),
                        state: None,
                        discussion: None,
                        authors: None,
                        labels: None,
                        content: String::new(),
                        content_format: rfd_model::schema_ext::ContentFormat::Asciidoc,
                        sha: String::new(),
                        commit_sha: String::new(),
                        committed_at: Utc::now(),
                        created_at: Utc::now(),
                        updated_at: Utc::now(),
                        deleted_at: None,
                    },
                    RfdRevision {
                        id: Uuid::new_v4(),
                        rfd_id: private_rfd_id_2,
                        title: "Private Test RFD 2".to_string(),
                        state: None,
                        discussion: None,
                        authors: None,
                        labels: None,
                        content: String::new(),
                        content_format: rfd_model::schema_ext::ContentFormat::Asciidoc,
                        sha: String::new(),
                        commit_sha: String::new(),
                        committed_at: Utc::now(),
                        created_at: Utc::now(),
                        updated_at: Utc::now(),
                        deleted_at: None,
                    },
                ];

                results.retain(|revision| {
                    filter.rfd.is_none() || filter.rfd.as_ref().unwrap().contains(&revision.rfd_id)
                });

                Ok(results)
            });

        let mut rfd_pdf_store = MockRfdPdfStore::new();
        rfd_pdf_store
            .expect_list()
            .returning(move |_, _| Ok(vec![]));

        let mut storage = MockStorage::new();
        storage.rfd_store = Some(Arc::new(rfd_store));
        storage.rfd_revision_store = Some(Arc::new(rfd_revision_store));
        storage.rfd_pdf_store = Some(Arc::new(rfd_pdf_store));

        mock_context(storage).await
    }

    // Test RFD access via the global All RFDs permission

    #[tokio::test]
    async fn list_rfds_via_all_permission() {
        let ctx = ctx().await;
        let caller = Caller {
            id: Uuid::new_v4(),
            permissions: vec![ApiPermission::GetRfdsAll].into(),
        };

        let HttpResponseOk(rfds) = get_rfds_op(&ctx, &caller).await.unwrap();
        assert_eq!(3, rfds.len());
        assert_eq!(789, rfds[0].rfd_number);
        assert_eq!(456, rfds[1].rfd_number);
        assert_eq!(123, rfds[2].rfd_number);
    }

    #[tokio::test]
    async fn get_rfd_via_all_permission() {
        let ctx = ctx().await;
        let caller = Caller {
            id: Uuid::new_v4(),
            permissions: vec![ApiPermission::GetRfdsAll].into(),
        };

        let HttpResponseOk(rfd) = get_rfd_op(&ctx, &caller, "0123".to_string()).await.unwrap();
        assert_eq!(123, rfd.rfd_number);

        let HttpResponseOk(rfd) = get_rfd_op(&ctx, &caller, "0456".to_string()).await.unwrap();
        assert_eq!(456, rfd.rfd_number);
    }

    // Test RFD access via the direct permission to a RFD

    #[tokio::test]
    async fn list_rfds_with_direct_permission() {
        let ctx = ctx().await;
        let caller = Caller {
            id: Uuid::new_v4(),
            permissions: vec![ApiPermission::GetRfd(123)].into(),
        };

        let HttpResponseOk(rfds) = get_rfds_op(&ctx, &caller).await.unwrap();
        assert_eq!(2, rfds.len());
        assert_eq!(456, rfds[0].rfd_number);
        assert_eq!(123, rfds[1].rfd_number);
    }

    #[tokio::test]
    async fn get_rfd_with_direct_permission() {
        let ctx = ctx().await;
        let caller = Caller {
            id: Uuid::new_v4(),
            permissions: vec![ApiPermission::GetRfd(123)].into(),
        };

        let HttpResponseOk(rfd) = get_rfd_op(&ctx, &caller, "0123".to_string()).await.unwrap();
        assert_eq!(123, rfd.rfd_number);

        let HttpResponseOk(rfd) = get_rfd_op(&ctx, &caller, "0456".to_string()).await.unwrap();
        assert_eq!(456, rfd.rfd_number);
    }

    // Test RFD access fails when a caller does not have permission

    #[tokio::test]
    async fn list_rfds_without_permission() {
        let ctx = ctx().await;
        let caller = Caller {
            id: Uuid::new_v4(),
            permissions: vec![].into(),
        };

        let HttpResponseOk(rfds) = get_rfds_op(&ctx, &caller).await.unwrap();
        assert_eq!(1, rfds.len());
        assert_eq!(456, rfds[0].rfd_number);
    }

    #[tokio::test]
    async fn get_rfd_without_permission() {
        let ctx = ctx().await;
        let caller = Caller {
            id: Uuid::new_v4(),
            permissions: vec![].into(),
        };

        let result = get_rfd_op(&ctx, &caller, "0123".to_string()).await;

        match result {
            Err(err) => assert_eq!(StatusCode::FORBIDDEN, err.status_code),
            Ok(response) => panic!(
                "Expected a 403 error, but instead found a RFD {:?}",
                response.0
            ),
        }

        let HttpResponseOk(rfd) = get_rfd_op(&ctx, &caller, "0456".to_string()).await.unwrap();
        assert_eq!(456, rfd.rfd_number);
    }

    // Test RFD access to public RFDs as the unauthenticated user

    #[tokio::test]
    async fn list_rfds_as_unauthenticated() {
        let ctx = ctx().await;

        let HttpResponseOk(rfds) = get_rfds_op(&ctx, &ctx.builtin_unauthenticated_caller())
            .await
            .unwrap();
        assert_eq!(1, rfds.len());
        assert_eq!(456, rfds[0].rfd_number);
    }

    #[tokio::test]
    async fn get_rfd_as_unauthenticated() {
        let ctx = ctx().await;
        let caller = ctx.builtin_unauthenticated_caller();

        let result = get_rfd_op(&ctx, caller, "0123".to_string()).await;
        match result {
            Err(err) => assert_eq!(StatusCode::FORBIDDEN, err.status_code),
            Ok(response) => panic!(
                "Expected a 403 error, but instead found a RFD {:?}",
                response.0
            ),
        }

        let HttpResponseOk(rfd) = get_rfd_op(&ctx, caller, "0456".to_string()).await.unwrap();
        assert_eq!(456, rfd.rfd_number);
    }
}
