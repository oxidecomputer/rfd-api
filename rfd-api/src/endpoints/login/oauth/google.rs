use std::fmt;

use serde::{Deserialize, Serialize};

use crate::endpoints::login::{ExternalUserId, UserInfo, UserInfoError};

use super::{ExtractUserInfo, OAuthProvider, OAuthProviderName};

pub struct GoogleOAuthProvider {
    public: GooglePublicProvider,
    private: Option<GooglePrivateProvider>,
}

impl fmt::Debug for GoogleOAuthProvider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("GoogleOAuthProvider").finish()
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GooglePublicProvider {
    client_id: String,
}

pub struct GooglePrivateProvider {
    client_secret: String,
}

impl GoogleOAuthProvider {
    pub fn new(client_id: String, client_secret: String) -> Self {
        Self {
            public: GooglePublicProvider { client_id },
            private: Some(GooglePrivateProvider { client_secret }),
        }
    }
}

#[derive(Debug, Deserialize)]
struct GoogleUserInfo {
    sub: String,
    email: String,
    email_verified: bool,
}

impl ExtractUserInfo for GoogleOAuthProvider {
    fn extract_user_info(&self, data: &[u8]) -> Result<UserInfo, UserInfoError> {
        let remote_info: GoogleUserInfo = serde_json::from_slice(data)?;
        let verified_emails = if remote_info.email_verified {
            vec![remote_info.email]
        } else {
            vec![]
        };

        Ok(UserInfo {
            external_id: ExternalUserId::Google(remote_info.sub),
            verified_emails,
        })
    }
}

impl OAuthProvider for GoogleOAuthProvider {
    fn name(&self) -> OAuthProviderName {
        OAuthProviderName::Google
    }

    fn scopes(&self) -> Vec<&str> {
        vec![
            "https://www.googleapis.com/auth/userinfo.email",
            "openid"
        ]
    }

    fn client_id(&self) -> &str {
        &self.public.client_id
    }

    fn client_secret(&self) -> Option<&str> {
        self.private
            .as_ref()
            .map(|private| private.client_secret.as_str())
    }

    fn user_info_endpoint(&self) -> &str {
        "https://openidconnect.googleapis.com/v1/userinfo"
    }

    fn device_code_endpoint(&self) -> &str {
        "https://oauth2.googleapis.com/device/code"
    }

    fn auth_url_endpoint(&self) -> &str {
        "https://accounts.google.com/o/oauth2/auth"
    }

    fn token_exchange_content_type(&self) -> &str {
        "application/x-www-form-urlencoded"
    }

    fn token_exchange_endpoint(&self) -> &str {
        "https://oauth2.googleapis.com/token"
    }
}
