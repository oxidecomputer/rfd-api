// The contents of this file are generated; do not modify them.

use rfd_sdk::*;

pub struct Cli<T: CliOverride = (), U: CliOutput = ()> {
    client: rfd_sdk::Client,
    over: T,
    output: U,
}

impl Cli {
    pub fn new(client: rfd_sdk::Client) -> Self {
        Self {
            client,
            over: (),
            output: (),
        }
    }

    pub fn get_command(cmd: CliCommand) -> clap::Command {
        match cmd {
            CliCommand::CreateApiUser => Self::cli_create_api_user(),
            CliCommand::GetApiUser => Self::cli_get_api_user(),
            CliCommand::UpdateApiUser => Self::cli_update_api_user(),
            CliCommand::AddApiUserToGroup => Self::cli_add_api_user_to_group(),
            CliCommand::RemoveApiUserFromGroup => Self::cli_remove_api_user_from_group(),
            CliCommand::ListApiUserTokens => Self::cli_list_api_user_tokens(),
            CliCommand::CreateApiUserToken => Self::cli_create_api_user_token(),
            CliCommand::GetApiUserToken => Self::cli_get_api_user_token(),
            CliCommand::DeleteApiUserToken => Self::cli_delete_api_user_token(),
            CliCommand::GithubWebhook => Self::cli_github_webhook(),
            CliCommand::GetGroups => Self::cli_get_groups(),
            CliCommand::CreateGroup => Self::cli_create_group(),
            CliCommand::UpdateGroup => Self::cli_update_group(),
            CliCommand::DeleteGroup => Self::cli_delete_group(),
            CliCommand::AuthzCodeRedirect => Self::cli_authz_code_redirect(),
            CliCommand::AuthzCodeCallback => Self::cli_authz_code_callback(),
            CliCommand::AuthzCodeExchange => Self::cli_authz_code_exchange(),
            CliCommand::GetDeviceProvider => Self::cli_get_device_provider(),
            CliCommand::ExchangeDeviceToken => Self::cli_exchange_device_token(),
            CliCommand::ListOauthClients => Self::cli_list_oauth_clients(),
            CliCommand::CreateOauthClient => Self::cli_create_oauth_client(),
            CliCommand::GetOauthClient => Self::cli_get_oauth_client(),
            CliCommand::CreateOauthClientRedirectUri => {
                Self::cli_create_oauth_client_redirect_uri()
            }
            CliCommand::DeleteOauthClientRedirectUri => {
                Self::cli_delete_oauth_client_redirect_uri()
            }
            CliCommand::CreateOauthClientSecret => Self::cli_create_oauth_client_secret(),
            CliCommand::DeleteOauthClientSecret => Self::cli_delete_oauth_client_secret(),
            CliCommand::GetRfds => Self::cli_get_rfds(),
            CliCommand::GetRfd => Self::cli_get_rfd(),
            CliCommand::SearchRfds => Self::cli_search_rfds(),
            CliCommand::GetSelf => Self::cli_get_self(),
        }
    }

