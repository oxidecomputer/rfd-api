// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use dropshot::HttpError;
use octorust::ClientError as OctorustError;
use reqwest::Error as ReqwestError;
use rfd_github::GitHubError;
use thiserror::Error;
use v_api::response::{conflict, forbidden, internal_error, not_found, ResourceError};
use v_model::storage::StoreError;

use rfd_secret::SecretResolutionError;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Failed to construct HTTP client")]
    ClientConstruction(ReqwestError),
    #[error("GitHub communication error")]
    GitHub(#[from] GitHubError),
    #[error("Invalid GitHub private key")]
    InvalidGitHubPrivateKey(#[from] rsa::pkcs1::Error),
    #[error("A template for new RFDs must be defined")]
    MissingNewRfdTemplate,
    #[error("At least one JWT signing key must be configured")]
    NoConfiguredJwtKeys,
    #[error("Failed to construct GitHub client")]
    Octorust(#[from] OctorustError),
    #[error("Failed to resolve secret")]
    SecretResolution(#[from] SecretResolutionError),
}

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Conflict with an existing resource")]
    Conflict,
    #[error("Caller does not have the required permissions")]
    Forbidden,
    #[error("Resource could not be found")]
    NotFound,
    #[error("Internal storage failed {0}")]
    Storage(#[from] StoreError),
}

impl From<ApiError> for HttpError {
    fn from(error: ApiError) -> Self {
        match error {
            ApiError::Conflict => conflict(),
            ApiError::Forbidden => forbidden(),
            ApiError::NotFound => not_found(""),
            ApiError::Storage(_) => internal_error("Internal storage failed"),
        }
    }
}

impl From<ResourceError<StoreError>> for ApiError {
    fn from(value: ResourceError<StoreError>) -> Self {
        match value {
            ResourceError::Conflict => ApiError::Conflict,
            ResourceError::DoesNotExist => ApiError::NotFound,
            ResourceError::InternalError(err) => ApiError::Storage(err),
            ResourceError::Restricted => ApiError::Forbidden,
        }
    }
}
