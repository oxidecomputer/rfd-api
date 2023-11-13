use config::{Config, ConfigError, Environment, File};
use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppConfigError {
    #[error("Encountered invalid log format.")]
    InvalidLogFormatVariant,
}

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub log_format: ServerLogFormat,
    pub server_port: u16,
    pub host_regex: String,
    pub github_template: String,
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

impl AppConfig {
    pub fn new() -> Result<Self, ConfigError> {
        let config = Config::builder()
            .add_source(File::with_name("config.toml"))
            .add_source(Environment::default())
            .build()?;

        config.try_deserialize()
    }
}
