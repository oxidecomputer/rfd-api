// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::{
    fs::{create_dir_all, File as StdFile, OpenOptions},
    io::Write,
    path::PathBuf,
};

use anyhow::Result;
use config::{Config, Environment, File};
use serde::{Deserialize, Serialize};
use v_cli_sdk::{FormatStyle, VCliConfig};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CliConfig {
    host: Option<String>,
    token: Option<String>,
    default_format: Option<FormatStyle>,
    mlink_redirect: Option<String>,
    mlink_secret: Option<String>,
}

impl CliConfig {
    pub fn new() -> Result<Self> {
        let (path, _) = Self::file(false)?;
        let config = Config::builder()
            .add_source(File::from(path))
            .add_source(Environment::default())
            .build()?;

        Ok(config.try_deserialize()?)
    }

    fn file(clear: bool) -> Result<(PathBuf, StdFile), std::io::Error> {
        let mut path = dirs::config_dir().expect("Failed to determine configs path");
        path.push("rfd-cli");
        create_dir_all(&path).expect("Failed to create configs path");

        path.push("config.toml");
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(clear)
            .open(&path)?;

        Ok((path, file))
    }
}

impl VCliConfig for CliConfig {
    fn host(&self) -> Option<&str> {
        self.host.as_deref()
    }
    fn set_host(&mut self, host: String) {
        self.host = Some(host);
    }
    fn token(&self) -> Option<&str> {
        self.token.as_deref()
    }
    fn set_token(&mut self, token: String) {
        self.token = Some(token);
    }
    fn default_format(&self) -> FormatStyle {
        self.default_format.unwrap_or_default()
    }
    fn set_default_format(&mut self, format: FormatStyle) {
        self.default_format = Some(format);
    }
    fn mlink_redirect(&self) -> Option<&str> {
        self.mlink_redirect.as_deref()
    }
    fn set_mlink_redirect(&mut self, redirect: String) {
        self.mlink_redirect = Some(redirect);
    }
    fn mlink_secret(&self) -> Option<&str> {
        self.mlink_secret.as_deref()
    }
    fn set_mlink_secret(&mut self, secret: String) {
        self.mlink_secret = Some(secret);
    }
    fn save(&self) -> Result<(), std::io::Error> {
        let (filename, mut file) = Self::file(true)?;
        file.write_all(toml::to_string(&self).unwrap().as_bytes())?;

        println!("Configuration updated. Wrote to: {}", filename.display());
        Ok(())
    }
}
