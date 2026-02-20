// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Secret string utilities for RFD API configuration.
//!
//! This crate provides a [`SecretString`] type that can be deserialized from either
//! an inline value or a file path, allowing secrets to be stored outside of
//! configuration files.
//!
//! # TOML Usage
//!
//! Inline value:
//! ```toml
//! key = "my-secret-value"
//! ```
//!
//! Path-based value (reads secret from file at runtime):
//! ```toml
//! key = { path = "/run/secrets/my-key" }
//! ```

use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SecretResolutionError {
    #[error("Failed to read secret from path '{path}'")]
    FileRead {
        path: String,
        #[source]
        source: std::io::Error,
    },
}

/// A secret string that can be specified either inline or as a path to a file.
///
/// When deserialized from TOML/JSON, accepts either:
/// - A plain string: `"my-secret"`
/// - An object with path: `{ path = "/path/to/secret" }`
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum SecretString {
    /// Secret value specified directly inline
    Inline(String),
    /// Path to a file containing the secret
    FromPath { path: PathBuf },
}

impl SecretString {
    /// Resolves the secret value, reading from file if necessary.
    ///
    /// For inline values, returns the value directly.
    /// For path-based values, reads the file contents and trims trailing whitespace.
    pub fn resolve(&self) -> Result<String, SecretResolutionError> {
        match self {
            SecretString::Inline(value) => Ok(value.clone()),
            SecretString::FromPath { path } => {
                let content =
                    std::fs::read_to_string(path).map_err(|source| SecretResolutionError::FileRead {
                        path: path.display().to_string(),
                        source,
                    })?;
                // Trim trailing whitespace/newlines that are common in secret files
                Ok(content.trim_end().to_string())
            }
        }
    }
}

impl Default for SecretString {
    fn default() -> Self {
        SecretString::Inline(String::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_inline_value() {
        let secret = SecretString::Inline("my-secret".to_string());
        assert_eq!(secret.resolve().unwrap(), "my-secret");
    }

    #[test]
    fn test_from_path() {
        let mut file = NamedTempFile::new().unwrap();
        write!(file, "file-secret").unwrap();

        let secret = SecretString::FromPath {
            path: file.path().to_path_buf(),
        };
        assert_eq!(secret.resolve().unwrap(), "file-secret");
    }

    #[test]
    fn test_from_path_trims_trailing_whitespace() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "file-secret").unwrap();
        writeln!(file).unwrap();

        let secret = SecretString::FromPath {
            path: file.path().to_path_buf(),
        };
        assert_eq!(secret.resolve().unwrap(), "file-secret");
    }

    #[test]
    fn test_from_path_file_not_found() {
        let secret = SecretString::FromPath {
            path: PathBuf::from("/nonexistent/path"),
        };
        let result = secret.resolve();
        assert!(matches!(result, Err(SecretResolutionError::FileRead { .. })));
    }

    #[test]
    fn test_deserialize_inline() {
        let toml = r#"key = "inline-value""#;

        #[derive(Deserialize)]
        struct Config {
            key: SecretString,
        }

        let config: Config = toml::from_str(toml).unwrap();
        assert_eq!(config.key.resolve().unwrap(), "inline-value");
    }

    #[test]
    fn test_deserialize_from_path() {
        let mut file = NamedTempFile::new().unwrap();
        write!(file, "path-value").unwrap();

        let toml = format!(r#"key = {{ path = "{}" }}"#, file.path().display());

        #[derive(Deserialize)]
        struct Config {
            key: SecretString,
        }

        let config: Config = toml::from_str(&toml).unwrap();
        assert_eq!(config.key.resolve().unwrap(), "path-value");
    }
}
