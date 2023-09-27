use base64::{prelude::BASE64_URL_SAFE_NO_PAD, Engine};
use chrono::{Duration, Utc};
use dropshot::{
    endpoint, http_response_temporary_redirect, HttpError, HttpResponseOk,
    HttpResponseTemporaryRedirect, Path, Query, RequestContext, RequestInfo, TypedBody,
};
use http::{
    header::{LOCATION, SET_COOKIE},
    HeaderValue, StatusCode,
};
use hyper::{client::HttpConnector, Body, Client, Response};
use hyper_tls::HttpsConnector;
use oauth2::{
    reqwest::async_http_client, AuthorizationCode, CsrfToken, PkceCodeChallenge, PkceCodeVerifier,
    Scope, TokenResponse,
};
use rfd_model::{schema_ext::LoginAttemptState, LoginAttempt, NewLoginAttempt, OAuthClient};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
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
        LoginError, UserInfo,
    },
    error::ApiError,
    util::{
        request::RequestCookies,
        response::{bad_request, client_error, internal_error, to_internal_error, unauthorized},
    },
};

static LOGIN_ATTEMPT_COOKIE: &str = "__rfd_login";

#[derive(Debug, Deserialize, JsonSchema, Serialize, PartialEq, Eq)]
struct OAuthError {
    error: OAuthErrorCode,
    #[serde(skip_serializing_if = "Option::is_none")]
    error_description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error_uri: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
enum OAuthErrorCode {
    InvalidRequest,
    InvalidClient,
    InvalidGrant,
    UnauthorizedClient,
    UnsupportedGrantType,
    InvalidScope,
}

impl From<OAuthError> for HttpError {
    fn from(value: OAuthError) -> Self {
        let serialized = serde_json::to_string(&value).unwrap();
        HttpError {
            status_code: StatusCode::BAD_REQUEST,
            error_code: None,
            external_message: serialized.clone(),
            internal_message: serialized,
        }
    }
}

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

    http_response_temporary_redirect(
        authz_code_callback_op(ctx, &attempt_id, query.code, query.error).await?,
    )
}

#[instrument(skip(ctx, code), err(Debug))]
pub async fn authz_code_callback_op(
    ctx: &ApiContext,
    attempt_id: &Uuid,
    code: Option<String>,
    error: Option<String>,
) -> Result<String, HttpError> {
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
        })
        .and_then(|attempt| {
            if attempt.attempt_state == LoginAttemptState::New {
                Ok(attempt)
            } else {
                Err(unauthorized())
            }
        })?;

    attempt = match (code, error) {
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

            // When a user has explicitly denied access we want to forward that error message
            // onwards to the upstream requester. All other errors should be opaque to the
            // original requester and are returned as server errors
            let error_message = match error.as_deref() {
                Some("access_denied") => "access_denied",
                _ => "server_error",
            };

            // TODO: Specialize the returned error
            ctx.fail_login_attempt(attempt, Some(error_message), error.as_deref())
                .await
                .map_err(to_internal_error)?
        }
    };

    // Redirect back to the original authenticator
    Ok(attempt.callback_url())
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

    // Verify the submitted client credentials
    authorize_exchange(
        &ctx,
        &body.grant_type,
        &body.client_id,
        &body.client_secret,
        &body.redirect_uri,
    )
    .await?;

    tracing::debug!("Authorized code exchange");

    // Lookup the request assigned to this code
    let attempt = ctx
        .get_login_attempt_for_code(&body.code)
        .await
        .map_err(to_internal_error)?
        // TODO: Bad request is ok, but a json body with invalid_grant should be returned
        .ok_or_else(|| bad_request("invalid_grant".to_string()))?;

    // Verify that the login attempt is valid and matches the submitted client credentials
    verify_login_attempt(
        &attempt,
        &body.client_id,
        &body.redirect_uri,
        body.pkce_verifier.as_deref(),
    )?;

    tracing::debug!("Verified login attempt");

    // Now that the attempt has been confirmed, use it to fetch user information form the remote
    // provider
    let info = fetch_user_info(&ctx.https_client, &*provider, &attempt).await?;

    tracing::debug!("Retrieved user information from remote provider");

    // Register this user as an API user if needed
    let api_user = ctx.register_api_user(info).await?;

    tracing::info!(api_user_id = ?api_user.id, "Retrieved api user to generate access token for");

    // Generate a new access token for the user with an expiration matching the value given to us
    // by the remote service
    let token = ctx
        .register_access_token(
            &api_user,
            &api_user.permissions,
            Some(Utc::now().add(Duration::seconds(7 * 24 * 60 * 60))),
        )
        .await?;

    tracing::info!(provider = ?path.provider, api_user_id = ?api_user.id, "Generated access token");

    Ok(HttpResponseOk(OAuthAuthzCodeExchangeResponse {
        token_type: "Bearer".to_string(),
        access_token: token.signed_token,
        expires_in: token.expires_at.timestamp() - Utc::now().timestamp(),
    }))
}

