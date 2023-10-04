use std::sync::Arc;

use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use jsonwebtoken::{
    decode, decode_header,
    jwk::{AlgorithmParameters, CommonParameters, Jwk, PublicKeyUse, RSAKeyParameters, RSAKeyType},
    Algorithm, DecodingKey, Header, Validation,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tracing::instrument;
use uuid::Uuid;

use crate::{config::AsymmetricKey, context::ApiContext};

use super::{Signer, SigningKeyError};

#[derive(Debug, Error)]
pub enum JwtError {
    #[error("Failed to decode token: {0}")]
    Decode(jsonwebtoken::errors::Error),
    #[error("Header is not well formed")]
    MalformedHeader(jsonwebtoken::errors::Error),
    #[error("Failed to construct decoding key: {0}")]
    InvalidJwk(jsonwebtoken::errors::Error),
    #[error("Header does not have a defined kid")]
    MissingKid,
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
    pub scp: Vec<String>,
    pub exp: i64,
    pub nbf: i64,
    pub jti: Uuid,
}

impl Jwt {
    pub async fn new(ctx: &ApiContext, token: &str) -> Result<Self, JwtError> {
        tracing::trace!("Decode JWT from headers");

        let header = decode_header(token).map_err(|err| {
            tracing::warn!(?err, "Token header is malformed");
            JwtError::MalformedHeader(err)
        })?;

        tracing::trace!("Found header containing JWT");

        // We require that the header contains a kid attribute for determining which decoding key
        // to use, even in the case that we are using a single key
        let kid = header.kid.ok_or(JwtError::MissingKid)?;

        tracing::debug!(?kid, "JWT with kid present");

        // The only JWKs supported are those that are available in the server context
        let jwk = ctx.jwks().await.find(&kid).ok_or(JwtError::NoMatchingKey)?;
        let (key, algorithm) = DecodingKey::from_jwk(&jwk)
            .map(|key| (key, Jwt::algo(&jwk)))
            .map_err(JwtError::InvalidJwk)?;

        tracing::debug!(?jwk, ?algorithm, "Kid matched known decoding key");

        let data = decode(token, &key, &Validation::new(algorithm?)).map_err(JwtError::Decode)?;

        tracing::debug!("Decoded JWT claims from request");

        Ok(Jwt {
            claims: data.claims,
        })
    }

    // Check the algorithm defined in the JWK. Ensure that it is an RSA variant.
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

#[derive(Debug, Error)]
pub enum JwtSignerError {
    #[error("Failed to encode header")]
    Header(serde_json::Error),
    #[error("Failed to generate signer: {0}")]
    InvalidKey(SigningKeyError),
    #[error("Failed to serialize claims: {0}")]
    Serialize(#[from] serde_json::Error),
    #[error("Failed to generate signature: {0}")]
    Signature(SigningKeyError),
}

pub struct JwtSigner {
    header: Header,
    encoded_header: String,
    jwk: Jwk,
    signer: Arc<dyn Signer>,
}

impl JwtSigner {
    pub async fn new(key: &AsymmetricKey) -> Result<Self, JwtSignerError> {
        let jwk = key.as_jwk().await?;
        let mut header = Header::new(Algorithm::RS256);
        header.kid = Some(key.kid().to_string());
        let encoded_header = to_base64_json(&header)?;

        let signer = key.as_signer().await.map_err(JwtSignerError::InvalidKey)?;

        Ok(Self {
            header,
            encoded_header,
            jwk,
            signer,
        })
    }

    #[instrument(skip(self, claims))]
    pub async fn sign<C>(&self, claims: &C) -> Result<String, JwtSignerError>
    where
        C: Serialize,
    {
        tracing::debug!(?self.jwk.common.key_id, "Signing claims with");

        let encoded_claims = to_base64_json(claims)?;

        tracing::debug!("Serialized claims to sign");

        let message = format!("{}.{}", self.encoded_header, encoded_claims);

        tracing::debug!("Generating signature");

        let signature = self
            .signer
            .sign(message.as_bytes())
            .await
            .map_err(JwtSignerError::Signature)?;

        let enc_signature = URL_SAFE_NO_PAD.encode(signature);
        Ok(format!("{}.{}", message, enc_signature))
    }

    pub fn header(&self) -> &Header {
        &self.header
    }

    pub fn jwk(&self) -> &Jwk {
        &self.jwk
    }
}

use rsa::traits::PublicKeyParts;

impl AsymmetricKey {
    pub async fn as_jwk(&self) -> Result<Jwk, JwtSignerError> {
        let key_id = self.kid();
        let public_key = self
            .public_key()
            .await
            .map_err(JwtSignerError::InvalidKey)?;

        Ok(Jwk {
            common: CommonParameters {
                public_key_use: Some(PublicKeyUse::Signature),
                key_operations: None,
                algorithm: Some(Algorithm::RS256),
                key_id: Some(key_id.to_string()),
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
}

fn to_base64_json<T>(value: &T) -> Result<String, serde_json::error::Error>
where
    T: Serialize,
{
    let json = serde_json::to_vec(value)?;
    Ok(URL_SAFE_NO_PAD.encode(json))
}
