// The contents of this file are generated; do not modify them.

use rfd_sdk::*;

use rfd_sdk::*;
pub struct Cli<T: CliConfig> {
    client: Client,
    config: T,
}

impl<T: CliConfig> Cli<T> {
    pub fn new(client: Client, config: T) -> Self {
        Self { client, config }
    }

    pub fn get_command(cmd: CliCommand) -> clap::Command {
        match cmd {
            CliCommand::JwksJson => Self::cli_jwks_json(),
            CliCommand::OpenidConfiguration => Self::cli_openid_configuration(),
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
            CliCommand::GetMappers => Self::cli_get_mappers(),
            CliCommand::CreateMapper => Self::cli_create_mapper(),
            CliCommand::DeleteMapper => Self::cli_delete_mapper(),
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
            CliCommand::GetRfdAttr => Self::cli_get_rfd_attr(),
            CliCommand::SetRfdAttr => Self::cli_set_rfd_attr(),
            CliCommand::UpdateRfdVisibility => Self::cli_update_rfd_visibility(),
            CliCommand::SearchRfds => Self::cli_search_rfds(),
            CliCommand::GetSelf => Self::cli_get_self(),
        }
    }

    pub fn cli_jwks_json() -> clap::Command {
        clap::Command::new("")
    }

    pub fn cli_openid_configuration() -> clap::Command {
        clap::Command::new("")
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
                            types::OAuthProviderName::Github.to_string(),
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
                clap::Arg::new("scope")
                    .long("scope")
                    .value_parser(clap::value_parser!(String))
                    .required(false),
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
                            types::OAuthProviderName::Github.to_string(),
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
                    .value_parser(clap::value_parser!(types::SecretString))
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
                            types::OAuthProviderName::Github.to_string(),
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
                        types::OAuthProviderName::Github.to_string(),
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
                            types::OAuthProviderName::Github.to_string(),
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

    pub fn cli_get_mappers() -> clap::Command {
        clap::Command::new("").arg(
            clap::Arg::new("include-depleted")
                .long("include-depleted")
                .value_parser(clap::value_parser!(bool))
                .required(false)
                .help("Include depleted mappers in the returned results"),
        )
    }

    pub fn cli_create_mapper() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("max-activations")
                    .long("max-activations")
                    .value_parser(clap::value_parser!(i32))
                    .required(false),
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

    pub fn cli_delete_mapper() -> clap::Command {
        clap::Command::new("").arg(
            clap::Arg::new("identifier")
                .long("identifier")
                .value_parser(clap::value_parser!(uuid::Uuid))
                .required(true),
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

    pub fn cli_get_rfd_attr() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("attr")
                    .long("attr")
                    .value_parser(clap::builder::TypedValueParser::map(
                        clap::builder::PossibleValuesParser::new([
                            types::RfdAttrName::Discussion.to_string(),
                            types::RfdAttrName::Labels.to_string(),
                            types::RfdAttrName::State.to_string(),
                        ]),
                        |s| types::RfdAttrName::try_from(s).unwrap(),
                    ))
                    .required(true),
            )
            .arg(
                clap::Arg::new("number")
                    .long("number")
                    .value_parser(clap::value_parser!(String))
                    .required(true),
            )
            .about("Get an attribute of a given RFD")
    }

    pub fn cli_set_rfd_attr() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("attr")
                    .long("attr")
                    .value_parser(clap::builder::TypedValueParser::map(
                        clap::builder::PossibleValuesParser::new([
                            types::RfdAttrName::Discussion.to_string(),
                            types::RfdAttrName::Labels.to_string(),
                            types::RfdAttrName::State.to_string(),
                        ]),
                        |s| types::RfdAttrName::try_from(s).unwrap(),
                    ))
                    .required(true),
            )
            .arg(
                clap::Arg::new("number")
                    .long("number")
                    .value_parser(clap::value_parser!(String))
                    .required(true),
            )
            .arg(
                clap::Arg::new("value")
                    .long("value")
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
            .about("Set an attribute of a given RFD")
    }