async fn authz_code_exchange_op() {}

async fn authorize_exchange(
    ctx: &ApiContext,
    grant_type: &str,
    client_id: &Uuid,
    client_secret: &str,
    redirect_uri: &str,
) -> Result<(), HttpError> {
    let client = get_oauth_client(ctx, &client_id, &redirect_uri).await?;

    // Verify that we received the expected grant type
    if grant_type != "authorization_code" {
        // TODO: Needs to be json body
        return Err(bad_request("unsupported_grant_type"));
    }

    let client_secret = RawApiKey::try_from(client_secret).map_err(|err| {
        tracing::warn!(?err, "Failed to parse OAuth client secret");

        // TODO: Change this to a bad request with invalid_client ?
        unauthorized()
    })?;

    if !client.is_secret_valid(&client_secret, &*ctx.secrets.signer) {
        // TODO: Change this to a bad request with invalid_client ?
        Err(client_error(StatusCode::UNAUTHORIZED, "invalid_client"))
    } else {
        Ok(())
    }
}

fn verify_login_attempt(
    attempt: &LoginAttempt,
    client_id: &Uuid,
    redirect_uri: &str,
    pkce_verifier: Option<&str>,
) -> Result<(), OAuthError> {
    if attempt.client_id != *client_id {
        Err(OAuthError {
            error: OAuthErrorCode::InvalidGrant,
            error_description: Some("Invalid client id".to_string()),
            error_uri: None,
        })
    } else if attempt.redirect_uri != redirect_uri {
        Err(OAuthError {
            error: OAuthErrorCode::InvalidGrant,
            error_description: Some("Invalid redirect uri".to_string()),
            error_uri: None,
        })
    } else if attempt.attempt_state != LoginAttemptState::RemoteAuthenticated {
        Err(OAuthError {
            error: OAuthErrorCode::InvalidGrant,
            error_description: Some("Grant is in an invalid state".to_string()),
            error_uri: None,
        })
    } else if attempt.expires_at.map(|t| t <= Utc::now()).unwrap_or(true) {
        Err(OAuthError {
            error: OAuthErrorCode::InvalidGrant,
            error_description: Some("Grant has expired".to_string()),
            error_uri: None,
        })
    } else {
        match (attempt.pkce_challenge.as_deref(), pkce_verifier) {
            (Some(_), None) => Err(OAuthError {
                error: OAuthErrorCode::InvalidRequest,
                error_description: Some("Missing pkce verifier".to_string()),
                error_uri: None,
            }),
            (Some(challenge), Some(verifier)) => {
                let mut hasher = Sha256::new();
                hasher.update(verifier);
                let hash = hasher.finalize();
                let computed_challenge = BASE64_URL_SAFE_NO_PAD.encode(hash);

                if challenge == computed_challenge {
                    Ok(())
                } else {
                    Err(OAuthError {
                        error: OAuthErrorCode::InvalidGrant,
                        error_description: Some("Invalid pkce verifier".to_string()),
                        error_uri: None,
                    })
                }
            }
            (None, _) => Ok(()),
        }
    }
}