    pub fn cli_create_api_user() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("json-body")
                    .long("json-body")
                    .value_name("JSON-FILE")
                    .required(true)
                    .value_parser(clap::value_parser!(std::path::PathBuf))
                    .help("Path to a file that contains the full json body."),
            )
            .arg(
                clap::Arg::new("json-body-template")
                    .long("json-body-template")
                    .action(clap::ArgAction::SetTrue)
                    .help("XXX"),
            )
            .about("Create a new user with a given set of permissions")
    }

    pub fn cli_get_api_user() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("identifier")
                    .long("identifier")
                    .value_parser(clap::value_parser!(uuid::Uuid))
                    .required(true),
            )
            .about("Get user information for a given user id")
    }

    pub fn cli_update_api_user() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("identifier")
                    .long("identifier")
                    .value_parser(clap::value_parser!(uuid::Uuid))
                    .required(true),
            )
            .arg(
                clap::Arg::new("json-body")
                    .long("json-body")
                    .value_name("JSON-FILE")
                    .required(true)
                    .value_parser(clap::value_parser!(std::path::PathBuf))
                    .help("Path to a file that contains the full json body."),
            )
            .arg(
                clap::Arg::new("json-body-template")
                    .long("json-body-template")
                    .action(clap::ArgAction::SetTrue)
                    .help("XXX"),
            )
            .about("Update the permissions assigned to a given user")
    }

    pub fn cli_add_api_user_to_group() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("group-id")
                    .long("group-id")
                    .value_parser(clap::value_parser!(uuid::Uuid))
                    .required_unless_present("json-body"),
            )
            .arg(
                clap::Arg::new("identifier")
                    .long("identifier")
                    .value_parser(clap::value_parser!(uuid::Uuid))
                    .required(true),
            )
            .arg(
                clap::Arg::new("json-body")
                    .long("json-body")
                    .value_name("JSON-FILE")
                    .required(false)
                    .value_parser(clap::value_parser!(std::path::PathBuf))
                    .help("Path to a file that contains the full json body."),
            )
            .arg(
                clap::Arg::new("json-body-template")
                    .long("json-body-template")
                    .action(clap::ArgAction::SetTrue)
                    .help("XXX"),
            )
    }

    pub fn cli_remove_api_user_from_group() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("group-id")
                    .long("group-id")
                    .value_parser(clap::value_parser!(uuid::Uuid))
                    .required(true),
            )
            .arg(
                clap::Arg::new("identifier")
                    .long("identifier")
                    .value_parser(clap::value_parser!(uuid::Uuid))
                    .required(true),
            )
    }

    pub fn cli_list_api_user_tokens() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("identifier")
                    .long("identifier")
                    .value_parser(clap::value_parser!(uuid::Uuid))
                    .required(true),
            )
            .about("List the active and expired API tokens for a given user")
    }

    pub fn cli_create_api_user_token() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("expires-at")
                    .long("expires-at")
                    .value_parser(clap::value_parser!(chrono::DateTime<chrono::offset::Utc>))
                    .required_unless_present("json-body"),
            )
            .arg(
                clap::Arg::new("identifier")
                    .long("identifier")
                    .value_parser(clap::value_parser!(uuid::Uuid))
                    .required(true),
            )
            .arg(
                clap::Arg::new("json-body")
                    .long("json-body")
                    .value_name("JSON-FILE")
                    .required(true)
                    .value_parser(clap::value_parser!(std::path::PathBuf))
                    .help("Path to a file that contains the full json body."),
            )
            .arg(
                clap::Arg::new("json-body-template")
                    .long("json-body-template")
                    .action(clap::ArgAction::SetTrue)
                    .help("XXX"),
            )
    }

    pub fn cli_get_api_user_token() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("identifier")
                    .long("identifier")
                    .value_parser(clap::value_parser!(uuid::Uuid))
                    .required(true),
            )
            .arg(
                clap::Arg::new("token-identifier")
                    .long("token-identifier")
                    .value_parser(clap::value_parser!(uuid::Uuid))
                    .required(true),
            )
    }

    pub fn cli_delete_api_user_token() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("identifier")
                    .long("identifier")
                    .value_parser(clap::value_parser!(uuid::Uuid))
                    .required(true),
            )
            .arg(
                clap::Arg::new("token-identifier")
                    .long("token-identifier")
                    .value_parser(clap::value_parser!(uuid::Uuid))
                    .required(true),
            )
    }

    pub fn cli_github_webhook() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("ref")
                    .long("ref")
                    .value_parser(clap::value_parser!(String))
                    .required_unless_present("json-body"),
            )
            .arg(
                clap::Arg::new("json-body")
                    .long("json-body")
                    .value_name("JSON-FILE")
                    .required(true)
                    .value_parser(clap::value_parser!(std::path::PathBuf))
                    .help("Path to a file that contains the full json body."),
            )
            .arg(
                clap::Arg::new("json-body-template")
                    .long("json-body-template")
                    .action(clap::ArgAction::SetTrue)
                    .help("XXX"),
            )
    }

    pub fn cli_get_groups() -> clap::Command {
        clap::Command::new("")
    }

    pub fn cli_create_group() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("name")
                    .long("name")
                    .value_parser(clap::value_parser!(String))
                    .required_unless_present("json-body"),
            )
            .arg(
                clap::Arg::new("json-body")
                    .long("json-body")
                    .value_name("JSON-FILE")
                    .required(true)
                    .value_parser(clap::value_parser!(std::path::PathBuf))
                    .help("Path to a file that contains the full json body."),
            )
            .arg(
                clap::Arg::new("json-body-template")
                    .long("json-body-template")
                    .action(clap::ArgAction::SetTrue)
                    .help("XXX"),
            )
    }

    pub fn cli_update_group() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("group-id")
                    .long("group-id")
                    .value_parser(clap::value_parser!(uuid::Uuid))
                    .required(true),
            )
            .arg(
                clap::Arg::new("name")
                    .long("name")
                    .value_parser(clap::value_parser!(String))
                    .required_unless_present("json-body"),
            )
            .arg(
                clap::Arg::new("json-body")
                    .long("json-body")
                    .value_name("JSON-FILE")
                    .required(true)
                    .value_parser(clap::value_parser!(std::path::PathBuf))
                    .help("Path to a file that contains the full json body."),
            )
            .arg(
                clap::Arg::new("json-body-template")
                    .long("json-body-template")
                    .action(clap::ArgAction::SetTrue)
                    .help("XXX"),
            )
    }

    pub fn cli_delete_group() -> clap::Command {
        clap::Command::new("").arg(
            clap::Arg::new("group-id")
                .long("group-id")
                .value_parser(clap::value_parser!(uuid::Uuid))
                .required(true),
        )
    }

    pub fn cli_authz_code_redirect() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("client-id")
                    .long("client-id")
                    .value_parser(clap::value_parser!(uuid::Uuid))
                    .required(true),
            )
            .arg(
                clap::Arg::new("provider")
                    .long("provider")
                    .value_parser(clap::builder::TypedValueParser::map(
                        clap::builder::PossibleValuesParser::new([
                            types::OAuthProviderName::GitHub.to_string(),
                            types::OAuthProviderName::Google.to_string(),
                        ]),
                        |s| types::OAuthProviderName::try_from(s).unwrap(),
                    ))
                    .required(true),
            )
            .arg(
                clap::Arg::new("redirect-uri")
                    .long("redirect-uri")
                    .value_parser(clap::value_parser!(String))
                    .required(true),
            )
            .arg(
                clap::Arg::new("response-type")
                    .long("response-type")
                    .value_parser(clap::value_parser!(String))
                    .required(true),
            )
            .arg(
                clap::Arg::new("state")
                    .long("state")
                    .value_parser(clap::value_parser!(String))
                    .required(true),
            )
            .about("Generate the remote provider login url and redirect the user")
    }

    pub fn cli_authz_code_callback() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("code")
                    .long("code")
                    .value_parser(clap::value_parser!(String))
                    .required(false),
            )
            .arg(
                clap::Arg::new("error")
                    .long("error")
                    .value_parser(clap::value_parser!(String))
                    .required(false),
            )
            .arg(
                clap::Arg::new("provider")
                    .long("provider")
                    .value_parser(clap::builder::TypedValueParser::map(
                        clap::builder::PossibleValuesParser::new([
                            types::OAuthProviderName::GitHub.to_string(),
                            types::OAuthProviderName::Google.to_string(),
                        ]),
                        |s| types::OAuthProviderName::try_from(s).unwrap(),
                    ))
                    .required(true),
            )
            .arg(
                clap::Arg::new("state")
                    .long("state")
                    .value_parser(clap::value_parser!(String))
                    .required(false),
            )
            .about("Handle return calls from a remote OAuth provider")
    }

    pub fn cli_authz_code_exchange() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("client-id")
                    .long("client-id")
                    .value_parser(clap::value_parser!(uuid::Uuid))
                    .required_unless_present("json-body"),
            )
            .arg(
                clap::Arg::new("client-secret")
                    .long("client-secret")
                    .value_parser(clap::value_parser!(String))
                    .required_unless_present("json-body"),
            )
            .arg(
                clap::Arg::new("code")
                    .long("code")
                    .value_parser(clap::value_parser!(String))
                    .required_unless_present("json-body"),
            )
            .arg(
                clap::Arg::new("grant-type")
                    .long("grant-type")
                    .value_parser(clap::value_parser!(String))
                    .required_unless_present("json-body"),
            )
            .arg(
                clap::Arg::new("pkce-verifier")
                    .long("pkce-verifier")
                    .value_parser(clap::value_parser!(String))
                    .required(false),
            )
            .arg(
                clap::Arg::new("provider")
                    .long("provider")
                    .value_parser(clap::builder::TypedValueParser::map(
                        clap::builder::PossibleValuesParser::new([
                            types::OAuthProviderName::GitHub.to_string(),
                            types::OAuthProviderName::Google.to_string(),
                        ]),
                        |s| types::OAuthProviderName::try_from(s).unwrap(),
                    ))
                    .required(true),
            )
            .arg(
                clap::Arg::new("redirect-uri")
                    .long("redirect-uri")
                    .value_parser(clap::value_parser!(String))
                    .required_unless_present("json-body"),
            )
            .arg(
                clap::Arg::new("json-body")
                    .long("json-body")
                    .value_name("JSON-FILE")
                    .required(false)
                    .value_parser(clap::value_parser!(std::path::PathBuf))
                    .help("Path to a file that contains the full json body."),
            )
            .arg(
                clap::Arg::new("json-body-template")
                    .long("json-body-template")
                    .action(clap::ArgAction::SetTrue)
                    .help("XXX"),
            )
            .about("Exchange an authorization code for an access token")
    }

    pub fn cli_get_device_provider() -> clap::Command {
        clap::Command::new("").arg(
            clap::Arg::new("provider")
                .long("provider")
                .value_parser(clap::builder::TypedValueParser::map(
                    clap::builder::PossibleValuesParser::new([
                        types::OAuthProviderName::GitHub.to_string(),
                        types::OAuthProviderName::Google.to_string(),
                    ]),
                    |s| types::OAuthProviderName::try_from(s).unwrap(),
                ))
                .required(true),
        )
    }

    pub fn cli_exchange_device_token() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("device-code")
                    .long("device-code")
                    .value_parser(clap::value_parser!(String))
                    .required_unless_present("json-body"),
            )
            .arg(
                clap::Arg::new("expires-at")
                    .long("expires-at")
                    .value_parser(clap::value_parser!(chrono::DateTime<chrono::offset::Utc>))
                    .required(false),
            )
            .arg(
                clap::Arg::new("grant-type")
                    .long("grant-type")
                    .value_parser(clap::value_parser!(String))
                    .required_unless_present("json-body"),
            )
            .arg(
                clap::Arg::new("provider")
                    .long("provider")
                    .value_parser(clap::builder::TypedValueParser::map(
                        clap::builder::PossibleValuesParser::new([
                            types::OAuthProviderName::GitHub.to_string(),
                            types::OAuthProviderName::Google.to_string(),
                        ]),
                        |s| types::OAuthProviderName::try_from(s).unwrap(),
                    ))
                    .required(true),
            )
            .arg(
                clap::Arg::new("json-body")
                    .long("json-body")
                    .value_name("JSON-FILE")
                    .required(false)
                    .value_parser(clap::value_parser!(std::path::PathBuf))
                    .help("Path to a file that contains the full json body."),
            )
            .arg(
                clap::Arg::new("json-body-template")
                    .long("json-body-template")
                    .action(clap::ArgAction::SetTrue)
                    .help("XXX"),
            )
    }

    pub fn cli_list_oauth_clients() -> clap::Command {
        clap::Command::new("").about("List OAuth clients")
    }

    pub fn cli_create_oauth_client() -> clap::Command {
        clap::Command::new("").about("Create a new OAuth Client")
    }

    pub fn cli_get_oauth_client() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("client-id")
                    .long("client-id")
                    .value_parser(clap::value_parser!(uuid::Uuid))
                    .required(true),
            )
            .about("Get an new OAuth Client")
    }

    pub fn cli_create_oauth_client_redirect_uri() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("client-id")
                    .long("client-id")
                    .value_parser(clap::value_parser!(uuid::Uuid))
                    .required(true),
            )
            .arg(
                clap::Arg::new("redirect-uri")
                    .long("redirect-uri")
                    .value_parser(clap::value_parser!(String))
                    .required_unless_present("json-body"),
            )
            .arg(
                clap::Arg::new("json-body")
                    .long("json-body")
                    .value_name("JSON-FILE")
                    .required(false)
                    .value_parser(clap::value_parser!(std::path::PathBuf))
                    .help("Path to a file that contains the full json body."),
            )
            .arg(
                clap::Arg::new("json-body-template")
                    .long("json-body-template")
                    .action(clap::ArgAction::SetTrue)
                    .help("XXX"),
            )
            .about("Add an OAuth client redirect uri")
    }

    pub fn cli_delete_oauth_client_redirect_uri() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("client-id")
                    .long("client-id")
                    .value_parser(clap::value_parser!(uuid::Uuid))
                    .required(true),
            )
            .arg(
                clap::Arg::new("redirect-uri-id")
                    .long("redirect-uri-id")
                    .value_parser(clap::value_parser!(uuid::Uuid))
                    .required(true),
            )
            .about("Delete an OAuth client redirect uri")
    }

    pub fn cli_create_oauth_client_secret() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("client-id")
                    .long("client-id")
                    .value_parser(clap::value_parser!(uuid::Uuid))
                    .required(true),
            )
            .about("Add an OAuth client secret")
    }

    pub fn cli_delete_oauth_client_secret() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("client-id")
                    .long("client-id")
                    .value_parser(clap::value_parser!(uuid::Uuid))
                    .required(true),
            )
            .arg(
                clap::Arg::new("secret-id")
                    .long("secret-id")
                    .value_parser(clap::value_parser!(uuid::Uuid))
                    .required(true),
            )
            .about("Delete an OAuth client secret")
    }

    pub fn cli_get_rfds() -> clap::Command {
        clap::Command::new("").about("List all available RFDs")
    }

    pub fn cli_get_rfd() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("number")
                    .long("number")
                    .value_parser(clap::value_parser!(String))
                    .required(true),
            )
            .about("Get the latest representation of an RFD")
    }

    pub fn cli_search_rfds() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("q")
                    .long("q")
                    .value_parser(clap::value_parser!(String))
                    .required(true),
            )
            .about("Search the RFD index and get a list of results")
    }

    pub fn cli_get_self() -> clap::Command {
        clap::Command::new("").about("Retrieve the user information of the calling user")
    }
}

