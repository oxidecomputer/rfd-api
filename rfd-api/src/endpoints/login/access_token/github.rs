use async_trait::async_trait;
use octorust::{auth::Credentials, Client as Octorust};

use crate::endpoints::login::{ExternalUserId, UserInfo};

use super::{AccessTokenError, AccessTokenProvider, AccessTokenProviderName};

// A GitHub token that has been acquired through a GitHub OAuth flow
pub struct GitHubAccessTokenIdentity {
    client: Octorust,
}

impl GitHubAccessTokenIdentity {
    pub fn new(token: String) -> Result<Self, AccessTokenError> {
        Ok(Self {
            client: Octorust::new("rfd-api", Credentials::Token(token)).map_err(|err| {
                tracing::error!(?err, "Failed to construct GitHub client");
                AccessTokenError::FailedToConstructClient
            })?,
        })
    }

    pub async fn get_user_id(&self) -> Result<String, AccessTokenError> {
        let user = self
            .client
            .users()
            .get_authenticated()
            .await
            .map_err(|err| {
                tracing::error!(?err, "Failed to retrieve authenticated user");
                AccessTokenError::FailedToGetUserInfo
            })?;

        let user_info = user.body.public_user().ok_or_else(|| {
            tracing::error!("GitHub user record does not have public information");
            AccessTokenError::FailedToGetUserInfo
        })?;

        Ok(user_info.id.to_string())
    }
}

#[async_trait]
impl AccessTokenProvider for GitHubAccessTokenIdentity {
    fn name(&self) -> AccessTokenProviderName {
        AccessTokenProviderName::GitHub
    }

    // Use the provided access token to get the user information of the external user it represents
    async fn get_user_info(&self) -> Result<UserInfo, AccessTokenError> {
        let user_id = self.get_user_id().await?;

        // GitHub accounts have an arbitrary number of emails associated with them
        let emails = self
            .client
            .users()
            .list_emails_for_authenticated(100, 1)
            .await
            .map_err(|err| {
                tracing::error!(?err, "Failed to get authenticated user from access token");
                AccessTokenError::FailedToGetUserInfo
            })?;

        let verified_emails = emails
            .body
            .into_iter()
            .filter_map(|email| {
                if email.verified {
                    Some(email.email)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        Ok(UserInfo {
            external_id: ExternalUserId::GitHub(user_id),
            verified_emails,
        })
    }
}