async fn fetch_user_info(
    https: &Client<HttpsConnector<HttpConnector>, Body>,
    provider: &dyn OAuthProvider,
    attempt: &LoginAttempt,
) -> Result<UserInfo, HttpError> {
    // Exchange the stored authorization code with the remote provider for a remote access token
    let client = provider
        .as_client(&ClientType::Web)
        .map_err(to_internal_error)?;
    let mut request = client.exchange_code(AuthorizationCode::new(
        attempt
            .provider_authz_code
            .as_ref()
            .ok_or_else(|| {
                internal_error("Expected authorization code to exist due to attempt state")
            })?
            .to_string(),
    ));

    if let Some(pkce_verifier) = &attempt.provider_pkce_verifier {
        request = request.set_pkce_verifier(PkceCodeVerifier::new(pkce_verifier.to_string()))
    }

    let response = request
        .request_async(async_http_client)
        .await
        .map_err(to_internal_error)?;

    tracing::info!("Fetched access token from remote service");

    // Use the retrieved access token to fetch the user information from the remote API
    let info = provider
        .get_user_info(&https, response.access_token().secret())
        .await
        .map_err(LoginError::UserInfo)
        .tap_err(|err| tracing::error!(?err, "Failed to look up user information"))?;

    // Now that we are done with fetching user information from the remote API, we can revoke it
    client
        .revoke_token(response.access_token().into())
        .map_err(internal_error)?
        .request_async(async_http_client)
        .await
        .map_err(internal_error)?;

    Ok(info)
}

#[cfg(test)]
mod tests {
    use std::{
        net::{Ipv4Addr, SocketAddrV4},
        ops::Add,
        sync::{Arc, Mutex},
    };

    use chrono::{Duration, Utc};
    use dropshot::RequestInfo;
    use http::{
        header::{COOKIE, LOCATION, SET_COOKIE},
        HeaderValue, StatusCode,
    };
    use hyper::Body;
    use mockall::predicate::eq;
    use oauth2::PkceCodeChallenge;
    use rfd_model::{
        schema_ext::LoginAttemptState,
        storage::{MockLoginAttemptStore, MockOAuthClientStore},
        LoginAttempt, OAuthClient, OAuthClientRedirectUri, OAuthClientSecret,
    };
    use uuid::Uuid;

    use crate::{
        authn::key::RawApiKey,
        context::{
            tests::{mock_context, MockStorage},
            ApiContext,
        },
        endpoints::login::oauth::{
            code::{
                verify_csrf, verify_login_attempt, OAuthAuthzCodeReturnQuery, OAuthError,
                OAuthErrorCode, LOGIN_ATTEMPT_COOKIE,
            },
            OAuthProviderName,
        },
    };

    use super::{
        authorize_exchange, authz_code_callback_op, get_oauth_client, oauth_redirect_response,
    };

    async fn mock_client() -> (ApiContext, OAuthClient, String) {
        let ctx = mock_context(MockStorage::new()).await;
        let client_id = Uuid::new_v4();
        let key = RawApiKey::generate::<8>(&Uuid::new_v4())
            .sign(&*ctx.secrets.signer)
            .await
            .unwrap();
        let secret_signature = key.signature().to_string();
        let client_secret = key.key();
        let redirect_uri = "callback-destination";

        (
            ctx,
            OAuthClient {
                id: client_id,
                secrets: vec![OAuthClientSecret {
                    id: Uuid::new_v4(),
                    oauth_client_id: client_id,
                    secret_signature,
                    created_at: Utc::now(),
                    deleted_at: None,
                }],
                redirect_uris: vec![OAuthClientRedirectUri {
                    id: Uuid::new_v4(),
                    oauth_client_id: client_id,
                    redirect_uri: redirect_uri.to_string(),
                    created_at: Utc::now(),
                    deleted_at: None,
                }],
                created_at: Utc::now(),
                deleted_at: None,
            },
            client_secret,
        )
    }

