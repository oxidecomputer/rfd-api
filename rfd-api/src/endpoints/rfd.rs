// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use dropshot::{
    endpoint, ClientErrorStatusCode, HttpError, HttpResponseAccepted, HttpResponseOk, Path, Query,
    RequestContext, TypedBody,
};
use newtype_uuid::TypedUuid;
use rfd_data::{
    content::{RfdAsciidoc, RfdContent, RfdDocument, RfdMarkdown},
    RfdState,
};
use rfd_model::{
    schema_ext::{ContentFormat, Visibility},
    Rfd, RfdRevisionId,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use trace_request::trace_request;
use tracing::instrument;
use v_api::{response::not_found, ApiContext};
use v_model::{permissions::Caller, storage::ListPagination};

use crate::{
    caller::CallerExt,
    context::{
        RfdContext, RfdRevisionIdentifier, RfdRevisionMeta, RfdRevisionMetadataChange, RfdWithPdf,
        RfdWithRaw, RfdWithoutContent,
    },
    endpoints::UNLIMITED,
    permissions::RfdPermission,
    search::{MeiliSearchResult, SearchRequest},
    util::response::{client_error, internal_error, unauthorized},
};

#[derive(Debug, Deserialize, JsonSchema)]
pub struct RfdPathParams {
    /// The RFD number (examples: 1 or 123)
    number: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct RfdRevisionPathParams {
    /// The RFD number (examples: 1 or 123)
    number: String,
    /// The revision id of the RFD
    revision: TypedUuid<RfdRevisionId>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct RfdAttrPathParams {
    /// The RFD number (examples: 1 or 123)
    number: String,
    /// An attribute that can be defined in an RFD document
    attr: RfdAttrName,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct RfdRevisionAttrPathParams {
    /// The RFD number (examples: 1 or 123)
    number: String,
    /// The revision id of the RFD
    revision: TypedUuid<RfdRevisionId>,
    /// An attribute that can be defined in an RFD document
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

// Read Endpoints

/// List all available RFDs
#[trace_request]
#[endpoint {
    method = GET,
    path = "/rfd",
}]
#[instrument(skip(rqctx), fields(request_id = rqctx.request_id), err(Debug))]
pub async fn list_rfds(
    rqctx: RequestContext<RfdContext>,
) -> Result<HttpResponseOk<Vec<RfdWithoutContent>>, HttpError> {
    let ctx = rqctx.context();
    let caller = ctx.v_ctx().get_caller(&rqctx).await?;
    list_rfds_op(ctx, &caller).await
}

// Latest RFD revision endpoints

/// Get the latest representation of an RFD's metadata
#[trace_request]
#[endpoint {
    method = GET,
    path = "/rfd/{number}",
}]
#[instrument(skip(rqctx), fields(request_id = rqctx.request_id), err(Debug))]
pub async fn view_rfd_meta(
    rqctx: RequestContext<RfdContext>,
    path: Path<RfdPathParams>,
) -> Result<HttpResponseOk<RfdWithoutContent>, HttpError> {
    let ctx = rqctx.context();
    let caller = ctx.v_ctx().get_caller(&rqctx).await?;
    let path = path.into_inner();
    view_rfd_meta_op(ctx, &caller, path.number, None).await
}

/// Get the raw contents of the latest revision of a RFD
#[trace_request]
#[endpoint {
    method = GET,
    path = "/rfd/{number}/raw",
}]
#[instrument(skip(rqctx), fields(request_id = rqctx.request_id), err(Debug))]
pub async fn view_rfd(
    rqctx: RequestContext<RfdContext>,
    path: Path<RfdPathParams>,
) -> Result<HttpResponseOk<RfdWithRaw>, HttpError> {
    let ctx = rqctx.context();
    let caller = ctx.v_ctx().get_caller(&rqctx).await?;
    let path = path.into_inner();
    view_rfd_op(ctx, &caller, path.number, None).await
}

/// Get the PDF locations of the latest revision of a RFD
#[trace_request]
#[endpoint {
    method = GET,
    path = "/rfd/{number}/pdf",
}]
#[instrument(skip(rqctx), fields(request_id = rqctx.request_id), err(Debug))]
pub async fn view_rfd_pdf(
    rqctx: RequestContext<RfdContext>,
    path: Path<RfdPathParams>,
) -> Result<HttpResponseOk<RfdWithPdf>, HttpError> {
    let ctx = rqctx.context();
    let caller = ctx.v_ctx().get_caller(&rqctx).await?;
    let path = path.into_inner();
    view_rfd_pdf_op(ctx, &caller, path.number, None).await
}

/// Get the an attribute of the latest revision of a RFD
#[trace_request]
#[endpoint {
    method = GET,
    path = "/rfd/{number}/attr/{attr}",
}]
#[instrument(skip(rqctx), fields(request_id = rqctx.request_id), err(Debug))]
pub async fn view_rfd_attr(
    rqctx: RequestContext<RfdContext>,
    path: Path<RfdAttrPathParams>,
) -> Result<HttpResponseOk<RfdAttr>, HttpError> {
    let ctx = rqctx.context();
    let caller = ctx.v_ctx().get_caller(&rqctx).await?;
    let path = path.into_inner();
    view_rfd_attr_op(ctx, &caller, path.number, None, path.attr).await
}

