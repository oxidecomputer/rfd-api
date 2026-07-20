// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::time::Duration;

use anyhow::anyhow;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use rfd_sdk::Client;
use thiserror::Error;
use v_cli_sdk::{
    cmd::auth::{login::CliMagicLinkAdapter, oauth::CliOAuthAdapter},
    printer::Printer,
    VCliConfig, VCliContext, VerbosityLevel,
};

use crate::{
    auth::{AdapterToken, MagicLinkAdapter, OAuthAdapter},
    store::CliConfig,
};

#[derive(Debug, Error)]
pub enum ContextError {
    #[error(transparent)]
    Client(#[from] rfd_sdk::Error<rfd_sdk::types::Error>),
    #[error("No client configured. Run `rfd config set host <HOST>` to configure a host.")]
    NoClient,
    #[error("No host configured. Run `rfd config set host <HOST>` to configure a host.")]
    NoHost,
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
    #[error("SDK error: {0}")]
    Sdk(String),
    #[error("Unsupported OAuth provider")]
    UnsupportedOAuthProvider,
}

#[derive(Debug, Clone)]
pub struct Context {
    config: CliConfig,
    printer: Option<Printer>,
    verbosity: VerbosityLevel,
}

impl Context {
    pub fn new() -> anyhow::Result<Self> {
        let config = CliConfig::new()?;

        Ok(Self {
            config,
            printer: None,
            verbosity: VerbosityLevel::None,
        })
    }

    pub fn new_client(host: &str, token: Option<&str>) -> anyhow::Result<Client> {
        let mut default_headers = HeaderMap::new();

        if let Some(token) = token {
            let mut auth_header = HeaderValue::from_str(&format!("Bearer {}", token))?;
            auth_header.set_sensitive(true);
            default_headers.insert(AUTHORIZATION, auth_header);
        }

        let http_client = reqwest::Client::builder()
            .default_headers(default_headers)
            .connect_timeout(Duration::from_secs(5))
            .timeout(Duration::from_secs(10))
            .build()?;

        Ok(Client::new_with_client(host, http_client))
    }

    /// Build a client, returning a helpful error when no host has been
    /// configured. Used by the RFD-specific shortcut commands.
    pub fn require_client(&self) -> anyhow::Result<Client> {
        let host = self
            .config
            .host()
            .ok_or_else(|| anyhow!("Host must be configured. Run `rfd config set host <HOST>`."))?;
        Self::new_client(host, self.config.token())
    }

    /// The configured host, if any.
    pub fn host(&self) -> Option<String> {
        self.config.host().map(|s| s.to_string())
    }

    /// Access the configured printer, returning an error when none is set.
    /// Used by the RFD-specific shortcut commands.
    pub fn require_printer(&self) -> anyhow::Result<&Printer> {
        self.printer
            .as_ref()
            .ok_or_else(|| anyhow!("No printer configured"))
    }

    pub fn set_printer(&mut self, printer: Option<Printer>) {
        self.printer = printer;
    }

    pub fn set_verbosity(&mut self, verbosity: VerbosityLevel) {
        self.verbosity = verbosity;
    }
}

impl VCliContext<Client, Printer> for Context {
    type ShortToken = AdapterToken;
    type LongToken = AdapterToken;
    type Error = ContextError;

    fn config(&self) -> &impl VCliConfig {
        &self.config
    }
    fn config_mut(&mut self) -> &mut impl VCliConfig {
        &mut self.config
    }
    fn client(&self) -> Option<Client> {
        Self::new_client(self.config.host()?, self.config.token()).ok()
    }
    fn printer(&self) -> Option<&Printer> {
        self.printer.as_ref()
    }
    fn verbosity(&self) -> VerbosityLevel {
        self.verbosity
    }

    fn oauth_adapter(
        &self,
    ) -> impl CliOAuthAdapter<
        ShortToken = Self::ShortToken,
        LongToken = Self::LongToken,
        Error = Self::Error,
    > + Send
           + Sync
           + 'static {
        OAuthAdapter::new(self.clone())
    }
    fn mlink_adapter(
        &self,
    ) -> impl CliMagicLinkAdapter<Token = Self::LongToken, Error = Self::Error> + Send + Sync + 'static
    {
        MagicLinkAdapter {}
    }
}
