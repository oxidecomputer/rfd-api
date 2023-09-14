use config::{Config, ConfigError, Environment, File};
use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer,
};
use thiserror::Error;

use crate::{permissions::ApiPermission, server::SpecConfig};

#[derive(Debug, Error)]
pub enum AppConfigError {
    #[error("Encountered invalid log format.")]
    InvalidLogFormatVariant,
}

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub log_format: ServerLogFormat,
    pub public_url: String,
    pub server_port: u16,
    pub database_url: String,
    pub permissions: PermissionsConfig,
    pub keys: Vec<AsymmetricKey>,
    pub jwt: JwtConfig,
    pub spec: Option<SpecConfig>,
    pub authn: AuthnProviders,
}

#[derive(Debug)]
pub enum ServerLogFormat {
    Json,
    Pretty,
}

impl<'de> Deserialize<'de> for ServerLogFormat {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ExternalId;

        impl<'de> Visitor<'de> for ExternalId {
            type Value = ServerLogFormat;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("string")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                match value {
                    "json" => Ok(Self::Value::Json),
                    "pretty" => Ok(Self::Value::Pretty),
                    _ => Err(de::Error::custom(AppConfigError::InvalidLogFormatVariant)),
                }
            }
        }

        deserializer.deserialize_any(ExternalId)
    }
}

#[derive(Debug, Default, Deserialize)]
pub struct PermissionsConfig {
    pub default: Vec<ApiPermission>,
}

#[derive(Debug, Default, Deserialize)]
pub struct JwtConfig {
    pub default_expiration: i64,
    pub max_expiration: i64,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum AsymmetricKey {
    Local {
        kid: String,
        // #[serde(with = "serde_bytes")]
        private: String,
        public: String,
    },
    // Kms {
    //     id: String,
    // },
    Ckms {
        kid: String,
        version: u16,
        key: String,
        keyring: String,
        location: String,
        project: String,
    },
}

impl AsymmetricKey {
    pub fn kid(&self) -> &str {
        match self {
            Self::Local { kid, .. } => kid,
            Self::Ckms { kid, .. } => kid,
        }
    }

    pub fn mock_private_key(&self) -> &str {
        match &self {
            Self::Local { private, .. } => private,
            _ => unimplemented!(),
        }
    }

    pub fn mock_public_key(&self) -> &str {
        match &self {
            Self::Local { public, .. } => public,
            _ => unimplemented!(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct AuthnProviders {
    pub oauth: OAuthProviders,
}

#[derive(Debug, Deserialize)]
pub struct OAuthProviders {
    pub github: Option<GitHubOAuthConfig>,
    pub google: Option<GoogleOAuthConfig>,
}

#[derive(Debug, Deserialize)]
pub struct GitHubOAuthConfig {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
}

#[derive(Debug, Deserialize)]
pub struct GoogleOAuthConfig {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
}

impl AppConfig {
    pub fn new() -> Result<Self, ConfigError> {
        let config = Config::builder()
            .add_source(File::with_name("config.toml"))
            .add_source(Environment::default())
            .build()?;

        config.try_deserialize()
    }
}
