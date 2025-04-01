// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::time::Duration;

use anyhow::Result;
use oauth2::basic::BasicClient;
use oauth2::basic::{BasicErrorResponseType, BasicTokenType};
use oauth2::{
    AuthType, AuthUrl, ClientId, DeviceAuthorizationUrl, EmptyExtraTokenFields, EndpointNotSet,
    EndpointSet, RequestTokenError, Scope, StandardErrorResponse, StandardTokenResponse, TokenUrl,
};
use oauth2::{DeviceCodeErrorResponseType, StandardDeviceAuthorizationResponse};
use rfd_sdk::types::OAuthProviderInfo;

type DeviceClient = BasicClient<
    // HasAuthUrl
    EndpointSet,
    // HasDeviceAuthUrl
    EndpointSet,
    // HasIntrospectionUrl
    EndpointNotSet,
    // HasRevocationUrl
    EndpointNotSet,
    // HasTokenUrl
    EndpointSet,
>;

pub struct DeviceOAuth {
    client: DeviceClient,
    http: reqwest::Client,
    scopes: Vec<String>,
}

impl DeviceOAuth {
    pub fn new(provider: OAuthProviderInfo) -> Result<Self> {
        let device_auth_url = DeviceAuthorizationUrl::new(provider.device_code_endpoint)?;

        let client = BasicClient::new(ClientId::new(provider.client_id))
            .set_auth_uri(AuthUrl::new(provider.auth_url_endpoint)?)
            .set_auth_type(AuthType::RequestBody)
            .set_token_uri(TokenUrl::new(provider.token_endpoint)?)
            .set_device_authorization_url(device_auth_url);

        Ok(Self {
            client,
            http: reqwest::ClientBuilder::new()
                .redirect(reqwest::redirect::Policy::none())
                .build()
                .unwrap(),
            scopes: provider.scopes,
        })
    }

    pub async fn login(
        &self,
        details: &StandardDeviceAuthorizationResponse,
    ) -> Result<StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>> {
        let token = self
            .client
            .exchange_device_access_token(&details)
            .set_max_backoff_interval(details.interval())
            .request_async(&self.http, tokio::time::sleep, Some(details.expires_in()))
            .await;

        Ok(token?)
    }

    pub async fn get_device_authorization(&self) -> Result<StandardDeviceAuthorizationResponse> {
        let mut req = self.client.exchange_device_code();

        for scope in &self.scopes {
            req = req.add_scope(Scope::new(scope.to_string()));
        }

        let res = req.request_async(&self.http).await;

        Ok(res?)
    }
}
