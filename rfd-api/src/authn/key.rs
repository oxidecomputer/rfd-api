use argon2::password_hash::rand_core::{OsRng, RngCore};
use hex::FromHexError;
use rsa::pkcs1v15::Signature;
use thiserror::Error;
use uuid::Uuid;

use super::{Signer, SigningKeyError};

pub struct RawApiKey {
    clear: Vec<u8>,
}

#[derive(Debug, Error)]
pub enum ApiKeyError {
    #[error("Failed to decode component: {0}")]
    Decode(#[from] FromHexError),
    #[error("Failed to parse API key")]
    FailedToParse,
    #[error("Signature is malformed: {0}")]
    MalformedSignature(#[from] rsa::signature::Error),
    #[error("Failed to sign API key: {0}")]
    Signing(SigningKeyError),
}

impl RawApiKey {
    // Generate a new API key
    pub fn generate<const N: usize>(id: &Uuid) -> Self {
        // Generate random data to extend the token id with
        let mut token_raw = [0; N];
        OsRng.fill_bytes(&mut token_raw);

        let mut clear = id.as_bytes().to_vec();
        clear.append(&mut token_raw.to_vec());

        Self { clear }
    }

    pub fn id(&self) -> &[u8] {
        &self.clear[0..24]
    }

    pub async fn sign(self, signer: &dyn Signer) -> Result<SignedApiKey, ApiKeyError> {
        let signature = hex::encode(signer.sign(&self.clear).await.map_err(ApiKeyError::Signing)?);
        Ok(SignedApiKey::new(hex::encode(self.clear), signature))
    }

    pub fn verify(&self, signer: &dyn Signer, signature: &Signature) -> bool {
        signer.verify(&self.clear, signature).is_ok()
    }
}

impl TryFrom<&str> for RawApiKey {
    type Error = ApiKeyError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let decoded = hex::decode(value)?;

        if decoded.len() > 24 {
            Ok(RawApiKey { clear: decoded })
        } else {
            Err(ApiKeyError::FailedToParse)
        }
    }
}

pub struct SignedApiKey {
    key: String,
    signature: String,
}

impl SignedApiKey {
    fn new(key: String, signature: String) -> Self {
        Self { key, signature }
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
    use uuid::Uuid;

    use super::RawApiKey;
    use crate::util::tests::mock_key;

    #[tokio::test]
    async fn test_generates_signatures() {
        let id = Uuid::new_v4();
        let signer = mock_key().as_signer().await.unwrap();

        let raw1 = RawApiKey::generate::<8>(&id);
        let signed1 = raw1.sign(&*signer).await.unwrap();

        let raw2 = RawApiKey::generate::<8>(&id);
        let signed2 = raw2.sign(&*signer).await.unwrap();

        assert_ne!(signed1.signature(), signed2.signature())
    }
}
