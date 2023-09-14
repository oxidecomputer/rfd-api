use chrono::{Duration, Utc};
use dropshot::{
    endpoint, http_response_temporary_redirect, HttpError, HttpResponseOk,
    HttpResponseTemporaryRedirect, Path, Query, RequestContext, TypedBody,
};
use http::{
    header::{LOCATION, SET_COOKIE},
    HeaderValue, StatusCode,
};
use hyper::{Body, Response};
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
    authn::key::SignedApiKey,
    context::ApiContext,
    endpoints::login::LoginError,
    error::ApiError,
    util::{
        request::RequestCookies,
        response::{bad_request, client_error, internal_error, to_internal_error, unauthorized},
    },
};

static LOGIN_ATTEMPT_COOKIE: &str = "__rfd_login";

#[derive(Debug, Deserialize, JsonSchema, Serialize)]
pub struct OAuthAuthzCodeQuery {
    pub client_id: Uuid,
    pub redirect_uri: String,
    pub response_type: String,
    pub state: String,
}

#[derive(Debug, Deserialize, JsonSchema, Serialize)]
pub struct OAuthAuthzCodeRedirectHeaders {
    #[serde(rename = "set-cookies")]
    cookies: String,
    location: String,
}

/// Generate the remote provider login url and redirect the user
#[endpoint {
    method = GET,
    path = "/login/oauth/{provider}/code/authorize"
}]
#[instrument(skip(rqctx), fields(request_id = rqctx.request_id), err(Debug))]
pub async fn authz_code_redirect(
    rqctx: RequestContext<ApiContext>,
    path: Path<OAuthProviderNameParam>,
    query: Query<OAuthAuthzCodeQuery>,
) -> Result<Response<Body>, HttpError> {
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
    )
    .map_err(|err| {
        tracing::error!(?err, "Attempted to construct invalid login attempt");
        internal_error("Attempted to construct invalid login attempt".to_string())
    })?;

    if provider.supports_pkce() {
        attempt.provider_pkce_verifier = Some(pkce_verifier.secret().to_string());
    }

    // Add in the user defined state and redirect uri
    attempt.state = Some(query.state);

    // Store the generated attempt
    let attempt = ctx
        .create_login_attempt(attempt)
        .await
        .map_err(to_internal_error)?;

    // Create an attempt cookie header for storing the login attempt
    let login_cookie = HeaderValue::from_str(&format!("{}={}", LOGIN_ATTEMPT_COOKIE, attempt.id))
        .map_err(to_internal_error)?;

    // Generate the url to the remote provider that the user will be redirected to
    let (url, _) = client
        .authorize_url(|| CsrfToken::new(attempt.id.to_string()))
        .set_pkce_challenge(pkce_challenge)
        .add_scopes(
            provider
                .scopes()
                .into_iter()
                .map(|s| Scope::new(s.to_string()))
                .collect::<Vec<_>>(),
        )
        .url();

    Ok(Response::builder()
        .status(StatusCode::TEMPORARY_REDIRECT)
        .header(SET_COOKIE, login_cookie)
        .header(
            LOCATION,
            HeaderValue::from_str(url.as_str()).map_err(to_internal_error)?,
        )
        .body(Body::empty())?)
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
    path = "/login/oauth/{provider}/code/callback"
}]
#[instrument(skip(rqctx), fields(request_id = rqctx.request_id), err(Debug))]
pub async fn authz_code_callback(
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

    // If we are missing the expected state parameter then we can not proceed at all with verifying
    // this callback request. We also do not have a redirect uri to send the user to so we instead
    // report unauthorized
    let attempt_id = query
        .state
        .ok_or_else(|| {
            tracing::warn!("OAuth callback is missing a state parameter");
            unauthorized()
        })?
        .parse()
        .map_err(|err| {
            tracing::warn!(?err, "Failed to parse state");
            unauthorized()
        })?;

    // The client must present the attempt cookie at a minimum. Without it we are unable to lookup a
    // login attempt to match against. Without the cookie to verify the state parameter we can not
    // determine a redirect uri so we instead report unauthorized
    let attempt_cookie: Uuid = rqctx
        .request
        .cookie(LOGIN_ATTEMPT_COOKIE)
        .ok_or_else(|| {
            tracing::warn!("OAuth callback is missing a login state cookie");
            unauthorized()
        })?
        .value()
        .parse()
        .map_err(|err| {
            tracing::warn!(?err, "Failed to parse state");
            unauthorized()
        })?;

    // Verify that the attempt_id returned from the state matches the expected client value. If they
    // do not match we can not lookup a redirect uri so we instead return unauthorized
    if attempt_id != attempt_cookie {
        tracing::warn!(
            ?attempt_id,
            ?attempt_cookie,
            "OAuth state does not match expected cookie value"
        );
        return Err(unauthorized());
    }

    // We have now verified the attempt id and can use it to look up the rest of the login attempt
    // material to try and complete the flow
    let mut attempt = ctx
        .get_login_attempt(&attempt_id)
        .await
        .map_err(to_internal_error)?
        .ok_or_else(|| {
            // If we fail to find a matching attempt, there is not much we can do other than return
            // unauthorized
            unauthorized()
        })?;

    attempt = match (query.code, query.error) {
        (Some(code), None) => {
            tracing::info!(?attempt.id, "Received valid login attempt. Storing authorization code");

            // Store the authorization code returned by the underlying OAuth provider and transition the
            // attempt to the awaiting state
            ctx.set_login_provider_authz_code(attempt, code.to_string())
                .await
                .map_err(to_internal_error)?
        }
        (code, error) => {
            tracing::info!(?attempt.id, ?error, "Received an error response from the remote server");

            // Store the provider return error for future debugging, but if an error has been
            // returned or there is a missing code, then we can not report a successful process
            attempt.provider_authz_code = code;

            // TODO: Specialize the returned error
            ctx.fail_login_attempt(attempt, Some("server_error"), error.as_deref())
                .await
                .map_err(to_internal_error)?
        }
    };

    // Redirect back to the original authenticator
    http_response_temporary_redirect(attempt.callback_url())
}

