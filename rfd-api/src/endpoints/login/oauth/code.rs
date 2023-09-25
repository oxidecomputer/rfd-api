use chrono::{Duration, Utc};
use dropshot::{
    endpoint, http_response_temporary_redirect, HttpError, HttpResponseOk,
    HttpResponseTemporaryRedirect, Path, Query, RequestContext, RequestInfo, TypedBody,
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
use rfd_model::{schema_ext::LoginAttemptState, LoginAttempt, NewLoginAttempt, OAuthClient};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::ops::Add;
use tap::TapFallible;
use tracing::instrument;
use uuid::Uuid;

use super::{OAuthProvider, OAuthProviderNameParam, UserInfoProvider};
use crate::{
    authn::key::RawApiKey,
    context::ApiContext,
    endpoints::login::{
        oauth::{CheckOAuthClient, ClientType},
        LoginError,
    },
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

// Lookup the client specified by the provided client id and verify that the redirect uri
// is a valid for this client. If either of these fail we return an unauthorized response
async fn get_oauth_client(
    ctx: &ApiContext,
    client_id: &Uuid,
    redirect_uri: &str,
) -> Result<OAuthClient, HttpError> {
    let client = ctx
        .get_oauth_client(&client_id)
        .await
        .map_err(to_internal_error)?
        .ok_or_else(|| client_error(StatusCode::UNAUTHORIZED, "Invalid client"))?;

    if client.is_redirect_uri_valid(&redirect_uri) {
        Ok(client)
    } else {
        Err(client_error(
            StatusCode::UNAUTHORIZED,
            "Invalid redirect uri",
        ))
    }
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

    get_oauth_client(ctx, &query.client_id, &query.redirect_uri).await?;

    tracing::debug!(?query.client_id, ?query.redirect_uri, "Verified client id and redirect uri");

    // Find the configured provider for the requested remote backend. We should always have a valid
    // provider value, so if this fails then a 500 is returned
    let provider = ctx
        .get_oauth_provider(&path.provider)
        .await
        .map_err(ApiError::OAuth)?;

    tracing::debug!(provider = ?provider.name(), "Acquired OAuth provider for authz code login");

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

    let pkce_challenge = if provider.supports_pkce() {
        let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();
        attempt.provider_pkce_verifier = Some(pkce_verifier.secret().to_string());
        Some(pkce_challenge)
    } else {
        None
    };

    // Add in the user defined state and redirect uri
    attempt.state = Some(query.state);

    // Store the generated attempt
    let attempt = ctx
        .create_login_attempt(attempt)
        .await
        .map_err(to_internal_error)?;

    tracing::info!(?attempt.id, "Created login attempt");

    Ok(oauth_redirect_response(
        &*provider,
        &attempt,
        pkce_challenge,
    )?)
}

fn oauth_redirect_response(
    provider: &dyn OAuthProvider,
    attempt: &LoginAttempt,
    code_challenge: Option<PkceCodeChallenge>,
) -> Result<Response<Body>, HttpError> {
    // We may fail if the provider configuration is not correctly configured
    // TODO: This behavior should be changed so that clients are precomputed. We do not need to be
    // constructing a new client on every request. That said, we need to ensure the client does not
    // maintain state between requests
    let client = provider
        .as_client(&ClientType::Web)
        .map_err(to_internal_error)?;

    // Create an attempt cookie header for storing the login attempt. This also acts as our csrf
    // check
    let login_cookie = HeaderValue::from_str(&format!("{}={}", LOGIN_ATTEMPT_COOKIE, attempt.id))
        .map_err(to_internal_error)?;

    // Generate the url to the remote provider that the user will be redirected to
    let mut authz_url = client
        .authorize_url(|| CsrfToken::new(attempt.id.to_string()))
        .add_scopes(
            provider
                .scopes()
                .into_iter()
                .map(|s| Scope::new(s.to_string()))
                .collect::<Vec<_>>(),
        );

    // If the caller has provided a code challenge, add it to the url
    if let Some(challenge) = code_challenge {
        authz_url = authz_url.set_pkce_challenge(challenge);
    };

    Ok(Response::builder()
        .status(StatusCode::TEMPORARY_REDIRECT)
        .header(SET_COOKIE, login_cookie)
        .header(
            LOCATION,
            HeaderValue::from_str(authz_url.url().0.as_str()).map_err(to_internal_error)?,
        )
        .body(Body::empty())?)
}

fn verify_csrf(
    request: &RequestInfo,
    query: &OAuthAuthzCodeReturnQuery,
) -> Result<Uuid, HttpError> {
    // If we are missing the expected state parameter then we can not proceed at all with verifying
    // this callback request. We also do not have a redirect uri to send the user to so we instead
    // report unauthorized
    let attempt_id = query
        .state
        .as_ref()
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
    let attempt_cookie: Uuid = request
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
        Err(unauthorized())
    } else {
        Ok(attempt_id)
    }
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
    let query = query.into_inner();
    let provider = ctx
        .get_oauth_provider(&path.provider)
        .await
        .map_err(ApiError::OAuth)?;

    tracing::debug!(provider = ?provider.name(), "Acquired OAuth provider for authz code exchange");

    // Verify and extract the attempt id before performing any work
    let attempt_id = verify_csrf(&rqctx.request, &query)?;

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
    let body = body.into_inner();
    let provider = ctx
        .get_oauth_provider(&path.provider)
        .await
        .map_err(ApiError::OAuth)?;

    // Verify that we received the expected grant type
    if &body.grant_type != "authorization_code" {
        // TODO: Needs to be json body
        return Err(bad_request("Invalid grant type"));
    }

    let client_secret = RawApiKey::try_from(body.client_secret.as_str()).map_err(|err| {
        tracing::warn!(?err, "Failed to parse OAuth client secret");

        // TODO: Change this to a bad request with invalid_client ?
        unauthorized()
    })?;

    ctx.get_oauth_client(&body.client_id)
        .await
        .map_err(to_internal_error)?
        // TODO: This should be a bad request with a json body
        .ok_or_else(|| client_error(StatusCode::UNAUTHORIZED, "Invalid client"))
        .and_then(|client| {
            if client.is_redirect_uri_valid(&body.redirect_uri) {
                if client.is_secret_valid(&client_secret, &*ctx.secrets.signer) {
                    Ok(client)
                } else {
                    // TODO: Change this to a bad request with invalid_client ?
                    Err(client_error(StatusCode::UNAUTHORIZED, "Invalid secret"))
                }
            } else {
                // TODO: Change this to a bad request with invalid_client ?
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
        // TODO: Bad request is ok, but a json body with invalid_grant should be returned
        .ok_or_else(|| bad_request("Invalid code".to_string()))
        .and_then(|attempt| {
            if attempt.client_id != body.client_id {
                // TODO: Bad request is ok, but a json body with invalid_grant should be returned
                Err(bad_request("Invalid client id".to_string()))
            } else if attempt.redirect_uri != body.redirect_uri {
                // TODO: Bad request is ok, but a json body with invalid_grant should be returned
                Err(bad_request("Invalid redirect uri".to_string()))
            } else if attempt.attempt_state != LoginAttemptState::RemoteAuthenticated {
                // TODO: Bad request is ok, but a json body with invalid_client should be returned
                Err(bad_request("Invalid login state".to_string()))
            } else if attempt.expires_at.map(|t| t <= Utc::now()).unwrap_or(true) {
                // TODO: Bad request is ok, but a json body with invalid_client should be returned
                Err(bad_request("Login attempt expired".to_string()))
            } else {
                // TODO: Perform pkce check

                Ok(attempt)
            }
        })?;

    // Exchange the stored authorization code with the remote provider for a remote access token
    let client = provider
        .as_client(&ClientType::Web)
        .map_err(to_internal_error)?;
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

    tracing::info!("Fetched access token from remote service");

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

#[cfg(test)]
mod tests {
    use std::net::{Ipv4Addr, SocketAddrV4};

    use chrono::Utc;
    use dropshot::RequestInfo;
    use http::{
        header::{COOKIE, LOCATION, SET_COOKIE},
        HeaderValue,
    };
    use hyper::Body;
    use oauth2::PkceCodeChallenge;
    use rfd_model::{schema_ext::LoginAttemptState, LoginAttempt};
    use uuid::Uuid;

    use crate::{
        context::tests::{mock_context, MockStorage},
        endpoints::login::oauth::{
            code::{verify_csrf, OAuthAuthzCodeReturnQuery, LOGIN_ATTEMPT_COOKIE},
            OAuthProviderName,
        },
    };

    use super::oauth_redirect_response;

    #[tokio::test]
    async fn test_csrf_check() {
        let id = Uuid::new_v4();

        let mut rq = hyper::Request::new(Body::empty());
        rq.headers_mut().insert(
            COOKIE,
            HeaderValue::from_str(&format!("{}={}", LOGIN_ATTEMPT_COOKIE, id)).unwrap(),
        );

        let request = RequestInfo::new(
            &rq,
            std::net::SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 8888)),
        );

        let query = OAuthAuthzCodeReturnQuery {
            state: Some(id.to_string()),
            code: None,
            error: None,
        };

        assert_eq!(id, verify_csrf(&request, &query).unwrap());
    }

    #[tokio::test]
    async fn test_remote_provider_redirect_url() {
        let storage = MockStorage::new();
        let ctx = mock_context(storage).await;

        let (challenge, _) = PkceCodeChallenge::new_random_sha256();
        let attempt = LoginAttempt {
            id: Uuid::new_v4(),
            attempt_state: LoginAttemptState::New,
            client_id: Uuid::new_v4(),
            redirect_uri: "https://test.oxeng.dev/callback".to_string(),
            state: Some("ox_state".to_string()),
            pkce_challenge: Some("ox_challenge".to_string()),
            pkce_challenge_method: Some("S256".to_string()),
            authz_code: None,
            expires_at: None,
            error: None,
            provider: "google".to_string(),
            provider_pkce_verifier: Some("rfd_verifier".to_string()),
            provider_authz_code: None,
            provider_error: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let response = oauth_redirect_response(
            &*ctx
                .get_oauth_provider(&OAuthProviderName::Google)
                .await
                .unwrap(),
            &attempt,
            Some(challenge.clone()),
        )
        .unwrap();

        let expected_location = format!("https://accounts.google.com/o/oauth2/auth?response_type=code&client_id=google_web_client_id&state={}&code_challenge={}&code_challenge_method=S256&scope=https%3A%2F%2Fwww.googleapis.com%2Fauth%2Fuserinfo.email+openid", attempt.id, challenge.as_str());

        assert_eq!(
            expected_location,
            String::from_utf8(
                response
                    .headers()
                    .get(LOCATION)
                    .unwrap()
                    .as_bytes()
                    .to_vec()
            )
            .unwrap()
        );
        assert_eq!(
            attempt.id.to_string().as_str(),
            String::from_utf8(
                response
                    .headers()
                    .get(SET_COOKIE)
                    .unwrap()
                    .as_bytes()
                    .to_vec()
            )
            .unwrap()
            .split_once('=')
            .unwrap()
            .1
        )
    }
}