    #[tokio::test]
    async fn test_oauth_client_lookup_checks_redirect_uri() {
        let client_id = Uuid::new_v4();
        let client = OAuthClient {
            id: client_id,
            secrets: vec![],
            redirect_uris: vec![OAuthClientRedirectUri {
                id: Uuid::new_v4(),
                oauth_client_id: client_id,
                redirect_uri: "https://test.oxeng.dev/callback".to_string(),
                created_at: Utc::now(),
                deleted_at: None,
            }],
            created_at: Utc::now(),
            deleted_at: None,
        };

        let mut client_store = MockOAuthClientStore::new();
        client_store
            .expect_get()
            .with(eq(client_id), eq(false))
            .returning(move |_, _| Ok(Some(client.clone())));

        let mut storage = MockStorage::new();
        storage.oauth_client_store = Some(Arc::new(client_store));
        let ctx = mock_context(storage).await;

        let failure =
            get_oauth_client(&ctx, &client_id, "https://not-test.oxeng.dev/callback").await;
        assert_eq!(StatusCode::UNAUTHORIZED, failure.unwrap_err().status_code);

        let success = get_oauth_client(&ctx, &client_id, "https://test.oxeng.dev/callback").await;
        assert_eq!(client_id, success.unwrap().id);
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

    #[tokio::test]
    async fn test_csrf_check() {
        let id = Uuid::new_v4();

        let mut rq = hyper::Request::new(Body::empty());
        rq.headers_mut().insert(
            COOKIE,
            HeaderValue::from_str(&format!("{}={}", LOGIN_ATTEMPT_COOKIE, id)).unwrap(),
        );
        let with_valid_cookie = RequestInfo::new(
            &rq,
            std::net::SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 8888)),
        );
        let query = OAuthAuthzCodeReturnQuery {
            state: Some(id.to_string()),
            code: None,
            error: None,
        };
        assert_eq!(id, verify_csrf(&with_valid_cookie, &query).unwrap());

        let query = OAuthAuthzCodeReturnQuery {
            state: None,
            code: None,
            error: None,
        };
        assert_eq!(
            StatusCode::UNAUTHORIZED,
            verify_csrf(&with_valid_cookie, &query)
                .unwrap_err()
                .status_code
        );

