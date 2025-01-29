// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

mod json;
mod tab;

pub use json::RfdJsonPrinter;
use rfd_sdk::types;
pub use tab::RfdTabPrinter;

#[derive(Debug, Clone)]
pub enum Printer {
    Json(RfdJsonPrinter),
    Tab(RfdTabPrinter),
}

pub trait CliOutput {
    fn output_api_user(&self, value: types::ApiUserForRfdPermission) {}
    fn output_user(&self, value: types::GetUserResponseForRfdPermission) {}
    fn output_api_key_list(&self, value: Vec<types::ApiKeyResponseForRfdPermission>) {}
    fn output_api_key_initial(&self, value: types::InitialApiKeyResponseForRfdPermission) {}
    fn output_api_key(&self, value: types::ApiKeyResponseForRfdPermission) {}
    fn output_group_list(&self, value: Vec<types::AccessGroupForRfdPermission>) {}
    fn output_group(&self, value: types::AccessGroupForRfdPermission) {}
    fn output_mapper_list(&self, value: Vec<types::Mapper>) {}
    fn output_mapper(&self, value: types::Mapper) {}
    fn output_oauth_client_list(&self, value: Vec<types::OAuthClient>) {}
    fn output_oauth_client(&self, value: types::OAuthClient) {}
    fn output_oauth_redirect_uri(&self, value: types::OAuthClientRedirectUri) {}
    fn output_oauth_secret_initial(&self, value: types::InitialOAuthClientSecretResponse) {}
    fn output_oauth_secret(&self, value: types::OAuthClientSecret) {}
    fn output_rfd_list(&self, value: Vec<types::RfdWithoutContent>) {}
    fn output_rfd_full(&self, value: types::RfdWithRaw) {}
    fn output_rfd(&self, value: types::Rfd) {}
    fn output_rfd_attr(&self, value: types::RfdAttr) {}
    fn output_search_results(&self, value: types::SearchResults) {}
    fn output_reserved_rfd(&self, value: types::ReserveRfdResponse) {}
    fn output_error<T>(&self, value: &progenitor_client::Error<T>)
    where
        T: schemars::JsonSchema + serde::Serialize + std::fmt::Debug;
}

impl CliOutput for Printer {
    fn output_api_user(&self, value: types::ApiUserForRfdPermission) {
        match self {
            Self::Json(printer) => printer.output_api_user(value),
            Self::Tab(printer) => printer.output_api_user(value),
        }
    }

    fn output_user(&self, value: types::GetUserResponseForRfdPermission) {
        match self {
            Self::Json(printer) => printer.output_user(value),
            Self::Tab(printer) => printer.output_user(value),
        }
    }

    fn output_api_key_list(&self, value: Vec<types::ApiKeyResponseForRfdPermission>) {
        match self {
            Self::Json(printer) => printer.output_api_key_list(value),
            Self::Tab(printer) => printer.output_api_key_list(value),
        }
    }

    fn output_api_key_initial(&self, value: types::InitialApiKeyResponseForRfdPermission) {
        match self {
            Self::Json(printer) => printer.output_api_key_initial(value),
            Self::Tab(printer) => printer.output_api_key_initial(value),
        }
    }

    fn output_api_key(&self, value: types::ApiKeyResponseForRfdPermission) {
        match self {
            Self::Json(printer) => printer.output_api_key(value),
            Self::Tab(printer) => printer.output_api_key(value),
        }
    }

    fn output_group_list(&self, value: Vec<types::AccessGroupForRfdPermission>) {
        match self {
            Self::Json(printer) => printer.output_group_list(value),
            Self::Tab(printer) => printer.output_group_list(value),
        }
    }

    fn output_group(&self, value: types::AccessGroupForRfdPermission) {
        match self {
            Self::Json(printer) => printer.output_group(value),
            Self::Tab(printer) => printer.output_group(value),
        }
    }

    fn output_mapper_list(&self, value: Vec<types::Mapper>) {
        match self {
            Self::Json(printer) => printer.output_mapper_list(value),
            Self::Tab(printer) => printer.output_mapper_list(value),
        }
    }

    fn output_mapper(&self, value: types::Mapper) {
        match self {
            Self::Json(printer) => printer.output_mapper(value),
            Self::Tab(printer) => printer.output_mapper(value),
        }
    }

    fn output_oauth_client_list(&self, value: Vec<types::OAuthClient>) {
        match self {
            Self::Json(printer) => printer.output_oauth_client_list(value),
            Self::Tab(printer) => printer.output_oauth_client_list(value),
        }
    }

    fn output_oauth_client(&self, value: types::OAuthClient) {
        match self {
            Self::Json(printer) => printer.output_oauth_client(value),
            Self::Tab(printer) => printer.output_oauth_client(value),
        }
    }

    fn output_oauth_redirect_uri(&self, value: types::OAuthClientRedirectUri) {
        match self {
            Self::Json(printer) => printer.output_oauth_redirect_uri(value),
            Self::Tab(printer) => printer.output_oauth_redirect_uri(value),
        }
    }

    fn output_oauth_secret_initial(&self, value: types::InitialOAuthClientSecretResponse) {
        match self {
            Self::Json(printer) => printer.output_oauth_secret_initial(value),
            Self::Tab(printer) => printer.output_oauth_secret_initial(value),
        }
    }

    fn output_oauth_secret(&self, value: types::OAuthClientSecret) {
        match self {
            Self::Json(printer) => printer.output_oauth_secret(value),
            Self::Tab(printer) => printer.output_oauth_secret(value),
        }
    }

    fn output_rfd_list(&self, value: Vec<types::RfdWithoutContent>) {
        match self {
            Self::Json(printer) => printer.output_rfd_list(value),
            Self::Tab(printer) => printer.output_rfd_list(value),
        }
    }

    fn output_rfd_full(&self, value: types::RfdWithRaw) {
        match self {
            Self::Json(printer) => printer.output_rfd_full(value),
            Self::Tab(printer) => printer.output_rfd_full(value),
        }
    }

    fn output_rfd(&self, value: types::Rfd) {
        match self {
            Self::Json(printer) => printer.output_rfd(value),
            Self::Tab(printer) => printer.output_rfd(value),
        }
    }

    fn output_rfd_attr(&self, value: types::RfdAttr) {
        match self {
            Self::Json(printer) => printer.output_rfd_attr(value),
            Self::Tab(printer) => printer.output_rfd_attr(value),
        }
    }

    fn output_search_results(&self, value: types::SearchResults) {
        match self {
            Self::Json(printer) => printer.output_search_results(value),
            Self::Tab(printer) => printer.output_search_results(value),
        }
    }

    fn output_reserved_rfd(&self, value: types::ReserveRfdResponse) {
        match self {
            Self::Json(printer) => printer.output_reserved_rfd(value),
            Self::Tab(printer) => printer.output_reserved_rfd(value),
        }
    }

    fn output_error<T>(&self, value: &progenitor_client::Error<T>)
    where
        T: schemars::JsonSchema + serde::Serialize + std::fmt::Debug,
    {
        match self {
            Self::Json(printer) => printer.output_error(value),
            Self::Tab(printer) => printer.output_error(value),
        }
    }
}
