use argon2::password_hash::rand_core::{OsRng, RngCore};
use async_trait::async_trait;
use google_cloudkms1::{hyper_rustls, CloudKMS};
use rsa::{pkcs8::DecodePublicKey, Pkcs1v15Encrypt, PublicKey, RsaPublicKey};
use thiserror::Error;
use tracing::instrument;

use crate::config::AsymmetricKey;

use super::jwt::CloudKmsError;

pub struct RawApiKey {
    clear: String,
}

#[derive(Debug, Error)]
pub enum KeyEncryptionFailure {
    #[error(transparent)]
    EncryptionFailure(#[from] EncryptorError),
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

    // To get the token out of an API key it must be consumed so that it can not be used again
    pub fn consume(self) -> String {
        self.clear
    }

    pub async fn encrypt(
        &self,
        encryptor: &dyn KeyEncryptor,
    ) -> Result<EncryptedApiKey, KeyEncryptionFailure> {
        let encrypted = encryptor.encrypt(&self.clear).await?;
        Ok(EncryptedApiKey { encrypted })
    }
}

pub struct EncryptedApiKey {
    pub encrypted: String,
}

impl From<&str> for RawApiKey {
    fn from(value: &str) -> Self {
        RawApiKey {
            clear: value.to_string(),
        }
    }
}

// Represents a service for encrypting tokens
#[async_trait]
pub trait KeyEncryptor: Send + Sync {
    async fn encrypt(&self, value: &str) -> Result<String, EncryptorError>;
}

#[derive(Debug, Error)]
pub enum EncryptorError {
    #[error(transparent)]
    CloudKms(#[from] CloudKmsError),
    #[error(transparent)]
    Encryption(#[from] rsa::errors::Error),
    #[error(transparent)]
    PemDecode(#[from] rsa::pkcs8::spki::Error),
    #[error("Failed to construct credentials for remote key storage")]
    RemoteKeyAuthMissing,
    #[error("Key input does not match requested output")]
    SigningConfigurationMismatch,
}

// A signer that stores a local in memory key for signing new JWTs
pub struct LocalKey {
    public_key: RsaPublicKey,
}

#[async_trait]
impl KeyEncryptor for LocalKey {
    #[instrument(skip(self, value), err(Debug))]
    async fn encrypt(&self, value: &str) -> Result<String, EncryptorError> {
        let mut rng = rand::thread_rng();
        Ok(hex::encode(self.public_key.encrypt(
            &mut rng,
            Pkcs1v15Encrypt,
            value.as_bytes(),
        )?))
    }
}

#[instrument(skip(key), err(Debug))]
pub async fn key_to_encryptor(
    key: &AsymmetricKey,
) -> Result<Box<dyn KeyEncryptor>, EncryptorError> {
    let pem = match key {
        AsymmetricKey::Local { public, .. } => public.to_string(),
        AsymmetricKey::Ckms {
            version,
            key,
            keyring,
            location,
            project,
            ..
        } => {
            let opts = yup_oauth2::ApplicationDefaultCredentialsFlowOpts::default();

            tracing::trace!(?opts, "Request GCP credentials");

            let gcp_credentials =
                yup_oauth2::ApplicationDefaultCredentialsAuthenticator::builder(opts).await;

            tracing::trace!("Retrieved GCP credentials");

            let gcp_auth = match gcp_credentials {
                yup_oauth2::authenticator::ApplicationDefaultCredentialsTypes::ServiceAccount(
                    auth,
                ) => {
                    tracing::debug!("Create GCP service account based credentials");

                    auth.build().await.map_err(|err| {
                        tracing::error!(
                            ?err,
                            "Failed to construct Cloud KMS credentials from service account"
                        );
                        EncryptorError::RemoteKeyAuthMissing
                    })?
                }
                yup_oauth2::authenticator::ApplicationDefaultCredentialsTypes::InstanceMetadata(
                    auth,
                ) => {
                    tracing::debug!("Create GCP instance based credentials");

                    auth.build().await.map_err(|err| {
                        tracing::error!(
                            ?err,
                            "Failed to construct Cloud KMS credentials from instance metadata"
                        );
                        EncryptorError::RemoteKeyAuthMissing
                    })?
                }
            };

            let gcp_kms = CloudKMS::new(
                hyper::Client::builder().build(
                    hyper_rustls::HttpsConnectorBuilder::new()
                        .with_native_roots()
                        .https_only()
                        .enable_http2()
                        .build(),
                ),
                gcp_auth,
            );

            let key_name = format!(
                "projects/{}/locations/{}/keyRings/{}/cryptoKeys/{}/cryptoKeyVersions/{}",
                project, location, keyring, key, version
            );
            let public_key = gcp_kms
                .projects()
                .locations_key_rings_crypto_keys_crypto_key_versions_get_public_key(&key_name)
                .doit()
                .await
                .map_err(|err| CloudKmsError::from(err))?
                .1;

            let pem = public_key.pem.ok_or(CloudKmsError::MissingPem)?;

            pem
        }
    };

    Ok(Box::new(LocalKey {
        public_key: RsaPublicKey::from_public_key_pem(&pem)?,
    }))
}

// #[cfg(test)]
// mod tests {
//     use super::{ApiKey, NewApiKey};
//     use dropshot_authorization_header::bearer::BearerAuth;
//     use uuid::Uuid;

//     #[test]
//     fn test_decodes_token() {
//         let id = Uuid::new_v4();
//         let (token, hash) = NewApiKey::generate::<24>(&id).consume();

//         let bearer = BearerAuth::new(token);

//         let authn: ApiKey = bearer.key().unwrap().try_into().unwrap();

//         let verified = authn.verify(&hash);

//         assert!(verified);
//     }

//     #[test]
//     fn test_fails_to_decode_invalid_token() {
//         let id = Uuid::new_v4();
//         let (token1, _hash1) = NewApiKey::generate::<24>(&id).consume();
//         let (_token2, hash2) = NewApiKey::generate::<24>(&id).consume();

//         let bearer = BearerAuth::new(token1);

//         let authn: ApiKey = bearer.key().unwrap().try_into().unwrap();

//         let verified = authn.verify(&hash2);

//         assert!(!verified);
//     }
// }