#[derive(Debug, Deserialize, JsonSchema, Serialize)]
pub struct OAuthAuthzCodeExchangeBody {
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
    method = POST,
    path = "/login/oauth/{provider}/code/token",
    content_type = "application/x-www-form-urlencoded",
}]
#[instrument(skip(rqctx), fields(request_id = rqctx.request_id), err(Debug))]
pub async fn authz_code_exchange(
    rqctx: RequestContext<ApiContext>,
    path: Path<OAuthProviderNameParam>,
    body: TypedBody<OAuthAuthzCodeExchangeBody>,
) -> Result<HttpResponseOk<OAuthAuthzCodeExchangeResponse>, HttpError> {
    let ctx = rqctx.context();
    let path = path.into_inner();
    let provider = ctx
        .get_oauth_provider(&path.provider)
        .await
        .map_err(ApiError::OAuth)?;
    let body = body.into_inner();

    // Verify that we received the expected grant type
    if &body.grant_type != "authorization_code" {
        return Err(bad_request("Invalid grant type"));
    }

    let client_secret = SignedApiKey::parse(&body.client_secret, &*ctx.secrets.signer)
        .map_err(to_internal_error)?;

    ctx.get_oauth_client(&body.client_id)
        .await
        .map_err(to_internal_error)?
        .ok_or_else(|| client_error(StatusCode::UNAUTHORIZED, "Invalid client"))
        .and_then(|client| {
            if client.is_redirect_uri_valid(&body.redirect_uri) {
                if client.is_secret_valid(client_secret.signature()) {
                    Ok(client)
                } else {
                    Err(client_error(StatusCode::UNAUTHORIZED, "Invalid secret"))
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
        .get_login_attempt_for_code(&body.code)
        .await
        .map_err(to_internal_error)?
        .ok_or_else(|| bad_request("Invalid code".to_string()))
        .and_then(|attempt| {
            if attempt.client_id != body.client_id {
                Err(bad_request("Invalid client id".to_string()))
            } else if attempt.redirect_uri != body.redirect_uri {
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
    let mut request = client.exchange_code(AuthorizationCode::new(
        attempt.provider_authz_code.ok_or_else(|| {
            internal_error("Expected authorization code to exist due to attempt state")
        })?,
    ));

    if let Some(pkce_verifier) = attempt.provider_pkce_verifier {
        request = request.set_pkce_verifier(PkceCodeVerifier::new(pkce_verifier))
    }

    let response = request
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
