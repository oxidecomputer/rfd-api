use async_trait::async_trait;
use jsonwebtoken::{
    decode,
    jwk::{AlgorithmParameters, JwkSet},
    Algorithm, Validation,
};
use reqwest::Client;
use serde::Deserialize;
use tap::TapFallible;

use crate::endpoints::login::{ExternalUserId, UserInfo};

use super::{JwksProvider, JwtError, JwtOpenIdConfiguration, JwtProvider, JwtProviderName};

pub struct GoogleOidcJwks {
    client: Client,
    issuer: String,
    uri: String,
}

impl GoogleOidcJwks {
    pub fn new(issuer: String, uri: String) -> Self {
        Self {
            client: Client::new(),
            issuer,
            uri,
        }
    }
}

#[async_trait]
impl JwksProvider for GoogleOidcJwks {
    fn issuer(&self) -> &str {
        &self.issuer
    }

    async fn keys(&self) -> Result<JwkSet, JwtError> {
        let config: JwtOpenIdConfiguration = self.client.get(&self.uri)
            .send()
            .await
            .map_err(|err| {
                tracing::error!(uri = ?self.uri, ?err, "Failed to retrieve OpenId config from remote");
                JwtError::InvalidJWT
            })?
            .json()
            .await
            .map_err(|err| {
                tracing::error!(uri = ?self.uri, ?err, "Failed to deserialize OpenId config response");
                JwtError::InvalidJWT
            })?;

        if self.issuer == config.issuer {
            self.client
                .get(config.jwks_uri)
                .send()
                .await
                .map_err(|err| {
                    tracing::error!(uri = ?self.uri, ?err, "Failed to retrieve JWKS from remote");
                    JwtError::InvalidJWT
                })?
                .json()
                .await
                .map_err(|err| {
                    tracing::error!(uri = ?self.uri, ?err, "Failed to deserialize JWKS response");
                    JwtError::InvalidJWT
                })
        } else {
            Err(JwtError::IssuerMismatch)
        }
    }
}

// A Google token that has been acquired through a Google OAuth flow
pub struct GoogleOidcIdentity<'a> {
    token: String,
    jwks_client: &'a Box<dyn JwksProvider>,
}

#[derive(Debug, Deserialize)]
struct OidcClaims {
    iss: String,
    sub: String,
    email: String,
    email_verified: bool,
}

impl<'a> GoogleOidcIdentity<'a> {
    pub fn new(token: String, jwks_client: &'a Box<dyn JwksProvider>) -> Self {
        Self { token, jwks_client }
    }

    async fn verify(&self) -> Result<OidcClaims, JwtError> {
        let keys = self.jwks_client.keys().await?;
        let header = jsonwebtoken::decode_header(&self.token)
            .tap_err(|err| tracing::info!(?err, "Failed to decode JWT header"))?;

        let key = header
            .kid
            .as_ref()
            .and_then(|kid| keys.find(kid))
            .ok_or(JwtError::InvalidJWT)?;

        match &key.algorithm {
            AlgorithmParameters::RSA(_) => {
                let decoding_key = jsonwebtoken::DecodingKey::from_jwk(&key).map_err(|err| {
                    tracing::info!(?err, "Failed to create decoding key");
                    JwtError::InvalidJWT
                })?;
                let validation = Validation::new(Algorithm::RS256);

                let claims = decode::<OidcClaims>(&self.token, &decoding_key, &validation)
                    .map_err(|err| {
                        tracing::info!(?err, "Failed to decode token");
                        JwtError::FailedToVerifyToken
                    })?;

                // Require that the issuer matches our configured issuer
                if claims.claims.iss == self.jwks_client.issuer() {
                    Ok(claims.claims)
                } else {
                    tracing::warn!(iss = ?claims.claims.iss, "Decoded key had an invalid issuer specified");
                    Err(JwtError::InvalidJWT)
                }
            }
            alg => {
                tracing::warn!(
                    ?alg,
                    "Attempted to authenticate with JWT signed by unsupported algorithm"
                );
                Err(JwtError::InvalidJWT)
            }
        }
    }
}

#[async_trait]
impl JwtProvider for GoogleOidcIdentity<'_> {
    fn name(&self) -> JwtProviderName {
        JwtProviderName::Google
    }

    // Use the provided access token to get the user information of the external user it represents
    async fn get_user_info(&self) -> Result<UserInfo, JwtError> {
        let claims = self.verify().await?;

        let user_id = claims.sub;

        let mut verified_emails = vec![];

        if claims.email_verified {
            verified_emails.push(claims.email);
        }

        Ok(UserInfo {
            external_id: ExternalUserId::Google(user_id),
            verified_emails,
        })
    }
}
