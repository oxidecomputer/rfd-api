use std::fmt::Debug;

use dropshot::{HttpError, RequestContext, SharedExtractor};
use dropshot_authorization_header::bearer::BearerAuth;
use thiserror::Error;

use crate::{authn::key::RawApiKey, context::ApiContext, util::response::unauthorized};

use self::{jwt::Jwt, key::EncryptedApiKey};

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
    ApiKey(EncryptedApiKey),
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
        let jwt = Jwt::new(ctx, &token).await;

        match jwt {
            Ok(token) => Ok(AuthToken::Jwt(token)),
            Err(err) => {
                tracing::debug!(?err, "Token is not a JWT, falling back to API key");

                Ok(AuthToken::ApiKey(
                    RawApiKey::new(token)
                        .encrypt(&*ctx.secrets.encryptor)
                        .await
                        .map_err(|err| {
                            tracing::error!(?err, "Failed to encrypt authn token");
                            AuthError::FailedToExtract
                        })?,
                ))
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
