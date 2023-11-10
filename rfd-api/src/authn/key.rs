use hex::FromHexError;
use rand::{rngs::OsRng, RngCore};
use secrecy::{ExposeSecret, SecretString, SecretVec};
use thiserror::Error;
use uuid::Uuid;

use super::{Signer, SigningKeyError};

pub struct RawApiKey {
    clear: SecretVec<u8>,
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
    #[error("Failed to sign API key: {0}")]
    Verify(SigningKeyError),
}

impl RawApiKey {
    // Generate a new API key
    pub fn generate<const N: usize>(id: &Uuid) -> Self {
        // Generate random data to extend the token id with
        let mut token_raw = [0; N];
        OsRng.fill_bytes(&mut token_raw);

        let mut clear = id.as_bytes().to_vec();
        clear.append(&mut token_raw.to_vec());

        Self {
            clear: clear.into(),
        }
    }

    pub fn id(&self) -> &[u8] {
        &self.clear.expose_secret()[0..16]
    }

    pub async fn sign(self, signer: &dyn Signer) -> Result<SignedApiKey, ApiKeyError> {
        let signature = hex::encode(
            signer
                .sign(&self.clear.expose_secret())
                .await
                .map_err(ApiKeyError::Signing)?,
        );
        Ok(SignedApiKey::new(
            hex::encode(self.clear.expose_secret()).into(),
            signature,
        ))
    }

    pub fn verify(&self, signer: &dyn Signer, signature: &[u8]) -> Result<(), ApiKeyError> {
        let signature = hex::decode(signature)?;
        Ok(signer
            .verify(&self.clear.expose_secret(), &signature)
            .map_err(ApiKeyError::Verify)?)
    }
}

impl TryFrom<&str> for RawApiKey {
    type Error = ApiKeyError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let decoded = hex::decode(value)?;

        if decoded.len() > 16 {
            Ok(RawApiKey {
                clear: decoded.into(),
            })
        } else {
            tracing::debug!(len = ?decoded.len(), "API key is too short");
            Err(ApiKeyError::FailedToParse)
        }
    }
}

impl TryFrom<&SecretString> for RawApiKey {
    type Error = ApiKeyError;

    fn try_from(value: &SecretString) -> Result<Self, Self::Error> {
        let decoded = hex::decode(value.expose_secret())?;

        if decoded.len() > 16 {
            Ok(RawApiKey {
                clear: decoded.into(),
            })
        } else {
            tracing::debug!(len = ?decoded.len(), "API key is too short");
            Err(ApiKeyError::FailedToParse)
        }
    }
}

pub struct SignedApiKey {
    key: SecretString,
    signature: String,
}

impl SignedApiKey {
    fn new(key: SecretString, signature: String) -> Self {
        Self { key, signature }
    }

    pub fn key(self) -> SecretString {
        self.key
    }

    pub fn signature(&self) -> &str {
        &self.signature
    }
}

#[cfg(test)]
mod tests {
    use secrecy::ExposeSecret;
    use uuid::Uuid;

    use super::RawApiKey;
    use crate::util::tests::mock_key;

    #[tokio::test]
    async fn test_verifies_signature() {
        let id = Uuid::new_v4();
        let signer = mock_key().as_signer().await.unwrap();

        let raw = RawApiKey::generate::<8>(&id);
        let signed = raw.sign(&*signer).await.unwrap();

        let raw2 = RawApiKey::try_from(signed.key.expose_secret().as_str()).unwrap();

        assert_eq!(
            (),
            raw2.verify(&*signer, signed.signature.as_bytes()).unwrap()
        )
    }

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
