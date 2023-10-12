use std::fmt;

use hyper::body::Bytes;
use serde::{Deserialize, Serialize};

use crate::endpoints::login::{ExternalUserId, UserInfo, UserInfoError};

use super::{ClientType, ExtractUserInfo, OAuthProvider, OAuthProviderName};

pub struct GitHubOAuthProvider {
    public: GitHubPublicProvider,
    private: Option<GitHubPrivateProvider>,
}

impl fmt::Debug for GitHubOAuthProvider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("GitHubOAuthProvider").finish()
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GitHubPublicProvider {
    client_id: String,
}

pub struct GitHubPrivateProvider {
    client_secret: String,
}

impl GitHubOAuthProvider {
    pub fn new(client_id: String, client_secret: String) -> Self {
        Self {
            public: GitHubPublicProvider { client_id },
            private: Some(GitHubPrivateProvider { client_secret }),
        }
    }
}

#[derive(Debug, Deserialize)]
struct GitHubUser {
    id: String,
}

#[derive(Debug, Deserialize)]
struct GitHubUserEmails {
    email: String,
    verified: bool,
    // TODO: Add ability to mask non-visible emails?
    _visibility: Option<String>,
}

impl ExtractUserInfo for GitHubOAuthProvider {
    // There should always be as many entries in the data list as there are endpoints. This should
    // be changed in the future to be a static check
    fn extract_user_info(&self, data: &[Bytes]) -> Result<UserInfo, UserInfoError> {
        let user: GitHubUser = serde_json::from_slice(&data[1])?;

        let remote_emails: Vec<GitHubUserEmails> = serde_json::from_slice(&data[1])?;
        let verified_emails = remote_emails
            .into_iter()
            .filter(|email| email.verified)
            .map(|e| e.email)
            .collect::<Vec<_>>();

        Ok(UserInfo {
            external_id: ExternalUserId::GitHub(user.id),
            verified_emails,
        })
    }
}

impl OAuthProvider for GitHubOAuthProvider {
    fn name(&self) -> OAuthProviderName {
        OAuthProviderName::GitHub
    }

    fn scopes(&self) -> Vec<&str> {
        vec!["user:email"]
    }

    fn client_id(&self, _client_type: &ClientType) -> &str {
        &self.public.client_id
    }

    fn client_secret(&self, _client_type: &ClientType) -> Option<&str> {
        self.private
            .as_ref()
            .map(|private| private.client_secret.as_str())
    }

    fn user_info_endpoints(&self) -> Vec<&str> {
        vec![
            "https://api.github.com/user",
            "https://api.github.com/user/emails",
        ]
    }

    fn device_code_endpoint(&self) -> &str {
        "https://github.com/login/device/code"
    }

    fn auth_url_endpoint(&self) -> &str {
        "https://github.com/login/oauth/authorize"
    }

    fn token_exchange_content_type(&self) -> &str {
        "application/x-www-form-urlencoded"
    }

    fn token_exchange_endpoint(&self) -> &str {
        "https://github.com/login/oauth/access_token"
    }

    fn supports_pkce(&self) -> bool {
        true
    }
}
