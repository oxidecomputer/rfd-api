// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use chrono::{TimeDelta, Utc};
use clap::Subcommand;
use rfd_sdk::types::{
    OAuthProviderAuthorizationCodePkceInfo, OAuthProviderDeviceInfo, OAuthProviderName,
    TypedUuidForOAuthClientId,
};
use std::{future::Future, ops::Add, pin::Pin};
use uuid::Uuid;
use v_cli_sdk::cmd::auth::{
    login::{CliAdapterToken, CliMagicLinkAdapter, LoginProvider as VLoginProvider},
    oauth::{AuthorizationCodeExchange, CliOAuthAdapter, CliOAuthProviderInfo},
};

use crate::context::{Context, ContextError};

/// Identity providers supported for CLI login.
#[derive(Debug, Copy, Clone, Subcommand)]
pub enum LoginProvider {
    /// GitHub identity provider
    #[command(name = "github")]
    GitHub,
    /// Google identity provider
    Google,
}

impl From<LoginProvider> for VLoginProvider {
    fn from(value: LoginProvider) -> Self {
        match value {
            LoginProvider::GitHub => Self::GitHub,
            LoginProvider::Google => Self::Google,
        }
    }
}

pub struct AdapterToken {
    token: String,
    idp_token: Option<String>,
}

impl CliAdapterToken for AdapterToken {
    fn access_token(&self) -> &str {
        &self.token
    }

    fn idp_token(&self) -> Option<&str> {
        self.idp_token.as_deref()
    }
}

pub struct OAuthAdapter {
    ctx: Context,
}

impl OAuthAdapter {
    pub fn new(ctx: Context) -> Self {
        Self { ctx }
    }
}

impl CliOAuthAdapter for OAuthAdapter {
    type ShortToken = AdapterToken;
    type LongToken = AdapterToken;
    type Error = ContextError;

    fn provider(
        &self,
        provider: VLoginProvider,
    ) -> Pin<Box<dyn Future<Output = Result<impl CliOAuthProviderInfo, Self::Error>> + Send>> {
        let client = self.ctx.require_client().ok();
        Box::pin(async move {
            let client = client.ok_or(ContextError::NoClient)?;
            let provider_name = match provider {
                VLoginProvider::GitHub => OAuthProviderName::Github,
                VLoginProvider::Google => OAuthProviderName::Google,
                _ => return Err(ContextError::UnsupportedOAuthProvider),
            };

            // Try the PKCE authorization code flow first, falling back to the
            // device flow for providers that do not support it.
            if let Ok(resp) = client
                .get_web_pkce_provider()
                .provider(provider_name)
                .send()
                .await
            {
                return Ok(OAuthProvider {
                    provider,
                    info: OAuthProviderInfo::Pkce(resp.into_inner()),
                    scope: vec!["full".to_string()],
                });
            }

            if let Ok(resp) = client
                .get_device_provider()
                .provider(provider_name)
                .send()
                .await
            {
                return Ok(OAuthProvider {
                    provider,
                    info: OAuthProviderInfo::Device(resp.into_inner()),
                    scope: vec!["full".to_string()],
                });
            }

            Err(ContextError::UnsupportedOAuthProvider)
        })
    }

    fn exchange_authorization_code(
        &self,
        exchange: AuthorizationCodeExchange,
    ) -> Pin<Box<dyn Future<Output = Result<Self::ShortToken, Self::Error>> + Send>> {
        let client = self.ctx.require_client().ok();
        Box::pin(async move {
            let client = client.ok_or(ContextError::NoClient)?;
            let provider = match exchange.provider {
                VLoginProvider::GitHub => OAuthProviderName::Github,
                VLoginProvider::Google => OAuthProviderName::Google,
                _ => return Err(ContextError::UnsupportedOAuthProvider),
            };
            let response = client
                .authz_code_exchange()
                .provider(provider)
                .request_idp_token(exchange.request_idp_token)
                .body_map(|body| {
                    body.client_id(TypedUuidForOAuthClientId(exchange.client_id))
                        .code(exchange.code)
                        .redirect_uri(exchange.redirect_uri)
                        .grant_type(exchange.grant_type)
                        .code_verifier(exchange.pkce_verifier.secret().to_string())
                })
                .send()
                .await
                .map_err(|err| ContextError::Sdk(err.to_string()))?
                .into_inner();
            Ok(AdapterToken {
                token: response.access_token,
                idp_token: response.idp_token,
            })
        })
    }

