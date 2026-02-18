// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use rfd_sdk::types;
use serde::Serialize;

use super::CliOutput;

#[derive(Debug, Clone)]
pub struct RfdJsonPrinter;

impl CliOutput for RfdJsonPrinter {
    fn output_api_user(&self, value: types::ApiUserForRfdPermission) {
        println!("{}", serde_json::to_string(&value).unwrap())
    }

    fn output_api_user_list(&self, value: Vec<types::GetUserResponseForRfdPermission>) {
        println!("{}", serde_json::to_string(&value).unwrap())
    }

    fn output_user(&self, value: types::GetUserResponseForRfdPermission) {
        println!("{}", serde_json::to_string(&value).unwrap())
    }

    fn output_api_user_contact_email(&self, value: types::ApiUserContactEmail) {
        println!("{}", serde_json::to_string(&value).unwrap())
    }

    fn output_api_user_link_request_response(&self, value: types::ApiUserLinkRequestResponse) {
        println!("{}", serde_json::to_string(&value).unwrap())
    }

    fn output_api_key_list(&self, value: Vec<types::ApiKeyResponseForRfdPermission>) {
        println!("{}", serde_json::to_string(&value).unwrap())
    }

    fn output_api_key_initial(&self, value: types::InitialApiKeyResponseForRfdPermission) {
        println!("{}", serde_json::to_string(&value).unwrap())
    }

    fn output_api_key(&self, value: types::ApiKeyResponseForRfdPermission) {
        println!("{}", serde_json::to_string(&value).unwrap())
    }

    fn output_group_list(&self, value: Vec<types::AccessGroupForRfdPermission>) {
        println!("{}", serde_json::to_string(&value).unwrap())
    }

    fn output_group(&self, value: types::AccessGroupForRfdPermission) {
        println!("{}", serde_json::to_string(&value).unwrap())
    }

    fn output_mapper_list(&self, value: Vec<types::Mapper>) {
        println!("{}", serde_json::to_string(&value).unwrap())
    }

    fn output_mapper(&self, value: types::Mapper) {
        println!("{}", serde_json::to_string(&value).unwrap())
    }

    fn output_oauth_client_list(&self, value: Vec<types::OAuthClient>) {
        println!("{}", serde_json::to_string(&value).unwrap())
    }

    fn output_oauth_client(&self, value: types::OAuthClient) {
        println!("{}", serde_json::to_string(&value).unwrap())
    }

    fn output_oauth_redirect_uri(&self, value: types::OAuthClientRedirectUri) {
        println!("{}", serde_json::to_string(&value).unwrap())
    }

    fn output_oauth_secret_initial(&self, value: types::InitialOAuthClientSecretResponse) {
        println!("{}", serde_json::to_string(&value).unwrap())
    }

    fn output_oauth_secret(&self, value: types::OAuthClientSecret) {
        println!("{}", serde_json::to_string(&value).unwrap())
    }

    fn output_oauth_authz_code_exchange_response(
        &self,
        value: types::OAuthAuthzCodeExchangeResponse,
    ) {
        println!("{}", serde_json::to_string(&value).unwrap())
    }

    fn output_oauth_provider_info(&self, value: types::OAuthProviderInfo) {
        println!("{}", serde_json::to_string(&value).unwrap())
    }

    fn output_rfd_meta(&self, value: types::RfdWithoutContent) {
        println!("{}", serde_json::to_string(&value).unwrap())
    }

    fn output_rfd_list(&self, value: Vec<types::RfdWithoutContent>) {
        println!("{}", serde_json::to_string(&value).unwrap())
    }

    fn output_rfd_full(&self, value: types::RfdWithRaw) {
        println!("{}", serde_json::to_string(&value).unwrap())
    }

    fn output_rfd_with_pdf(&self, value: types::RfdWithPdf) {
        println!("{}", serde_json::to_string(&value).unwrap())
    }

    fn output_rfd(&self, value: types::Rfd) {
        println!("{}", serde_json::to_string(&value).unwrap())
    }

    fn output_search_results(&self, value: types::SearchResults) {
        println!("{}", serde_json::to_string(&value).unwrap())
    }

    fn output_rfd_attr(&self, value: types::RfdAttr) {
        println!("{}", serde_json::to_string(&value).unwrap())
    }

    fn output_rfd_revision_meta(&self, value: types::RfdRevisionMeta) {
        println!("{}", serde_json::to_string(&value).unwrap())
    }

    fn output_rfd_revision_meta_list(&self, value: Vec<types::RfdRevisionMeta>) {
        println!("{}", serde_json::to_string(&value).unwrap())
    }

    fn output_reserved_rfd(&self, value: types::ReserveRfdResponse) {
        println!("{}", serde_json::to_string(&value).unwrap())
    }

    fn output_job_list(&self, value: Vec<types::Job>) {
        println!("{}", serde_json::to_string(&value).unwrap())
    }

    fn output_magic_link_client_list(&self, value: Vec<types::MagicLink>) {
        println!("{}", serde_json::to_string(&value).unwrap())
    }

    fn output_magic_link_client(&self, value: types::MagicLink) {
        println!("{}", serde_json::to_string(&value).unwrap())
    }

    fn output_magic_link_redirect_uri(&self, value: types::MagicLinkRedirectUri) {
        println!("{}", serde_json::to_string(&value).unwrap())
    }

    fn output_magic_link_secret_initial(&self, value: types::InitialMagicLinkSecretResponse) {
        println!("{}", serde_json::to_string(&value).unwrap())
    }

    fn output_magic_link_secret(&self, value: types::MagicLinkSecret) {
        println!("{}", serde_json::to_string(&value).unwrap())
    }

    fn output_magic_link_exchange_response(&self, value: types::MagicLinkExchangeResponse) {
        println!("{}", serde_json::to_string(&value).unwrap())
    }

    fn output_magic_link_send_response(&self, value: types::MagicLinkSendResponse) {
        println!("{}", serde_json::to_string(&value).unwrap())
    }

    fn output_jwks(&self, value: types::Jwks) {
        println!("{}", serde_json::to_string(&value).unwrap())
    }

    fn output_openid_configuration(&self, value: types::OpenIdConfiguration) {
        println!("{}", serde_json::to_string(&value).unwrap())
    }

    fn output_error<T>(&self, value: &progenitor_client::Error<T>)
    where
        T: schemars::JsonSchema + serde::Serialize + std::fmt::Debug,
    {
        eprintln!("{}", value)
    }
}
