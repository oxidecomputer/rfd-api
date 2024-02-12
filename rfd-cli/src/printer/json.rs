// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use rfd_sdk::types;
use serde::Serialize;

use super::CliOutput;

#[derive(Debug, Clone)]
pub struct RfdJsonPrinter;

impl CliOutput for RfdJsonPrinter {
    fn output_api_user(&self, value: types::ApiUserForApiPermissionResponse) {
        println!("{}", serde_json::to_string(&value).unwrap())
    }

    fn output_user(&self, value: types::GetUserResponse) {
        println!("{}", serde_json::to_string(&value).unwrap())
    }

    fn output_api_key_list(&self, value: Vec<types::ApiKeyResponse>) {
        println!("{}", serde_json::to_string(&value).unwrap())
    }

    fn output_api_key_initial(&self, value: types::InitialApiKeyResponse) {
        println!("{}", serde_json::to_string(&value).unwrap())
    }

    fn output_api_key(&self, value: types::ApiKeyResponse) {
        println!("{}", serde_json::to_string(&value).unwrap())
    }

    fn output_group_list(&self, value: Vec<types::AccessGroupForApiPermissionResponse>) {
        println!("{}", serde_json::to_string(&value).unwrap())
    }

    fn output_group(&self, value: types::AccessGroupForApiPermissionResponse) {
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

    fn output_rfd_list(&self, value: Vec<types::ListRfd>) {
        println!("{}", serde_json::to_string(&value).unwrap())
    }

    fn output_rfd_full(&self, value: types::FullRfd) {
        println!("{}", serde_json::to_string(&value).unwrap())
    }

    fn output_rfd(&self, value: types::Rfd) {
        println!("{}", serde_json::to_string(&value).unwrap())
    }

    fn output_search_results(&self, value: types::SearchResults) {
        println!("{}", serde_json::to_string(&value).unwrap())
    }

    fn output_error<T>(&self, value: &progenitor_client::Error<T>)
    where
        T: schemars::JsonSchema + serde::Serialize + std::fmt::Debug,
    {
        eprintln!("{}", value)
    }
}
