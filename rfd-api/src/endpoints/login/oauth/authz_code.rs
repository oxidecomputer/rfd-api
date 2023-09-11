use chrono::{Duration, Utc};
use dropshot::{
    endpoint, http_response_temporary_redirect, HttpError, HttpResponseOk,
    HttpResponseTemporaryRedirect, Path, Query, RequestContext,
};
use http::StatusCode;
use oauth2::{
    reqwest::async_http_client, AuthorizationCode, CsrfToken, PkceCodeChallenge, PkceCodeVerifier,
    Scope, TokenResponse,
};
use rfd_model::{schema_ext::LoginAttemptState, NewLoginAttempt};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::ops::Add;
use tap::TapFallible;
use tracing::instrument;
use uuid::Uuid;

use super::{OAuthProviderNameParam, UserInfoProvider};
use crate::{
    context::ApiContext,
    endpoints::login::LoginError,
    error::ApiError,
    util::response::{bad_request, client_error, internal_error, to_internal_error}, authn::key::RawApiKey,
};

#[derive(Debug, Deserialize, JsonSchema, Serialize)]
pub struct OAuthAuthzCodeQuery {
    pub client_id: Uuid,
    pub redirect_uri: String,
    pub response_type: String,
    pub state: String,
}

/// Generate the remote provider login url and redirect the user
#[endpoint {
    method = GET,
    path = "/login/oauth/{provider}/authz_code/authorize"
}]
#[instrument(skip(rqctx), fields(request_id = rqctx.request_id), err(Debug))]
pub async fn authz_code_redirect(
    rqctx: RequestContext<ApiContext>,
    path: Path<OAuthProviderNameParam>,
    query: Query<OAuthAuthzCodeQuery>,
) -> Result<HttpResponseTemporaryRedirect, HttpError> {
    let ctx = rqctx.context();
    let path = path.into_inner();
    let query = query.into_inner();

    ctx.get_oauth_client(&query.client_id)
        .await
        .map_err(to_internal_error)?
        .ok_or_else(|| client_error(StatusCode::UNAUTHORIZED, "Invalid client"))
        .and_then(|client| {
            if client.is_redirect_uri_valid(&query.redirect_uri) {
                Ok(client)
            } else {
                Err(client_error(
                    StatusCode::UNAUTHORIZED,
                    "Invalid redirect uri",
                ))
            }
        })?;

    let provider = ctx
        .get_oauth_provider(&path.provider)
        .await
        .map_err(ApiError::OAuth)?;

    tracing::debug!(provider = ?provider.name(), "Acquired OAuth provider for authz code login");

    let client = provider.as_client().map_err(to_internal_error)?;
    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    // Construct a new login attempt with the minimum required values
    let mut attempt = NewLoginAttempt::new(
        query.client_id,
        query.redirect_uri,
        provider.name().to_string(),
        CsrfToken::new_random().secret().to_string(),
        pkce_verifier.secret().to_string(),
    )
    .map_err(|err| {
        tracing::error!(?err, "Attempted to construct invalid login attempt");
        internal_error("Attempted to construct invalid login attempt".to_string())
    })?;

    // Add in the user defined state and redirect uri
    attempt.state = Some(query.state);

    // Store the generate attempt
    let attempt = ctx
        .create_login_attempt(attempt)
        .await
        .map_err(to_internal_error)?;

    // Generate the url to the remote provider that the user will be redirected to
    let (url, _) = client
        .authorize_url(|| CsrfToken::new(format!("{}:{}", attempt.id, attempt.provider_state)))
        .set_pkce_challenge(pkce_challenge)
        .add_scopes(
            provider
                .scopes()
                .into_iter()
                .map(|s| Scope::new(s.to_string()))
                .collect::<Vec<_>>(),
        )
        .url();

    http_response_temporary_redirect(url.to_string())
}

