use chrono::{DateTime, Utc};
use dropshot::{endpoint, HttpError, HttpResponseOk, Method, Path, RequestContext, TypedBody};
use http::{header, Request, Response, StatusCode, HeaderValue};
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

    tracing::trace!("Getting OAuth data for {}", path.provider);

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

#[derive(Debug, Deserialize, JsonSchema, Serialize)]
pub struct ProxyTokenError {
    error: String,
    error_description: Option<String>,
    error_uri: Option<String>,
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
            .header(header::ACCEPT, HeaderValue::from_static("application/json"))
            .uri(token_exchange_endpoint)
            .body(body)
            .tap_err(|err| tracing::error!(?err, "Failed to construct token exchange request"))?;

        let response = ctx
            .https_client
            .request(request)
            .await
            .tap_err(|err| tracing::error!(?err, "Token exchange request failed"))?;

        let (parts, body) = response.into_parts();

        // We unfortunately can not trust our providers to follow specs and therefore need to do
        // our own inspection of the response to determine what to do
        if !parts.status.is_success() {

            // If the server returned a non-success status then we are going to trust the server and
            // report their error back to the client
            tracing::debug!(provider = ?path.provider, "Received error response from OAuth provider");

            Ok(Response::from_parts(parts, body))
        } else {

            // The server gave us back a non-error response but it still may not be a success.
            // GitHub for instance does not use a status code for indicating the success or failure
            // of a call. So instead we try to deserialize the body into an access token, with the
            // understanding that it may fail and we will need to try and treat the response as
            // an error instead.

            let bytes = to_bytes(body).await?;
            let parsed: Result<StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>, serde_json::Error> = serde_json::from_slice(&bytes);

            match parsed {
                Ok(parsed) => {
                    let info = provider
                    .get_user_info(provider.client(), parsed.access_token().secret())
                    .await
                    .map_err(LoginError::UserInfo)
                    .tap_err(|err| tracing::error!(?err, "Failed to look up user information"))?;

                    tracing::debug!("Verified and validated OAuth user");

                    let api_user = ctx.register_api_user(info).await?;

                    tracing::info!(api_user_id = ?api_user.id, "Retrieved api user to generate device token for");

                    let token = ctx
                        .register_access_token(
                            &api_user,
                            vec![
                                "user:info:r".to_string(),
                                "user:token:r".to_string(),
                                "user:token:w".to_string(),
                                "group:r".to_string(),
                                "rfd:content:r".to_string(),
                                "rfd:discussion:r".to_string(),
                                "search".to_string(),
                            ],
                            exchange.expires_at,
                        )
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
                },
                Err(_) => {

                    // Do not log the error here as we want to ensure we do not leak token information
                    tracing::debug!("Failed to parse a success response from the remote token endpoint");

                    // Try to deserialize the body again, but this time as an error
                    let mut error_response = match serde_json::from_slice::<ProxyTokenError>(&bytes) {
                        Ok(error) => {

                            // We found an error in the message body. This is not ideal, but we at
                            // least can understand what the server was trying to tell us
                            tracing::debug!(?error, provider = ?path.provider, "Parsed error response from OAuth provider");
                            Response::from_parts(parts, Body::from(bytes))
                        }
                        Err(_) => {
                            
                            // We still do not know what the remote server is doing... and need to
                            // cancel the request ourselves
                            tracing::warn!("Remote OAuth provide returned a response that we do not undestand");

                            Response::new(Body::from(serde_json::to_string(&ProxyTokenError {
                                error: "access_denied".to_string(),
                                error_description: Some(format!("{} returned a malformed response", path.provider)),
                                error_uri: None,
                            }).unwrap()))
                        }
                    };

                    *error_response.status_mut() = StatusCode::BAD_REQUEST;
                    error_response.headers_mut().insert(
                        header::CONTENT_TYPE,
                        HeaderValue::from_static("application/json")
                    );

                    Ok(error_response)
                }
            }

        }
    } else {
        tracing::info!(provider = ?path.provider, "Found an OAuth provider, but it is not configured properly");

        Err(bad_request("Invalid provider"))
    }
}
