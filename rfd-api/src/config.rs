// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::{collections::HashMap, path::PathBuf};

use config::{Config, ConfigError, Environment, File};
use rfd_data::content::RfdTemplate;
use serde::Deserialize;
use v_api::config::{AsymmetricKey, JwtConfig};
use v_model::schema_ext::MagicLinkMedium;

use rfd_secret::{SecretResolutionError, SecretString};

use crate::server::SpecConfig;

// ============================================================================
// Wrapper types for v_api configuration with SecretString support
// ============================================================================

/// Wrapper for v_api::config::AsymmetricKey that supports path-based secrets.
///
/// Use `resolve()` to convert to the actual v_api AsymmetricKey type.
#[derive(Debug, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum AsymmetricKeyConfig {
    LocalSigner {
        kid: String,
        private: SecretString,
    },
    LocalVerifier {
        kid: String,
        public: SecretString,
    },
    CkmsSigner {
        kid: String,
        version: u16,
        key: String,
        keyring: String,
        location: String,
        project: String,
    },
    CkmsVerifier {
        kid: String,
        version: u16,
        key: String,
        keyring: String,
        location: String,
        project: String,
    },
}

impl AsymmetricKeyConfig {
    /// Resolves any path-based secrets and converts to v_api AsymmetricKey.
    pub fn resolve(self) -> Result<AsymmetricKey, SecretResolutionError> {
        match self {
            AsymmetricKeyConfig::LocalSigner { kid, private } => Ok(AsymmetricKey::LocalSigner {
                kid,
                private: private.resolve()?,
            }),
            AsymmetricKeyConfig::LocalVerifier { kid, public } => {
                Ok(AsymmetricKey::LocalVerifier {
                    kid,
                    public: public.resolve()?,
                })
            }
            AsymmetricKeyConfig::CkmsSigner {
                kid,
                version,
                key,
                keyring,
                location,
                project,
            } => Ok(AsymmetricKey::CkmsSigner {
                kid,
                version,
                key,
                keyring,
                location,
                project,
            }),
            AsymmetricKeyConfig::CkmsVerifier {
                kid,
                version,
                key,
                keyring,
                location,
                project,
            } => Ok(AsymmetricKey::CkmsVerifier {
                kid,
                version,
                key,
                keyring,
                location,
                project,
            }),
        }
    }
}

/// OAuth client configuration with SecretString support.
#[derive(Debug, Clone, Deserialize)]
pub struct OAuthClientConfig {
    pub client_id: SecretString,
    pub client_secret: SecretString,
}

/// OAuth web client configuration with SecretString support.
#[derive(Debug, Clone, Deserialize)]
pub struct OAuthWebClientConfig {
    pub client_id: SecretString,
    pub client_secret: SecretString,
    // pub redirect_uri: String,
}

/// Per-provider OAuth configuration with device and web clients.
#[derive(Debug, Clone, Deserialize)]
pub struct OAuthProviderConfig {
    pub device: OAuthClientConfig,
    pub web: OAuthWebClientConfig,
}

/// OAuth providers configuration wrapper.
#[derive(Debug, Default, Deserialize)]
pub struct OAuthProvidersConfig {
    pub github: Option<OAuthProviderConfig>,
    pub google: Option<OAuthProviderConfig>,
}

/// Authentication providers configuration wrapper.
#[derive(Debug, Default, Deserialize)]
pub struct AuthnProvidersConfig {
    #[serde(default)]
    pub oauth: OAuthProvidersConfig,
}

// ============================================================================
// Main application configuration
// ============================================================================

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub log_format: ServerLogFormat,
    pub log_filter: Option<String>,
    pub log_directory: Option<PathBuf>,
    pub initial_mappers: Option<String>,
    pub public_url: String,
    pub server_port: u16,
    pub database: DatabaseConfig,
    pub keys: Vec<AsymmetricKeyConfig>,
    pub jwt: JwtConfig,
    pub spec: Option<SpecConfig>,
    pub authn: AuthnProvidersConfig,
    pub magic_link: MagicLinkConfig,
    pub search: SearchConfig,
    pub content: ContentConfig,
    pub services: ServicesConfig,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ServerLogFormat {
    Json,
    Pretty,
}

#[derive(Debug, Default, Deserialize)]
pub struct SearchConfig {
    pub host: String,
    pub key: SecretString,
    pub index: String,
}

#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: SecretString,
    pub database: String,
}

impl DatabaseConfig {
    pub fn to_url(&self) -> Result<String, SecretResolutionError> {
        let password = self.password.resolve()?;
        Ok(format!(
            "postgres://{}:{}@{}:{}/{}",
            self.user, password, self.host, self.port, self.database
        ))
    }
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
        private_key: SecretString,
    },
    User {
        token: SecretString,
    },
}

#[derive(Debug, Deserialize)]
pub struct MagicLinkConfig {
    pub templates: Vec<MagicLinkTemplate>,
    pub email_service: Option<EmailService>,
}

#[derive(Debug, Deserialize)]
pub struct MagicLinkTemplate {
    #[allow(dead_code)]
    pub medium: MagicLinkMedium,
    pub channel: String,
    pub from: String,
    pub subject: Option<String>,
    pub text: String,
    pub html: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EmailService {
    Resend { key: SecretString },
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