/// Get the comments related to the latest revision of a RFD
#[trace_request]
#[endpoint {
    method = GET,
    path = "/rfd/{number}/discussion",
}]
#[instrument(skip(rqctx), fields(request_id = rqctx.request_id), err(Debug))]
pub async fn view_rfd_discussion(
    rqctx: RequestContext<RfdContext>,
    path: Path<RfdPathParams>,
) -> Result<HttpResponseOk<()>, HttpError> {
    unimplemented!()
}

// Specific RFD revision endpoints

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ListRevisionQuery {
    limit: Option<i64>,
    offset: Option<i64>,
}

/// List all revisions of an RFD
#[trace_request]
#[endpoint {
    method = GET,
    path = "/rfd/{number}/revision"
}]
#[instrument(skip(rqctx), fields(request_id = rqctx.request_id), err(Debug))]
pub async fn list_rfd_revisions(
    rqctx: RequestContext<RfdContext>,
    path: Path<RfdPathParams>,
    query: Query<ListRevisionQuery>,
) -> Result<HttpResponseOk<Vec<RfdRevisionMeta>>, HttpError> {
    let ctx = rqctx.context();
    let caller = ctx.v_ctx().get_caller(&rqctx).await?;
    let number = path.into_inner().number;
    list_rfd_revisions_op(ctx, &caller, number, query.into_inner()).await
}

/// Get an RFD revision's metadata
#[trace_request]
#[endpoint {
    method = GET,
    path = "/rfd/{number}/revision/{revision}",
}]
#[instrument(skip(rqctx), fields(request_id = rqctx.request_id), err(Debug))]
pub async fn view_rfd_revision_meta(
    rqctx: RequestContext<RfdContext>,
    path: Path<RfdRevisionPathParams>,
) -> Result<HttpResponseOk<RfdWithoutContent>, HttpError> {
    let ctx = rqctx.context();
    let caller = ctx.v_ctx().get_caller(&rqctx).await?;
    let path = path.into_inner();
    view_rfd_meta_op(ctx, &caller, path.number, Some(path.revision.into())).await
}

/// Get the raw contents of a revision of a RFD
#[trace_request]
#[endpoint {
    method = GET,
    path = "/rfd/{number}/revision/{revision}/raw",
}]
#[instrument(skip(rqctx), fields(request_id = rqctx.request_id), err(Debug))]
pub async fn view_rfd_revision(
    rqctx: RequestContext<RfdContext>,
    path: Path<RfdRevisionPathParams>,
) -> Result<HttpResponseOk<RfdWithRaw>, HttpError> {
    let ctx = rqctx.context();
    let caller = ctx.v_ctx().get_caller(&rqctx).await?;
    let path = path.into_inner();
    view_rfd_op(ctx, &caller, path.number, Some(path.revision.into())).await
}

/// Get the PDF locations of a revision of a RFD
#[trace_request]
#[endpoint {
    method = GET,
    path = "/rfd/{number}/revision/{revision}/pdf",
}]
#[instrument(skip(rqctx), fields(request_id = rqctx.request_id), err(Debug))]
pub async fn view_rfd_revision_pdf(
    rqctx: RequestContext<RfdContext>,
    path: Path<RfdRevisionPathParams>,
) -> Result<HttpResponseOk<RfdWithPdf>, HttpError> {
    let ctx = rqctx.context();
    let caller = ctx.v_ctx().get_caller(&rqctx).await?;
    let path = path.into_inner();
    view_rfd_pdf_op(ctx, &caller, path.number, Some(path.revision.into())).await
}

/// Get the an attribute of a revision of a RFD
#[trace_request]
#[endpoint {
    method = GET,
    path = "/rfd/{number}/revision/{revision}/attr/{attr}",
}]
#[instrument(skip(rqctx), fields(request_id = rqctx.request_id), err(Debug))]
pub async fn view_rfd_revision_attr(
    rqctx: RequestContext<RfdContext>,
    path: Path<RfdRevisionAttrPathParams>,
) -> Result<HttpResponseOk<RfdAttr>, HttpError> {
    let ctx = rqctx.context();
    let caller = ctx.v_ctx().get_caller(&rqctx).await?;
    let path = path.into_inner();
    view_rfd_attr_op(
        ctx,
        &caller,
        path.number,
        Some(path.revision.into()),
        path.attr,
    )
    .await
}

/// Get the comments related to a revision of a RFD
#[trace_request]
#[endpoint {
    method = GET,
    path = "/rfd/{number}/revision/{revision}/discussion",
}]
#[instrument(skip(rqctx), fields(request_id = rqctx.request_id), err(Debug))]
pub async fn view_rfd_revision_discussion(
    rqctx: RequestContext<RfdContext>,
    path: Path<RfdRevisionPathParams>,
) -> Result<HttpResponseOk<()>, HttpError> {
    unimplemented!()
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

/// Search the RFD index and get a list of results
#[trace_request]
#[endpoint {
    method = GET,
    path = "/rfd-search",
}]
#[instrument(skip(rqctx), fields(request_id = rqctx.request_id), err(Debug))]
pub async fn search_rfds(
    rqctx: RequestContext<RfdContext>,
    query: Query<RfdSearchQuery>,
) -> Result<HttpResponseOk<SearchResults>, HttpError> {
    let ctx = rqctx.context();
    let caller = ctx.v_ctx().get_caller(&rqctx).await?;
    search_rfds_op(ctx, &caller, query.into_inner()).await
}

// Read operation

