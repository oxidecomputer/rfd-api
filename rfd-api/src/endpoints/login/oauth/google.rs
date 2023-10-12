use std::fmt;

use hyper::body::Bytes;
use serde::Deserialize;

use crate::endpoints::login::{ExternalUserId, UserInfo, UserInfoError};

use super::{
    ClientType, ExtractUserInfo, OAuthPrivateCredentials, OAuthProvider, OAuthProviderName,
    OAuthPublicCredentials,
};

pub struct GoogleOAuthProvider {
    device_public: OAuthPublicCredentials,
    device_private: Option<OAuthPrivateCredentials>,
    web_public: OAuthPublicCredentials,
    web_private: Option<OAuthPrivateCredentials>,
}

impl fmt::Debug for GoogleOAuthProvider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("GoogleOAuthProvider").finish()
    }
}

impl GoogleOAuthProvider {
    pub fn new(
        device_client_id: String,
        device_client_secret: String,
        web_client_id: String,
        web_client_secret: String,
    ) -> Self {
        Self {
            device_public: OAuthPublicCredentials {
                client_id: device_client_id,
            },
            device_private: Some(OAuthPrivateCredentials {
                client_secret: device_client_secret,
            }),
            web_public: OAuthPublicCredentials {
                client_id: web_client_id,
            },
            web_private: Some(OAuthPrivateCredentials {
                client_secret: web_client_secret,
            }),
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
    // There should always be as many entries in the data list as there are endpoints. This should
    // be changed in the future to be a static check
    fn extract_user_info(&self, data: &[Bytes]) -> Result<UserInfo, UserInfoError> {
        let remote_info: GoogleUserInfo = serde_json::from_slice(&data[0])?;
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
        vec!["https://www.googleapis.com/auth/userinfo.email", "openid"]
    }

    fn client_id(&self, client_type: &ClientType) -> &str {
        match client_type {
            ClientType::Device => &self.device_public.client_id,
            ClientType::Web => &self.web_public.client_id,
        }
    }

    fn client_secret(&self, client_type: &ClientType) -> Option<&str> {
        match client_type {
            ClientType::Device => self
                .device_private
                .as_ref()
                .map(|private| private.client_secret.as_str()),
            ClientType::Web => self
                .web_private
                .as_ref()
                .map(|private| private.client_secret.as_str()),
        }
    }

    fn user_info_endpoints(&self) -> Vec<&str> {
        vec!["https://openidconnect.googleapis.com/v1/userinfo"]
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

    fn supports_pkce(&self) -> bool {
        true
    }
}
