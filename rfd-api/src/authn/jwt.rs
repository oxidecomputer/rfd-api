use async_trait::async_trait;
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, prelude::BASE64_STANDARD, Engine};
use crc32c::crc32c;
use google_cloudkms1::{
    api::AsymmetricSignRequest,
    hyper_rustls::{self, HttpsConnector},
    CloudKMS,
};
use hyper::client::HttpConnector;
use jsonwebtoken::{
    decode, decode_header, encode,
    jwk::{AlgorithmParameters, CommonParameters, Jwk, PublicKeyUse, RSAKeyParameters, RSAKeyType},
    Algorithm, DecodingKey, EncodingKey, Header, Validation,
};
use rsa::{pkcs8::DecodePublicKey, PublicKeyParts, RsaPublicKey};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use thiserror::Error;
use tracing::instrument;
use uuid::Uuid;

use crate::{config::JwtKey, context::ApiContext, ApiPermissions};

#[derive(Debug, Error)]
pub enum JwtError {
    #[error("Failed to decode JWT")]
    FailedToDecode(#[from] jsonwebtoken::errors::Error),
    #[error("Header is not well formed")]
    InvalidHeader,
    #[error("Failed to find a matching key as requested by token")]
    NoMatchingKey,
    #[error("Unsupported algorithm")]
    UnsupportedAlgorithm,
}

pub struct Jwt {
    pub claims: Claims,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub aud: Uuid,
    pub prm: ApiPermissions,
    pub exp: i64,
    pub nbf: i64,
    pub jti: Uuid,
}

impl Jwt {
    pub async fn new(ctx: &ApiContext, token: &str) -> Result<Self, JwtError> {
        tracing::trace!("Decode JWT from headers");

        let header = decode_header(token)?;

        tracing::trace!("Found header containing JWT");

        let kid = header.kid.ok_or(JwtError::InvalidHeader)?;

        tracing::debug!(?kid, "JWT with kid present");

        // The only JWKs supported are those that are available in the server contet
        let jwk = ctx.jwks().await.find(&kid).ok_or(JwtError::NoMatchingKey)?;
        let (key, algorithm) =
            DecodingKey::from_jwk(&jwk).and_then(|key| Ok((key, Jwt::algo(&jwk))))?;

        tracing::debug!("Kid matched known decoding key");

        let data = decode(token, &key, &Validation::new(algorithm?))?;

        tracing::debug!("Decoded JWT claims from request");

        Ok(Jwt {
            claims: data.claims,
        })
    }

    // Check the algorithm defined in the JWK. Ensure that it is an RSA variant as it is the only
    // kind that supported
    pub fn algo(key: &Jwk) -> Result<Algorithm, JwtError> {
        match &key.algorithm {
            AlgorithmParameters::RSA(_) => Ok(Algorithm::RS256),
            algo => {
                tracing::warn!(?algo, "Encountered unsupported algorithm");
                Err(JwtError::UnsupportedAlgorithm)
            }
        }
    }
}

// Represents a service for signing JWTs and providing a readable JWK that can be used in verifying
// token signatures
#[async_trait]
pub trait JwtSigner: Send + Sync {
    type Claims: Serialize;