#[instrument(skip(ctx, caller), fields(caller = ?caller.id), err(Debug))]
async fn list_rfds_op(
    ctx: &RfdContext,
    caller: &Caller<RfdPermission>,
) -> Result<HttpResponseOk<Vec<RfdWithoutContent>>, HttpError> {
    let rfds = ctx.list_rfds(caller, None).await?;
    Ok(HttpResponseOk(rfds))
}

#[instrument(skip(ctx, caller), fields(caller = ?caller.id), err(Debug))]
async fn list_rfd_revisions_op(
    ctx: &RfdContext,
    caller: &Caller<RfdPermission>,
    number: String,
    query: ListRevisionQuery,
) -> Result<HttpResponseOk<Vec<RfdRevisionMeta>>, HttpError> {
    if let Ok(rfd_number) = number.parse::<i32>() {
        Ok(HttpResponseOk(
            ctx.list_revisions(
                caller,
                rfd_number,
                &ListPagination::default()
                    .limit(query.limit.unwrap_or(UNLIMITED))
                    .offset(query.offset.unwrap_or(0)),
            )
            .await?,
        ))
    } else {
        Err(client_error(
            ClientErrorStatusCode::BAD_REQUEST,
            "Malformed RFD number",
        ))
    }
}

#[instrument(skip(ctx, caller), fields(caller = ?caller.id), err(Debug))]
async fn view_rfd_meta_op(
    ctx: &RfdContext,
    caller: &Caller<RfdPermission>,
    number: String,
    revision: Option<RfdRevisionIdentifier>,
) -> Result<HttpResponseOk<RfdWithoutContent>, HttpError> {
    if let Ok(rfd_number) = number.parse::<i32>() {
        Ok(HttpResponseOk(
            ctx.view_rfd_meta(caller, rfd_number, revision).await?,
        ))
    } else {
        Err(client_error(
            ClientErrorStatusCode::BAD_REQUEST,
            "Malformed RFD number",
        ))
    }
}

#[instrument(skip(ctx, caller), fields(caller = ?caller.id), err(Debug))]
async fn view_rfd_op(
    ctx: &RfdContext,
    caller: &Caller<RfdPermission>,
    number: String,
    revision: Option<RfdRevisionIdentifier>,
) -> Result<HttpResponseOk<RfdWithRaw>, HttpError> {
    if let Ok(rfd_number) = number.parse::<i32>() {
        Ok(HttpResponseOk(
            ctx.view_rfd(caller, rfd_number, revision).await?,
        ))
    } else {
        Err(client_error(
            ClientErrorStatusCode::BAD_REQUEST,
            "Malformed RFD number",
        ))
    }
}

#[instrument(skip(ctx, caller), fields(caller = ?caller.id), err(Debug))]
async fn view_rfd_pdf_op(
    ctx: &RfdContext,
    caller: &Caller<RfdPermission>,
    number: String,
    revision: Option<RfdRevisionIdentifier>,
) -> Result<HttpResponseOk<RfdWithPdf>, HttpError> {
    if let Ok(rfd_number) = number.parse::<i32>() {
        Ok(HttpResponseOk(
            ctx.view_rfd_pdfs(caller, rfd_number, revision).await?,
        ))
    } else {
        Err(client_error(
            ClientErrorStatusCode::BAD_REQUEST,
            "Malformed RFD number",
        ))
    }
}

#[instrument(skip(ctx, caller), fields(caller = ?caller.id), err(Debug))]
async fn view_rfd_attr_op(
    ctx: &RfdContext,
    caller: &Caller<RfdPermission>,
    number: String,
    revision: Option<RfdRevisionIdentifier>,
    attr: RfdAttrName,
) -> Result<HttpResponseOk<RfdAttr>, HttpError> {
    if let Ok(rfd_number) = number.parse::<i32>() {
        let rfd = ctx.view_rfd(caller, rfd_number, None).await?;
        let content = match (rfd.content, rfd.format) {
            (Some(content), Some(ContentFormat::Asciidoc)) => {
                RfdContent::Asciidoc(RfdAsciidoc::new(content).map_err(|err| {
                    tracing::warn!(?err, "Failed to parse RFD content");
                    HttpError::for_internal_error(format!(
                        "Failed to parse RFD content for RFD {}",
                        number
                    ))
                })?)
            }
            (Some(content), Some(ContentFormat::Markdown)) => {
                RfdContent::Markdown(RfdMarkdown::new(content))
            }
            _ => Err(not_found("RFD does not have any assigned revisions"))?,
        };

        extract_attr(&attr, &content).map(HttpResponseOk)
    } else {
        Err(client_error(
            ClientErrorStatusCode::BAD_REQUEST,
            "Malformed RFD number",
        ))
    }
}

#[allow(dead_code)]
#[instrument(skip(_ctx, caller), fields(caller = ?caller.id), err(Debug))]
async fn view_rfd_discussion_op(
    _ctx: &RfdContext,
    caller: &Caller<RfdPermission>,
    number: String,
    revision: Option<RfdRevisionIdentifier>,
) -> Result<HttpResponseOk<RfdWithRaw>, HttpError> {
    unimplemented!()
}