        let mut rq = hyper::Request::new(Body::empty());
        rq.headers_mut().insert(
            COOKIE,
            HeaderValue::from_str(&format!("{}={}", LOGIN_ATTEMPT_COOKIE, Uuid::new_v4())).unwrap(),
        );
        let with_invalid_cookie = RequestInfo::new(
            &rq,
            std::net::SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 8888)),
        );
        let query = OAuthAuthzCodeReturnQuery {
            state: Some(id.to_string()),
            code: None,
            error: None,
        };
        assert_eq!(
            StatusCode::UNAUTHORIZED,
            verify_csrf(&with_invalid_cookie, &query)
                .unwrap_err()
                .status_code
        );

        let rq = hyper::Request::new(Body::empty());
        let with_missing_cookie = RequestInfo::new(
            &rq,
            std::net::SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 8888)),
        );
        let query = OAuthAuthzCodeReturnQuery {
            state: Some(id.to_string()),
            code: None,
            error: None,
        };
        assert_eq!(
            StatusCode::UNAUTHORIZED,
            verify_csrf(&with_missing_cookie, &query)
                .unwrap_err()
                .status_code
        );
    }

    #[tokio::test]
    async fn test_callback_fails_when_not_in_new_state() {
        let invalid_states = [
            LoginAttemptState::Complete,
            LoginAttemptState::Failed,
            LoginAttemptState::RemoteAuthenticated,
        ];

        for state in invalid_states {
            let attempt_id = Uuid::new_v4();
            let attempt = LoginAttempt {
                id: attempt_id,
                attempt_state: state,
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

            let mut storage = MockStorage::new();
            let mut attempt_store = MockLoginAttemptStore::new();
            attempt_store
                .expect_get()
                .with(eq(attempt.id))
                .returning(move |_| Ok(Some(attempt.clone())));
            storage.login_attempt_store = Some(Arc::new(attempt_store));

            let ctx = mock_context(storage).await;
            let err =
                authz_code_callback_op(&ctx, &attempt_id, Some("remote-code".to_string()), None)
                    .await;

            assert_eq!(StatusCode::UNAUTHORIZED, err.unwrap_err().status_code);
        }
    }

    #[tokio::test]
    async fn test_callback_fails_when_error_is_passed() {
        let attempt_id = Uuid::new_v4();
        let attempt = LoginAttempt {
            id: attempt_id,
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

        let mut attempt_store = MockLoginAttemptStore::new();
        let original_attempt = attempt.clone();
        attempt_store
            .expect_get()
            .with(eq(attempt.id))
            .returning(move |_| Ok(Some(original_attempt.clone())));

        attempt_store
            .expect_upsert()
            .withf(|attempt| attempt.attempt_state == LoginAttemptState::Failed)
            .returning(move |arg| {
                let mut returned = attempt.clone();
                returned.attempt_state = arg.attempt_state;
                returned.authz_code = arg.authz_code;
                returned.error = arg.error;
                Ok(returned)
            });

        let mut storage = MockStorage::new();
        storage.login_attempt_store = Some(Arc::new(attempt_store));
        let ctx = mock_context(storage).await;

        let location = authz_code_callback_op(
            &ctx,
            &attempt_id,
            Some("remote-code".to_string()),
            Some("not_access_denied".to_string()),
        )
        .await
        .unwrap();

        assert_eq!(
            format!("https://test.oxeng.dev/callback?error=server_error&state=ox_state",),
            location
        );
    }

    #[tokio::test]
    async fn test_callback_forwards_access_denied() {
        let attempt_id = Uuid::new_v4();
        let attempt = LoginAttempt {
            id: attempt_id,
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

        let mut attempt_store = MockLoginAttemptStore::new();
        let original_attempt = attempt.clone();
        attempt_store
            .expect_get()
            .with(eq(attempt.id))
            .returning(move |_| Ok(Some(original_attempt.clone())));

        attempt_store
            .expect_upsert()
            .withf(|attempt| attempt.attempt_state == LoginAttemptState::Failed)
            .returning(move |arg| {
                let mut returned = attempt.clone();
                returned.attempt_state = arg.attempt_state;
                returned.authz_code = arg.authz_code;
                returned.error = arg.error;
                Ok(returned)
            });

        let mut storage = MockStorage::new();
        storage.login_attempt_store = Some(Arc::new(attempt_store));
        let ctx = mock_context(storage).await;

        let location = authz_code_callback_op(
            &ctx,
            &attempt_id,
            Some("remote-code".to_string()),
            Some("access_denied".to_string()),
        )
        .await
        .unwrap();

        assert_eq!(
            format!("https://test.oxeng.dev/callback?error=access_denied&state=ox_state",),
            location
        );
    }

    #[tokio::test]
    async fn test_handles_callback_with_code() {
        let attempt_id = Uuid::new_v4();
        let attempt = LoginAttempt {
            id: attempt_id,
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

        let mut attempt_store = MockLoginAttemptStore::new();
        let original_attempt = attempt.clone();
        attempt_store
            .expect_get()
            .with(eq(attempt.id))
            .returning(move |_| Ok(Some(original_attempt.clone())));

        let extracted_code = Arc::new(Mutex::new(None));
        let extractor = extracted_code.clone();
        attempt_store
            .expect_upsert()
            .withf(|attempt| attempt.attempt_state == LoginAttemptState::RemoteAuthenticated)
            .returning(move |arg| {
                let mut returned = attempt.clone();
                returned.attempt_state = arg.attempt_state;
                returned.authz_code = arg.authz_code;
                *extractor.lock().unwrap() = returned.authz_code.clone();
                Ok(returned)
            });

        let mut storage = MockStorage::new();
        storage.login_attempt_store = Some(Arc::new(attempt_store));
        let ctx = mock_context(storage).await;

        let location =
            authz_code_callback_op(&ctx, &attempt_id, Some("remote-code".to_string()), None)
                .await
                .unwrap();

        let lock = extracted_code.lock();
        assert_eq!(
            format!(
                "https://test.oxeng.dev/callback?code={}&state=ox_state",
                lock.unwrap().as_ref().unwrap()
            ),
            location
        );
    }

    #[tokio::test]
    async fn test_fails_callback_with_error() {}

    #[tokio::test]
    async fn test_exchange_checks_client_id_and_redirect() {
        let (mut ctx, client, client_secret) = mock_client().await;
        let client_id = client.id;
        let redirect_uri = client.redirect_uris[0].redirect_uri.clone();
        let wrong_client_id = Uuid::new_v4();

        let mut client_store = MockOAuthClientStore::new();
        client_store
            .expect_get()
            .with(eq(wrong_client_id), eq(false))
            .returning(move |_, _| Ok(None));
        client_store
            .expect_get()
            .with(eq(client_id), eq(false))
            .returning(move |_, _| Ok(Some(client.clone())));

        let mut storage = MockStorage::new();
        storage.oauth_client_store = Some(Arc::new(client_store));

        ctx.set_storage(Arc::new(storage));

        assert_eq!(
            StatusCode::UNAUTHORIZED,
            authorize_exchange(
                &ctx,
                "authorization_code",
                &wrong_client_id,
                &client_secret,
                &redirect_uri,
            )
            .await
            .unwrap_err()
            .status_code
        );

        assert_eq!(
            StatusCode::UNAUTHORIZED,
            authorize_exchange(
                &ctx,
                "authorization_code",
                &client_id,
                &client_secret,
                "wrong-callback-destination",
            )
            .await
            .unwrap_err()
            .status_code
        );

        assert_eq!(
            (),
            authorize_exchange(
                &ctx,
                "authorization_code",
                &client_id,
                &client_secret,
                &redirect_uri,
            )
            .await
            .unwrap()
        );
    }

    #[tokio::test]
    async fn test_exchange_checks_grant_type() {
        let (mut ctx, client, client_secret) = mock_client().await;
        let client_id = client.id;
        let redirect_uri = client.redirect_uris[0].redirect_uri.clone();

        let mut client_store = MockOAuthClientStore::new();
        client_store
            .expect_get()
            .with(eq(client_id), eq(false))
            .returning(move |_, _| Ok(Some(client.clone())));

        let mut storage = MockStorage::new();
        storage.oauth_client_store = Some(Arc::new(client_store));

        ctx.set_storage(Arc::new(storage));

        assert_eq!(
            StatusCode::BAD_REQUEST,
            authorize_exchange(
                &ctx,
                "not_authorization_code",
                &client_id,
                &client_secret,
                &redirect_uri
            )
            .await
            .unwrap_err()
            .status_code
        );

        assert_eq!(
            (),
            authorize_exchange(
                &ctx,
                "authorization_code",
                &client_id,
                &client_secret,
                &redirect_uri
            )
            .await
            .unwrap()
        );
    }

    #[tokio::test]
    async fn test_exchange_checks_for_valid_secret() {
        let (mut ctx, client, client_secret) = mock_client().await;
        let client_id = client.id;
        let redirect_uri = client.redirect_uris[0].redirect_uri.clone();

        let mut client_store = MockOAuthClientStore::new();
        client_store
            .expect_get()
            .with(eq(client_id), eq(false))
            .returning(move |_, _| Ok(Some(client.clone())));

        let mut storage = MockStorage::new();
        storage.oauth_client_store = Some(Arc::new(client_store));

        ctx.set_storage(Arc::new(storage));

        let invalid_secret = RawApiKey::generate::<8>(&Uuid::new_v4())
            .sign(&*ctx.secrets.signer)
            .await
            .unwrap()
            .signature()
            .to_string();

        assert_eq!(
            StatusCode::UNAUTHORIZED,
            authorize_exchange(
                &ctx,
                "authorization_code",
                &client_id,
                "too-short",
                &redirect_uri
            )
            .await
            .unwrap_err()
            .status_code
        );

        assert_eq!(
            StatusCode::UNAUTHORIZED,
            authorize_exchange(
                &ctx,
                "authorization_code",
                &client_id,
                &invalid_secret,
                &redirect_uri
            )
            .await
            .unwrap_err()
            .status_code
        );

        assert_eq!(
            (),
            authorize_exchange(
                &ctx,
                "authorization_code",
                &client_id,
                &client_secret,
                &redirect_uri
            )
            .await
            .unwrap()
        );
    }

    #[tokio::test]
    async fn test_login_attempt_verification() {
        let (challenge, verifier) = PkceCodeChallenge::new_random_sha256();
        let attempt = LoginAttempt {
            id: Uuid::new_v4(),
            attempt_state: LoginAttemptState::RemoteAuthenticated,
            client_id: Uuid::new_v4(),
            redirect_uri: "https://test.oxeng.dev/callback".to_string(),
            state: Some("ox_state".to_string()),
            pkce_challenge: Some(challenge.as_str().to_string()),
            pkce_challenge_method: Some("S256".to_string()),
            authz_code: None,
            expires_at: Some(Utc::now().add(Duration::seconds(60))),
            error: None,
            provider: "google".to_string(),
            provider_pkce_verifier: Some("rfd_verifier".to_string()),
            provider_authz_code: None,
            provider_error: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let bad_client_id = LoginAttempt {
            client_id: Uuid::new_v4(),
            ..attempt.clone()
        };

        assert_eq!(
            OAuthError {
                error: OAuthErrorCode::InvalidGrant,
                error_description: Some("Invalid client id".to_string()),
                error_uri: None,
            },
            verify_login_attempt(
                &bad_client_id,
                &attempt.client_id,
                &attempt.redirect_uri,
                Some(verifier.secret().as_str()),
            )
            .unwrap_err()
        );

        let bad_redirect_uri = LoginAttempt {
            redirect_uri: "https://bad.oxeng.dev/callback".to_string(),
            ..attempt.clone()
        };

        assert_eq!(
            OAuthError {
                error: OAuthErrorCode::InvalidGrant,
                error_description: Some("Invalid redirect uri".to_string()),
                error_uri: None,
            },
            verify_login_attempt(
                &bad_redirect_uri,
                &attempt.client_id,
                &attempt.redirect_uri,
                Some(verifier.secret().as_str()),
            )
            .unwrap_err()
        );

        let unconfirmed_state = LoginAttempt {
            attempt_state: LoginAttemptState::New,
            ..attempt.clone()
        };

        assert_eq!(
            OAuthError {
                error: OAuthErrorCode::InvalidGrant,
                error_description: Some("Grant is in an invalid state".to_string()),
                error_uri: None,
            },
            verify_login_attempt(
                &unconfirmed_state,
                &attempt.client_id,
                &attempt.redirect_uri,
                Some(verifier.secret().as_str()),
            )
            .unwrap_err()
        );

        let already_used_state = LoginAttempt {
            attempt_state: LoginAttemptState::Complete,
            ..attempt.clone()
        };

        assert_eq!(
            OAuthError {
                error: OAuthErrorCode::InvalidGrant,
                error_description: Some("Grant is in an invalid state".to_string()),
                error_uri: None,
            },
            verify_login_attempt(
                &already_used_state,
                &attempt.client_id,
                &attempt.redirect_uri,
                Some(verifier.secret().as_str()),
            )
            .unwrap_err()
        );

        let failed_state = LoginAttempt {
            attempt_state: LoginAttemptState::Failed,
            ..attempt.clone()
        };

        assert_eq!(
            OAuthError {
                error: OAuthErrorCode::InvalidGrant,
                error_description: Some("Grant is in an invalid state".to_string()),
                error_uri: None,
            },
            verify_login_attempt(
                &failed_state,
                &attempt.client_id,
                &attempt.redirect_uri,
                Some(verifier.secret().as_str()),
            )
            .unwrap_err()
        );

        let expired = LoginAttempt {
            expires_at: Some(Utc::now()),
            ..attempt.clone()
        };

        assert_eq!(
            OAuthError {
                error: OAuthErrorCode::InvalidGrant,
                error_description: Some("Grant has expired".to_string()),
                error_uri: None,
            },
            verify_login_attempt(
                &expired,
                &attempt.client_id,
                &attempt.redirect_uri,
                Some(verifier.secret().as_str()),
            )
            .unwrap_err()
        );

        let missing_pkce = LoginAttempt { ..attempt.clone() };

        assert_eq!(
            OAuthError {
                error: OAuthErrorCode::InvalidRequest,
                error_description: Some("Missing pkce verifier".to_string()),
                error_uri: None,
            },
            verify_login_attempt(
                &missing_pkce,
                &attempt.client_id,
                &attempt.redirect_uri,
                None,
            )
            .unwrap_err()
        );

        let invalid_pkce = LoginAttempt {
            pkce_challenge: Some("no-the-correct-value".to_string()),
            ..attempt.clone()
        };

        assert_eq!(
            OAuthError {
                error: OAuthErrorCode::InvalidGrant,
                error_description: Some("Invalid pkce verifier".to_string()),
                error_uri: None,
            },
            verify_login_attempt(
                &invalid_pkce,
                &attempt.client_id,
                &attempt.redirect_uri,
                Some(verifier.secret().as_str()),
            )
            .unwrap_err()
        );

        assert_eq!(
            (),
            verify_login_attempt(
                &attempt,
                &attempt.client_id,
                &attempt.redirect_uri,
                Some(verifier.secret().as_str()),
            )
            .unwrap()
        );
    }
}
