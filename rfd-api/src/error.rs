use dropshot::HttpError;
use rfd_model::storage::StoreError;
use thiserror::Error;

use crate::{
    authn::{jwt::JwtSignerError, SigningKeyError},
    endpoints::login::{oauth::OAuthProviderError, LoginError},
    util::response::internal_error,
};

#[derive(Debug, Error)]
pub enum AppError {
    #[error("At least one JWT signing key must be configured")]
    NoConfiguredJwtKeys,
    #[error(transparent)]
    SignerError(#[from] SigningKeyError),
}

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("JWT credential failed: {0}")]
    Jwt(#[from] JwtSignerError),
    #[error("Invalid signing key: {0}")]
    Key(#[from] SigningKeyError),
    #[error(transparent)]
    Login(#[from] LoginError),
    #[error("Internal OAuth provider failed {0}")]
    OAuth(#[from] OAuthProviderError),
    #[error("Internal storage failed {0}")]
    Storage(#[from] StoreError),
}

impl From<ApiError> for HttpError {
    fn from(error: ApiError) -> Self {
        match error {
            ApiError::Jwt(_) => internal_error("JWT operation failed"),
            ApiError::Key(_) => internal_error("User credential signing failed"),
            ApiError::Login(inner) => inner.into(),
            ApiError::OAuth(_) => internal_error("OAuth provider failed"),
            ApiError::Storage(_) => internal_error("Internal storage failed"),
        }
    }
}