#[derive(Debug, Deserialize, JsonSchema, Serialize)]
pub struct OAuthAuthzCodeReturnQuery {
    pub state: Option<String>,
    pub code: Option<String>,
    pub error: Option<String>,
}

/// Handle return calls from a remote OAuth provider
#[endpoint {
    method = GET,
    path = "/login/oauth/{provider}/authz_code/return"
}]
#[instrument(skip(rqctx), fields(request_id = rqctx.request_id), err(Debug))]
pub async fn authz_code_return(
    rqctx: RequestContext<ApiContext>,
    path: Path<OAuthProviderNameParam>,
    query: Query<OAuthAuthzCodeReturnQuery>,
) -> Result<HttpResponseTemporaryRedirect, HttpError> {
    let ctx = rqctx.context();
    let path = path.into_inner();
    let provider = ctx
        .get_oauth_provider(&path.provider)
        .await
        .map_err(ApiError::OAuth)?;
    let query = query.into_inner();

    tracing::debug!(provider = ?provider.name(), "Acquired OAuth provider for authz code exchange");

    let original_attempt = match query.state {
        Some(state) => {

            // Attempt to extract the request id and csrf token from the state parameter. These
            // must both be present
            if let Some((id, csrf)) = state
                .split_once(":")
                .and_then(|(id, csrf)| id.parse::<Uuid>().ok().map(|id| (id, csrf))) {

                    // Look up the login attempt referenced in the state and verify that has the
                    // csrf value still matches
                    ctx
                        .get_login_attempt(&id)
                        .await
                        .map_err(to_internal_error)?
                        .and_then(|attempt| {
                            if attempt.state.as_ref().map(|s| s == csrf).unwrap_or(false) {
                                Some(attempt)
                            } else {
                                None
                            }
                        })
                } else {
                    None
                }
        },
        None => None
    };

    // If an attempt could not be found than the server needs to fail with an internal server error.
    // We do not have a redirect_uri to send the user to. This is a deficiency in how login attempts
    // are tracked. They are currently tracked without storing any state on the client, and as such
    // are fully dependent on the state parameter. Instead a very short lived cookie should be used
    // to track the attempt that the client is making so that we can restore the attempt without
    // use of the state parameter
    let mut attempt = original_attempt
        .ok_or_else(|| internal_error("Failed to load matching login attempt"))?;

    attempt = match (query.code, query.error) {
        (Some(code), None) => {

            // Store the authorization code returned by the underlying OAuth provider and transition the
            // attempt to the awaiting state
            ctx
                .set_login_provider_authz_code(attempt, code.to_string())
                .await
                .map_err(to_internal_error)?
        }
        (code, error) => {

            // Store the provider return error for future debugging, but if an error has been
            // returned or there is a missing code, then we can not report a successful process
            attempt.provider_authz_code = code;

            // TODO: Specialize the returned error
            ctx.fail_login_attempt(attempt, Some("server_error"), error.as_deref()).await.map_err(to_internal_error)?
        }
    };

    // Redirect back to the original authenticator
    http_response_temporary_redirect(attempt.callback_url())
}



#[derive(Debug, Deserialize, JsonSchema, Serialize)]
pub struct OAuthAuthzCodeExchangeQuery {
    pub client_id: Uuid,
    pub client_secret: String,
    pub redirect_uri: String,
    pub grant_type: String,
    pub code: String,
    pub pkce_verifier: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema, Serialize)]
pub struct OAuthAuthzCodeExchangeResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i64,
}