impl<T: CliOverride, U: CliOutput> Cli<T, U> {
    pub fn new_with_override(client: rfd_sdk::Client, over: T, output: U) -> Self {
        Self {
            client,
            over,
            output,
        }
    }

    pub async fn execute(&self, cmd: CliCommand, matches: &clap::ArgMatches) {
        match cmd {
            CliCommand::CreateApiUser => {
                self.execute_create_api_user(matches).await;
            }
            CliCommand::GetApiUser => {
                self.execute_get_api_user(matches).await;
            }
            CliCommand::UpdateApiUser => {
                self.execute_update_api_user(matches).await;
            }
            CliCommand::AddApiUserToGroup => {
                self.execute_add_api_user_to_group(matches).await;
            }
            CliCommand::RemoveApiUserFromGroup => {
                self.execute_remove_api_user_from_group(matches).await;
            }
            CliCommand::ListApiUserTokens => {
                self.execute_list_api_user_tokens(matches).await;
            }
            CliCommand::CreateApiUserToken => {
                self.execute_create_api_user_token(matches).await;
            }
            CliCommand::GetApiUserToken => {
                self.execute_get_api_user_token(matches).await;
            }
            CliCommand::DeleteApiUserToken => {
                self.execute_delete_api_user_token(matches).await;
            }
            CliCommand::GithubWebhook => {
                self.execute_github_webhook(matches).await;
            }
            CliCommand::GetGroups => {
                self.execute_get_groups(matches).await;
            }
            CliCommand::CreateGroup => {
                self.execute_create_group(matches).await;
            }
            CliCommand::UpdateGroup => {
                self.execute_update_group(matches).await;
            }
            CliCommand::DeleteGroup => {
                self.execute_delete_group(matches).await;
            }
            CliCommand::AuthzCodeRedirect => {
                self.execute_authz_code_redirect(matches).await;
            }
            CliCommand::AuthzCodeCallback => {
                self.execute_authz_code_callback(matches).await;
            }
            CliCommand::AuthzCodeExchange => {
                self.execute_authz_code_exchange(matches).await;
            }
            CliCommand::GetDeviceProvider => {
                self.execute_get_device_provider(matches).await;
            }
            CliCommand::ExchangeDeviceToken => {
                self.execute_exchange_device_token(matches).await;
            }
            CliCommand::ListOauthClients => {
                self.execute_list_oauth_clients(matches).await;
            }
            CliCommand::CreateOauthClient => {
                self.execute_create_oauth_client(matches).await;
            }
            CliCommand::GetOauthClient => {
                self.execute_get_oauth_client(matches).await;
            }
            CliCommand::CreateOauthClientRedirectUri => {
                self.execute_create_oauth_client_redirect_uri(matches).await;
            }
            CliCommand::DeleteOauthClientRedirectUri => {
                self.execute_delete_oauth_client_redirect_uri(matches).await;
            }
            CliCommand::CreateOauthClientSecret => {
                self.execute_create_oauth_client_secret(matches).await;
            }
            CliCommand::DeleteOauthClientSecret => {
                self.execute_delete_oauth_client_secret(matches).await;
            }
            CliCommand::GetRfds => {
                self.execute_get_rfds(matches).await;
            }
            CliCommand::GetRfd => {
                self.execute_get_rfd(matches).await;
            }
            CliCommand::SearchRfds => {
                self.execute_search_rfds(matches).await;
            }
            CliCommand::GetSelf => {
                self.execute_get_self(matches).await;
            }
        }
    }