#[instrument(skip(ctx, caller), fields(caller = ?caller.id), err(Debug))]
async fn search_rfds_op(
    ctx: &RfdContext,
    caller: &Caller<RfdPermission>,
    query: RfdSearchQuery,
) -> Result<HttpResponseOk<SearchResults>, HttpError> {
    // TODO: Move all of this into a ctx

    // Ensure that the user has the search permission before searching
    if caller.can(&RfdPermission::SearchRfds) {
        tracing::debug!("Fetching from remote search API");

        // Transform the inbound query into a meilisearch request
        let mut search_request: SearchRequest = query.into();

        // Construct a meilisearch formatted filter. Either the caller has permission to search across
        // all RFDs or they access to some smaller set. If we need to filter down the RFD list we
        // construct a filter that will search across the RFDs the caller has direct access to as
        // well as any RFDs that are marked as publicly accessible.
        search_request.filter = if caller.can(&RfdPermission::GetRfdsAll) {
            None
        } else {
            let mut filter = "public = true".to_string();

            let allowed_rfds = caller
                .allow_rfds()
                .iter()
                .map(|num| num.to_string())
                .collect::<Vec<_>>()
                .join(", ");
            if !allowed_rfds.is_empty() {
                filter = filter + &format!("OR rfd_number in [{}]", allowed_rfds);
            }

            Some(filter)
        };

        tracing::debug!(?search_request, "Submitting search request to backend");

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

// Write Endpoints

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
    rqctx: RequestContext<RfdContext>,
    body: TypedBody<ReserveRfdBody>,
) -> Result<HttpResponseAccepted<ReserveRfdResponse>, HttpError> {
    let ctx = rqctx.context();
    let caller = ctx.v_ctx().get_caller(&rqctx).await?;
    reserve_rfd_op(ctx, &caller, body.into_inner()).await
}

#[instrument(skip(ctx, caller), fields(caller = ?caller.id), err(Debug))]
async fn reserve_rfd_op(
    ctx: &RfdContext,
    caller: &Caller<RfdPermission>,
    body: ReserveRfdBody,
) -> Result<HttpResponseAccepted<ReserveRfdResponse>, HttpError> {
    let number = ctx.create_rfd(caller, body.title, body.content).await?;
    Ok(HttpResponseAccepted(ReserveRfdResponse {
        number: number.into(),
    }))
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
    path = "/rfd/{number}/raw",
}]
pub async fn set_rfd_document(
    rqctx: RequestContext<RfdContext>,
    path: Path<RfdPathParams>,
    body: TypedBody<RfdUpdateBody>,
) -> Result<HttpResponseAccepted<()>, HttpError> {
    let ctx = rqctx.context();
    let caller = ctx.v_ctx().get_caller(&rqctx).await?;
    set_rfd_document_op(ctx, &caller, path.into_inner().number, body.into_inner()).await
}

