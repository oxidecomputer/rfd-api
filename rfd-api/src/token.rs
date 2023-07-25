use argon2::{
    password_hash::{
        rand_core::{OsRng, RngCore},
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString,
    },
    Argon2, ParamsBuilder, Version,
};
use async_trait::async_trait;
use dropshot::{
    ApiEndpointBodyContentType, ExtensionMode, Extractor, ExtractorMetadata, HttpError,
    RequestContext, ServerContext,
};
use dropshot_authorization_header::bearer::BearerAuth;
use http::StatusCode;
use std::{sync::Arc, time::Instant};
use uuid::Uuid;

use crate::util::response::client_error;

// Parameters and algorithm are based on recommendations in:
// https://soatok.blog/2022/12/29/what-we-do-in-the-etc-shadow-cryptography-with-passwords/
// https://cheatsheetseries.owasp.org/cheatsheets/Password_Storage_Cheat_Sheet.html
fn argon2() -> Argon2<'static> {
    // Given that our parameters are static, we should never fail to build an instance
    let mut params = ParamsBuilder::new();
    params.m_cost(24 * 1024).unwrap();
    params.t_cost(6).unwrap();
    params.p_cost(1).unwrap();

    Argon2::new(
        argon2::Algorithm::Argon2id,
        Version::default(),
        params.params().unwrap(),
    )
}

pub struct Token {
    clear: String,
    hash: String,
}

impl Token {
    pub fn generate<const N: usize>(id: &Uuid) -> Self {
        // Generate random data to extend the token id with
        let mut token_raw = [0; N];
        OsRng.fill_bytes(&mut token_raw);

        // Append the random data to the token's id
        let mut to_encode = id.as_bytes().to_vec();
        to_encode.extend(token_raw);

        let clear = base64::encode(to_encode);
        let salt = SaltString::generate(&mut OsRng);

        // Given that our Argon2 parameters are static, and our passwords are always the same size,
        // we should not be able to actually hit an error case here
        let hash = argon2()
            .hash_password(clear.as_bytes(), &salt)
            .unwrap()
            .to_string();

        Self { clear, hash }
    }

    pub fn consume(self) -> (String, String) {
        (self.clear, self.hash)
    }
}

pub struct AuthnToken {
    id: Uuid,
    token: String,
}

impl AuthnToken {
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    pub fn verify(&self, hash: &str) -> bool {
        // If we somehow stored an invalid hash, immediately fail
        let start = Instant::now();

        let result = PasswordHash::new(hash)
            .ok()
            .and_then(|parsed_hash| {
                argon2()
                    .verify_password(self.token.as_bytes(), &parsed_hash)
                    .ok()
            })
            .is_some();

        let end = Instant::now();
        let duration = end - start;

        tracing::debug!(?duration, "Token verification measurement");

        result
    }
}

#[async_trait]
impl Extractor for AuthnToken {
    async fn from_request<Context: ServerContext>(
        rqctx: Arc<RequestContext<Context>>,
    ) -> Result<AuthnToken, HttpError> {
        let bearer: BearerAuth = Extractor::from_request(rqctx).await?;
        bearer.try_into().map_err(|_| {
            tracing::info!("Failed to extract bearer");

            let mut err = client_error(StatusCode::UNAUTHORIZED, "Failed to authenticate");
            err.internal_message = "Failed to decode token".to_string();
            err
        })
    }

    fn metadata(_body_content_type: ApiEndpointBodyContentType) -> ExtractorMetadata {
        ExtractorMetadata {
            parameters: vec![],
            extension_mode: ExtensionMode::None,
        }
    }
}

#[derive(Debug)]
pub struct FailedToParseToken {}

impl TryFrom<&BearerAuth> for AuthnToken {
    type Error = FailedToParseToken;
    fn try_from(bearer: &BearerAuth) -> Result<Self, Self::Error> {
        bearer
            .key()
            .and_then(|token| base64::decode(&token).ok().map(|decoded| (token, decoded)))
            .and_then(|(token, decoded)| {
                if let Some(id) = Uuid::from_slice(&decoded[0..16]).ok() {
                    Some(AuthnToken { id, token })
                } else {
                    tracing::info!("Failed to decode token");
                    None
                }
            })
            .ok_or(FailedToParseToken {})
    }
}

#[cfg(test)]
mod tests {
    use super::{AuthnToken, Token};
    use dropshot_authorization_header::bearer::BearerAuth;
    use uuid::Uuid;

    #[test]
    fn test_decodes_token() {
        let id = Uuid::new_v4();
        let (token, hash) = Token::generate::<24>(&id).consume();

        let bearer = BearerAuth::new(token);

        let authn: AuthnToken = bearer.try_into().unwrap();

        let verified = authn.verify(&hash);

        assert!(verified);
    }

    #[test]
    fn test_fails_to_decode_invalid_token() {
        let id = Uuid::new_v4();
        let (token1, _hash1) = Token::generate::<24>(&id).consume();
        let (_token2, hash2) = Token::generate::<24>(&id).consume();

        let bearer = BearerAuth::new(token1);

        let authn: AuthnToken = bearer.try_into().unwrap();

        let verified = authn.verify(&hash2);

        assert!(!verified);
    }
}
