use std::fmt::Debug;

use dropshot::{HttpError, RequestContext, SharedExtractor};
use dropshot_authorization_header::bearer::BearerAuth;
use thiserror::Error;

use crate::{context::ApiContext, util::response::unauthorized};

use self::{jwt::Jwt, key::ApiKey};

pub mod jwt;
pub mod key;

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("Failed to extract token")]
    FailedToExtract,
    #[error("Request does not have a token")]
    NoToken,
}

// A token that provides authentication and optionally (JWT) authorization claims
pub enum AuthToken {
    ApiKey(ApiKey),
    Jwt(Jwt),
}

impl Debug for AuthToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AuthToken").finish()
    }
}

impl AuthToken {
    // Extract an AuthToken from a Dropshot request
    pub async fn extract(rqctx: &RequestContext<ApiContext>) -> Result<AuthToken, AuthError> {
        // Ensure there is a bearer, without it there is nothing else to do
        let bearer = BearerAuth::from_request(rqctx).await.map_err(|err| {
            tracing::info!(?err, "Failed to extract bearer auth");
            AuthError::NoToken
        })?;

        // Check that the extracted token actually contains a value
        let token = bearer.consume().ok_or(AuthError::NoToken)?;

        let ctx = rqctx.context();

        // Attempt to decode an API key from the token. If that fails then attempt to verify the
        // token as a JWT
        match ApiKey::try_from(token.as_str()) {
            Ok(api_key) => Ok(AuthToken::ApiKey(api_key)),
            Err(err) => {
                tracing::trace!(?err, "Bearer token is not an api key");

                Jwt::new(ctx, &token)
                    .await
                    .map(AuthToken::Jwt)
                    .map_err(|err| {
                        tracing::trace!(?err, "Bearer token is not a valid JWT");
                        AuthError::FailedToExtract
                    })
            }
        }
    }
}

impl From<AuthError> for HttpError {
    fn from(err: AuthError) -> Self {
        tracing::trace!(?err, "Failed to extract auth token");
        unauthorized()
    }
}
