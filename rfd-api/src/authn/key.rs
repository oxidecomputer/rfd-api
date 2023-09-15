use argon2::password_hash::rand_core::{OsRng, RngCore};
use hex::FromHexError;
use thiserror::Error;

use super::{Signer, SigningKeyError};

pub struct RawApiKey {
    clear: String,
}

#[derive(Debug, Error)]
pub enum ApiKeyError {
    #[error("Failed to decode signature: {0}")]
    Decode(#[from] FromHexError),
    #[error("Signature is malformed: {0}")]
    MalformedSignature(#[from] rsa::signature::Error),
    #[error("Failed to sign API key: {0}")]
    Signing(SigningKeyError),
}

impl RawApiKey {
    pub fn new(key: String) -> Self {
        Self { clear: key }
    }

    // Generate a new API key
    pub fn generate<const N: usize>() -> Self {
        // Generate random data to extend the token id with
        let mut token_raw = [0; N];
        OsRng.fill_bytes(&mut token_raw);

        let clear = hex::encode(token_raw);

        Self { clear }
    }

    pub async fn sign(self, signer: &dyn Signer) -> Result<SignedApiKey, ApiKeyError> {
        let signature = hex::encode(
            signer
                .sign(&self.clear)
                .await
                .map_err(ApiKeyError::Signing)?,
        );
        Ok(SignedApiKey::new(self.clear, signature))
    }
}

impl From<&str> for RawApiKey {
    fn from(value: &str) -> Self {
        RawApiKey {
            clear: value.to_string(),
        }
    }
}

pub struct SignedApiKey {
    key: String,
    signature: String,
}

impl SignedApiKey {
    fn new(key: String, signature: String) -> Self {
        Self {
            key,
            signature,
        }
    }

    pub fn key(self) -> String {
        self.key
    }

    pub fn signature(&self) -> &str {
        &self.signature
    }
}

#[cfg(test)]
mod tests {
    use super::RawApiKey;
    use crate::util::tests::mock_key;

    #[tokio::test]
    async fn test_signs_twice() {
        let not_real = "not-a-real-token".to_string();

        let signer = mock_key().as_signer().await.unwrap();

        let raw1 = RawApiKey::new(not_real.clone());
        let signed1 = raw1
            .sign(&*signer)
            .await
            .unwrap();

        let raw2 = RawApiKey::new(not_real.clone());
        let signed2 = raw2
            .sign(&*signer)
            .await
            .unwrap();

        assert_eq!(signed1.signature(), signed2.signature());
    }

    #[tokio::test]
    async fn test_rejects_invalidd_source() {
        let signer = mock_key().as_signer().await.unwrap();

        let raw1 = RawApiKey::generate::<8>();
        let signed1 = raw1.sign(&*signer).await.unwrap();

        let raw2 = RawApiKey::generate::<8>();
        let signed2 = raw2.sign(&*signer).await.unwrap();

        assert_ne!(signed1.signature(), signed2.signature())
    }
}
