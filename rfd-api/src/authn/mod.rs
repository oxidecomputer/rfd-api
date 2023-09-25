use async_trait::async_trait;
use base64::{prelude::BASE64_STANDARD, Engine};
use crc32c::crc32c;
use dropshot::{HttpError, RequestContext, SharedExtractor};
use dropshot_authorization_header::bearer::BearerAuth;
use google_cloudkms1::{api::AsymmetricSignRequest, hyper_rustls::HttpsConnector, CloudKMS};
use hyper::client::HttpConnector;
use rsa::{
    pkcs1v15::Signature,
    pkcs1v15::{SigningKey, VerifyingKey},
    pkcs8::{DecodePrivateKey, DecodePublicKey},
    signature::{Keypair, RandomizedSigner, SignatureEncoding, Verifier},
    RsaPrivateKey, RsaPublicKey,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{fmt::Debug, sync::Arc};
use thiserror::Error;
use tracing::instrument;

use crate::{
    authn::key::RawApiKey,
    config::AsymmetricKey,
    context::ApiContext,
    util::{cloud_kms_client, response::unauthorized},
};

use self::jwt::Jwt;

pub mod jwt;
pub mod key;

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("Failed to extract token")]
    FailedToExtract,
    #[error("Request does not have a token")]
    NoToken,
}

// A token that provides authentication and optionally (JWT) authorization claims
pub enum AuthToken {
    ApiKey(RawApiKey),
    Jwt(Jwt),
}

impl Debug for AuthToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AuthToken").finish()
    }
}

impl AuthToken {
    // Extract an AuthToken from a Dropshot request
    pub async fn extract(rqctx: &RequestContext<ApiContext>) -> Result<AuthToken, AuthError> {
        // Ensure there is a bearer, without it there is nothing else to do
        let bearer = BearerAuth::from_request(rqctx).await.map_err(|err| {
            tracing::info!(?err, "Failed to extract bearer auth");
            AuthError::NoToken
        })?;

        // Check that the extracted token actually contains a value
        let token = bearer.consume().ok_or(AuthError::NoToken)?;

        let ctx = rqctx.context();

        // Attempt to decode an API key from the token. If that fails then attempt to verify the
        // token as a JWT
        let jwt = Jwt::new(ctx, &token).await;

        match jwt {
            Ok(token) => Ok(AuthToken::Jwt(token)),
            Err(err) => {
                tracing::debug!(?err, ?token, "Token is not a JWT, falling back to API key");

                Ok(AuthToken::ApiKey(
                    RawApiKey::try_from(token.as_str()).map_err(|err| {
                        tracing::info!(?err, "Failed to parse API key");
                        AuthError::FailedToExtract
                    })?,
                ))
            }
        }
    }
}

impl From<AuthError> for HttpError {
    fn from(err: AuthError) -> Self {
        tracing::trace!(?err, "Failed to extract auth token");
        unauthorized()
    }
}

