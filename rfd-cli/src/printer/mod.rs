mod json;
mod tab;

pub use json::RfdJsonPrinter;
use rfd_sdk::types;
pub use tab::RfdTabPrinter;

use crate::generated::cli::CliOutput;

pub enum Printer {
    Json(RfdJsonPrinter),
    Tab(RfdTabPrinter),
}

impl CliOutput for Printer {
    fn output_create_api_user(
        &self,
        response: Result<types::ApiUserForApiPermission, progenitor_client::Error<types::Error>>,
    ) {
        match self {
            Printer::Json(printer) => printer.output_create_api_user(response),
            Printer::Tab(printer) => printer.output_create_api_user(response),
        }
    }

    fn output_get_api_user(
        &self,
        response: Result<types::ApiUserForApiPermission, progenitor_client::Error<types::Error>>,
    ) {
        match self {
            Printer::Json(printer) => printer.output_get_api_user(response),
            Printer::Tab(printer) => printer.output_get_api_user(response),
        }
    }

    fn output_update_api_user(
        &self,
        response: Result<types::ApiUserForApiPermission, progenitor_client::Error<types::Error>>,
    ) {
        match self {
            Printer::Json(printer) => printer.output_update_api_user(response),
            Printer::Tab(printer) => printer.output_update_api_user(response),
        }
    }

    fn output_add_api_user_to_group(
        &self,
        response: Result<types::ApiUserForApiPermission, progenitor_client::Error<types::Error>>,
    ) {
        match self {
            Printer::Json(printer) => printer.output_add_api_user_to_group(response),
            Printer::Tab(printer) => printer.output_add_api_user_to_group(response),
        }
    }

    fn output_remove_api_user_from_group(
        &self,
        response: Result<types::ApiUserForApiPermission, progenitor_client::Error<types::Error>>,
    ) {
        match self {
            Printer::Json(printer) => printer.output_remove_api_user_from_group(response),
            Printer::Tab(printer) => printer.output_remove_api_user_from_group(response),
        }
    }

    fn output_list_api_user_tokens(
        &self,
        response: Result<Vec<types::ApiKeyResponse>, progenitor_client::Error<types::Error>>,
    ) {
        match self {
            Printer::Json(printer) => printer.output_list_api_user_tokens(response),
            Printer::Tab(printer) => printer.output_list_api_user_tokens(response),
        }
    }

    fn output_create_api_user_token(
        &self,
        response: Result<types::InitialApiKeyResponse, progenitor_client::Error<types::Error>>,
    ) {
        match self {
            Printer::Json(printer) => printer.output_create_api_user_token(response),
            Printer::Tab(printer) => printer.output_create_api_user_token(response),
        }
    }

    fn output_get_api_user_token(
        &self,
        response: Result<types::ApiKeyResponse, progenitor_client::Error<types::Error>>,
    ) {
        match self {
            Printer::Json(printer) => printer.output_get_api_user_token(response),
            Printer::Tab(printer) => printer.output_get_api_user_token(response),
        }
    }

    fn output_delete_api_user_token(
        &self,
        response: Result<types::ApiKeyResponse, progenitor_client::Error<types::Error>>,
    ) {
        match self {
            Printer::Json(printer) => printer.output_delete_api_user_token(response),
            Printer::Tab(printer) => printer.output_delete_api_user_token(response),
        }
    }

    fn output_github_webhook(&self, response: Result<(), progenitor_client::Error<types::Error>>) {
        match self {
            Printer::Json(printer) => printer.output_github_webhook(response),
            Printer::Tab(printer) => printer.output_github_webhook(response),
        }
    }

    fn output_get_groups(
        &self,
        response: Result<
            Vec<types::AccessGroupForApiPermission>,
            progenitor_client::Error<types::Error>,
        >,
    ) {
        match self {
            Printer::Json(printer) => printer.output_get_groups(response),
            Printer::Tab(printer) => printer.output_get_groups(response),
        }
    }

    fn output_create_group(
        &self,
        response: Result<
            types::AccessGroupForApiPermission,
            progenitor_client::Error<types::Error>,
        >,
    ) {
        match self {
            Printer::Json(printer) => printer.output_create_group(response),
            Printer::Tab(printer) => printer.output_create_group(response),
        }
    }

    fn output_update_group(
        &self,
        response: Result<
            types::AccessGroupForApiPermission,
            progenitor_client::Error<types::Error>,
        >,
    ) {
        match self {
            Printer::Json(printer) => printer.output_update_group(response),
            Printer::Tab(printer) => printer.output_update_group(response),
        }
    }

    fn output_delete_group(
        &self,
        response: Result<
            types::AccessGroupForApiPermission,
            progenitor_client::Error<types::Error>,
        >,
    ) {
        match self {
            Printer::Json(printer) => printer.output_delete_group(response),
            Printer::Tab(printer) => printer.output_delete_group(response),
        }
    }

    fn output_authz_code_redirect(&self, response: Result<(), progenitor_client::Error<()>>) {
        match self {
            Printer::Json(printer) => printer.output_authz_code_redirect(response),
            Printer::Tab(printer) => printer.output_authz_code_redirect(response),
        }
    }

