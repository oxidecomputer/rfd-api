use chrono::{DateTime, Utc};
use dropshot::{endpoint, HttpError, HttpResponseOk, Method, Path, RequestContext, TypedBody};
use http::{header, Request, Response, StatusCode};
use hyper::{body::to_bytes, Body};
use oauth2::{basic::BasicTokenType, EmptyExtraTokenFields, StandardTokenResponse, TokenResponse};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use tap::TapFallible;
use trace_request::trace_request;
use tracing::instrument;

use super::{
    ClientType, OAuthProvider, OAuthProviderInfo, OAuthProviderNameParam, UserInfoProvider,
};
use crate::{
    context::ApiContext, endpoints::login::LoginError, error::ApiError, util::response::bad_request,
};

// Get the metadata about an OAuth provider necessary to begin a device code exchange
#[trace_request]
#[endpoint {
    method = GET,
    path = "/login/oauth/{provider}/device"
}]
#[instrument(skip(rqctx), fields(request_id = rqctx.request_id), err(Debug))]
pub async fn get_device_provider(
    rqctx: RequestContext<ApiContext>,
    path: Path<OAuthProviderNameParam>,
) -> Result<HttpResponseOk<OAuthProviderInfo>, HttpError> {
    let path = path.into_inner();

    let provider = rqctx
        .context()
        .get_oauth_provider(&path.provider)
        .await
        .map_err(ApiError::OAuth)?;

    Ok(HttpResponseOk(provider.provider_info(
        &rqctx.context().public_url,
        &ClientType::Device,
    )))
}

#[derive(Debug, Deserialize, JsonSchema, Serialize)]
pub struct AccessTokenExchangeRequest {
    pub device_code: String,
    pub grant_type: String,
    pub expires_at: Option<DateTime<Utc>>,
}

#[derive(Serialize)]
pub struct AccessTokenExchange {
    provider: ProviderTokenExchange,
    expires_at: Option<DateTime<Utc>>,
}

#[derive(Serialize)]
pub struct ProviderTokenExchange {
    client_id: String,
    device_code: String,
    grant_type: String,
    client_secret: String,
}

impl AccessTokenExchange {
    pub fn new(
        req: AccessTokenExchangeRequest,
        provider: &Box<dyn OAuthProvider + Send + Sync>,
    ) -> Option<Self> {
        provider
            .client_secret(&ClientType::Device)
            .map(|client_secret| Self {
                provider: ProviderTokenExchange {
                    client_id: provider.client_id(&ClientType::Device).to_string(),
                    device_code: req.device_code,
                    grant_type: req.grant_type,
                    client_secret: client_secret.to_string(),
                },
                expires_at: req.expires_at,
            })
    }
}

#[derive(Debug, Deserialize, JsonSchema, Serialize)]
pub struct ProxyTokenResponse {
    access_token: String,
    token_type: String,
    expires_in: Option<u32>,
    refresh_token: Option<String>,
    scopes: Option<Vec<String>>,
}

// Complete a device exchange request against the specified provider. This effectively proxies the
// requests that would go to the provider, captures the returned access tokens, and registers a
// new internal user as needed. The user is then returned an token that is valid for interacting
// with the RFD API
#[endpoint {
    method = POST,
    path = "/login/oauth/{provider}/device/exchange",
    content_type = "application/x-www-form-urlencoded",
}]
#[instrument(skip(rqctx, body), fields(request_id = rqctx.request_id), err(Debug))]
pub async fn exchange_device_token(
    rqctx: RequestContext<ApiContext>,
    path: Path<OAuthProviderNameParam>,
    body: TypedBody<AccessTokenExchangeRequest>,
) -> Result<Response<Body>, HttpError> {
    let ctx = rqctx.context();
    let path = path.into_inner();
    let mut provider = ctx
        .get_oauth_provider(&path.provider)
        .await
        .map_err(ApiError::OAuth)?;

    tracing::debug!(provider = ?provider.name(), "Acquired OAuth provider for token exchange");

    let exchange_request = body.into_inner();

    if let Some(mut exchange) = AccessTokenExchange::new(exchange_request, &mut provider) {
        exchange.provider.client_secret = exchange.provider.client_secret;

        let token_exchange_endpoint = provider.token_exchange_endpoint();

        // We know that this is safe to unwrap as we just deserialized it via the body Extractor
        let body: Body = serde_urlencoded::to_string(&exchange.provider)
            .unwrap()
            .into();

        let request = Request::builder()
            .method(Method::POST)
            .header(header::CONTENT_TYPE, provider.token_exchange_content_type())
            .uri(token_exchange_endpoint)
            .body(body)
            .tap_err(|err| tracing::error!(?err, "Failed to construct token exchange request"))?;

        let response = ctx
            .https_client
            .request(request)
            .await
            .tap_err(|err| tracing::error!(?err, "Token exchange request failed"))?;

        if response.status().is_success() {
            tracing::debug!("Successfully exchanged token with provider");

            let (_, body) = response.into_parts();
            let bytes = to_bytes(body).await?;
            let parsed: StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType> =
                serde_json::from_slice(&bytes).map_err(LoginError::FailedToParseToken)?;

            let info = provider
                .get_user_info(&ctx.https_client, parsed.access_token().secret())
                .await
                .map_err(LoginError::UserInfo)
                .tap_err(|err| tracing::error!(?err, "Failed to look up user information"))?;

            tracing::debug!("Verified and validated OAuth user");

            let api_user = ctx.register_api_user(info).await?;

            tracing::info!(api_user_id = ?api_user.id, "Retrieved api user to generate device token for");

            let token = ctx
                .register_access_token(&api_user, &api_user.permissions, exchange.expires_at)
                .await?;

            tracing::info!(provider = ?path.provider, api_user_id = ?api_user.id, "Generated access token");

            Ok(Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, "application/json")
                .body(
                    serde_json::to_string(&ProxyTokenResponse {
                        access_token: token.signed_token,
                        token_type: "Bearer".to_string(),
                        expires_in: Some(
                            (token.expires_at - Utc::now())
                                .num_seconds()
                                .try_into()
                                .unwrap_or(0),
                        ),
                        refresh_token: None,
                        scopes: None,
                    })
                    .unwrap()
                    .into(),
                )?)
        } else {
            tracing::warn!(provider = ?path.provider, "Received error response from OAuth provider");

            Ok(response)
        }
    } else {
        tracing::info!(provider = ?path.provider, "Found an OAuth provider, but it is not configured properly");

        Err(bad_request("Invalid provider"))
    }
}