async fn set_rfd_document_op(
    ctx: &RfdContext,
    caller: &Caller<RfdPermission>,
    number: String,
    body: RfdUpdateBody,
) -> Result<HttpResponseAccepted<()>, HttpError> {
    if let Ok(rfd_number) = number.parse::<i32>() {
        ctx.update_rfd_document(
            caller,
            rfd_number,
            &body.document,
            body.message.as_deref(),
            None,
        )
        .await?;
        Ok(HttpResponseAccepted(()))
    } else {
        Err(client_error(
            ClientErrorStatusCode::BAD_REQUEST,
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
    rqctx: RequestContext<RfdContext>,
    path: Path<RfdPathParams>,
    body: TypedBody<RfdUpdateContentBody>,
) -> Result<HttpResponseAccepted<()>, HttpError> {
    let ctx = rqctx.context();
    let caller = ctx.v_ctx().get_caller(&rqctx).await?;
    set_rfd_content_op(ctx, &caller, path.into_inner().number, body.into_inner()).await
}

async fn set_rfd_content_op(
    ctx: &RfdContext,
    caller: &Caller<RfdPermission>,
    number: String,
    body: RfdUpdateContentBody,
) -> Result<HttpResponseAccepted<()>, HttpError> {
    if let Ok(rfd_number) = number.parse::<i32>() {
        ctx.update_rfd_document(
            caller,
            rfd_number,
            &body.content,
            body.message.as_deref(),
            None,
        )
        .await?;
        Ok(HttpResponseAccepted(()))
    } else {
        Err(client_error(
            ClientErrorStatusCode::BAD_REQUEST,
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
    rqctx: RequestContext<RfdContext>,
    path: Path<RfdAttrPathParams>,
    body: TypedBody<RfdAttrValue>,
) -> Result<HttpResponseAccepted<RfdAttr>, HttpError> {
    let ctx = rqctx.context();
    let caller = ctx.v_ctx().get_caller(&rqctx).await?;
    let path = path.into_inner();
    set_rfd_attr_op(ctx, &caller, path.number, path.attr, &body.into_inner()).await
}

#[instrument(skip(ctx, caller), fields(caller = ?caller.id), err(Debug))]
async fn set_rfd_attr_op(
    ctx: &RfdContext,
    caller: &Caller<RfdPermission>,
    number: String,
    attr: RfdAttrName,
    body: &RfdAttrValue,
) -> Result<HttpResponseAccepted<RfdAttr>, HttpError> {
    if let Ok(rfd_number) = number.parse::<i32>() {
        // Get the latest revision
        let revision = ctx.view_rfd_revision(caller, rfd_number, None).await?;

        // TODO: Get rid of these clones
        let mut content = match revision.content_format {
            ContentFormat::Asciidoc => {
                RfdContent::Asciidoc(RfdAsciidoc::new(revision.content).map_err(|err| {
                    tracing::warn!(?err, "Failed to parse RFD content");
                    HttpError::for_internal_error(format!(
                        "Failed to parse RFD content for RFD {}",
                        number
                    ))
                })?)
            }
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
                content.update_state(&state.to_string())
            }
        }
        .map_err(|err| {
            tracing::info!(?err, "Update resulted in malfored RFD");
            HttpError::for_bad_request(
                None,
                "Update would result in a malformed RFD. Update has not been applied".to_string(),
            )
        })?;

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
            ClientErrorStatusCode::BAD_REQUEST,
            "Malformed RFD number",
        ))
    }
}

/// Open a RFD for discussion
#[trace_request]
#[endpoint {
    method = POST,
    path = "/rfd/{number}/state/discuss",
}]
#[instrument(skip(rqctx), fields(request_id = rqctx.request_id), err(Debug))]
pub async fn discuss_rfd(
    rqctx: RequestContext<RfdContext>,
    path: Path<RfdPathParams>,
) -> Result<HttpResponseAccepted<RfdAttr>, HttpError> {
    let ctx = rqctx.context();
    let caller = ctx.v_ctx().get_caller(&rqctx).await?;
    let path = path.into_inner();
    set_rfd_attr_op(
        ctx,
        &caller,
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
    path = "/rfd/{number}/state/publish",
}]
#[instrument(skip(rqctx), fields(request_id = rqctx.request_id), err(Debug))]
pub async fn publish_rfd(
    rqctx: RequestContext<RfdContext>,
    path: Path<RfdPathParams>,
) -> Result<HttpResponseAccepted<RfdAttr>, HttpError> {
    let ctx = rqctx.context();
    let caller = ctx.v_ctx().get_caller(&rqctx).await?;
    let path = path.into_inner();
    set_rfd_attr_op(
        ctx,
        &caller,
        path.number,
        RfdAttrName::State,
        &RfdAttrValue {
            value: RfdState::Published.to_string(),
            message: Some("Publish".to_string()),
        },
    )
    .await
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct UpdateRfdAttrBody {
    major_change: Option<bool>,
}

/// Update the metadata of an RFD's revision
#[trace_request]
#[endpoint {
    method = PATCH,
    path = "/rfd/{number}/revision/{revision}",
}]
#[instrument(skip(rqctx), fields(request_id = rqctx.request_id), err(Debug))]
pub async fn update_rfd_revision(
    rqctx: RequestContext<RfdContext>,
    path: Path<RfdRevisionPathParams>,
    body: TypedBody<UpdateRfdAttrBody>,
) -> Result<HttpResponseOk<RfdRevisionMeta>, HttpError> {
    let ctx = rqctx.context();
    let caller = ctx.v_ctx().get_caller(&rqctx).await?;
    let path = path.into_inner();
    update_rfd_revision_op(ctx, &caller, path.number, path.revision, body.into_inner()).await
}