    fn output_authz_code_callback(
        &self,
        response: Result<(), progenitor_client::Error<types::Error>>,
    ) {
        match self {
            Printer::Json(printer) => printer.output_authz_code_callback(response),
            Printer::Tab(printer) => printer.output_authz_code_callback(response),
        }
    }

    fn output_authz_code_exchange(
        &self,
        response: Result<
            types::OAuthAuthzCodeExchangeResponse,
            progenitor_client::Error<types::Error>,
        >,
    ) {
        match self {
            Printer::Json(printer) => printer.output_authz_code_exchange(response),
            Printer::Tab(printer) => printer.output_authz_code_exchange(response),
        }
    }

    fn output_get_device_provider(
        &self,
        response: Result<types::OAuthProviderInfo, progenitor_client::Error<types::Error>>,
    ) {
        match self {
            Printer::Json(printer) => printer.output_get_device_provider(response),
            Printer::Tab(printer) => printer.output_get_device_provider(response),
        }
    }

    fn output_exchange_device_token(&self, response: Result<(), progenitor_client::Error<()>>) {
        match self {
            Printer::Json(printer) => printer.output_exchange_device_token(response),
            Printer::Tab(printer) => printer.output_exchange_device_token(response),
        }
    }

    fn output_list_oauth_clients(
        &self,
        response: Result<Vec<types::OAuthClient>, progenitor_client::Error<types::Error>>,
    ) {
        match self {
            Printer::Json(printer) => printer.output_list_oauth_clients(response),
            Printer::Tab(printer) => printer.output_list_oauth_clients(response),
        }
    }

    fn output_create_oauth_client(
        &self,
        response: Result<types::OAuthClient, progenitor_client::Error<types::Error>>,
    ) {
        match self {
            Printer::Json(printer) => printer.output_create_oauth_client(response),
            Printer::Tab(printer) => printer.output_create_oauth_client(response),
        }
    }

    fn output_get_oauth_client(
        &self,
        response: Result<types::OAuthClient, progenitor_client::Error<types::Error>>,
    ) {
        match self {
            Printer::Json(printer) => printer.output_get_oauth_client(response),
            Printer::Tab(printer) => printer.output_get_oauth_client(response),
        }
    }

    fn output_create_oauth_client_redirect_uri(
        &self,
        response: Result<types::OAuthClientRedirectUri, progenitor_client::Error<types::Error>>,
    ) {
        match self {
            Printer::Json(printer) => printer.output_create_oauth_client_redirect_uri(response),
            Printer::Tab(printer) => printer.output_create_oauth_client_redirect_uri(response),
        }
    }

    fn output_delete_oauth_client_redirect_uri(
        &self,
        response: Result<types::OAuthClientRedirectUri, progenitor_client::Error<types::Error>>,
    ) {
        match self {
            Printer::Json(printer) => printer.output_delete_oauth_client_redirect_uri(response),
            Printer::Tab(printer) => printer.output_delete_oauth_client_redirect_uri(response),
        }
    }

    fn output_create_oauth_client_secret(
        &self,
        response: Result<
            types::InitialOAuthClientSecretResponse,
            progenitor_client::Error<types::Error>,
        >,
    ) {
        match self {
            Printer::Json(printer) => printer.output_create_oauth_client_secret(response),
            Printer::Tab(printer) => printer.output_create_oauth_client_secret(response),
        }
    }

    fn output_delete_oauth_client_secret(
        &self,
        response: Result<types::OAuthClientSecret, progenitor_client::Error<types::Error>>,
    ) {
        match self {
            Printer::Json(printer) => printer.output_delete_oauth_client_secret(response),
            Printer::Tab(printer) => printer.output_delete_oauth_client_secret(response),
        }
    }

    fn output_get_rfds(
        &self,
        response: Result<Vec<types::ListRfd>, progenitor_client::Error<types::Error>>,
    ) {
        match self {
            Printer::Json(printer) => printer.output_get_rfds(response),
            Printer::Tab(printer) => printer.output_get_rfds(response),
        }
    }

    fn output_get_rfd(
        &self,
        response: Result<types::FullRfd, progenitor_client::Error<types::Error>>,
    ) {
        match self {
            Printer::Json(printer) => printer.output_get_rfd(response),
            Printer::Tab(printer) => printer.output_get_rfd(response),
        }
    }

    fn output_search_rfds(
        &self,
        response: Result<Vec<types::ListRfd>, progenitor_client::Error<types::Error>>,
    ) {
        match self {
            Printer::Json(printer) => printer.output_search_rfds(response),
            Printer::Tab(printer) => printer.output_search_rfds(response),
        }
    }

    fn output_get_self(
        &self,
        response: Result<types::ApiUserForApiPermission, progenitor_client::Error<types::Error>>,
    ) {
        match self {
            Printer::Json(printer) => printer.output_get_self(response),
            Printer::Tab(printer) => printer.output_get_self(response),
        }
    }
}
