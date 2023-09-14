use argon2::password_hash::rand_core::{OsRng, RngCore};
use hex::FromHexError;
use rsa::pss::Signature;
use thiserror::Error;

use super::{Signer, SigningKeyError};

pub struct RawApiKey {
    clear: String,
}

#[derive(Debug, Error)]
pub enum ApiKeyError {
    #[error("Failed to decode signature: {0}")]
    Decode(#[from] FromHexError),
    #[error("API key is not formatted as {{key}}.{{signature}}")]
    InvalidFormat,
    #[error("Signature is malformed: {0}")]
    MalformedSignature(#[from] rsa::signature::Error),
    #[error("Failed to sign API key: {0}")]
    Signing(SigningKeyError),
    #[error("Verifying API key failed: {0}")]
    Verification(SigningKeyError),
}

impl RawApiKey {
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
    signed: String,
}

impl SignedApiKey {
    fn new(key: String, signature: String) -> Self {
        let signed = format!("{}.{}", key, signature);
        Self {
            key,
            signature,
            signed,
        }
    }

    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn signature(&self) -> &str {
        &self.signature
    }

    pub fn signed(self) -> String {
        self.signed
    }

    pub fn parse(value: &str, signer: &dyn Signer) -> Result<Self, ApiKeyError> {
        match value.split_once(".") {
            Some((key, signature)) => {
                let signature = Signature::try_from(&hex::decode(signature)?[..])?;
                signer
                    .verify(&key, &signature)
                    .map_err(ApiKeyError::Verification)?;

                Ok(Self::new(key.to_string(), signature.to_string()))
            }
            None => Err(ApiKeyError::InvalidFormat),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{RawApiKey, SignedApiKey};
    use crate::util::tests::mock_key;

    #[tokio::test]
    async fn test_parses_valid_key() {
        let signer = mock_key().as_signer().await.unwrap();

        let signed = RawApiKey::generate::<8>()
            .sign(&*signer)
            .await
            .unwrap()
            .signed();

        println!("{}", signed);

        SignedApiKey::parse(&signed, &*signer).unwrap();
    }

    #[tokio::test]
    async fn test_rejects_manipulated_signature() {
        let signer = mock_key().as_signer().await.unwrap();

        let signed_key = RawApiKey::generate::<8>().sign(&*signer).await.unwrap();

        let key = signed_key.key().to_string();
        let signature = signed_key.signature().chars().rev().collect::<String>();

        assert!(SignedApiKey::parse(&SignedApiKey::new(key, signature).signed(), &*signer).is_err())
    }

    #[tokio::test]
    async fn test_rejects_manipulated_source() {
        let signer = mock_key().as_signer().await.unwrap();

        let signed_key = RawApiKey::generate::<8>().sign(&*signer).await.unwrap();

        let key = signed_key.key().chars().rev().collect::<String>();
        let signature = signed_key.signature().to_string();

        assert!(SignedApiKey::parse(&SignedApiKey::new(key, signature).signed(), &*signer).is_err())
    }
}