/// Exchange an authorization code for an access token
#[endpoint {
    method = GET,
    path = "/login/oauth/{provider}/authz_code/token"
}]
#[instrument(skip(rqctx), fields(request_id = rqctx.request_id), err(Debug))]
pub async fn authz_code_exchange(
    rqctx: RequestContext<ApiContext>,
    path: Path<OAuthProviderNameParam>,
    query: Query<OAuthAuthzCodeExchangeQuery>,
) -> Result<HttpResponseOk<OAuthAuthzCodeExchangeResponse>, HttpError> {
    let ctx = rqctx.context();
    let path = path.into_inner();
    let provider = ctx
        .get_oauth_provider(&path.provider)
        .await
        .map_err(ApiError::OAuth)?;
    let query = query.into_inner();

    // Verify that we received the expected grant type
    if &query.grant_type != "authorization_code" {
        return Err(bad_request("Invalid grant type"));
    }

    let client_secret = RawApiKey::new(query.client_secret.clone()).encrypt(&*ctx.api_key.encryptor).await.map_err(to_internal_error)?;

    ctx.get_oauth_client(&query.client_id)
        .await
        .map_err(to_internal_error)?
        .ok_or_else(|| client_error(StatusCode::UNAUTHORIZED, "Invalid client"))
        .and_then(|client| {
            if client.is_redirect_uri_valid(&query.redirect_uri) {
                if client.is_secret_valid(&client_secret.encrypted) {
                    Ok(client)
                } else {
                    Err(client_error(
                        StatusCode::UNAUTHORIZED,
                        "Invalid secret",
                    ))    
                }
            } else {
                Err(client_error(
                    StatusCode::UNAUTHORIZED,
                    "Invalid redirect uri",
                ))
            }
        })?;

    // Lookup the request assigned to this code and verify that it is a valid request
    let attempt = ctx
        .get_login_attempt_for_code(&query.code)
        .await
        .map_err(to_internal_error)?
        .ok_or_else(|| bad_request("Invalid code".to_string()))
        .and_then(|attempt| {
            if attempt.client_id != query.client_id {
                Err(bad_request("Invalid client id".to_string()))
            } else if attempt.redirect_uri != query.redirect_uri {
                Err(bad_request("Invalid redirect uri".to_string()))
            } else if attempt.attempt_state != LoginAttemptState::RemoteAuthenticated {
                Err(bad_request("Invalid login state".to_string()))
            } else if attempt.expires_at.map(|t| t <= Utc::now()).unwrap_or(true) {
                Err(bad_request("Login attempt expired".to_string()))
            } else {
                // TODO: Perform pkce check

                Ok(attempt)
            }
        })?;

    // Exchange the stored authorization code with the remote provider for a remote access token
    let client = provider.as_client().map_err(to_internal_error)?;
    let response = client
        .exchange_code(AuthorizationCode::new(
            attempt.provider_authz_code.ok_or_else(|| {
                internal_error("Expected authorization code to exist due to attempt state")
            })?,
        ))
        .set_pkce_verifier(PkceCodeVerifier::new(attempt.provider_pkce_verifier))
        .request_async(async_http_client)
        .await
        .map_err(to_internal_error)?;

    // Use the retrieved access token to fetch the user information from the remote API
    let info = provider
        .get_user_info(&ctx.https_client, response.access_token().secret())
        .await
        .map_err(LoginError::UserInfo)
        .tap_err(|err| tracing::error!(?err, "Failed to look up user information"))?;

    tracing::debug!("Verified and validated OAuth user");

    // Register this user as an API user if needed
    let api_user = ctx.register_api_user(info).await?;

    tracing::info!(api_user_id = ?api_user.id, "Retrieved api user to generate access token for");

    // Generate a new access token for the user with an expiration matching the value given to us
    // by the remote service
    let token = ctx
        .register_access_token(
            &api_user,
            &api_user.permissions,
            Some(
                Utc::now().add(Duration::seconds(
                    response
                        .expires_in()
                        .map(|d| d.as_secs() - 120)
                        .unwrap_or(0) as i64,
                )),
            ),
        )
        .await?;

    tracing::info!(provider = ?path.provider, api_user_id = ?api_user.id, "Generated access token");

    Ok(HttpResponseOk(OAuthAuthzCodeExchangeResponse {
        token_type: "Bearer".to_string(),
        access_token: token.signed_token,
        expires_in: token.expires_at.timestamp() - Utc::now().timestamp(),
    }))
}