    pub fn cli_update_rfd_visibility() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("number")
                    .long("number")
                    .value_parser(clap::value_parser!(String))
                    .required(true),
            )
            .arg(
                clap::Arg::new("visibility")
                    .long("visibility")
                    .value_parser(clap::builder::TypedValueParser::map(
                        clap::builder::PossibleValuesParser::new([
                            types::Visibility::Public.to_string(),
                            types::Visibility::Private.to_string(),
                        ]),
                        |s| types::Visibility::try_from(s).unwrap(),
                    ))
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
            .about("Modify the visibility of an RFD")
    }

    pub fn cli_search_rfds() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("attributes-to-crop")
                    .long("attributes-to-crop")
                    .value_parser(clap::value_parser!(String))
                    .required(false),
            )
            .arg(
                clap::Arg::new("highlight-post-tag")
                    .long("highlight-post-tag")
                    .value_parser(clap::value_parser!(String))
                    .required(false),
            )
            .arg(
                clap::Arg::new("highlight-pre-tag")
                    .long("highlight-pre-tag")
                    .value_parser(clap::value_parser!(String))
                    .required(false),
            )
            .arg(
                clap::Arg::new("limit")
                    .long("limit")
                    .value_parser(clap::value_parser!(u32))
                    .required(false),
            )
            .arg(
                clap::Arg::new("offset")
                    .long("offset")
                    .value_parser(clap::value_parser!(u32))
                    .required(false),
            )
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

    pub async fn execute(&self, cmd: CliCommand, matches: &clap::ArgMatches) -> anyhow::Result<()> {
        match cmd {
            CliCommand::JwksJson => self.execute_jwks_json(matches).await,
            CliCommand::OpenidConfiguration => self.execute_openid_configuration(matches).await,
            CliCommand::CreateApiUser => self.execute_create_api_user(matches).await,
            CliCommand::GetApiUser => self.execute_get_api_user(matches).await,
            CliCommand::UpdateApiUser => self.execute_update_api_user(matches).await,
            CliCommand::AddApiUserToGroup => self.execute_add_api_user_to_group(matches).await,
            CliCommand::RemoveApiUserFromGroup => {
                self.execute_remove_api_user_from_group(matches).await
            }
            CliCommand::ListApiUserTokens => self.execute_list_api_user_tokens(matches).await,
            CliCommand::CreateApiUserToken => self.execute_create_api_user_token(matches).await,
            CliCommand::GetApiUserToken => self.execute_get_api_user_token(matches).await,
            CliCommand::DeleteApiUserToken => self.execute_delete_api_user_token(matches).await,
            CliCommand::GithubWebhook => self.execute_github_webhook(matches).await,
            CliCommand::GetGroups => self.execute_get_groups(matches).await,
            CliCommand::CreateGroup => self.execute_create_group(matches).await,
            CliCommand::UpdateGroup => self.execute_update_group(matches).await,
            CliCommand::DeleteGroup => self.execute_delete_group(matches).await,
            CliCommand::AuthzCodeRedirect => self.execute_authz_code_redirect(matches).await,
            CliCommand::AuthzCodeCallback => self.execute_authz_code_callback(matches).await,
            CliCommand::AuthzCodeExchange => self.execute_authz_code_exchange(matches).await,
            CliCommand::GetDeviceProvider => self.execute_get_device_provider(matches).await,
            CliCommand::ExchangeDeviceToken => self.execute_exchange_device_token(matches).await,
            CliCommand::GetMappers => self.execute_get_mappers(matches).await,
            CliCommand::CreateMapper => self.execute_create_mapper(matches).await,
            CliCommand::DeleteMapper => self.execute_delete_mapper(matches).await,
            CliCommand::ListOauthClients => self.execute_list_oauth_clients(matches).await,
            CliCommand::CreateOauthClient => self.execute_create_oauth_client(matches).await,
            CliCommand::GetOauthClient => self.execute_get_oauth_client(matches).await,
            CliCommand::CreateOauthClientRedirectUri => {
                self.execute_create_oauth_client_redirect_uri(matches).await
            }
            CliCommand::DeleteOauthClientRedirectUri => {
                self.execute_delete_oauth_client_redirect_uri(matches).await
            }
            CliCommand::CreateOauthClientSecret => {
                self.execute_create_oauth_client_secret(matches).await
            }
            CliCommand::DeleteOauthClientSecret => {
                self.execute_delete_oauth_client_secret(matches).await
            }
            CliCommand::GetRfds => self.execute_get_rfds(matches).await,
            CliCommand::GetRfd => self.execute_get_rfd(matches).await,
            CliCommand::GetRfdAttr => self.execute_get_rfd_attr(matches).await,
            CliCommand::SetRfdAttr => self.execute_set_rfd_attr(matches).await,
            CliCommand::UpdateRfdVisibility => self.execute_update_rfd_visibility(matches).await,
            CliCommand::SearchRfds => self.execute_search_rfds(matches).await,
            CliCommand::GetSelf => self.execute_get_self(matches).await,
        }
    }

    pub async fn execute_jwks_json(&self, matches: &clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.jwks_json();
        self.config.execute_jwks_json(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.item_success(&r);
                Ok(())
            }
            Err(r) => {
                self.config.item_error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_openid_configuration(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.openid_configuration();
        self.config
            .execute_openid_configuration(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.item_success(&r);
                Ok(())
            }
            Err(r) => {
                self.config.item_error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_create_api_user(&self, matches: &clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.create_api_user();
        if let Some(value) = matches.get_one::<std::path::PathBuf>("json-body") {
            let body_txt = std::fs::read_to_string(value).unwrap();
            let body_value = serde_json::from_str::<types::ApiUserUpdateParams>(&body_txt).unwrap();
            request = request.body(body_value);
        }

        self.config.execute_create_api_user(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.item_success(&r);
                Ok(())
            }
            Err(r) => {
                self.config.item_error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_get_api_user(&self, matches: &clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.get_api_user();
        if let Some(value) = matches.get_one::<uuid::Uuid>("identifier") {
            request = request.identifier(value.clone());
        }

        self.config.execute_get_api_user(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.item_success(&r);
                Ok(())
            }
            Err(r) => {
                self.config.item_error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_update_api_user(&self, matches: &clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.update_api_user();
        if let Some(value) = matches.get_one::<uuid::Uuid>("identifier") {
            request = request.identifier(value.clone());
        }

        if let Some(value) = matches.get_one::<std::path::PathBuf>("json-body") {
            let body_txt = std::fs::read_to_string(value).unwrap();
            let body_value = serde_json::from_str::<types::ApiUserUpdateParams>(&body_txt).unwrap();
            request = request.body(body_value);
        }

        self.config.execute_update_api_user(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.item_success(&r);
                Ok(())
            }
            Err(r) => {
                self.config.item_error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_add_api_user_to_group(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
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

        self.config
            .execute_add_api_user_to_group(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.item_success(&r);
                Ok(())
            }
            Err(r) => {
                self.config.item_error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_remove_api_user_from_group(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.remove_api_user_from_group();
        if let Some(value) = matches.get_one::<uuid::Uuid>("group-id") {
            request = request.group_id(value.clone());
        }

        if let Some(value) = matches.get_one::<uuid::Uuid>("identifier") {
            request = request.identifier(value.clone());
        }

        self.config
            .execute_remove_api_user_from_group(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.item_success(&r);
                Ok(())
            }
            Err(r) => {
                self.config.item_error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_list_api_user_tokens(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.list_api_user_tokens();
        if let Some(value) = matches.get_one::<uuid::Uuid>("identifier") {
            request = request.identifier(value.clone());
        }

        self.config
            .execute_list_api_user_tokens(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.item_success(&r);
                Ok(())
            }
            Err(r) => {
                self.config.item_error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_create_api_user_token(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
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

        self.config
            .execute_create_api_user_token(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.item_success(&r);
                Ok(())
            }
            Err(r) => {
                self.config.item_error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_get_api_user_token(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.get_api_user_token();
        if let Some(value) = matches.get_one::<uuid::Uuid>("identifier") {
            request = request.identifier(value.clone());
        }

        if let Some(value) = matches.get_one::<uuid::Uuid>("token-identifier") {
            request = request.token_identifier(value.clone());
        }

        self.config
            .execute_get_api_user_token(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.item_success(&r);
                Ok(())
            }
            Err(r) => {
                self.config.item_error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_delete_api_user_token(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.delete_api_user_token();
        if let Some(value) = matches.get_one::<uuid::Uuid>("identifier") {
            request = request.identifier(value.clone());
        }

        if let Some(value) = matches.get_one::<uuid::Uuid>("token-identifier") {
            request = request.token_identifier(value.clone());
        }

        self.config
            .execute_delete_api_user_token(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.item_success(&r);
                Ok(())
            }
            Err(r) => {
                self.config.item_error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_github_webhook(&self, matches: &clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.github_webhook();
        if let Some(value) = matches.get_one::<String>("ref") {
            request = request.body_map(|body| body.ref_(value.clone()))
        }

        if let Some(value) = matches.get_one::<std::path::PathBuf>("json-body") {
            let body_txt = std::fs::read_to_string(value).unwrap();
            let body_value = serde_json::from_str::<types::GitHubCommitPayload>(&body_txt).unwrap();
            request = request.body(body_value);
        }

        self.config.execute_github_webhook(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.item_success(&r);
                Ok(())
            }
            Err(r) => {
                self.config.item_error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_get_groups(&self, matches: &clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.get_groups();
        self.config.execute_get_groups(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.item_success(&r);
                Ok(())
            }
            Err(r) => {
                self.config.item_error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_create_group(&self, matches: &clap::ArgMatches) -> anyhow::Result<()> {
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

        self.config.execute_create_group(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.item_success(&r);
                Ok(())
            }
            Err(r) => {
                self.config.item_error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_update_group(&self, matches: &clap::ArgMatches) -> anyhow::Result<()> {
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

        self.config.execute_update_group(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.item_success(&r);
                Ok(())
            }
            Err(r) => {
                self.config.item_error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_delete_group(&self, matches: &clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.delete_group();
        if let Some(value) = matches.get_one::<uuid::Uuid>("group-id") {
            request = request.group_id(value.clone());
        }

        self.config.execute_delete_group(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.item_success(&r);
                Ok(())
            }
            Err(r) => {
                self.config.item_error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_authz_code_redirect(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
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

        if let Some(value) = matches.get_one::<String>("scope") {
            request = request.scope(value.clone());
        }

        if let Some(value) = matches.get_one::<String>("state") {
            request = request.state(value.clone());
        }

        self.config
            .execute_authz_code_redirect(matches, &mut request)?;
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

    pub async fn execute_authz_code_callback(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
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

        self.config
            .execute_authz_code_callback(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                todo!()
            }
            Err(r) => {
                self.config.item_error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_authz_code_exchange(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.authz_code_exchange();
        if let Some(value) = matches.get_one::<uuid::Uuid>("client-id") {
            request = request.body_map(|body| body.client_id(value.clone()))
        }

        if let Some(value) = matches.get_one::<types::SecretString>("client-secret") {
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

        self.config
            .execute_authz_code_exchange(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.item_success(&r);
                Ok(())
            }
            Err(r) => {
                self.config.item_error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_get_device_provider(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.get_device_provider();
        if let Some(value) = matches.get_one::<types::OAuthProviderName>("provider") {
            request = request.provider(value.clone());
        }

        self.config
            .execute_get_device_provider(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.item_success(&r);
                Ok(())
            }
            Err(r) => {
                self.config.item_error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_exchange_device_token(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
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

        self.config
            .execute_exchange_device_token(matches, &mut request)?;
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

    pub async fn execute_get_mappers(&self, matches: &clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.get_mappers();
        if let Some(value) = matches.get_one::<bool>("include-depleted") {
            request = request.include_depleted(value.clone());
        }

        self.config.execute_get_mappers(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.item_success(&r);
                Ok(())
            }
            Err(r) => {
                self.config.item_error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_create_mapper(&self, matches: &clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.create_mapper();
        if let Some(value) = matches.get_one::<i32>("max-activations") {
            request = request.body_map(|body| body.max_activations(value.clone()))
        }

        if let Some(value) = matches.get_one::<String>("name") {
            request = request.body_map(|body| body.name(value.clone()))
        }

        if let Some(value) = matches.get_one::<std::path::PathBuf>("json-body") {
            let body_txt = std::fs::read_to_string(value).unwrap();
            let body_value = serde_json::from_str::<types::CreateMapper>(&body_txt).unwrap();
            request = request.body(body_value);
        }

        self.config.execute_create_mapper(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.item_success(&r);
                Ok(())
            }
            Err(r) => {
                self.config.item_error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_delete_mapper(&self, matches: &clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.delete_mapper();
        if let Some(value) = matches.get_one::<uuid::Uuid>("identifier") {
            request = request.identifier(value.clone());
        }

        self.config.execute_delete_mapper(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.item_success(&r);
                Ok(())
            }
            Err(r) => {
                self.config.item_error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_list_oauth_clients(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.list_oauth_clients();
        self.config
            .execute_list_oauth_clients(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.item_success(&r);
                Ok(())
            }
            Err(r) => {
                self.config.item_error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_create_oauth_client(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.create_oauth_client();
        self.config
            .execute_create_oauth_client(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.item_success(&r);
                Ok(())
            }
            Err(r) => {
                self.config.item_error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_get_oauth_client(&self, matches: &clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.get_oauth_client();
        if let Some(value) = matches.get_one::<uuid::Uuid>("client-id") {
            request = request.client_id(value.clone());
        }

        self.config
            .execute_get_oauth_client(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.item_success(&r);
                Ok(())
            }
            Err(r) => {
                self.config.item_error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_create_oauth_client_redirect_uri(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
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

        self.config
            .execute_create_oauth_client_redirect_uri(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.item_success(&r);
                Ok(())
            }
            Err(r) => {
                self.config.item_error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_delete_oauth_client_redirect_uri(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.delete_oauth_client_redirect_uri();
        if let Some(value) = matches.get_one::<uuid::Uuid>("client-id") {
            request = request.client_id(value.clone());
        }

        if let Some(value) = matches.get_one::<uuid::Uuid>("redirect-uri-id") {
            request = request.redirect_uri_id(value.clone());
        }

        self.config
            .execute_delete_oauth_client_redirect_uri(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.item_success(&r);
                Ok(())
            }
            Err(r) => {
                self.config.item_error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_create_oauth_client_secret(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.create_oauth_client_secret();
        if let Some(value) = matches.get_one::<uuid::Uuid>("client-id") {
            request = request.client_id(value.clone());
        }

        self.config
            .execute_create_oauth_client_secret(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.item_success(&r);
                Ok(())
            }
            Err(r) => {
                self.config.item_error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_delete_oauth_client_secret(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.delete_oauth_client_secret();
        if let Some(value) = matches.get_one::<uuid::Uuid>("client-id") {
            request = request.client_id(value.clone());
        }

        if let Some(value) = matches.get_one::<uuid::Uuid>("secret-id") {
            request = request.secret_id(value.clone());
        }

        self.config
            .execute_delete_oauth_client_secret(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.item_success(&r);
                Ok(())
            }
            Err(r) => {
                self.config.item_error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_get_rfds(&self, matches: &clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.get_rfds();
        self.config.execute_get_rfds(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.item_success(&r);
                Ok(())
            }
            Err(r) => {
                self.config.item_error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_get_rfd(&self, matches: &clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.get_rfd();
        if let Some(value) = matches.get_one::<String>("number") {
            request = request.number(value.clone());
        }

        self.config.execute_get_rfd(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.item_success(&r);
                Ok(())
            }
            Err(r) => {
                self.config.item_error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_get_rfd_attr(&self, matches: &clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.get_rfd_attr();
        if let Some(value) = matches.get_one::<types::RfdAttrName>("attr") {
            request = request.attr(value.clone());
        }

        if let Some(value) = matches.get_one::<String>("number") {
            request = request.number(value.clone());
        }

        self.config.execute_get_rfd_attr(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.item_success(&r);
                Ok(())
            }
            Err(r) => {
                self.config.item_error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_set_rfd_attr(&self, matches: &clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.set_rfd_attr();
        if let Some(value) = matches.get_one::<types::RfdAttrName>("attr") {
            request = request.attr(value.clone());
        }

        if let Some(value) = matches.get_one::<String>("number") {
            request = request.number(value.clone());
        }

        if let Some(value) = matches.get_one::<String>("value") {
            request = request.body_map(|body| body.value(value.clone()))
        }

        if let Some(value) = matches.get_one::<std::path::PathBuf>("json-body") {
            let body_txt = std::fs::read_to_string(value).unwrap();
            let body_value = serde_json::from_str::<types::RfdAttrValue>(&body_txt).unwrap();
            request = request.body(body_value);
        }

        self.config.execute_set_rfd_attr(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.item_success(&r);
                Ok(())
            }
            Err(r) => {
                self.config.item_error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_update_rfd_visibility(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.update_rfd_visibility();
        if let Some(value) = matches.get_one::<String>("number") {
            request = request.number(value.clone());
        }

        if let Some(value) = matches.get_one::<types::Visibility>("visibility") {
            request = request.body_map(|body| body.visibility(value.clone()))
        }

        if let Some(value) = matches.get_one::<std::path::PathBuf>("json-body") {
            let body_txt = std::fs::read_to_string(value).unwrap();
            let body_value = serde_json::from_str::<types::RfdVisibility>(&body_txt).unwrap();
            request = request.body(body_value);
        }

        self.config
            .execute_update_rfd_visibility(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.item_success(&r);
                Ok(())
            }
            Err(r) => {
                self.config.item_error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_search_rfds(&self, matches: &clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.search_rfds();
        if let Some(value) = matches.get_one::<String>("attributes-to-crop") {
            request = request.attributes_to_crop(value.clone());
        }

        if let Some(value) = matches.get_one::<String>("highlight-post-tag") {
            request = request.highlight_post_tag(value.clone());
        }

        if let Some(value) = matches.get_one::<String>("highlight-pre-tag") {
            request = request.highlight_pre_tag(value.clone());
        }

        if let Some(value) = matches.get_one::<u32>("limit") {
            request = request.limit(value.clone());
        }

        if let Some(value) = matches.get_one::<u32>("offset") {
            request = request.offset(value.clone());
        }

        if let Some(value) = matches.get_one::<String>("q") {
            request = request.q(value.clone());
        }

        self.config.execute_search_rfds(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.item_success(&r);
                Ok(())
            }
            Err(r) => {
                self.config.item_error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_get_self(&self, matches: &clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.get_self();
        self.config.execute_get_self(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.item_success(&r);
                Ok(())
            }
            Err(r) => {
                self.config.item_error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }
}

pub trait CliConfig {
    fn item_success<T>(&self, value: &ResponseValue<T>)
    where
        T: schemars::JsonSchema + serde::Serialize + std::fmt::Debug;
    fn item_error<T>(&self, value: &Error<T>)
    where
        T: schemars::JsonSchema + serde::Serialize + std::fmt::Debug;
    fn list_start<T>(&self)
    where
        T: schemars::JsonSchema + serde::Serialize + std::fmt::Debug;
    fn list_item<T>(&self, value: &T)
    where
        T: schemars::JsonSchema + serde::Serialize + std::fmt::Debug;
    fn list_end_success<T>(&self)
    where
        T: schemars::JsonSchema + serde::Serialize + std::fmt::Debug;
    fn list_end_error<T>(&self, value: &Error<T>)
    where
        T: schemars::JsonSchema + serde::Serialize + std::fmt::Debug;
    fn execute_jwks_json(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::JwksJson,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_openid_configuration(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::OpenidConfiguration,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_create_api_user(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::CreateApiUser,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_get_api_user(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::GetApiUser,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_update_api_user(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::UpdateApiUser,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_add_api_user_to_group(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::AddApiUserToGroup,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_remove_api_user_from_group(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::RemoveApiUserFromGroup,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_list_api_user_tokens(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::ListApiUserTokens,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_create_api_user_token(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::CreateApiUserToken,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_get_api_user_token(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::GetApiUserToken,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_delete_api_user_token(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::DeleteApiUserToken,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_github_webhook(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::GithubWebhook,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_get_groups(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::GetGroups,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_create_group(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::CreateGroup,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_update_group(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::UpdateGroup,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_delete_group(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::DeleteGroup,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_authz_code_redirect(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::AuthzCodeRedirect,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_authz_code_callback(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::AuthzCodeCallback,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_authz_code_exchange(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::AuthzCodeExchange,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_get_device_provider(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::GetDeviceProvider,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_exchange_device_token(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::ExchangeDeviceToken,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_get_mappers(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::GetMappers,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_create_mapper(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::CreateMapper,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_delete_mapper(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::DeleteMapper,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_list_oauth_clients(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::ListOauthClients,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_create_oauth_client(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::CreateOauthClient,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_get_oauth_client(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::GetOauthClient,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_create_oauth_client_redirect_uri(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::CreateOauthClientRedirectUri,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_delete_oauth_client_redirect_uri(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::DeleteOauthClientRedirectUri,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_create_oauth_client_secret(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::CreateOauthClientSecret,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_delete_oauth_client_secret(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::DeleteOauthClientSecret,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_get_rfds(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::GetRfds,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_get_rfd(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::GetRfd,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_get_rfd_attr(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::GetRfdAttr,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_set_rfd_attr(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::SetRfdAttr,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_update_rfd_visibility(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::UpdateRfdVisibility,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_search_rfds(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::SearchRfds,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_get_self(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::GetSelf,
    ) -> anyhow::Result<()> {
        Ok(())
    }
}

#[derive(Copy, Clone, Debug)]
pub enum CliCommand {
    JwksJson,
    OpenidConfiguration,
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
    GetMappers,
    CreateMapper,
    DeleteMapper,
    ListOauthClients,
    CreateOauthClient,
    GetOauthClient,
    CreateOauthClientRedirectUri,
    DeleteOauthClientRedirectUri,
    CreateOauthClientSecret,
    DeleteOauthClientSecret,
    GetRfds,
    GetRfd,
    GetRfdAttr,
    SetRfdAttr,
    UpdateRfdVisibility,
    SearchRfds,
    GetSelf,
}

impl CliCommand {
    pub fn iter() -> impl Iterator<Item = CliCommand> {
        vec![
            CliCommand::JwksJson,
            CliCommand::OpenidConfiguration,
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
            CliCommand::GetMappers,
            CliCommand::CreateMapper,
            CliCommand::DeleteMapper,
            CliCommand::ListOauthClients,
            CliCommand::CreateOauthClient,
            CliCommand::GetOauthClient,
            CliCommand::CreateOauthClientRedirectUri,
            CliCommand::DeleteOauthClientRedirectUri,
            CliCommand::CreateOauthClientSecret,
            CliCommand::DeleteOauthClientSecret,
            CliCommand::GetRfds,
            CliCommand::GetRfd,
            CliCommand::GetRfdAttr,
            CliCommand::SetRfdAttr,
            CliCommand::UpdateRfdVisibility,
            CliCommand::SearchRfds,
            CliCommand::GetSelf,
        ]
        .into_iter()
    }
}