#[derive(Debug, Error)]
pub enum SigningKeyError {
    #[error("Cloud signing failed: {0}")]
    CloudKmsError(#[from] CloudKmsError),
    #[error("Failed to immediately verify generated signature")]
    GeneratedInvalidSignature,
    #[error("Failed to parse public key: {0}")]
    InvalidPublicKey(#[from] rsa::pkcs8::spki::Error),
    #[error("Invalid signature: {0}")]
    Signature(#[from] rsa::signature::Error),
}

#[async_trait]
pub trait Signer: Send + Sync {
    async fn sign(&self, message: &[u8]) -> Result<Vec<u8>, SigningKeyError>;
    fn verify(&self, message: &[u8], signature: &Signature) -> Result<(), SigningKeyError>;
}

// A signer that stores a local in memory key for signing new JWTs
pub struct LocalKey {
    signing_key: SigningKey<Sha256>,
    verifying_key: VerifyingKey<Sha256>,
}

#[async_trait]
impl Signer for LocalKey {
    #[instrument(skip(self, message), err(Debug))]
    async fn sign(&self, message: &[u8]) -> Result<Vec<u8>, SigningKeyError> {
        tracing::trace!("Signing message");
        let mut rng = rand::thread_rng();
        let signature = self.signing_key.sign_with_rng(&mut rng, message).to_vec();

        self.verify(message, &Signature::try_from(signature.as_ref()).unwrap())
            .map_err(|_| SigningKeyError::GeneratedInvalidSignature)?;

        Ok(signature)
    }

    fn verify(&self, message: &[u8], signature: &Signature) -> Result<(), SigningKeyError> {
        tracing::trace!("Verifying message");
        Ok(self.verifying_key.verify(message, &signature)?)
    }
}

#[derive(Debug, Error)]
pub enum CloudKmsError {
    #[error(transparent)]
    ClientError(#[from] google_cloudkms1::Error),
    #[error("Signature received failed CRC check")]
    CorruptedSignature,
    #[error("Failed to decode signature: {0}")]
    FailedToDecodeSignature(#[from] base64::DecodeError),
    #[error("Failed to deserialize response: {0}")]
    FailedToDeserialize(serde_json::error::Error),
    #[error("CloudKMS returned an invalid public key: {0}")]
    InvalidPem(#[from] rsa::pkcs8::spki::Error),
    #[error("CloudKMS did not return a public key")]
    MissingPem,
    #[error("CloudKMS signing request did not return a signature")]
    MissingSignature,
    #[error("Failed to find remote key")]
    RemoteKeyAuthMissing,
}

// Signer that relies on a private key stored in GCP, and a locally store JWK. This signer never
// has direct access to the private key
pub struct CloudKmsSigner {
    client: CloudKMS<HttpsConnector<HttpConnector>>,
    key_name: String,
    verifying_key: VerifyingKey<Sha256>,
}

// A fallback type for deserializing signature responses. google-cloudkms1 currently fails to decode
// the base64 signature due to assuming it to be url safe
#[derive(Debug, Serialize, Deserialize)]
pub struct CloudKmsSignatureResponse {
    pub name: String,
    // #[serde(with = "serde_bytes")]
    pub signature: String,
    #[serde(rename = "signatureCrc32c")]
    pub signature_crc32c: String,
}

#[async_trait]
impl Signer for CloudKmsSigner {
    #[instrument(skip(self, message), err(Debug))]
    async fn sign(&self, message: &[u8]) -> Result<Vec<u8>, SigningKeyError> {
        let mut hasher = Sha256::new();
        hasher.update(message);
        let digest = hasher.finalize();

        let check = crc32c(&digest);

        let response = self
            .client
            .projects()
            .locations_key_rings_crypto_keys_crypto_key_versions_asymmetric_sign(
                AsymmetricSignRequest {
                    data: None,
                    data_crc32c: None,
                    digest: Some(google_cloudkms1::api::Digest {
                        sha256: Some(digest.to_vec()),
                        sha384: None,
                        sha512: None,
                    }),
                    digest_crc32c: Some(check as i64),
                },
                &self.key_name,
            )
            .doit()
            .await;

        tracing::info!("Received response from remote signer");

        let signature = match response {
            Ok((_, response)) => {
                tracing::info!("Library deserialization succeeded");
                response.signature.ok_or(CloudKmsError::MissingSignature)
            }
            Err(google_cloudkms1::Error::JsonDecodeError(body, _)) => {
                tracing::info!("Using fallback deserialization");
                serde_json::from_str::<CloudKmsSignatureResponse>(&body)
                    .map_err(|err| CloudKmsError::FailedToDeserialize(err))
                    .and_then(|resp| {
                        let decoded = BASE64_STANDARD
                            .decode(&resp.signature)
                            .map_err(CloudKmsError::FailedToDecodeSignature)
                            .and_then(|decoded| {
                                let check = crc32c(&decoded);
                                let check_valid = resp
                                    .signature_crc32c
                                    .parse::<u32>()
                                    .map(|resp_check| resp_check == check)
                                    .unwrap_or(false);

                                if check_valid {
                                    Ok(decoded)
                                } else {
                                    Err(CloudKmsError::CorruptedSignature)
                                }
                            });

                        decoded
                    })
            }
            Err(err) => Err(CloudKmsError::from(err)),
        }?;

        Ok(signature)
    }

    fn verify(&self, message: &[u8], signature: &Signature) -> Result<(), SigningKeyError> {
        Ok(self.verifying_key.verify(message, &signature)?)
    }
}

impl AsymmetricKey {
    fn cloud_kms_key_name(&self) -> Option<String> {
        match self {
            AsymmetricKey::Local { .. } => None,
            AsymmetricKey::Ckms {
                version,
                key,
                keyring,
                location,
                project,
                ..
            } => Some(format!(
                "projects/{}/locations/{}/keyRings/{}/cryptoKeys/{}/cryptoKeyVersions/{}",
                project, location, keyring, key, version
            )),
        }
    }

    pub async fn private_key(&self) -> Result<RsaPrivateKey, SigningKeyError> {
        Ok(match self {
            AsymmetricKey::Local { private, .. } => {
                RsaPrivateKey::from_pkcs8_pem(&private).unwrap()
            }
            _ => unimplemented!(),
        })
    }

    pub async fn public_key(&self) -> Result<RsaPublicKey, SigningKeyError> {
        Ok(match self {
            AsymmetricKey::Local { public, .. } => RsaPublicKey::from_public_key_pem(&public)?,
            AsymmetricKey::Ckms { .. } => {
                let kms_client = cloud_kms_client().await?;

                let public_key = kms_client
                    .projects()
                    .locations_key_rings_crypto_keys_crypto_key_versions_get_public_key(
                        &self.cloud_kms_key_name().unwrap(),
                    )
                    .doit()
                    .await
                    .map_err(|err| CloudKmsError::from(err))?
                    .1;

                let pem = public_key.pem.ok_or(CloudKmsError::MissingPem)?;
                RsaPublicKey::from_public_key_pem(&pem)?
            }
        })
    }

    pub async fn as_signer(&self) -> Result<Arc<dyn Signer>, SigningKeyError> {
        Ok(match self {
            AsymmetricKey::Local { private, .. } => {
                let private_key = RsaPrivateKey::from_pkcs8_pem(&private).unwrap();
                let signing_key = SigningKey::new(private_key);
                let verifying_key = signing_key.verifying_key();

                Arc::new(LocalKey {
                    signing_key,
                    verifying_key,
                })
            }
            AsymmetricKey::Ckms { .. } => {
                let verifying_key = VerifyingKey::new(self.public_key().await?);

                tracing::trace!("Generated Cloud KMS signer");

                Arc::new(CloudKmsSigner {
                    client: cloud_kms_client().await?,
                    key_name: self.cloud_kms_key_name().unwrap(),
                    verifying_key,
                })
            }
        })
    }
}
