use async_trait::async_trait;
use dropshot::HttpError;
use hyper::{client::connect::Connect, Client};
use schemars::JsonSchema;
use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};
use thiserror::Error;

use crate::{
    permissions::ApiPermission,
    util::response::{bad_request, internal_error},
};

// use self::access_token::AccessTokenError;

// pub mod access_token;
// pub mod jwt;
pub mod oauth;

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub enum LoginPermissions {
    All,
    Specific(Vec<ApiPermission>),
}

#[derive(Debug, Error)]
pub enum LoginError {
    // #[error(transparent)]
    // AccessTokenError(#[from] AccessTokenError),
    #[error("Requested token lifetime exceeds maximum configuration duration")]
    ExcessTokenExpiration,
    #[error("Failed to parse access token {0}")]
    FailedToParseToken(#[from] serde_json::Error),
    #[error("Unsupported provider specified")]
    InvalidProvider,
    #[error("Failed to fetch user info {0}")]
    UserInfo(#[from] UserInfoError),
}

impl From<LoginError> for HttpError {
    fn from(err: LoginError) -> Self {
        match err {
            // LoginError::AccessTokenError(_) => {
            //     internal_error("Failed to construct internal client to authenticate")
            // }
            LoginError::ExcessTokenExpiration => {
                let mut err =
                    bad_request("Requested expiration exceeds maximum allowed token duration");
                err.error_code = Some("INVALID_TOKEN_EXPIRATION".to_string());

                err
            }
            LoginError::FailedToParseToken(_) => internal_error("Failed to get access token"),
            LoginError::InvalidProvider => bad_request("Unsupported provider"),
            LoginError::UserInfo(_) => internal_error("Failed to fetch user info"),
        }
    }
}

#[derive(Debug)]
pub enum ExternalUserId {
    GitHub(String),
    Google(String),
}

impl ExternalUserId {
    pub fn id(&self) -> &str {
        match self {
            Self::GitHub(id) => id,
            Self::Google(id) => id,
        }
    }

    pub fn provider(&self) -> &str {
        match self {
            Self::GitHub(_) => "github",
            Self::Google(_) => "google",
        }
    }
}

#[derive(Debug, Error)]
pub enum ExternalUserIdDeserializeError {
    #[error("External identifier is empty")]
    Empty,
    #[error("External identifier did not have a valid prefix")]
    InvalidPrefix,
}

impl Serialize for ExternalUserId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            ExternalUserId::GitHub(id) => serializer.serialize_str(&format!("github-{}", id)),
            ExternalUserId::Google(id) => serializer.serialize_str(&format!("google-{}", id)),
        }
    }
}

impl<'de> Deserialize<'de> for ExternalUserId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ExternalId;

        impl<'de> Visitor<'de> for ExternalId {
            type Value = ExternalUserId;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("string")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                if let Some(("", id)) = value.split_once("github-") {
                    if !id.is_empty() {
                        Ok(ExternalUserId::GitHub(id.to_string()))
                    } else {
                        Err(de::Error::custom(ExternalUserIdDeserializeError::Empty))
                    }
                } else if let Some(("", id)) = value.split_once("google-") {
                    if !id.is_empty() {
                        Ok(ExternalUserId::Google(id.to_string()))
                    } else {
                        Err(de::Error::custom(ExternalUserIdDeserializeError::Empty))
                    }
                } else {
                    Err(de::Error::custom(
                        ExternalUserIdDeserializeError::InvalidPrefix,
                    ))
                }
            }
        }

        deserializer.deserialize_any(ExternalId)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserInfo {
    pub external_id: ExternalUserId,
    pub verified_emails: Vec<String>,
}

#[derive(Debug, Error)]
pub enum UserInfoError {
    #[error("Failed to create user info request {0}")]
    Http(#[from] http::Error),
    #[error("Failed to send user info request {0}")]
    Hyper(#[from] hyper::Error),
    #[error("Failed to deserialize user info {0}")]
    Deserialize(#[from] serde_json::Error),
}

#[async_trait]
pub trait UserInfoProvider {
    async fn get_user_info<C>(
        &self,
        client: &Client<C>,
        token: &str,
    ) -> Result<UserInfo, UserInfoError>
    where
        C: Connect + Clone + Send + Sync + 'static;
}