    pub async fn execute_create_api_user(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.create_api_user();
        if let Some(value) = matches.get_one::<std::path::PathBuf>("json-body") {
            let body_txt = std::fs::read_to_string(value).unwrap();
            let body_value = serde_json::from_str::<types::ApiUserUpdateParams>(&body_txt).unwrap();
            request = request.body(body_value);
        }

        self.over
            .execute_create_api_user(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => self.output.output_create_api_user(Ok(r.into_inner())),
            Err(r) => self.output.output_create_api_user(Err(r)),
        }
    }

    pub async fn execute_get_api_user(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.get_api_user();
        if let Some(value) = matches.get_one::<uuid::Uuid>("identifier") {
            request = request.identifier(value.clone());
        }

        self.over
            .execute_get_api_user(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => self.output.output_get_api_user(Ok(r.into_inner())),
            Err(r) => self.output.output_get_api_user(Err(r)),
        }
    }

    pub async fn execute_update_api_user(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.update_api_user();
        if let Some(value) = matches.get_one::<uuid::Uuid>("identifier") {
            request = request.identifier(value.clone());
        }

        if let Some(value) = matches.get_one::<std::path::PathBuf>("json-body") {
            let body_txt = std::fs::read_to_string(value).unwrap();
            let body_value = serde_json::from_str::<types::ApiUserUpdateParams>(&body_txt).unwrap();
            request = request.body(body_value);
        }

        self.over
            .execute_update_api_user(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => self.output.output_update_api_user(Ok(r.into_inner())),
            Err(r) => self.output.output_update_api_user(Err(r)),
        }
    }

    pub async fn execute_add_api_user_to_group(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.add_api_user_to_group();
        if let Some(value) = matches.get_one::<uuid::Uuid>("group-id") {
            request = request.body_map(|body| body.group_id(value.clone()))
        }

        if let Some(value) = matches.get_one::<uuid::Uuid>("identifier") {
            request = request.identifier(value.clone());
        }

        if let Some(value) = matches.get_one::<std::path::PathBuf>("json-body") {
            let body_txt = std::fs::read_to_string(value).unwrap();
            let body_value = serde_json::from_str::<types::AddGroupBody>(&body_txt).unwrap();
            request = request.body(body_value);
        }

        self.over
            .execute_add_api_user_to_group(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => self.output.output_add_api_user_to_group(Ok(r.into_inner())),
            Err(r) => self.output.output_add_api_user_to_group(Err(r)),
        }
    }

