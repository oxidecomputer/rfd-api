// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::{collections::HashMap, path::PathBuf};

use config::{Config, ConfigError, Environment, File};
use rfd_data::content::RfdTemplate;
use secrecy::SecretString;
use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer,
};
use thiserror::Error;

use crate::server::SpecConfig;

#[derive(Debug, Error)]
pub enum AppConfigError {
    #[error("Encountered invalid log format.")]
    InvalidLogFormatVariant,
}

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub log_format: ServerLogFormat,
    pub log_directory: Option<PathBuf>,
    pub initial_mappers: Option<String>,
    pub public_url: String,
    pub server_port: u16,
    pub database_url: String,
    pub keys: Vec<AsymmetricKey>,
    pub jwt: JwtConfig,
    pub spec: Option<SpecConfig>,
    pub authn: AuthnProviders,
    pub search: SearchConfig,
    pub content: ContentConfig,
    pub services: ServicesConfig,
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
pub struct JwtConfig {
    pub default_expiration: i64,
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
}

#[derive(Debug, Deserialize)]
pub struct AuthnProviders {
    pub oauth: OAuthProviders,
}

#[derive(Debug, Deserialize)]
pub struct OAuthProviders {
    pub github: Option<OAuthConfig>,
    pub google: Option<OAuthConfig>,
}

#[derive(Debug, Deserialize)]
pub struct OAuthConfig {
    pub device: OAuthDeviceConfig,
    pub web: OAuthWebConfig,
}

#[derive(Debug, Deserialize)]
pub struct OAuthDeviceConfig {
    pub client_id: String,
    pub client_secret: SecretString,
}

#[derive(Debug, Deserialize)]
pub struct OAuthWebConfig {
    pub client_id: String,
    pub client_secret: SecretString,
    pub redirect_uri: String,
}

#[derive(Debug, Default, Deserialize)]
pub struct SearchConfig {
    pub host: String,
    pub key: String,
    pub index: String,
}

#[derive(Debug, Default, Deserialize)]
pub struct ContentConfig {
    pub templates: HashMap<String, RfdTemplate>,
}

#[derive(Debug, Deserialize)]
pub struct ServicesConfig {
    pub github: GitHubConfig,
}

#[derive(Debug, Deserialize)]
pub struct GitHubConfig {
    pub auth: GitHubAuthConfig,
    pub owner: String,
    pub path: String,
    pub repo: String,
    pub default_branch: String,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum GitHubAuthConfig {
    Installation {
        app_id: i64,
        installation_id: i64,
        private_key: String,
    },
    User {
        token: String,
    },
}

impl AppConfig {
    pub fn new(config_sources: Option<Vec<String>>) -> Result<Self, ConfigError> {
        let mut config = Config::builder()
            .add_source(File::with_name("config.toml").required(false))
            .add_source(File::with_name("rfd-api/config.toml").required(false));

        for source in config_sources.unwrap_or_default() {
            config = config.add_source(File::with_name(&source).required(false));
        }

        config
            .add_source(Environment::default())
            .build()?
            .try_deserialize()
    }
}
