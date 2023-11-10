use std::time::Duration;

use anyhow::Result;
use oauth2::basic::BasicClient;
use oauth2::basic::{BasicErrorResponseType, BasicTokenType};
use oauth2::devicecode::{DeviceCodeErrorResponseType, StandardDeviceAuthorizationResponse};
use oauth2::reqwest::async_http_client;
use oauth2::{
    AuthType, AuthUrl, ClientId, DeviceAuthorizationUrl, EmptyExtraTokenFields, RequestTokenError,
    Scope, StandardErrorResponse, StandardTokenResponse, TokenUrl,
};
use rfd_sdk::types::OAuthProviderInfo;

type DeviceAuthorizationCodeError = RequestTokenError<
    oauth2::reqwest::Error<reqwest::Error>,
    StandardErrorResponse<BasicErrorResponseType>,
>;
type DeviceCodeExchangeError = RequestTokenError<
    oauth2::reqwest::Error<reqwest::Error>,
    StandardErrorResponse<DeviceCodeErrorResponseType>,
>;

pub struct DeviceOAuth {
    client: BasicClient,
    scopes: Vec<String>,
}

impl DeviceOAuth {
    pub fn new(provider: OAuthProviderInfo) -> Result<Self> {
        let device_auth_url = DeviceAuthorizationUrl::new(provider.device_code_endpoint)?;

        let client = BasicClient::new(
            ClientId::new(provider.client_id),
            None,
            AuthUrl::new(provider.auth_url_endpoint)?,
            Some(TokenUrl::new(provider.token_endpoint)?),
        )
        .set_auth_type(AuthType::RequestBody)
        .set_device_authorization_url(device_auth_url);

        Ok(Self {
            client,
            scopes: provider.scopes,
        })
    }

    pub async fn login(
        &self,
        details: &StandardDeviceAuthorizationResponse,
    ) -> Result<StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>, DeviceCodeExchangeError>
    {
        let token = self
            .client
            .exchange_device_access_token(&details)
            .set_max_backoff_interval(details.interval())
            .request_async(
                async_http_client,
                tokio::time::sleep,
                Some(details.expires_in()),
            )
            .await;

        token
    }

    pub async fn get_device_authorization(
        &self,
    ) -> Result<StandardDeviceAuthorizationResponse, DeviceAuthorizationCodeError> {
        let mut req = self
            .client
            .exchange_device_code()
            // This can be unwrapped as the the device url is configured during the creation of a
            // GoogleDeviceAuth
            .unwrap();

        for scope in &self.scopes {
            req = req.add_scope(Scope::new(scope.to_string()));
        }

        let res = req.request_async(async_http_client).await;

        res
    }
}