async fn update_rfd_revision_op(
    ctx: &RfdContext,
    caller: &Caller<RfdPermission>,
    rfd_number: String,
    revision: TypedUuid<RfdRevisionId>,
    raw_changes: UpdateRfdAttrBody,
) -> Result<HttpResponseOk<RfdRevisionMeta>, HttpError> {
    let Ok(rfd_number) = rfd_number.parse::<i32>() else {
        return Err(client_error(
            ClientErrorStatusCode::BAD_REQUEST,
            "Malformed RFD number",
        ));
    };

    // Destructuring to ensure we get an error if a new field is added without being handled.
    // Make sure to add an `if let` statement later in the fuction when adding a new field.
    let UpdateRfdAttrBody { major_change } = raw_changes;

    let mut changes = Vec::new();
    if let Some(major_change) = major_change {
        changes.push(RfdRevisionMetadataChange::MajorChange(major_change));
    }

    Ok(HttpResponseOk(
        ctx.update_rfd_revision_metadata(caller, rfd_number, revision, &changes)
            .await?
            .into(),
    ))
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
pub struct RfdVisibility {
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
    rqctx: RequestContext<RfdContext>,
    path: Path<RfdPathParams>,
    body: TypedBody<RfdVisibility>,
) -> Result<HttpResponseOk<Rfd>, HttpError> {
    let ctx = rqctx.context();
    let caller = ctx.v_ctx().get_caller(&rqctx).await?;
    update_rfd_visibility_op(ctx, &caller, path.into_inner().number, body.into_inner()).await
}

#[instrument(skip(ctx, caller), fields(caller = ?caller.id), err(Debug))]
async fn update_rfd_visibility_op(
    ctx: &RfdContext,
    caller: &Caller<RfdPermission>,
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
            ClientErrorStatusCode::BAD_REQUEST,
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
    use newtype_uuid::{GenericUuid, TypedUuid};
    use rfd_model::{
        schema_ext::ContentFormat,
        storage::{
            mock::MockStorage, MockRfdMetaStore, MockRfdPdfStore, MockRfdRevisionMetaStore,
            MockRfdRevisionStore, MockRfdStore,
        },
        CommitSha, FileSha, Rfd, RfdMeta, RfdRevision, RfdRevisionMeta,
    };
    use uuid::Uuid;
    use v_api::ApiContext;
    use v_model::{permissions::Caller, Permissions};

    use crate::{
        context::{test_mocks::mock_context, RfdContext},
        endpoints::rfd::view_rfd_op,
        permissions::RfdPermission,
    };

    use super::list_rfds_op;

    async fn ctx() -> RfdContext {
        let private_rfd_id_1 = Uuid::new_v4();
        let private_rfd_id_2 = Uuid::new_v4();
        let public_rfd_id = Uuid::new_v4();

        let mut rfd_store = MockRfdStore::new();
        rfd_store.expect_list().returning(move |filter, _| {
            let mut results = vec![
                Rfd {
                    id: TypedUuid::from_untyped_uuid(private_rfd_id_1),
                    rfd_number: 123,
                    link: None,
                    content: Some(RfdRevision {
                        id: TypedUuid::new_v4(),
                        rfd_id: TypedUuid::from_untyped_uuid(private_rfd_id_1),
                        title: String::new(),
                        state: None,
                        discussion: None,
                        authors: None,
                        labels: None,
                        content: String::new(),
                        content_format: ContentFormat::Asciidoc,
                        sha: FileSha(String::new()),
                        commit: CommitSha(String::new()),
                        committed_at: Utc::now(),
                        major_change: false,
                        created_at: Utc::now(),
                        updated_at: Utc::now(),
                        deleted_at: None,
                    }),
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                    deleted_at: None,
                    visibility: rfd_model::schema_ext::Visibility::Private,
                    latest_major_change_at: Some(Utc::now()),
                },
                Rfd {
                    id: TypedUuid::from_untyped_uuid(public_rfd_id),
                    rfd_number: 456,
                    link: None,
                    content: Some(RfdRevision {
                        id: TypedUuid::new_v4(),
                        rfd_id: TypedUuid::from_untyped_uuid(private_rfd_id_1),
                        title: String::new(),
                        state: None,
                        discussion: None,
                        authors: None,
                        labels: None,
                        content: String::new(),
                        content_format: ContentFormat::Asciidoc,
                        sha: FileSha(String::new()),
                        commit: CommitSha(String::new()),
                        committed_at: Utc::now(),
                        major_change: false,
                        created_at: Utc::now(),
                        updated_at: Utc::now(),
                        deleted_at: None,
                    }),
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                    deleted_at: None,
                    visibility: rfd_model::schema_ext::Visibility::Public,
                    latest_major_change_at: Some(Utc::now()),
                },
                Rfd {
                    id: TypedUuid::from_untyped_uuid(private_rfd_id_2),
                    rfd_number: 789,
                    link: None,
                    content: Some(RfdRevision {
                        id: TypedUuid::new_v4(),
                        rfd_id: TypedUuid::from_untyped_uuid(private_rfd_id_1),
                        title: String::new(),
                        state: None,
                        discussion: None,
                        authors: None,
                        labels: None,
                        content: String::new(),
                        content_format: ContentFormat::Asciidoc,
                        sha: FileSha(String::new()),
                        commit: CommitSha(String::new()),
                        committed_at: Utc::now(),
                        major_change: false,
                        created_at: Utc::now(),
                        updated_at: Utc::now(),
                        deleted_at: None,
                    }),
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                    deleted_at: None,
                    visibility: rfd_model::schema_ext::Visibility::Private,
                    latest_major_change_at: Some(Utc::now()),
                },
            ];

            results.retain(|rfd| {
                filter.len() == 0
                    || filter[0].rfd_number.is_none()
                    || filter[0]
                        .rfd_number
                        .as_ref()
                        .unwrap()
                        .contains(&rfd.rfd_number)
            });

            Ok(results)
        });

        let mut rfd_meta_store = MockRfdMetaStore::new();
        rfd_meta_store.expect_list().returning(move |filter, _| {
            let mut results = vec![
                RfdMeta {
                    id: TypedUuid::from_untyped_uuid(private_rfd_id_1),
                    rfd_number: 123,
                    link: None,
                    content: Some(RfdRevisionMeta {
                        id: TypedUuid::new_v4(),
                        rfd_id: TypedUuid::from_untyped_uuid(private_rfd_id_1),
                        title: String::new(),
                        state: None,
                        discussion: None,
                        authors: None,
                        labels: None,
                        content_format: ContentFormat::Asciidoc,
                        sha: FileSha(String::new()),
                        commit: CommitSha(String::new()),
                        committed_at: Utc::now(),
                        major_change: false,
                        created_at: Utc::now(),
                        updated_at: Utc::now(),
                        deleted_at: None,
                    }),
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                    deleted_at: None,
                    visibility: rfd_model::schema_ext::Visibility::Private,
                    latest_major_change_at: Some(Utc::now()),
                },
                RfdMeta {
                    id: TypedUuid::from_untyped_uuid(public_rfd_id),
                    rfd_number: 456,
                    link: None,
                    content: Some(RfdRevisionMeta {
                        id: TypedUuid::new_v4(),
                        rfd_id: TypedUuid::from_untyped_uuid(private_rfd_id_1),
                        title: String::new(),
                        state: None,
                        discussion: None,
                        authors: None,
                        labels: None,
                        content_format: ContentFormat::Asciidoc,
                        sha: FileSha(String::new()),
                        commit: CommitSha(String::new()),
                        committed_at: Utc::now(),
                        major_change: false,
                        created_at: Utc::now(),
                        updated_at: Utc::now(),
                        deleted_at: None,
                    }),
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                    deleted_at: None,
                    visibility: rfd_model::schema_ext::Visibility::Public,
                    latest_major_change_at: Some(Utc::now()),
                },
                RfdMeta {
                    id: TypedUuid::from_untyped_uuid(private_rfd_id_2),
                    rfd_number: 789,
                    link: None,
                    content: Some(RfdRevisionMeta {
                        id: TypedUuid::new_v4(),
                        rfd_id: TypedUuid::from_untyped_uuid(private_rfd_id_1),
                        title: String::new(),
                        state: None,
                        discussion: None,
                        authors: None,
                        labels: None,
                        content_format: ContentFormat::Asciidoc,
                        sha: FileSha(String::new()),
                        commit: CommitSha(String::new()),
                        committed_at: Utc::now(),
                        major_change: false,
                        created_at: Utc::now(),
                        updated_at: Utc::now(),
                        deleted_at: None,
                    }),
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                    deleted_at: None,
                    visibility: rfd_model::schema_ext::Visibility::Private,
                    latest_major_change_at: Some(Utc::now()),
                },
            ];

            results.retain(|rfd| {
                filter.len() == 0
                    || filter[0].rfd_number.is_none()
                    || filter[0]
                        .rfd_number
                        .as_ref()
                        .unwrap()
                        .contains(&rfd.rfd_number)
            });

            Ok(results)
        });

        let private_rfd_revision_id_1 = TypedUuid::new_v4();
        let public_rfd_revision_id = TypedUuid::new_v4();
        let private_rfd_revision_id_2 = TypedUuid::new_v4();
        let mut rfd_revision_store = MockRfdRevisionStore::new();
        rfd_revision_store
            .expect_list()
            .returning(move |filter, _| {
                let mut results = vec![
                    RfdRevision {
                        id: private_rfd_revision_id_1,
                        rfd_id: TypedUuid::from_untyped_uuid(private_rfd_id_1),
                        title: "Private Test RFD 1".to_string(),
                        state: None,
                        discussion: None,
                        authors: None,
                        labels: None,
                        content: String::new(),
                        content_format: rfd_model::schema_ext::ContentFormat::Asciidoc,
                        sha: String::new().into(),
                        commit: String::new().into(),
                        committed_at: Utc::now(),
                        major_change: false,
                        created_at: Utc::now(),
                        updated_at: Utc::now(),
                        deleted_at: None,
                    },
                    RfdRevision {
                        id: public_rfd_revision_id,
                        rfd_id: TypedUuid::from_untyped_uuid(public_rfd_id),
                        title: "Public Test RFD".to_string(),
                        state: None,
                        discussion: None,
                        authors: None,
                        labels: None,
                        content: String::new(),
                        content_format: rfd_model::schema_ext::ContentFormat::Asciidoc,
                        sha: String::new().into(),
                        commit: String::new().into(),
                        committed_at: Utc::now(),
                        major_change: false,
                        created_at: Utc::now(),
                        updated_at: Utc::now(),
                        deleted_at: None,
                    },
                    RfdRevision {
                        id: private_rfd_revision_id_2,
                        rfd_id: TypedUuid::from_untyped_uuid(private_rfd_id_2),
                        title: "Private Test RFD 2".to_string(),
                        state: None,
                        discussion: None,
                        authors: None,
                        labels: None,
                        content: String::new(),
                        content_format: rfd_model::schema_ext::ContentFormat::Asciidoc,
                        sha: String::new().into(),
                        commit: String::new().into(),
                        committed_at: Utc::now(),
                        major_change: false,
                        created_at: Utc::now(),
                        updated_at: Utc::now(),
                        deleted_at: None,
                    },
                ];

                results.retain(|revision| {
                    filter[0].rfd.is_none()
                        || filter[0].rfd.as_ref().unwrap().contains(&revision.rfd_id)
                });

                Ok(results)
            });

        let mut rfd_revision_meta_store = MockRfdRevisionMetaStore::new();
        rfd_revision_meta_store
            .expect_list()
            .returning(move |filter, _| {
                let mut results = vec![
                    RfdRevisionMeta {
                        id: private_rfd_revision_id_1,
                        rfd_id: TypedUuid::from_untyped_uuid(private_rfd_id_1),
                        title: "Private Test RFD 1".to_string(),
                        state: None,
                        discussion: None,
                        authors: None,
                        labels: None,
                        content_format: rfd_model::schema_ext::ContentFormat::Asciidoc,
                        sha: String::new().into(),
                        commit: String::new().into(),
                        committed_at: Utc::now(),
                        major_change: false,
                        created_at: Utc::now(),
                        updated_at: Utc::now(),
                        deleted_at: None,
                    },
                    RfdRevisionMeta {
                        id: public_rfd_revision_id,
                        rfd_id: TypedUuid::from_untyped_uuid(public_rfd_id),
                        title: "Public Test RFD".to_string(),
                        state: None,
                        discussion: None,
                        authors: None,
                        labels: None,
                        content_format: rfd_model::schema_ext::ContentFormat::Asciidoc,
                        sha: String::new().into(),
                        commit: String::new().into(),
                        committed_at: Utc::now(),
                        major_change: false,
                        created_at: Utc::now(),
                        updated_at: Utc::now(),
                        deleted_at: None,
                    },
                    RfdRevisionMeta {
                        id: private_rfd_revision_id_2,
                        rfd_id: TypedUuid::from_untyped_uuid(private_rfd_id_2),
                        title: "Private Test RFD 2".to_string(),
                        state: None,
                        discussion: None,
                        authors: None,
                        labels: None,
                        content_format: rfd_model::schema_ext::ContentFormat::Asciidoc,
                        sha: String::new().into(),
                        commit: String::new().into(),
                        committed_at: Utc::now(),
                        major_change: false,
                        created_at: Utc::now(),
                        updated_at: Utc::now(),
                        deleted_at: None,
                    },
                ];

                results.retain(|revision| {
                    filter.len() == 0
                        || filter[0].rfd.is_none()
                        || filter[0].rfd.as_ref().unwrap().contains(&revision.rfd_id)
                });

                Ok(results)
            });

        let mut rfd_pdf_store = MockRfdPdfStore::new();
        rfd_pdf_store
            .expect_list()
            .returning(move |_, _| Ok(vec![]));

        let mut storage = MockStorage::new();
        storage.rfd_store = Some(Arc::new(rfd_store));
        storage.rfd_meta_store = Some(Arc::new(rfd_meta_store));
        storage.rfd_revision_store = Some(Arc::new(rfd_revision_store));
        storage.rfd_revision_meta_store = Some(Arc::new(rfd_revision_meta_store));
        storage.rfd_pdf_store = Some(Arc::new(rfd_pdf_store));

        mock_context(storage).await
    }

    // Test RFD access via the global All RFDs permission

    #[tokio::test]
    async fn list_rfds_via_all_permission() {
        let ctx = ctx().await;
        let caller = Caller::from(Permissions::from(vec![RfdPermission::GetRfdsAll]));

        let HttpResponseOk(rfds) = list_rfds_op(&ctx, &caller).await.unwrap();
        assert_eq!(3, rfds.len());
        assert_eq!(789, rfds[0].rfd_number);
        assert_eq!(456, rfds[1].rfd_number);
        assert_eq!(123, rfds[2].rfd_number);
    }

    #[tokio::test]
    async fn view_rfd_via_all_permission() {
        let ctx = ctx().await;
        let caller = Caller::from(Permissions::from(vec![RfdPermission::GetRfdsAll]));

        let HttpResponseOk(rfd) = view_rfd_op(&ctx, &caller, "0123".to_string(), None)
            .await
            .unwrap();
        assert_eq!(123, rfd.rfd_number);

        let HttpResponseOk(rfd) = view_rfd_op(&ctx, &caller, "0456".to_string(), None)
            .await
            .unwrap();
        assert_eq!(456, rfd.rfd_number);
    }

    // Test RFD access via the direct permission to a RFD

    #[tokio::test]
    async fn list_rfds_with_direct_permission() {
        let ctx = ctx().await;
        let caller = Caller::from(Permissions::from(vec![RfdPermission::GetRfd(123)]));

        let HttpResponseOk(rfds) = list_rfds_op(&ctx, &caller).await.unwrap();
        assert_eq!(2, rfds.len());
        assert_eq!(456, rfds[0].rfd_number);
        assert_eq!(123, rfds[1].rfd_number);
    }

    #[tokio::test]
    async fn view_rfd_with_direct_permission() {
        let ctx = ctx().await;
        let caller = Caller::from(Permissions::from(vec![RfdPermission::GetRfd(123)]));

        let HttpResponseOk(rfd) = view_rfd_op(&ctx, &caller, "0123".to_string(), None)
            .await
            .unwrap();
        assert_eq!(123, rfd.rfd_number);

        let HttpResponseOk(rfd) = view_rfd_op(&ctx, &caller, "0456".to_string(), None)
            .await
            .unwrap();
        assert_eq!(456, rfd.rfd_number);
    }

    // Test RFD access fails when a caller does not have permission

    #[tokio::test]
    async fn list_rfds_without_permission() {
        let ctx = ctx().await;
        let caller = Caller::from(Permissions::<RfdPermission>::new());

        let HttpResponseOk(rfds) = list_rfds_op(&ctx, &caller).await.unwrap();
        assert_eq!(1, rfds.len());
        assert_eq!(456, rfds[0].rfd_number);
    }

    #[tokio::test]
    async fn view_rfd_without_permission() {
        let ctx = ctx().await;
        let caller = Caller::from(Permissions::<RfdPermission>::new());

        let result = view_rfd_op(&ctx, &caller, "0123".to_string(), None).await;
        match result {
            Err(err) => assert_eq!(StatusCode::NOT_FOUND, err.status_code),
            Ok(response) => panic!(
                "Expected a 404 error, but instead found a RFD {:?}",
                response.0
            ),
        }
    }

    // Test RFD access to public RFDs as the unauthenticated user

    #[tokio::test]
    async fn list_rfds_as_unauthenticated() {
        let ctx = ctx().await;

        let HttpResponseOk(rfds) =
            list_rfds_op(&ctx, &ctx.v_ctx().builtin_unauthenticated_caller())
                .await
                .unwrap();
        assert_eq!(1, rfds.len());
        assert_eq!(456, rfds[0].rfd_number);
    }

    #[tokio::test]
    async fn view_rfd_as_unauthenticated() {
        let ctx = ctx().await;
        let caller = ctx.v_ctx().builtin_unauthenticated_caller();

        let result = view_rfd_op(&ctx, &caller, "0123".to_string(), None).await;
        match result {
            Err(err) => assert_eq!(StatusCode::NOT_FOUND, err.status_code),
            Ok(response) => panic!(
                "Expected a 404 error, but instead found a RFD {:?}",
                response.0
            ),
        }
    }
}