    pub async fn execute_remove_api_user_from_group(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.remove_api_user_from_group();
        if let Some(value) = matches.get_one::<uuid::Uuid>("group-id") {
            request = request.group_id(value.clone());
        }

        if let Some(value) = matches.get_one::<uuid::Uuid>("identifier") {
            request = request.identifier(value.clone());
        }

        self.over
            .execute_remove_api_user_from_group(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => self
                .output
                .output_remove_api_user_from_group(Ok(r.into_inner())),
            Err(r) => self.output.output_remove_api_user_from_group(Err(r)),
        }
    }

    pub async fn execute_list_api_user_tokens(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.list_api_user_tokens();
        if let Some(value) = matches.get_one::<uuid::Uuid>("identifier") {
            request = request.identifier(value.clone());
        }

        self.over
            .execute_list_api_user_tokens(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => self.output.output_list_api_user_tokens(Ok(r.into_inner())),
            Err(r) => self.output.output_list_api_user_tokens(Err(r)),
        }
    }

    pub async fn execute_create_api_user_token(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.create_api_user_token();
        if let Some(value) = matches.get_one::<chrono::DateTime<chrono::offset::Utc>>("expires-at")
        {
            request = request.body_map(|body| body.expires_at(value.clone()))
        }

        if let Some(value) = matches.get_one::<uuid::Uuid>("identifier") {
            request = request.identifier(value.clone());
        }

        if let Some(value) = matches.get_one::<std::path::PathBuf>("json-body") {
            let body_txt = std::fs::read_to_string(value).unwrap();
            let body_value = serde_json::from_str::<types::ApiKeyCreateParams>(&body_txt).unwrap();
            request = request.body(body_value);
        }

        self.over
            .execute_create_api_user_token(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => self.output.output_create_api_user_token(Ok(r.into_inner())),
            Err(r) => self.output.output_create_api_user_token(Err(r)),
        }
    }

    pub async fn execute_get_api_user_token(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.get_api_user_token();
        if let Some(value) = matches.get_one::<uuid::Uuid>("identifier") {
            request = request.identifier(value.clone());
        }

        if let Some(value) = matches.get_one::<uuid::Uuid>("token-identifier") {
            request = request.token_identifier(value.clone());
        }

        self.over
            .execute_get_api_user_token(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => self.output.output_get_api_user_token(Ok(r.into_inner())),
            Err(r) => self.output.output_get_api_user_token(Err(r)),
        }
    }

    pub async fn execute_delete_api_user_token(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.delete_api_user_token();
        if let Some(value) = matches.get_one::<uuid::Uuid>("identifier") {
            request = request.identifier(value.clone());
        }

        if let Some(value) = matches.get_one::<uuid::Uuid>("token-identifier") {
            request = request.token_identifier(value.clone());
        }

        self.over
            .execute_delete_api_user_token(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => self.output.output_delete_api_user_token(Ok(r.into_inner())),
            Err(r) => self.output.output_delete_api_user_token(Err(r)),
        }
    }

    pub async fn execute_github_webhook(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.github_webhook();
        if let Some(value) = matches.get_one::<String>("ref") {
            request = request.body_map(|body| body.ref_(value.clone()))
        }

        if let Some(value) = matches.get_one::<std::path::PathBuf>("json-body") {
            let body_txt = std::fs::read_to_string(value).unwrap();
            let body_value = serde_json::from_str::<types::GitHubCommitPayload>(&body_txt).unwrap();
            request = request.body(body_value);
        }

        self.over
            .execute_github_webhook(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => self.output.output_github_webhook(Ok(r.into_inner())),
            Err(r) => self.output.output_github_webhook(Err(r)),
        }
    }