    fn get_long_lived_token(
        &self,
        access_token: &str,
    ) -> Pin<Box<dyn Future<Output = Result<Self::LongToken, Self::Error>> + Send>> {
        let token = access_token.to_string();
        let host = self.ctx.host().ok_or(ContextError::NoHost);

        Box::pin(async move {
            let client =
                Context::new_client(&host?, Some(&token)).map_err(|_| ContextError::NoClient)?;
            let user = client.get_self().send().await?;
            let key = client
                .create_api_user_token()
                .user_id(user.info.id.clone())
                .body_map(|body| {
                    body.permission_boundary(None)
                        .expires_at(Utc::now().add(TimeDelta::try_days(365).unwrap()))
                })
                .send()
                .await?
                .into_inner();
            Ok(AdapterToken {
                token: key.key.0,
                idp_token: None,
            })
        })
    }
}

pub struct MagicLinkAdapter {}

impl CliMagicLinkAdapter for MagicLinkAdapter {
    type Attempt = rfd_sdk::types::MagicLinkExchangeResponse;
    type Token = AdapterToken;
    type Error = ContextError;

    fn create_attempt(
        &self,
        _email: &str,
        _scope: Option<&str>,
    ) -> Pin<Box<dyn Future<Output = Result<Self::Attempt, Self::Error>> + Send>> {
        Box::pin(async { Err(ContextError::UnsupportedOAuthProvider) })
    }

    fn exchange(
        &self,
        _attempt: Self::Attempt,
        _email: &str,
        _token: &str,
    ) -> Pin<Box<dyn Future<Output = Result<Self::Token, Self::Error>> + Send>> {
        Box::pin(async { Err(ContextError::UnsupportedOAuthProvider) })
    }
}

pub struct OAuthProvider {
    provider: VLoginProvider,
    info: OAuthProviderInfo,
    scope: Vec<String>,
}

pub enum OAuthProviderInfo {
    Device(OAuthProviderDeviceInfo),
    Pkce(OAuthProviderAuthorizationCodePkceInfo),
}

impl CliOAuthProviderInfo for OAuthProvider {
    fn provider(&self) -> VLoginProvider {
        self.provider
    }
    fn client_id(&self) -> Uuid {
        match &self.info {
            OAuthProviderInfo::Device(inner) => inner.client_id.0,
            OAuthProviderInfo::Pkce(inner) => inner.client_id.0,
        }
    }
    fn public_pkce_port(&self) -> Option<u16> {
        match &self.info {
            OAuthProviderInfo::Device(_) => None,
            OAuthProviderInfo::Pkce(inner) => Some(inner.proxy_port),
        }
    }
    fn supports_pkce_only(&self) -> bool {
        match &self.info {
            OAuthProviderInfo::Device(_) => false,
            OAuthProviderInfo::Pkce(_) => true,
        }
    }
    fn auth_url_endpoint(&self) -> Option<&str> {
        match &self.info {
            OAuthProviderInfo::Device(_) => None,
            OAuthProviderInfo::Pkce(inner) => Some(&inner.web.auth_url_endpoint),
        }
    }
    fn token_endpoint(&self) -> &str {
        match &self.info {
            OAuthProviderInfo::Device(inner) => &inner.token_endpoint,
            OAuthProviderInfo::Pkce(inner) => &inner.web.token_endpoint,
        }
    }
    fn redirect_endpoint(&self) -> Option<&str> {
        match &self.info {
            OAuthProviderInfo::Device(_) => None,
            OAuthProviderInfo::Pkce(inner) => Some(&inner.redirect_endpoint),
        }
    }
    fn scopes(&self) -> &[String] {
        &self.scope
    }
    fn device_authorization_endpoint(&self) -> Option<&str> {
        match &self.info {
            OAuthProviderInfo::Device(inner) => Some(&inner.auth_url_endpoint),
            OAuthProviderInfo::Pkce(_) => None,
        }
    }
    fn device_token_endpoint(&self) -> Option<&str> {
        match &self.info {
            OAuthProviderInfo::Device(inner) => Some(&inner.token_endpoint),
            OAuthProviderInfo::Pkce(_) => None,
        }
    }
}
