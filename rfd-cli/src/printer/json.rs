use rfd_sdk::types;
use serde::Serialize;

use crate::generated::cli::CliOutput;

pub struct RfdJsonPrinter;

fn print_cli_output<T>(response: &Result<T, progenitor_client::Error<types::Error>>)
where
    T: Serialize,
{
    match response {
        Ok(res) => println!("{}", serde_json::to_string(&res).unwrap()),
        Err(err) => eprintln!("{}", err),
    }
}

impl CliOutput for RfdJsonPrinter {
    fn output_create_api_user(
        &self,
        response: Result<types::ApiUserForApiPermission, progenitor_client::Error<types::Error>>,
    ) {
        print_cli_output(&response)
    }

    fn output_get_api_user(
        &self,
        response: Result<types::GetApiUserResponse, progenitor_client::Error<types::Error>>,
    ) {
        print_cli_output(&response)
    }

    fn output_update_api_user(
        &self,
        response: Result<types::ApiUserForApiPermission, progenitor_client::Error<types::Error>>,
    ) {
        print_cli_output(&response)
    }

    fn output_add_api_user_to_group(
        &self,
        response: Result<types::ApiUserForApiPermission, progenitor_client::Error<types::Error>>,
    ) {
        print_cli_output(&response)
    }

    fn output_remove_api_user_from_group(
        &self,
        response: Result<types::ApiUserForApiPermission, progenitor_client::Error<types::Error>>,
    ) {
        print_cli_output(&response)
    }

    fn output_list_api_user_tokens(
        &self,
        response: Result<Vec<types::ApiKeyResponse>, progenitor_client::Error<types::Error>>,
    ) {
        print_cli_output(&response)
    }

    fn output_create_api_user_token(
        &self,
        response: Result<types::InitialApiKeyResponse, progenitor_client::Error<types::Error>>,
    ) {
        print_cli_output(&response)
    }

    fn output_get_api_user_token(
        &self,
        response: Result<types::ApiKeyResponse, progenitor_client::Error<types::Error>>,
    ) {
        print_cli_output(&response)
    }

    fn output_delete_api_user_token(
        &self,
        response: Result<types::ApiKeyResponse, progenitor_client::Error<types::Error>>,
    ) {
        print_cli_output(&response)
    }

    fn output_github_webhook(&self, response: Result<(), progenitor_client::Error<types::Error>>) {}

    fn output_get_groups(
        &self,
        response: Result<
            Vec<types::AccessGroupForApiPermission>,
            progenitor_client::Error<types::Error>,
        >,
    ) {
        print_cli_output(&response)
    }

    fn output_create_group(
        &self,
        response: Result<
            types::AccessGroupForApiPermission,
            progenitor_client::Error<types::Error>,
        >,
    ) {
        print_cli_output(&response)
    }

    fn output_update_group(
        &self,
        response: Result<
            types::AccessGroupForApiPermission,
            progenitor_client::Error<types::Error>,
        >,
    ) {
        print_cli_output(&response)
    }

    fn output_delete_group(
        &self,
        response: Result<
            types::AccessGroupForApiPermission,
            progenitor_client::Error<types::Error>,
        >,
    ) {
        print_cli_output(&response)
    }

    fn output_get_mappers(
        &self,
        response: Result<Vec<types::Mapper>, progenitor_client::Error<types::Error>>,
    ) {
        print_cli_output(&response)
    }

    fn output_create_mapper(
        &self,
        response: Result<types::Mapper, progenitor_client::Error<types::Error>>,
    ) {
        print_cli_output(&response)
    }

    fn output_delete_mapper(
        &self,
        response: Result<types::Mapper, progenitor_client::Error<types::Error>>,
    ) {
        print_cli_output(&response)
    }

    fn output_authz_code_redirect(&self, response: Result<(), progenitor_client::Error<()>>) {}

    fn output_authz_code_callback(
        &self,
        response: Result<(), progenitor_client::Error<types::Error>>,
    ) {
        print_cli_output(&response)
    }

    fn output_authz_code_exchange(
        &self,
        response: Result<
            types::OAuthAuthzCodeExchangeResponse,
            progenitor_client::Error<types::Error>,
        >,
    ) {
        print_cli_output(&response)
    }

    fn output_get_device_provider(
        &self,
        response: Result<types::OAuthProviderInfo, progenitor_client::Error<types::Error>>,
    ) {
        print_cli_output(&response)
    }

    fn output_exchange_device_token(&self, response: Result<(), progenitor_client::Error<()>>) {}

    fn output_list_oauth_clients(
        &self,
        response: Result<Vec<types::OAuthClient>, progenitor_client::Error<types::Error>>,
    ) {
        print_cli_output(&response)
    }

    fn output_create_oauth_client(
        &self,
        response: Result<types::OAuthClient, progenitor_client::Error<types::Error>>,
    ) {
        print_cli_output(&response)
    }

    fn output_get_oauth_client(
        &self,
        response: Result<types::OAuthClient, progenitor_client::Error<types::Error>>,
    ) {
        print_cli_output(&response)
    }

    fn output_create_oauth_client_redirect_uri(
        &self,
        response: Result<types::OAuthClientRedirectUri, progenitor_client::Error<types::Error>>,
    ) {
        print_cli_output(&response)
    }

    fn output_delete_oauth_client_redirect_uri(
        &self,
        response: Result<types::OAuthClientRedirectUri, progenitor_client::Error<types::Error>>,
    ) {
        print_cli_output(&response)
    }

    fn output_create_oauth_client_secret(
        &self,
        response: Result<
            types::InitialOAuthClientSecretResponse,
            progenitor_client::Error<types::Error>,
        >,
    ) {
        print_cli_output(&response)
    }

    fn output_delete_oauth_client_secret(
        &self,
        response: Result<types::OAuthClientSecret, progenitor_client::Error<types::Error>>,
    ) {
        print_cli_output(&response)
    }

    fn output_get_rfds(
        &self,
        response: Result<Vec<types::ListRfd>, progenitor_client::Error<types::Error>>,
    ) {
        print_cli_output(&response)
    }

    fn output_get_rfd(
        &self,
        response: Result<types::FullRfd, progenitor_client::Error<types::Error>>,
    ) {
        print_cli_output(&response)
    }

    fn output_search_rfds(
        &self,
        response: Result<Vec<types::ListRfd>, progenitor_client::Error<types::Error>>,
    ) {
        print_cli_output(&response)
    }

    fn output_get_self(
        &self,
        response: Result<types::GetApiUserResponse, progenitor_client::Error<types::Error>>,
    ) {
        print_cli_output(&response)
    }
}