    pub async fn execute_get_groups(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.get_groups();
        self.over.execute_get_groups(matches, &mut request).unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => self.output.output_get_groups(Ok(r.into_inner())),
            Err(r) => self.output.output_get_groups(Err(r)),
        }
    }

    pub async fn execute_create_group(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.create_group();
        if let Some(value) = matches.get_one::<String>("name") {
            request = request.body_map(|body| body.name(value.clone()))
        }

        if let Some(value) = matches.get_one::<std::path::PathBuf>("json-body") {
            let body_txt = std::fs::read_to_string(value).unwrap();
            let body_value =
                serde_json::from_str::<types::AccessGroupUpdateParams>(&body_txt).unwrap();
            request = request.body(body_value);
        }

        self.over
            .execute_create_group(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => self.output.output_create_group(Ok(r.into_inner())),
            Err(r) => self.output.output_create_group(Err(r)),
        }
    }

    pub async fn execute_update_group(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.update_group();
        if let Some(value) = matches.get_one::<uuid::Uuid>("group-id") {
            request = request.group_id(value.clone());
        }

        if let Some(value) = matches.get_one::<String>("name") {
            request = request.body_map(|body| body.name(value.clone()))
        }

        if let Some(value) = matches.get_one::<std::path::PathBuf>("json-body") {
            let body_txt = std::fs::read_to_string(value).unwrap();
            let body_value =
                serde_json::from_str::<types::AccessGroupUpdateParams>(&body_txt).unwrap();
            request = request.body(body_value);
        }

        self.over
            .execute_update_group(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => self.output.output_update_group(Ok(r.into_inner())),
            Err(r) => self.output.output_update_group(Err(r)),
        }
    }

    pub async fn execute_delete_group(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.delete_group();
        if let Some(value) = matches.get_one::<uuid::Uuid>("group-id") {
            request = request.group_id(value.clone());
        }

        self.over
            .execute_delete_group(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => self.output.output_delete_group(Ok(r.into_inner())),
            Err(r) => self.output.output_delete_group(Err(r)),
        }
    }

    pub async fn execute_authz_code_redirect(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.authz_code_redirect();
        if let Some(value) = matches.get_one::<uuid::Uuid>("client-id") {
            request = request.client_id(value.clone());
        }

        if let Some(value) = matches.get_one::<types::OAuthProviderName>("provider") {
            request = request.provider(value.clone());
        }

        if let Some(value) = matches.get_one::<String>("redirect-uri") {
            request = request.redirect_uri(value.clone());
        }

        if let Some(value) = matches.get_one::<String>("response-type") {
            request = request.response_type(value.clone());
        }

        if let Some(value) = matches.get_one::<String>("state") {
            request = request.state(value.clone());
        }

        self.over
            .execute_authz_code_redirect(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                todo!()
            }
            Err(r) => {
                todo!()
            }
        }
    }

    pub async fn execute_authz_code_callback(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.authz_code_callback();
        if let Some(value) = matches.get_one::<String>("code") {
            request = request.code(value.clone());
        }

        if let Some(value) = matches.get_one::<String>("error") {
            request = request.error(value.clone());
        }

        if let Some(value) = matches.get_one::<types::OAuthProviderName>("provider") {
            request = request.provider(value.clone());
        }

        if let Some(value) = matches.get_one::<String>("state") {
            request = request.state(value.clone());
        }

        self.over
            .execute_authz_code_callback(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                todo!()
            }
            Err(r) => self.output.output_authz_code_callback(Err(r)),
        }
    }

    pub async fn execute_authz_code_exchange(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.authz_code_exchange();
        if let Some(value) = matches.get_one::<uuid::Uuid>("client-id") {
            request = request.body_map(|body| body.client_id(value.clone()))
        }

        if let Some(value) = matches.get_one::<String>("client-secret") {
            request = request.body_map(|body| body.client_secret(value.clone()))
        }

        if let Some(value) = matches.get_one::<String>("code") {
            request = request.body_map(|body| body.code(value.clone()))
        }

        if let Some(value) = matches.get_one::<String>("grant-type") {
            request = request.body_map(|body| body.grant_type(value.clone()))
        }

        if let Some(value) = matches.get_one::<String>("pkce-verifier") {
            request = request.body_map(|body| body.pkce_verifier(value.clone()))
        }

        if let Some(value) = matches.get_one::<types::OAuthProviderName>("provider") {
            request = request.provider(value.clone());
        }

        if let Some(value) = matches.get_one::<String>("redirect-uri") {
            request = request.body_map(|body| body.redirect_uri(value.clone()))
        }

        if let Some(value) = matches.get_one::<std::path::PathBuf>("json-body") {
            let body_txt = std::fs::read_to_string(value).unwrap();
            let body_value =
                serde_json::from_str::<types::OAuthAuthzCodeExchangeBody>(&body_txt).unwrap();
            request = request.body(body_value);
        }

        self.over
            .execute_authz_code_exchange(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => self.output.output_authz_code_exchange(Ok(r.into_inner())),
            Err(r) => self.output.output_authz_code_exchange(Err(r)),
        }
    }

    pub async fn execute_get_device_provider(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.get_device_provider();
        if let Some(value) = matches.get_one::<types::OAuthProviderName>("provider") {
            request = request.provider(value.clone());
        }

        self.over
            .execute_get_device_provider(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => self.output.output_get_device_provider(Ok(r.into_inner())),
            Err(r) => self.output.output_get_device_provider(Err(r)),
        }
    }

    pub async fn execute_exchange_device_token(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.exchange_device_token();
        if let Some(value) = matches.get_one::<String>("device-code") {
            request = request.body_map(|body| body.device_code(value.clone()))
        }

        if let Some(value) = matches.get_one::<chrono::DateTime<chrono::offset::Utc>>("expires-at")
        {
            request = request.body_map(|body| body.expires_at(value.clone()))
        }

        if let Some(value) = matches.get_one::<String>("grant-type") {
            request = request.body_map(|body| body.grant_type(value.clone()))
        }

        if let Some(value) = matches.get_one::<types::OAuthProviderName>("provider") {
            request = request.provider(value.clone());
        }

        if let Some(value) = matches.get_one::<std::path::PathBuf>("json-body") {
            let body_txt = std::fs::read_to_string(value).unwrap();
            let body_value =
                serde_json::from_str::<types::AccessTokenExchangeRequest>(&body_txt).unwrap();
            request = request.body(body_value);
        }

        self.over
            .execute_exchange_device_token(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                todo!()
            }
            Err(r) => {
                todo!()
            }
        }
    }

    pub async fn execute_list_oauth_clients(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.list_oauth_clients();
        self.over
            .execute_list_oauth_clients(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => self.output.output_list_oauth_clients(Ok(r.into_inner())),
            Err(r) => self.output.output_list_oauth_clients(Err(r)),
        }
    }

    pub async fn execute_create_oauth_client(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.create_oauth_client();
        self.over
            .execute_create_oauth_client(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => self.output.output_create_oauth_client(Ok(r.into_inner())),
            Err(r) => self.output.output_create_oauth_client(Err(r)),
        }
    }

    pub async fn execute_get_oauth_client(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.get_oauth_client();
        if let Some(value) = matches.get_one::<uuid::Uuid>("client-id") {
            request = request.client_id(value.clone());
        }

        self.over
            .execute_get_oauth_client(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => self.output.output_get_oauth_client(Ok(r.into_inner())),
            Err(r) => self.output.output_get_oauth_client(Err(r)),
        }
    }

    pub async fn execute_create_oauth_client_redirect_uri(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.create_oauth_client_redirect_uri();
        if let Some(value) = matches.get_one::<uuid::Uuid>("client-id") {
            request = request.client_id(value.clone());
        }

        if let Some(value) = matches.get_one::<String>("redirect-uri") {
            request = request.body_map(|body| body.redirect_uri(value.clone()))
        }

        if let Some(value) = matches.get_one::<std::path::PathBuf>("json-body") {
            let body_txt = std::fs::read_to_string(value).unwrap();
            let body_value =
                serde_json::from_str::<types::AddOAuthClientRedirectBody>(&body_txt).unwrap();
            request = request.body(body_value);
        }

        self.over
            .execute_create_oauth_client_redirect_uri(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => self
                .output
                .output_create_oauth_client_redirect_uri(Ok(r.into_inner())),
            Err(r) => self.output.output_create_oauth_client_redirect_uri(Err(r)),
        }
    }

    pub async fn execute_delete_oauth_client_redirect_uri(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.delete_oauth_client_redirect_uri();
        if let Some(value) = matches.get_one::<uuid::Uuid>("client-id") {
            request = request.client_id(value.clone());
        }

        if let Some(value) = matches.get_one::<uuid::Uuid>("redirect-uri-id") {
            request = request.redirect_uri_id(value.clone());
        }

        self.over
            .execute_delete_oauth_client_redirect_uri(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => self
                .output
                .output_delete_oauth_client_redirect_uri(Ok(r.into_inner())),
            Err(r) => self.output.output_delete_oauth_client_redirect_uri(Err(r)),
        }
    }

    pub async fn execute_create_oauth_client_secret(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.create_oauth_client_secret();
        if let Some(value) = matches.get_one::<uuid::Uuid>("client-id") {
            request = request.client_id(value.clone());
        }

        self.over
            .execute_create_oauth_client_secret(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => self
                .output
                .output_create_oauth_client_secret(Ok(r.into_inner())),
            Err(r) => self.output.output_create_oauth_client_secret(Err(r)),
        }
    }

    pub async fn execute_delete_oauth_client_secret(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.delete_oauth_client_secret();
        if let Some(value) = matches.get_one::<uuid::Uuid>("client-id") {
            request = request.client_id(value.clone());
        }

        if let Some(value) = matches.get_one::<uuid::Uuid>("secret-id") {
            request = request.secret_id(value.clone());
        }

        self.over
            .execute_delete_oauth_client_secret(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => self
                .output
                .output_delete_oauth_client_secret(Ok(r.into_inner())),
            Err(r) => self.output.output_delete_oauth_client_secret(Err(r)),
        }
    }

    pub async fn execute_get_rfds(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.get_rfds();
        self.over.execute_get_rfds(matches, &mut request).unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => self.output.output_get_rfds(Ok(r.into_inner())),
            Err(r) => self.output.output_get_rfds(Err(r)),
        }
    }

    pub async fn execute_get_rfd(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.get_rfd();
        if let Some(value) = matches.get_one::<String>("number") {
            request = request.number(value.clone());
        }

        self.over.execute_get_rfd(matches, &mut request).unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => self.output.output_get_rfd(Ok(r.into_inner())),
            Err(r) => self.output.output_get_rfd(Err(r)),
        }
    }

    pub async fn execute_search_rfds(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.search_rfds();
        if let Some(value) = matches.get_one::<String>("q") {
            request = request.q(value.clone());
        }

        self.over
            .execute_search_rfds(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => self.output.output_search_rfds(Ok(r.into_inner())),
            Err(r) => self.output.output_search_rfds(Err(r)),
        }
    }

    pub async fn execute_get_self(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.get_self();
        self.over.execute_get_self(matches, &mut request).unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => self.output.output_get_self(Ok(r.into_inner())),
            Err(r) => self.output.output_get_self(Err(r)),
        }
    }
}

