use argon2::{
    password_hash::{
        rand_core::{OsRng, RngCore},
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString,
    },
    Argon2, ParamsBuilder, Version,
};
use base64::{prelude::BASE64_URL_SAFE_NO_PAD, Engine};
use std::time::Instant;
use uuid::Uuid;

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

pub struct NewApiKey {
    clear: String,
    hash: String,
}

impl NewApiKey {
    // Generate a new API key along with its hashed form
    pub fn generate<const N: usize>(id: &Uuid) -> Self {
        // Generate random data to extend the token id with
        let mut token_raw = [0; N];
        OsRng.fill_bytes(&mut token_raw);

        // Append the random data to the token's id
        let mut to_encode = id.as_bytes().to_vec();
        to_encode.extend(token_raw);

        let clear = BASE64_URL_SAFE_NO_PAD.encode(to_encode);
        let salt = SaltString::generate(&mut OsRng);

        // Given that our Argon2 parameters are static, and our passwords are always the same size,
        // we should not be able to actually hit an error case here
        let hash = argon2()
            .hash_password(clear.as_bytes(), &salt)
            .unwrap()
            .to_string();

        Self { clear, hash }
    }

    // To get the token and hash out of an API key it must be consumed so that it can not be used
    // again
    pub fn consume(self) -> (String, String) {
        (self.clear, self.hash)
    }
}

pub struct ApiKey {
    id: Uuid,
    token: String,
}

impl ApiKey {
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

        tracing::debug!(?duration, "Api key verification measurement");

        result
    }
}

#[derive(Debug)]
pub struct FailedToParseToken {}

impl TryFrom<&str> for ApiKey {
    type Error = FailedToParseToken;
    fn try_from(token: &str) -> Result<Self, Self::Error> {
        BASE64_URL_SAFE_NO_PAD
            .decode(&token)
            .ok()
            .map(|decoded| (token, decoded))
            .and_then(|(token, decoded)| {
                tracing::trace!("Decoded token {:?} {:?}", token, decoded);

                if let Some(id) = Uuid::from_slice(&decoded[0..16]).ok() {
                    Some(ApiKey {
                        id,
                        token: token.to_string(),
                    })
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
    use super::{ApiKey, NewApiKey};
    use dropshot_authorization_header::bearer::BearerAuth;
    use uuid::Uuid;

    #[test]
    fn test_decodes_token() {
        let id = Uuid::new_v4();
        let (token, hash) = NewApiKey::generate::<24>(&id).consume();

        let bearer = BearerAuth::new(token);

        let authn: ApiKey = bearer.key().unwrap().try_into().unwrap();

        let verified = authn.verify(&hash);

        assert!(verified);
    }

    #[test]
    fn test_fails_to_decode_invalid_token() {
        let id = Uuid::new_v4();
        let (token1, _hash1) = NewApiKey::generate::<24>(&id).consume();
        let (_token2, hash2) = NewApiKey::generate::<24>(&id).consume();

        let bearer = BearerAuth::new(token1);

        let authn: ApiKey = bearer.key().unwrap().try_into().unwrap();

        let verified = authn.verify(&hash2);

        assert!(!verified);
    }
}