    async fn sign(&self, claims: &Self::Claims) -> Result<String, SignerError>;
    fn header(&self) -> &Header;
    fn jwk(&self) -> &Jwk;
}

#[derive(Debug, Error)]
pub enum SignerError {
    #[error(transparent)]
    CloudKms(#[from] CloudKmsError),
    #[error("Internal key creation failed")]
    KeyError(#[from] jsonwebtoken::errors::Error),
    #[error("Key manipulation failed {0}")]
    InvalidPublicKey(#[from] rsa::pkcs8::spki::Error),
    #[error("Failed to construct credentials for remote key storage")]
    RemoteKeyAuthMissing,
    #[error("Key input does not match requested output")]
    SigningConfigurationMismatch,
}

// A signer that stores a local in memory key for signing new JWTs
pub struct LocalKey {
    encoding_key: EncodingKey,
    header: Header,
    jwk: Jwk,
}

#[async_trait]
impl JwtSigner for LocalKey {
    type Claims = Claims;

    #[instrument(skip(self, claims), err(Debug))]
    async fn sign(&self, claims: &Self::Claims) -> Result<String, SignerError> {
        Ok(encode(self.header(), claims, &self.encoding_key)?)
    }

    fn header(&self) -> &Header {
        &self.header
    }

    fn jwk(&self) -> &Jwk {
        &self.jwk
    }
}

// AWS KMS
// pub struct KmsSigner {}

// #[async_trait]
// impl JwtSigner for KmsSigner {
//     type Claims = Claims;
//     async fn sign(&self, header: &jsonwebtoken::Header, claims: &Self::Claims) -> Result<String, jsonwebtoken::errors::Error> {
//         unimplemented!()
//     }
// }

#[derive(Debug, Error)]
pub enum CloudKmsError {
    #[error(transparent)]
    ClientError(#[from] google_cloudkms1::Error),
    #[error("Signature received failed CRC check")]
    CorruptedSignature,
    #[error("Failed to serialize payload {0}")]
    FailedToSerialize(serde_json::error::Error),
    #[error("Failed to decode signature {0}")]
    FailedToDecodeSignature(#[from] base64::DecodeError),
    #[error("Failed to deserialize response {0}")]
    FailedToDeserialize(serde_json::error::Error),
    #[error("CloudKMS response is missing the public key")]
    MissingPem,
    #[error("CloudKMS signing request did not return a signature")]
    MissingSignature,
}

// Signer that relies on a private key stored in GCP, and a locally store JWK. This signer never
// has direct access to the private key
pub struct CloudKmSigner {
    client: CloudKMS<HttpsConnector<HttpConnector>>,
    key_name: String,
    header: Header,
    jwk: Jwk,
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
impl JwtSigner for CloudKmSigner {
    type Claims = Claims;

    #[instrument(skip(self, claims), err(Debug))]
    async fn sign(&self, claims: &Self::Claims) -> Result<String, SignerError> {
        tracing::debug!(?self.jwk.common.key_id, "Signing claims with");

        let enc_header = to_base64_json(self.header()).map_err(CloudKmsError::FailedToSerialize)?;
        let enc_claims = to_base64_json(claims).map_err(CloudKmsError::FailedToSerialize)?;

        tracing::debug!("Serialized claims to sign");

        let message = format!("{}.{}", enc_header, enc_claims);

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

        tracing::info!("Extracted remote generated signature");

        let enc_signature = URL_SAFE_NO_PAD.encode(signature);

        Ok(format!("{}.{}.{}", enc_header, enc_claims, enc_signature))
    }

    fn header(&self) -> &Header {
        &self.header
    }

    fn jwk(&self) -> &Jwk {
        &self.jwk
    }
}

fn pem_to_jwk(id: &str, pem: &str) -> Result<Jwk, SignerError> {
    let public_key = RsaPublicKey::from_public_key_pem(pem)?;

    Ok(Jwk {
        common: CommonParameters {
            public_key_use: Some(PublicKeyUse::Signature),
            key_operations: None,
            algorithm: Some(Algorithm::RS256),
            key_id: Some(id.to_string()),
            x509_chain: None,
            x509_sha1_fingerprint: None,
            x509_sha256_fingerprint: None,
            x509_url: None,
        },
        algorithm: AlgorithmParameters::RSA(RSAKeyParameters {
            key_type: RSAKeyType::RSA,
            n: URL_SAFE_NO_PAD.encode(public_key.n().to_bytes_be()),
            e: URL_SAFE_NO_PAD.encode(public_key.e().to_bytes_be()),
        }),
    })
}

#[instrument(skip(key), err(Debug))]
pub async fn key_to_signer(
    key: &JwtKey,
) -> Result<Box<dyn JwtSigner<Claims = Claims>>, SignerError> {
    Ok(match key {
        JwtKey::Local {
            kid,
            private,
            public,
        } => {
            let jwk = pem_to_jwk(kid, public)?;
            let mut header = Header::new(Algorithm::RS256);
            header.kid = Some(kid.clone());

            tracing::trace!(?header, ?jwk, "Generated local signer");

            Box::new(LocalKey {
                encoding_key: EncodingKey::from_rsa_pem(&private)?,
                header,
                jwk,
            })
        }
        JwtKey::Ckms {
            kid,
            version,
            key,
            keyring,
            location,
            project,
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
                        SignerError::RemoteKeyAuthMissing
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
                        SignerError::RemoteKeyAuthMissing
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
            let jwk = pem_to_jwk(kid, &pem)?;

            let mut header = Header::new(Algorithm::RS256);
            header.kid = Some(kid.clone());

            tracing::trace!(?header, ?jwk, "Generated Cloud KMS signer");

            Box::new(CloudKmSigner {
                client: gcp_kms,
                key_name,
                header,
                jwk,
            })
        }
    })
}

fn to_base64_json<T>(value: &T) -> Result<String, serde_json::error::Error>
where
    T: Serialize,
{
    let json = serde_json::to_vec(value)?;
    Ok(URL_SAFE_NO_PAD.encode(json))
}