pub trait CliOverride {
    fn execute_create_api_user(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::CreateApiUser,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_get_api_user(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::GetApiUser,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_update_api_user(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::UpdateApiUser,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_add_api_user_to_group(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::AddApiUserToGroup,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_remove_api_user_from_group(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::RemoveApiUserFromGroup,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_list_api_user_tokens(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::ListApiUserTokens,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_create_api_user_token(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::CreateApiUserToken,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_get_api_user_token(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::GetApiUserToken,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_delete_api_user_token(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::DeleteApiUserToken,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_github_webhook(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::GithubWebhook,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_get_groups(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::GetGroups,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_create_group(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::CreateGroup,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_update_group(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::UpdateGroup,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_delete_group(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::DeleteGroup,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_authz_code_redirect(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::AuthzCodeRedirect,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_authz_code_callback(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::AuthzCodeCallback,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_authz_code_exchange(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::AuthzCodeExchange,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_get_device_provider(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::GetDeviceProvider,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_exchange_device_token(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::ExchangeDeviceToken,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_list_oauth_clients(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::ListOauthClients,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_create_oauth_client(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::CreateOauthClient,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_get_oauth_client(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::GetOauthClient,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_create_oauth_client_redirect_uri(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::CreateOauthClientRedirectUri,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_delete_oauth_client_redirect_uri(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::DeleteOauthClientRedirectUri,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_create_oauth_client_secret(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::CreateOauthClientSecret,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_delete_oauth_client_secret(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::DeleteOauthClientSecret,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_get_rfds(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::GetRfds,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_get_rfd(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::GetRfd,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_search_rfds(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::SearchRfds,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_get_self(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::GetSelf,
    ) -> Result<(), String> {
        Ok(())
    }
}

impl CliOverride for () {}

pub trait CliOutput {
    fn output_create_api_user(
        &self,
        response: Result<types::ApiUserForApiPermission, progenitor_client::Error<types::Error>>,
    ) {
    }

    fn output_get_api_user(
        &self,
        response: Result<types::ApiUserForApiPermission, progenitor_client::Error<types::Error>>,
    ) {
    }

    fn output_update_api_user(
        &self,
        response: Result<types::ApiUserForApiPermission, progenitor_client::Error<types::Error>>,
    ) {
    }

    fn output_add_api_user_to_group(
        &self,
        response: Result<types::ApiUserForApiPermission, progenitor_client::Error<types::Error>>,
    ) {
    }

    fn output_remove_api_user_from_group(
        &self,
        response: Result<types::ApiUserForApiPermission, progenitor_client::Error<types::Error>>,
    ) {
    }

    fn output_list_api_user_tokens(
        &self,
        response: Result<Vec<types::ApiKeyResponse>, progenitor_client::Error<types::Error>>,
    ) {
    }

    fn output_create_api_user_token(
        &self,
        response: Result<types::InitialApiKeyResponse, progenitor_client::Error<types::Error>>,
    ) {
    }

    fn output_get_api_user_token(
        &self,
        response: Result<types::ApiKeyResponse, progenitor_client::Error<types::Error>>,
    ) {
    }

    fn output_delete_api_user_token(
        &self,
        response: Result<types::ApiKeyResponse, progenitor_client::Error<types::Error>>,
    ) {
    }

    fn output_github_webhook(&self, response: Result<(), progenitor_client::Error<types::Error>>) {}

    fn output_get_groups(
        &self,
        response: Result<
            Vec<types::AccessGroupForApiPermission>,
            progenitor_client::Error<types::Error>,
        >,
    ) {
    }

    fn output_create_group(
        &self,
        response: Result<
            types::AccessGroupForApiPermission,
            progenitor_client::Error<types::Error>,
        >,
    ) {
    }

    fn output_update_group(
        &self,
        response: Result<
            types::AccessGroupForApiPermission,
            progenitor_client::Error<types::Error>,
        >,
    ) {
    }

    fn output_delete_group(
        &self,
        response: Result<
            types::AccessGroupForApiPermission,
            progenitor_client::Error<types::Error>,
        >,
    ) {
    }

    fn output_authz_code_redirect(&self, response: Result<(), progenitor_client::Error<()>>) {}

    fn output_authz_code_callback(
        &self,
        response: Result<(), progenitor_client::Error<types::Error>>,
    ) {
    }

    fn output_authz_code_exchange(
        &self,
        response: Result<
            types::OAuthAuthzCodeExchangeResponse,
            progenitor_client::Error<types::Error>,
        >,
    ) {
    }

    fn output_get_device_provider(
        &self,
        response: Result<types::OAuthProviderInfo, progenitor_client::Error<types::Error>>,
    ) {
    }

    fn output_exchange_device_token(&self, response: Result<(), progenitor_client::Error<()>>) {}

    fn output_list_oauth_clients(
        &self,
        response: Result<Vec<types::OAuthClient>, progenitor_client::Error<types::Error>>,
    ) {
    }

    fn output_create_oauth_client(
        &self,
        response: Result<types::OAuthClient, progenitor_client::Error<types::Error>>,
    ) {
    }

    fn output_get_oauth_client(
        &self,
        response: Result<types::OAuthClient, progenitor_client::Error<types::Error>>,
    ) {
    }

    fn output_create_oauth_client_redirect_uri(
        &self,
        response: Result<types::OAuthClientRedirectUri, progenitor_client::Error<types::Error>>,
    ) {
    }

    fn output_delete_oauth_client_redirect_uri(
        &self,
        response: Result<types::OAuthClientRedirectUri, progenitor_client::Error<types::Error>>,
    ) {
    }

    fn output_create_oauth_client_secret(
        &self,
        response: Result<
            types::InitialOAuthClientSecretResponse,
            progenitor_client::Error<types::Error>,
        >,
    ) {
    }

    fn output_delete_oauth_client_secret(
        &self,
        response: Result<types::OAuthClientSecret, progenitor_client::Error<types::Error>>,
    ) {
    }

    fn output_get_rfds(
        &self,
        response: Result<Vec<types::ListRfd>, progenitor_client::Error<types::Error>>,
    ) {
    }

    fn output_get_rfd(
        &self,
        response: Result<types::FullRfd, progenitor_client::Error<types::Error>>,
    ) {
    }

    fn output_search_rfds(
        &self,
        response: Result<Vec<types::ListRfd>, progenitor_client::Error<types::Error>>,
    ) {
    }

    fn output_get_self(
        &self,
        response: Result<types::ApiUserForApiPermission, progenitor_client::Error<types::Error>>,
    ) {
    }
}

impl CliOutput for () {}

#[derive(Copy, Clone, Debug)]
pub enum CliCommand {
    CreateApiUser,
    GetApiUser,
    UpdateApiUser,
    AddApiUserToGroup,
    RemoveApiUserFromGroup,
    ListApiUserTokens,
    CreateApiUserToken,
    GetApiUserToken,
    DeleteApiUserToken,
    GithubWebhook,
    GetGroups,
    CreateGroup,
    UpdateGroup,
    DeleteGroup,
    AuthzCodeRedirect,
    AuthzCodeCallback,
    AuthzCodeExchange,
    GetDeviceProvider,
    ExchangeDeviceToken,
    ListOauthClients,
    CreateOauthClient,
    GetOauthClient,
    CreateOauthClientRedirectUri,
    DeleteOauthClientRedirectUri,
    CreateOauthClientSecret,
    DeleteOauthClientSecret,
    GetRfds,
    GetRfd,
    SearchRfds,
    GetSelf,
}

impl CliCommand {
    pub fn iter() -> impl Iterator<Item = CliCommand> {
        vec![
            CliCommand::CreateApiUser,
            CliCommand::GetApiUser,
            CliCommand::UpdateApiUser,
            CliCommand::AddApiUserToGroup,
            CliCommand::RemoveApiUserFromGroup,
            CliCommand::ListApiUserTokens,
            CliCommand::CreateApiUserToken,
            CliCommand::GetApiUserToken,
            CliCommand::DeleteApiUserToken,
            CliCommand::GithubWebhook,
            CliCommand::GetGroups,
            CliCommand::CreateGroup,
            CliCommand::UpdateGroup,
            CliCommand::DeleteGroup,
            CliCommand::AuthzCodeRedirect,
            CliCommand::AuthzCodeCallback,
            CliCommand::AuthzCodeExchange,
            CliCommand::GetDeviceProvider,
            CliCommand::ExchangeDeviceToken,
            CliCommand::ListOauthClients,
            CliCommand::CreateOauthClient,
            CliCommand::GetOauthClient,
            CliCommand::CreateOauthClientRedirectUri,
            CliCommand::DeleteOauthClientRedirectUri,
            CliCommand::CreateOauthClientSecret,
            CliCommand::DeleteOauthClientSecret,
            CliCommand::GetRfds,
            CliCommand::GetRfd,
            CliCommand::SearchRfds,
            CliCommand::GetSelf,
        ]
        .into_iter()
    }
}
