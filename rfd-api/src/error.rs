use dropshot::HttpError;
use rfd_model::storage::StoreError;
use thiserror::Error;

use crate::{
    authn::jwt::SignerError,
    endpoints::login::{oauth::OAuthProviderError, LoginError},
    util::response::internal_error,
};

#[derive(Debug, Error)]
pub enum AppError {
    #[error("At least one JWT signing key must be configured")]
    NoConfiguredJwtKeys,
    #[error(transparent)]
    SignerError(#[from] SignerError),
}

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Failed to sign access token")]
    Jwt(#[from] SignerError),
    #[error(transparent)]
    Login(LoginError),
    #[error("Internal OAuth provider failed {0}")]
    OAuth(#[from] OAuthProviderError),
    #[error("Internal storage failed {0}")]
    Storage(#[from] StoreError),
}

impl From<ApiError> for HttpError {
    fn from(error: ApiError) -> Self {
        match error {
            ApiError::Jwt(_) => internal_error("Access token signing failed"),
            ApiError::Login(inner) => inner.into(),
            ApiError::OAuth(_) => internal_error("OAuth provider failed"),
            ApiError::Storage(_) => internal_error("Internal storage failed"),
        }
    }
}
