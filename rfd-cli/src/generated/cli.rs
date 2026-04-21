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

    pub fn get_command(cmd: CliCommand) -> ::clap::Command {
        match cmd {
            CliCommand::JwksJson => Self::cli_jwks_json(),
            CliCommand::OpenidConfiguration => Self::cli_openid_configuration(),
            CliCommand::ListApiUsers => Self::cli_list_api_users(),
            CliCommand::CreateApiUser => Self::cli_create_api_user(),
            CliCommand::GetApiUser => Self::cli_get_api_user(),
            CliCommand::UpdateApiUser => Self::cli_update_api_user(),
            CliCommand::SetApiUserContactEmail => Self::cli_set_api_user_contact_email(),
            CliCommand::AddApiUserToGroup => Self::cli_add_api_user_to_group(),
            CliCommand::RemoveApiUserFromGroup => Self::cli_remove_api_user_from_group(),
            CliCommand::LinkProvider => Self::cli_link_provider(),
            CliCommand::ListApiUserTokens => Self::cli_list_api_user_tokens(),
            CliCommand::CreateApiUserToken => Self::cli_create_api_user_token(),
            CliCommand::GetApiUserToken => Self::cli_get_api_user_token(),
            CliCommand::DeleteApiUserToken => Self::cli_delete_api_user_token(),
            CliCommand::CreateLinkToken => Self::cli_create_link_token(),
            CliCommand::GithubWebhook => Self::cli_github_webhook(),
            CliCommand::GetGroups => Self::cli_get_groups(),
            CliCommand::CreateGroup => Self::cli_create_group(),
            CliCommand::UpdateGroup => Self::cli_update_group(),
            CliCommand::DeleteGroup => Self::cli_delete_group(),
            CliCommand::GetGroupMembers => Self::cli_get_group_members(),
            CliCommand::ListJobs => Self::cli_list_jobs(),
            CliCommand::MagicLinkExchange => Self::cli_magic_link_exchange(),
            CliCommand::MagicLinkSend => Self::cli_magic_link_send(),
            CliCommand::AuthzCodeRedirect => Self::cli_authz_code_redirect(),
            CliCommand::AuthzCodeCallback => Self::cli_authz_code_callback(),
            CliCommand::AuthzCodeExchange => Self::cli_authz_code_exchange(),
            CliCommand::GetDeviceProvider => Self::cli_get_device_provider(),
            CliCommand::ExchangeDeviceToken => Self::cli_exchange_device_token(),
            CliCommand::ListMagicLinks => Self::cli_list_magic_links(),
            CliCommand::CreateMagicLink => Self::cli_create_magic_link(),
            CliCommand::GetMagicLink => Self::cli_get_magic_link(),
            CliCommand::CreateMagicLinkRedirectUri => Self::cli_create_magic_link_redirect_uri(),
            CliCommand::DeleteMagicLinkRedirectUri => Self::cli_delete_magic_link_redirect_uri(),
            CliCommand::CreateMagicLinkSecret => Self::cli_create_magic_link_secret(),
            CliCommand::DeleteMagicLinkSecret => Self::cli_delete_magic_link_secret(),
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
            CliCommand::ListRfds => Self::cli_list_rfds(),
            CliCommand::ReserveRfd => Self::cli_reserve_rfd(),
            CliCommand::ViewRfdMeta => Self::cli_view_rfd_meta(),
            CliCommand::ViewRfdAttr => Self::cli_view_rfd_attr(),
            CliCommand::SetRfdAttr => Self::cli_set_rfd_attr(),
            CliCommand::SetRfdContent => Self::cli_set_rfd_content(),
            CliCommand::ViewRfdDiscussion => Self::cli_view_rfd_discussion(),
            CliCommand::ViewRfdPdf => Self::cli_view_rfd_pdf(),
            CliCommand::ViewRfd => Self::cli_view_rfd(),
            CliCommand::SetRfdDocument => Self::cli_set_rfd_document(),
            CliCommand::ListRfdRevisions => Self::cli_list_rfd_revisions(),
            CliCommand::ViewRfdRevisionMeta => Self::cli_view_rfd_revision_meta(),
            CliCommand::UpdateRfdRevision => Self::cli_update_rfd_revision(),
            CliCommand::ViewRfdRevisionAttr => Self::cli_view_rfd_revision_attr(),
            CliCommand::ViewRfdRevisionDiscussion => Self::cli_view_rfd_revision_discussion(),
            CliCommand::ViewRfdRevisionPdf => Self::cli_view_rfd_revision_pdf(),
            CliCommand::ViewRfdRevision => Self::cli_view_rfd_revision(),
            CliCommand::DiscussRfd => Self::cli_discuss_rfd(),
            CliCommand::PublishRfd => Self::cli_publish_rfd(),
            CliCommand::UpdateRfdVisibility => Self::cli_update_rfd_visibility(),
            CliCommand::SearchRfds => Self::cli_search_rfds(),
            CliCommand::GetSelf => Self::cli_get_self(),
        }
    }

    pub fn cli_jwks_json() -> ::clap::Command {
        ::clap::Command::new("")
    }

    pub fn cli_openid_configuration() -> ::clap::Command {
        ::clap::Command::new("")
    }

    pub fn cli_list_api_users() -> ::clap::Command {
        ::clap::Command::new("").about("List details for users")
    }

    pub fn cli_create_api_user() -> ::clap::Command {
        ::clap::Command::new("")
            .arg(
                ::clap::Arg::new("json-body")
                    .long("json-body")
                    .value_name("JSON-FILE")
                    .required(true)
                    .value_parser(::clap::value_parser!(std::path::PathBuf))
                    .help("Path to a file that contains the full json body."),
            )
            .arg(
                ::clap::Arg::new("json-body-template")
                    .long("json-body-template")
                    .action(::clap::ArgAction::SetTrue)
                    .help("XXX"),
            )
            .about("Create a new user")
    }

    pub fn cli_get_api_user() -> ::clap::Command {
        ::clap::Command::new("")
            .arg(
                ::clap::Arg::new("user-id")
                    .long("user-id")
                    .value_parser(::clap::value_parser!(types::TypedUuidForUserId))
                    .required(true),
            )
            .about("View details for a user")
    }

    pub fn cli_update_api_user() -> ::clap::Command {
        ::clap::Command::new("")
            .arg(
                ::clap::Arg::new("user-id")
                    .long("user-id")
                    .value_parser(::clap::value_parser!(types::TypedUuidForUserId))
                    .required(true),
            )
            .arg(
                ::clap::Arg::new("json-body")
                    .long("json-body")
                    .value_name("JSON-FILE")
                    .required(true)
                    .value_parser(::clap::value_parser!(std::path::PathBuf))
                    .help("Path to a file that contains the full json body."),
            )
            .arg(
                ::clap::Arg::new("json-body-template")
                    .long("json-body-template")
                    .action(::clap::ArgAction::SetTrue)
                    .help("XXX"),
            )
            .about("Update the permissions assigned to a given user")
    }

    pub fn cli_set_api_user_contact_email() -> ::clap::Command {
        ::clap::Command::new("")
            .arg(
                ::clap::Arg::new("email")
                    .long("email")
                    .value_parser(::clap::value_parser!(::std::string::String))
                    .required_unless_present("json-body"),
            )
            .arg(
                ::clap::Arg::new("user-id")
                    .long("user-id")
                    .value_parser(::clap::value_parser!(types::TypedUuidForUserId))
                    .required(true),
            )
            .arg(
                ::clap::Arg::new("json-body")
                    .long("json-body")
                    .value_name("JSON-FILE")
                    .required(false)
                    .value_parser(::clap::value_parser!(std::path::PathBuf))
                    .help("Path to a file that contains the full json body."),
            )
            .arg(
                ::clap::Arg::new("json-body-template")
                    .long("json-body-template")
                    .action(::clap::ArgAction::SetTrue)
                    .help("XXX"),
            )
            .about("Set the contact email for a user")
    }

    pub fn cli_add_api_user_to_group() -> ::clap::Command {
        ::clap::Command::new("")
            .arg(
                ::clap::Arg::new("group-id")
                    .long("group-id")
                    .value_parser(::clap::value_parser!(types::TypedUuidForAccessGroupId))
                    .required_unless_present("json-body"),
            )
            .arg(
                ::clap::Arg::new("user-id")
                    .long("user-id")
                    .value_parser(::clap::value_parser!(types::TypedUuidForUserId))
                    .required(true),
            )
            .arg(
                ::clap::Arg::new("json-body")
                    .long("json-body")
                    .value_name("JSON-FILE")
                    .required(false)
                    .value_parser(::clap::value_parser!(std::path::PathBuf))
                    .help("Path to a file that contains the full json body."),
            )
            .arg(
                ::clap::Arg::new("json-body-template")
                    .long("json-body-template")
                    .action(::clap::ArgAction::SetTrue)
                    .help("XXX"),
            )
            .about("Add a user to a group")
    }

    pub fn cli_remove_api_user_from_group() -> ::clap::Command {
        ::clap::Command::new("")
            .arg(
                ::clap::Arg::new("group-id")
                    .long("group-id")
                    .value_parser(::clap::value_parser!(types::TypedUuidForAccessGroupId))
                    .required(true),
            )
            .arg(
                ::clap::Arg::new("user-id")
                    .long("user-id")
                    .value_parser(::clap::value_parser!(types::TypedUuidForUserId))
                    .required(true),
            )
            .about("Remove a user from a group")
    }

    pub fn cli_link_provider() -> ::clap::Command {
        ::clap::Command::new("")
            .arg(
                ::clap::Arg::new("token")
                    .long("token")
                    .value_parser(::clap::value_parser!(::std::string::String))
                    .required_unless_present("json-body"),
            )
            .arg(
                ::clap::Arg::new("user-id")
                    .long("user-id")
                    .value_parser(::clap::value_parser!(types::TypedUuidForUserId))
                    .required(true),
            )
            .arg(
                ::clap::Arg::new("json-body")
                    .long("json-body")
                    .value_name("JSON-FILE")
                    .required(false)
                    .value_parser(::clap::value_parser!(std::path::PathBuf))
                    .help("Path to a file that contains the full json body."),
            )
            .arg(
                ::clap::Arg::new("json-body-template")
                    .long("json-body-template")
                    .action(::clap::ArgAction::SetTrue)
                    .help("XXX"),
            )
            .about("Link an existing login provider to this user")
    }

    pub fn cli_list_api_user_tokens() -> ::clap::Command {
        ::clap::Command::new("")
            .arg(
                ::clap::Arg::new("user-id")
                    .long("user-id")
                    .value_parser(::clap::value_parser!(types::TypedUuidForUserId))
                    .required(true),
            )
            .about("List api keys for a user")
    }

    pub fn cli_create_api_user_token() -> ::clap::Command {
        ::clap::Command::new("")
            .arg(
                ::clap::Arg::new("expires-at")
                    .long("expires-at")
                    .value_parser(::clap::value_parser!(
                        ::chrono::DateTime<::chrono::offset::Utc>
                    ))
                    .required_unless_present("json-body"),
            )
            .arg(
                ::clap::Arg::new("user-id")
                    .long("user-id")
                    .value_parser(::clap::value_parser!(types::TypedUuidForUserId))
                    .required(true),
            )
            .arg(
                ::clap::Arg::new("json-body")
                    .long("json-body")
                    .value_name("JSON-FILE")
                    .required(false)
                    .value_parser(::clap::value_parser!(std::path::PathBuf))
                    .help("Path to a file that contains the full json body."),
            )
            .arg(
                ::clap::Arg::new("json-body-template")
                    .long("json-body-template")
                    .action(::clap::ArgAction::SetTrue)
                    .help("XXX"),
            )
            .about("Create a new api key for a user")
    }

    pub fn cli_get_api_user_token() -> ::clap::Command {
        ::clap::Command::new("")
            .arg(
                ::clap::Arg::new("api-key-id")
                    .long("api-key-id")
                    .value_parser(::clap::value_parser!(types::TypedUuidForApiKeyId))
                    .required(true),
            )
            .arg(
                ::clap::Arg::new("user-id")
                    .long("user-id")
                    .value_parser(::clap::value_parser!(types::TypedUuidForUserId))
                    .required(true),
            )
            .about("View details of an api key for a user")
    }

    pub fn cli_delete_api_user_token() -> ::clap::Command {
        ::clap::Command::new("")
            .arg(
                ::clap::Arg::new("api-key-id")
                    .long("api-key-id")
                    .value_parser(::clap::value_parser!(types::TypedUuidForApiKeyId))
                    .required(true),
            )
            .arg(
                ::clap::Arg::new("user-id")
                    .long("user-id")
                    .value_parser(::clap::value_parser!(types::TypedUuidForUserId))
                    .required(true),
            )
            .about("Revoke an api key for a user")
    }

    pub fn cli_create_link_token() -> ::clap::Command {
        ::clap::Command::new("")
            .arg(
                ::clap::Arg::new("provider-id")
                    .long("provider-id")
                    .value_parser(::clap::value_parser!(types::TypedUuidForUserProviderId))
                    .required(true),
            )
            .arg(
                ::clap::Arg::new("user-id")
                    .long("user-id")
                    .value_parser(::clap::value_parser!(types::TypedUuidForUserId))
                    .required_unless_present("json-body"),
            )
            .arg(
                ::clap::Arg::new("json-body")
                    .long("json-body")
                    .value_name("JSON-FILE")
                    .required(false)
                    .value_parser(::clap::value_parser!(std::path::PathBuf))
                    .help("Path to a file that contains the full json body."),
            )
            .arg(
                ::clap::Arg::new("json-body-template")
                    .long("json-body-template")
                    .action(::clap::ArgAction::SetTrue)
                    .help("XXX"),
            )
            .about("Create a new link token for linking this provider to a different api user")
    }

    pub fn cli_github_webhook() -> ::clap::Command {
        ::clap::Command::new("")
            .arg(
                ::clap::Arg::new("ref")
                    .long("ref")
                    .value_parser(::clap::value_parser!(::std::string::String))
                    .required_unless_present("json-body"),
            )
            .arg(
                ::clap::Arg::new("json-body")
                    .long("json-body")
                    .value_name("JSON-FILE")
                    .required(true)
                    .value_parser(::clap::value_parser!(std::path::PathBuf))
                    .help("Path to a file that contains the full json body."),
            )
            .arg(
                ::clap::Arg::new("json-body-template")
                    .long("json-body-template")
                    .action(::clap::ArgAction::SetTrue)
                    .help("XXX"),
            )
    }

    pub fn cli_get_groups() -> ::clap::Command {
        ::clap::Command::new("").about("List all groups")
    }

    pub fn cli_create_group() -> ::clap::Command {
        ::clap::Command::new("")
            .arg(
                ::clap::Arg::new("name")
                    .long("name")
                    .value_parser(::clap::value_parser!(::std::string::String))
                    .required_unless_present("json-body"),
            )
            .arg(
                ::clap::Arg::new("json-body")
                    .long("json-body")
                    .value_name("JSON-FILE")
                    .required(true)
                    .value_parser(::clap::value_parser!(std::path::PathBuf))
                    .help("Path to a file that contains the full json body."),
            )
            .arg(
                ::clap::Arg::new("json-body-template")
                    .long("json-body-template")
                    .action(::clap::ArgAction::SetTrue)
                    .help("XXX"),
            )
            .about("Create a group")
    }

    pub fn cli_update_group() -> ::clap::Command {
        ::clap::Command::new("")
            .arg(
                ::clap::Arg::new("group-id")
                    .long("group-id")
                    .value_parser(::clap::value_parser!(types::TypedUuidForAccessGroupId))
                    .required(true),
            )
            .arg(
                ::clap::Arg::new("name")
                    .long("name")
                    .value_parser(::clap::value_parser!(::std::string::String))
                    .required_unless_present("json-body"),
            )
            .arg(
                ::clap::Arg::new("json-body")
                    .long("json-body")
                    .value_name("JSON-FILE")
                    .required(true)
                    .value_parser(::clap::value_parser!(std::path::PathBuf))
                    .help("Path to a file that contains the full json body."),
            )
            .arg(
                ::clap::Arg::new("json-body-template")
                    .long("json-body-template")
                    .action(::clap::ArgAction::SetTrue)
                    .help("XXX"),
            )
            .about("Update a group")
    }

    pub fn cli_delete_group() -> ::clap::Command {
        ::clap::Command::new("")
            .arg(
                ::clap::Arg::new("group-id")
                    .long("group-id")
                    .value_parser(::clap::value_parser!(types::TypedUuidForAccessGroupId))
                    .required(true),
            )
            .about("Delete a group")
    }

    pub fn cli_get_group_members() -> ::clap::Command {
        ::clap::Command::new("")
            .arg(
                ::clap::Arg::new("group-id")
                    .long("group-id")
                    .value_parser(::clap::value_parser!(types::TypedUuidForAccessGroupId))
                    .required(true),
            )
            .about("Get members of a group")
    }

    pub fn cli_list_jobs() -> ::clap::Command {
        ::clap::Command::new("")
            .arg(
                ::clap::Arg::new("limit")
                    .long("limit")
                    .value_parser(::clap::value_parser!(i64))
                    .required(false),
            )
            .arg(
                ::clap::Arg::new("offset")
                    .long("offset")
                    .value_parser(::clap::value_parser!(i64))
                    .required(false),
            )
            .arg(
                ::clap::Arg::new("rfd")
                    .long("rfd")
                    .value_parser(::clap::value_parser!(::std::string::String))
                    .required(true),
            )
            .about("List all jobs for a RFD")
    }

    pub fn cli_magic_link_exchange() -> ::clap::Command {
        ::clap::Command::new("")
            .arg(
                ::clap::Arg::new("attempt-id")
                    .long("attempt-id")
                    .value_parser(::clap::value_parser!(types::TypedUuidForMagicLinkAttemptId))
                    .required_unless_present("json-body"),
            )
            .arg(
                ::clap::Arg::new("channel")
                    .long("channel")
                    .value_parser(::clap::value_parser!(::std::string::String))
                    .required(true),
            )
            .arg(
                ::clap::Arg::new("recipient")
                    .long("recipient")
                    .value_parser(::clap::value_parser!(::std::string::String))
                    .required_unless_present("json-body"),
            )
            .arg(
                ::clap::Arg::new("secret")
                    .long("secret")
                    .value_parser(::clap::value_parser!(::std::string::String))
                    .required_unless_present("json-body"),
            )
            .arg(
                ::clap::Arg::new("json-body")
                    .long("json-body")
                    .value_name("JSON-FILE")
                    .required(false)
                    .value_parser(::clap::value_parser!(std::path::PathBuf))
                    .help("Path to a file that contains the full json body."),
            )
            .arg(
                ::clap::Arg::new("json-body-template")
                    .long("json-body-template")
                    .action(::clap::ArgAction::SetTrue)
                    .help("XXX"),
            )
            .about("Exchange a magic link access code for an access token")
    }

    pub fn cli_magic_link_send() -> ::clap::Command {
        ::clap::Command::new("")
            .arg(
                ::clap::Arg::new("channel")
                    .long("channel")
                    .value_parser(::clap::value_parser!(::std::string::String))
                    .required(true),
            )
            .arg(
                ::clap::Arg::new("expires-in")
                    .long("expires-in")
                    .value_parser(::clap::value_parser!(i64))
                    .required_unless_present("json-body"),
            )
            .arg(
                ::clap::Arg::new("medium")
                    .long("medium")
                    .value_parser(::clap::builder::TypedValueParser::map(
                        ::clap::builder::PossibleValuesParser::new([
                            types::MagicLinkMedium::Email.to_string()
                        ]),
                        |s| types::MagicLinkMedium::try_from(s).unwrap(),
                    ))
                    .required_unless_present("json-body"),
            )
            .arg(
                ::clap::Arg::new("recipient")
                    .long("recipient")
                    .value_parser(::clap::value_parser!(::std::string::String))
                    .required_unless_present("json-body"),
            )
            .arg(
                ::clap::Arg::new("redirect-uri")
                    .long("redirect-uri")
                    .value_parser(::clap::value_parser!(::std::string::String))
                    .required_unless_present("json-body"),
            )
            .arg(
                ::clap::Arg::new("scope")
                    .long("scope")
                    .value_parser(::clap::value_parser!(::std::string::String))
                    .required(false),
            )
            .arg(
                ::clap::Arg::new("secret")
                    .long("secret")
                    .value_parser(::clap::value_parser!(::std::string::String))
                    .required_unless_present("json-body"),
            )
            .arg(
                ::clap::Arg::new("json-body")
                    .long("json-body")
                    .value_name("JSON-FILE")
                    .required(false)
                    .value_parser(::clap::value_parser!(std::path::PathBuf))
                    .help("Path to a file that contains the full json body."),
            )
            .arg(
                ::clap::Arg::new("json-body-template")
                    .long("json-body-template")
                    .action(::clap::ArgAction::SetTrue)
                    .help("XXX"),
            )
            .about("Send a new magic link authentication link")
    }

    pub fn cli_authz_code_redirect() -> ::clap::Command {
        ::clap::Command::new("")
            .arg(
                ::clap::Arg::new("client-id")
                    .long("client-id")
                    .value_parser(::clap::value_parser!(types::TypedUuidForOAuthClientId))
                    .required(true),
            )
            .arg(
                ::clap::Arg::new("provider")
                    .long("provider")
                    .value_parser(::clap::builder::TypedValueParser::map(
                        ::clap::builder::PossibleValuesParser::new([
                            types::OAuthProviderName::Github.to_string(),
                            types::OAuthProviderName::Google.to_string(),
                        ]),
                        |s| types::OAuthProviderName::try_from(s).unwrap(),
                    ))
                    .required(true),
            )
            .arg(
                ::clap::Arg::new("redirect-uri")
                    .long("redirect-uri")
                    .value_parser(::clap::value_parser!(::std::string::String))
                    .required(true),
            )
            .arg(
                ::clap::Arg::new("response-type")
                    .long("response-type")
                    .value_parser(::clap::value_parser!(::std::string::String))
                    .required(true),
            )
            .arg(
                ::clap::Arg::new("scope")
                    .long("scope")
                    .value_parser(::clap::value_parser!(::std::string::String))
                    .required(false),
            )
            .arg(
                ::clap::Arg::new("state")
                    .long("state")
                    .value_parser(::clap::value_parser!(::std::string::String))
                    .required(true),
            )
            .about("Generate the remote provider login url and redirect the user")
    }

    pub fn cli_authz_code_callback() -> ::clap::Command {
        ::clap::Command::new("")
            .arg(
                ::clap::Arg::new("code")
                    .long("code")
                    .value_parser(::clap::value_parser!(::std::string::String))
                    .required(false),
            )
            .arg(
                ::clap::Arg::new("error")
                    .long("error")
                    .value_parser(::clap::value_parser!(::std::string::String))
                    .required(false),
            )
            .arg(
                ::clap::Arg::new("provider")
                    .long("provider")
                    .value_parser(::clap::builder::TypedValueParser::map(
                        ::clap::builder::PossibleValuesParser::new([
                            types::OAuthProviderName::Github.to_string(),
                            types::OAuthProviderName::Google.to_string(),
                        ]),
                        |s| types::OAuthProviderName::try_from(s).unwrap(),
                    ))
                    .required(true),
            )
            .arg(
                ::clap::Arg::new("state")
                    .long("state")
                    .value_parser(::clap::value_parser!(::std::string::String))
                    .required(false),
            )
            .about("Handle return calls from a remote OAuth provider")
    }

    pub fn cli_authz_code_exchange() -> ::clap::Command {
        ::clap::Command::new("")
            .arg(
                ::clap::Arg::new("client-id")
                    .long("client-id")
                    .value_parser(::clap::value_parser!(types::TypedUuidForOAuthClientId))
                    .required(false),
            )
            .arg(
                ::clap::Arg::new("client-secret")
                    .long("client-secret")
                    .value_parser(::clap::value_parser!(types::SecretString))
                    .required(false),
            )
            .arg(
                ::clap::Arg::new("code")
                    .long("code")
                    .value_parser(::clap::value_parser!(::std::string::String))
                    .required_unless_present("json-body"),
            )
            .arg(
                ::clap::Arg::new("grant-type")
                    .long("grant-type")
                    .value_parser(::clap::value_parser!(::std::string::String))
                    .required_unless_present("json-body"),
            )
            .arg(
                ::clap::Arg::new("pkce-verifier")
                    .long("pkce-verifier")
                    .value_parser(::clap::value_parser!(::std::string::String))
                    .required(false),
            )
            .arg(
                ::clap::Arg::new("provider")
                    .long("provider")
                    .value_parser(::clap::builder::TypedValueParser::map(
                        ::clap::builder::PossibleValuesParser::new([
                            types::OAuthProviderName::Github.to_string(),
                            types::OAuthProviderName::Google.to_string(),
                        ]),
                        |s| types::OAuthProviderName::try_from(s).unwrap(),
                    ))
                    .required(true),
            )
            .arg(
                ::clap::Arg::new("redirect-uri")
                    .long("redirect-uri")
                    .value_parser(::clap::value_parser!(::std::string::String))
                    .required_unless_present("json-body"),
            )
            .arg(
                ::clap::Arg::new("json-body")
                    .long("json-body")
                    .value_name("JSON-FILE")
                    .required(false)
                    .value_parser(::clap::value_parser!(std::path::PathBuf))
                    .help("Path to a file that contains the full json body."),
            )
            .arg(
                ::clap::Arg::new("json-body-template")
                    .long("json-body-template")
                    .action(::clap::ArgAction::SetTrue)
                    .help("XXX"),
            )
            .about("Exchange an authorization code for an access token")
    }

    pub fn cli_get_device_provider() -> ::clap::Command {
        ::clap::Command::new("")
            .arg(
                ::clap::Arg::new("provider")
                    .long("provider")
                    .value_parser(::clap::builder::TypedValueParser::map(
                        ::clap::builder::PossibleValuesParser::new([
                            types::OAuthProviderName::Github.to_string(),
                            types::OAuthProviderName::Google.to_string(),
                        ]),
                        |s| types::OAuthProviderName::try_from(s).unwrap(),
                    ))
                    .required(true),
            )
            .about("Retrieve the metadata about an OAuth provider")
    }

    pub fn cli_exchange_device_token() -> ::clap::Command {
        ::clap::Command::new("")
            .arg(
                ::clap::Arg::new("device-code")
                    .long("device-code")
                    .value_parser(::clap::value_parser!(::std::string::String))
                    .required_unless_present("json-body"),
            )
            .arg(
                ::clap::Arg::new("expires-at")
                    .long("expires-at")
                    .value_parser(::clap::value_parser!(
                        ::chrono::DateTime<::chrono::offset::Utc>
                    ))
                    .required(false),
            )
            .arg(
                ::clap::Arg::new("grant-type")
                    .long("grant-type")
                    .value_parser(::clap::value_parser!(::std::string::String))
                    .required_unless_present("json-body"),
            )
            .arg(
                ::clap::Arg::new("provider")
                    .long("provider")
                    .value_parser(::clap::builder::TypedValueParser::map(
                        ::clap::builder::PossibleValuesParser::new([
                            types::OAuthProviderName::Github.to_string(),
                            types::OAuthProviderName::Google.to_string(),
                        ]),
                        |s| types::OAuthProviderName::try_from(s).unwrap(),
                    ))
                    .required(true),
            )
            .arg(
                ::clap::Arg::new("json-body")
                    .long("json-body")
                    .value_name("JSON-FILE")
                    .required(false)
                    .value_parser(::clap::value_parser!(std::path::PathBuf))
                    .help("Path to a file that contains the full json body."),
            )
            .arg(
                ::clap::Arg::new("json-body-template")
                    .long("json-body-template")
                    .action(::clap::ArgAction::SetTrue)
                    .help("XXX"),
            )
            .about("Exchange an OAuth device code request for an access token")
    }

    pub fn cli_list_magic_links() -> ::clap::Command {
        ::clap::Command::new("").about("List Magic Link clients")
    }

    pub fn cli_create_magic_link() -> ::clap::Command {
        ::clap::Command::new("").about("Create a new Magic Link Client")
    }

    pub fn cli_get_magic_link() -> ::clap::Command {
        ::clap::Command::new("")
            .arg(
                ::clap::Arg::new("client-id")
                    .long("client-id")
                    .value_parser(::clap::value_parser!(types::TypedUuidForMagicLinkId))
                    .required(true),
            )
            .about("Get a Magic Link Client")
    }

    pub fn cli_create_magic_link_redirect_uri() -> ::clap::Command {
        ::clap::Command::new("")
            .arg(
                ::clap::Arg::new("client-id")
                    .long("client-id")
                    .value_parser(::clap::value_parser!(types::TypedUuidForMagicLinkId))
                    .required(true),
            )
            .arg(
                ::clap::Arg::new("redirect-uri")
                    .long("redirect-uri")
                    .value_parser(::clap::value_parser!(::std::string::String))
                    .required_unless_present("json-body"),
            )
            .arg(
                ::clap::Arg::new("json-body")
                    .long("json-body")
                    .value_name("JSON-FILE")
                    .required(false)
                    .value_parser(::clap::value_parser!(std::path::PathBuf))
                    .help("Path to a file that contains the full json body."),
            )
            .arg(
                ::clap::Arg::new("json-body-template")
                    .long("json-body-template")
                    .action(::clap::ArgAction::SetTrue)
                    .help("XXX"),
            )
            .about("Add a Magic Link client redirect uri")
    }

    pub fn cli_delete_magic_link_redirect_uri() -> ::clap::Command {
        ::clap::Command::new("")
            .arg(
                ::clap::Arg::new("client-id")
                    .long("client-id")
                    .value_parser(::clap::value_parser!(types::TypedUuidForMagicLinkId))
                    .required(true),
            )
            .arg(
                ::clap::Arg::new("redirect-uri-id")
                    .long("redirect-uri-id")
                    .value_parser(::clap::value_parser!(
                        types::TypedUuidForMagicLinkRedirectUriId
                    ))
                    .required(true),
            )
            .about("Delete a Magic Link client redirect uri")
    }

    pub fn cli_create_magic_link_secret() -> ::clap::Command {
        ::clap::Command::new("")
            .arg(
                ::clap::Arg::new("client-id")
                    .long("client-id")
                    .value_parser(::clap::value_parser!(types::TypedUuidForMagicLinkId))
                    .required(true),
            )
            .about("Add a Magic Link client secret")
    }

    pub fn cli_delete_magic_link_secret() -> ::clap::Command {
        ::clap::Command::new("")
            .arg(
                ::clap::Arg::new("client-id")
                    .long("client-id")
                    .value_parser(::clap::value_parser!(types::TypedUuidForMagicLinkId))
                    .required(true),
            )
            .arg(
                ::clap::Arg::new("secret-id")
                    .long("secret-id")
                    .value_parser(::clap::value_parser!(types::TypedUuidForMagicLinkSecretId))
                    .required(true),
            )
            .about("Delete a Magic Link client secret")
    }

    pub fn cli_get_mappers() -> ::clap::Command {
        ::clap::Command::new("")
            .arg(
                ::clap::Arg::new("include-depleted")
                    .long("include-depleted")
                    .value_parser(::clap::value_parser!(bool))
                    .required(false)
                    .help("Include depleted mappers in the returned results"),
            )
            .about("List all mappers")
    }

    pub fn cli_create_mapper() -> ::clap::Command {
        ::clap::Command::new("")
            .arg(
                ::clap::Arg::new("max-activations")
                    .long("max-activations")
                    .value_parser(::clap::value_parser!(i32))
                    .required(false),
            )
            .arg(
                ::clap::Arg::new("name")
                    .long("name")
                    .value_parser(::clap::value_parser!(::std::string::String))
                    .required_unless_present("json-body"),
            )
            .arg(
                ::clap::Arg::new("json-body")
                    .long("json-body")
                    .value_name("JSON-FILE")
                    .required(true)
                    .value_parser(::clap::value_parser!(std::path::PathBuf))
                    .help("Path to a file that contains the full json body."),
            )
            .arg(
                ::clap::Arg::new("json-body-template")
                    .long("json-body-template")
                    .action(::clap::ArgAction::SetTrue)
                    .help("XXX"),
            )
            .about("Create a mapper")
    }

    pub fn cli_delete_mapper() -> ::clap::Command {
        ::clap::Command::new("")
            .arg(
                ::clap::Arg::new("mapper-id")
                    .long("mapper-id")
                    .value_parser(::clap::value_parser!(types::TypedUuidForMapperId))
                    .required(true),
            )
            .about("Delete a mapper")
    }

    pub fn cli_list_oauth_clients() -> ::clap::Command {
        ::clap::Command::new("").about("List OAuth clients")
    }

    pub fn cli_create_oauth_client() -> ::clap::Command {
        ::clap::Command::new("").about("Create a new OAuth Client")
    }

    pub fn cli_get_oauth_client() -> ::clap::Command {
        ::clap::Command::new("")
            .arg(
                ::clap::Arg::new("client-id")
                    .long("client-id")
                    .value_parser(::clap::value_parser!(types::TypedUuidForOAuthClientId))
                    .required(true),
            )
            .about("Get an new OAuth Client")
    }

    pub fn cli_create_oauth_client_redirect_uri() -> ::clap::Command {
        ::clap::Command::new("")
            .arg(
                ::clap::Arg::new("client-id")
                    .long("client-id")
                    .value_parser(::clap::value_parser!(types::TypedUuidForOAuthClientId))
                    .required(true),
            )
            .arg(
                ::clap::Arg::new("redirect-uri")
                    .long("redirect-uri")
                    .value_parser(::clap::value_parser!(::std::string::String))
                    .required_unless_present("json-body"),
            )
            .arg(
                ::clap::Arg::new("json-body")
                    .long("json-body")
                    .value_name("JSON-FILE")
                    .required(false)
                    .value_parser(::clap::value_parser!(std::path::PathBuf))
                    .help("Path to a file that contains the full json body."),
            )
            .arg(
                ::clap::Arg::new("json-body-template")
                    .long("json-body-template")
                    .action(::clap::ArgAction::SetTrue)
                    .help("XXX"),
            )
            .about("Add an OAuth client redirect uri")
    }

    pub fn cli_delete_oauth_client_redirect_uri() -> ::clap::Command {
        ::clap::Command::new("")
            .arg(
                ::clap::Arg::new("client-id")
                    .long("client-id")
                    .value_parser(::clap::value_parser!(types::TypedUuidForOAuthClientId))
                    .required(true),
            )
            .arg(
                ::clap::Arg::new("redirect-uri-id")
                    .long("redirect-uri-id")
                    .value_parser(::clap::value_parser!(types::TypedUuidForOAuthRedirectUriId))
                    .required(true),
            )
            .about("Delete an OAuth client redirect uri")
    }

    pub fn cli_create_oauth_client_secret() -> ::clap::Command {
        ::clap::Command::new("")
            .arg(
                ::clap::Arg::new("client-id")
                    .long("client-id")
                    .value_parser(::clap::value_parser!(types::TypedUuidForOAuthClientId))
                    .required(true),
            )
            .about("Add an OAuth client secret")
    }

    pub fn cli_delete_oauth_client_secret() -> ::clap::Command {
        ::clap::Command::new("")
            .arg(
                ::clap::Arg::new("client-id")
                    .long("client-id")
                    .value_parser(::clap::value_parser!(types::TypedUuidForOAuthClientId))
                    .required(true),
            )
            .arg(
                ::clap::Arg::new("secret-id")
                    .long("secret-id")
                    .value_parser(::clap::value_parser!(types::TypedUuidForOAuthSecretId))
                    .required(true),
            )
            .about("Delete an OAuth client secret")
    }

    pub fn cli_list_rfds() -> ::clap::Command {
        ::clap::Command::new("").about("List all available RFDs")
    }

    pub fn cli_reserve_rfd() -> ::clap::Command {
        ::clap::Command::new("")
            .arg(
                ::clap::Arg::new("content")
                    .long("content")
                    .value_parser(::clap::value_parser!(::std::string::String))
                    .required(false)
                    .help("Optional contents of the RFD"),
            )
            .arg(
                ::clap::Arg::new("title")
                    .long("title")
                    .value_parser(::clap::value_parser!(::std::string::String))
                    .required_unless_present("json-body")
                    .help("Title of the RFD"),
            )
            .arg(
                ::clap::Arg::new("json-body")
                    .long("json-body")
                    .value_name("JSON-FILE")
                    .required(false)
                    .value_parser(::clap::value_parser!(std::path::PathBuf))
                    .help("Path to a file that contains the full json body."),
            )
            .arg(
                ::clap::Arg::new("json-body-template")
                    .long("json-body-template")
                    .action(::clap::ArgAction::SetTrue)
                    .help("XXX"),
            )
            .about("Create a new RFD")
    }

    pub fn cli_view_rfd_meta() -> ::clap::Command {
        ::clap::Command::new("")
            .arg(
                ::clap::Arg::new("number")
                    .long("number")
                    .value_parser(::clap::value_parser!(::std::string::String))
                    .required(true)
                    .help("The RFD number (examples: 1 or 123)"),
            )
            .about("Get the latest representation of an RFD's metadata")
    }

    pub fn cli_view_rfd_attr() -> ::clap::Command {
        ::clap::Command::new("")
            .arg(
                ::clap::Arg::new("attr")
                    .long("attr")
                    .value_parser(::clap::builder::TypedValueParser::map(
                        ::clap::builder::PossibleValuesParser::new([
                            types::RfdAttrName::Discussion.to_string(),
                            types::RfdAttrName::Labels.to_string(),
                            types::RfdAttrName::State.to_string(),
                        ]),
                        |s| types::RfdAttrName::try_from(s).unwrap(),
                    ))
                    .required(true)
                    .help("An attribute that can be defined in an RFD document"),
            )
            .arg(
                ::clap::Arg::new("number")
                    .long("number")
                    .value_parser(::clap::value_parser!(::std::string::String))
                    .required(true)
                    .help("The RFD number (examples: 1 or 123)"),
            )
            .about("Get the an attribute of the latest revision of a RFD")
    }

    pub fn cli_set_rfd_attr() -> ::clap::Command {
        ::clap::Command::new("")
            .arg(
                ::clap::Arg::new("attr")
                    .long("attr")
                    .value_parser(::clap::builder::TypedValueParser::map(
                        ::clap::builder::PossibleValuesParser::new([
                            types::RfdAttrName::Discussion.to_string(),
                            types::RfdAttrName::Labels.to_string(),
                            types::RfdAttrName::State.to_string(),
                        ]),
                        |s| types::RfdAttrName::try_from(s).unwrap(),
                    ))
                    .required(true)
                    .help("An attribute that can be defined in an RFD document"),
            )
            .arg(
                ::clap::Arg::new("message")
                    .long("message")
                    .value_parser(::clap::value_parser!(::std::string::String))
                    .required(false)
                    .help("Optional Git commit message to send with this update (recommended)"),
            )
            .arg(
                ::clap::Arg::new("number")
                    .long("number")
                    .value_parser(::clap::value_parser!(::std::string::String))
                    .required(true)
                    .help("The RFD number (examples: 1 or 123)"),
            )
            .arg(
                ::clap::Arg::new("value")
                    .long("value")
                    .value_parser(::clap::value_parser!(::std::string::String))
                    .required_unless_present("json-body")
                    .help("Full value to set this attribute to in the existing RFD contents"),
            )
            .arg(
                ::clap::Arg::new("json-body")
                    .long("json-body")
                    .value_name("JSON-FILE")
                    .required(false)
                    .value_parser(::clap::value_parser!(std::path::PathBuf))
                    .help("Path to a file that contains the full json body."),
            )
            .arg(
                ::clap::Arg::new("json-body-template")
                    .long("json-body-template")
                    .action(::clap::ArgAction::SetTrue)
                    .help("XXX"),
            )
            .about("Set an attribute of a RFD")
    }

    pub fn cli_set_rfd_content() -> ::clap::Command {
        ::clap::Command::new("")
            .arg(
                ::clap::Arg::new("content")
                    .long("content")
                    .value_parser(::clap::value_parser!(::std::string::String))
                    .required_unless_present("json-body")
                    .help("Asciidoc content to store for this RFD"),
            )
            .arg(
                ::clap::Arg::new("message")
                    .long("message")
                    .value_parser(::clap::value_parser!(::std::string::String))
                    .required(false)
                    .help("Optional Git commit message to send with this update (recommended)"),
            )
            .arg(
                ::clap::Arg::new("number")
                    .long("number")
                    .value_parser(::clap::value_parser!(::std::string::String))
                    .required(true)
                    .help("The RFD number (examples: 1 or 123)"),
            )
            .arg(
                ::clap::Arg::new("json-body")
                    .long("json-body")
                    .value_name("JSON-FILE")
                    .required(false)
                    .value_parser(::clap::value_parser!(std::path::PathBuf))
                    .help("Path to a file that contains the full json body."),
            )
            .arg(
                ::clap::Arg::new("json-body-template")
                    .long("json-body-template")
                    .action(::clap::ArgAction::SetTrue)
                    .help("XXX"),
            )
            .about("Replace the contents of a RFD")
    }

    pub fn cli_view_rfd_discussion() -> ::clap::Command {
        ::clap::Command::new("")
            .arg(
                ::clap::Arg::new("number")
                    .long("number")
                    .value_parser(::clap::value_parser!(::std::string::String))
                    .required(true)
                    .help("The RFD number (examples: 1 or 123)"),
            )
            .about("Get the comments related to the latest revision of a RFD")
    }

    pub fn cli_view_rfd_pdf() -> ::clap::Command {
        ::clap::Command::new("")
            .arg(
                ::clap::Arg::new("number")
                    .long("number")
                    .value_parser(::clap::value_parser!(::std::string::String))
                    .required(true)
                    .help("The RFD number (examples: 1 or 123)"),
            )
            .about("Get the PDF locations of the latest revision of a RFD")
    }

    pub fn cli_view_rfd() -> ::clap::Command {
        ::clap::Command::new("")
            .arg(
                ::clap::Arg::new("number")
                    .long("number")
                    .value_parser(::clap::value_parser!(::std::string::String))
                    .required(true)
                    .help("The RFD number (examples: 1 or 123)"),
            )
            .about("Get the raw contents of the latest revision of a RFD")
    }

    pub fn cli_set_rfd_document() -> ::clap::Command {
        ::clap::Command::new("")
            .arg(
                ::clap::Arg::new("document")
                    .long("document")
                    .value_parser(::clap::value_parser!(::std::string::String))
                    .required_unless_present("json-body")
                    .help("Full Asciidoc document to store for this RFD"),
            )
            .arg(
                ::clap::Arg::new("message")
                    .long("message")
                    .value_parser(::clap::value_parser!(::std::string::String))
                    .required(false)
                    .help("Optional Git commit message to send with this update (recommended)"),
            )
            .arg(
                ::clap::Arg::new("number")
                    .long("number")
                    .value_parser(::clap::value_parser!(::std::string::String))
                    .required(true)
                    .help("The RFD number (examples: 1 or 123)"),
            )
            .arg(
                ::clap::Arg::new("json-body")
                    .long("json-body")
                    .value_name("JSON-FILE")
                    .required(false)
                    .value_parser(::clap::value_parser!(std::path::PathBuf))
                    .help("Path to a file that contains the full json body."),
            )
            .arg(
                ::clap::Arg::new("json-body-template")
                    .long("json-body-template")
                    .action(::clap::ArgAction::SetTrue)
                    .help("XXX"),
            )
            .about("Replace the full document of a RFD")
    }

    pub fn cli_list_rfd_revisions() -> ::clap::Command {
        ::clap::Command::new("")
            .arg(
                ::clap::Arg::new("limit")
                    .long("limit")
                    .value_parser(::clap::value_parser!(i64))
                    .required(false),
            )
            .arg(
                ::clap::Arg::new("number")
                    .long("number")
                    .value_parser(::clap::value_parser!(::std::string::String))
                    .required(true)
                    .help("The RFD number (examples: 1 or 123)"),
            )
            .arg(
                ::clap::Arg::new("offset")
                    .long("offset")
                    .value_parser(::clap::value_parser!(i64))
                    .required(false),
            )
            .about("List all revisions of an RFD")
    }

    pub fn cli_view_rfd_revision_meta() -> ::clap::Command {
        ::clap::Command::new("")
            .arg(
                ::clap::Arg::new("number")
                    .long("number")
                    .value_parser(::clap::value_parser!(::std::string::String))
                    .required(true)
                    .help("The RFD number (examples: 1 or 123)"),
            )
            .arg(
                ::clap::Arg::new("revision")
                    .long("revision")
                    .value_parser(::clap::value_parser!(types::TypedUuidForRfdRevisionId))
                    .required(true)
                    .help("The revision id of the RFD"),
            )
            .about("Get an RFD revision's metadata")
    }

    pub fn cli_update_rfd_revision() -> ::clap::Command {
        ::clap::Command::new("")
            .arg(
                ::clap::Arg::new("major-change")
                    .long("major-change")
                    .value_parser(::clap::value_parser!(bool))
                    .required(false),
            )
            .arg(
                ::clap::Arg::new("number")
                    .long("number")
                    .value_parser(::clap::value_parser!(::std::string::String))
                    .required(true)
                    .help("The RFD number (examples: 1 or 123)"),
            )
            .arg(
                ::clap::Arg::new("revision")
                    .long("revision")
                    .value_parser(::clap::value_parser!(types::TypedUuidForRfdRevisionId))
                    .required(true)
                    .help("The revision id of the RFD"),
            )
            .arg(
                ::clap::Arg::new("json-body")
                    .long("json-body")
                    .value_name("JSON-FILE")
                    .required(false)
                    .value_parser(::clap::value_parser!(std::path::PathBuf))
                    .help("Path to a file that contains the full json body."),
            )
            .arg(
                ::clap::Arg::new("json-body-template")
                    .long("json-body-template")
                    .action(::clap::ArgAction::SetTrue)
                    .help("XXX"),
            )
            .about("Update the metadata of an RFD's revision")
    }

    pub fn cli_view_rfd_revision_attr() -> ::clap::Command {
        ::clap::Command::new("")
            .arg(
                ::clap::Arg::new("attr")
                    .long("attr")
                    .value_parser(::clap::builder::TypedValueParser::map(
                        ::clap::builder::PossibleValuesParser::new([
                            types::RfdAttrName::Discussion.to_string(),
                            types::RfdAttrName::Labels.to_string(),
                            types::RfdAttrName::State.to_string(),
                        ]),
                        |s| types::RfdAttrName::try_from(s).unwrap(),
                    ))
                    .required(true)
                    .help("An attribute that can be defined in an RFD document"),
            )
            .arg(
                ::clap::Arg::new("number")
                    .long("number")
                    .value_parser(::clap::value_parser!(::std::string::String))
                    .required(true)
                    .help("The RFD number (examples: 1 or 123)"),
            )
            .arg(
                ::clap::Arg::new("revision")
                    .long("revision")
                    .value_parser(::clap::value_parser!(types::TypedUuidForRfdRevisionId))
                    .required(true)
                    .help("The revision id of the RFD"),
            )
            .about("Get the an attribute of a revision of a RFD")
    }

    pub fn cli_view_rfd_revision_discussion() -> ::clap::Command {
        ::clap::Command::new("")
            .arg(
                ::clap::Arg::new("number")
                    .long("number")
                    .value_parser(::clap::value_parser!(::std::string::String))
                    .required(true)
                    .help("The RFD number (examples: 1 or 123)"),
            )
            .arg(
                ::clap::Arg::new("revision")
                    .long("revision")
                    .value_parser(::clap::value_parser!(types::TypedUuidForRfdRevisionId))
                    .required(true)
                    .help("The revision id of the RFD"),
            )
            .about("Get the comments related to a revision of a RFD")
    }

    pub fn cli_view_rfd_revision_pdf() -> ::clap::Command {
        ::clap::Command::new("")
            .arg(
                ::clap::Arg::new("number")
                    .long("number")
                    .value_parser(::clap::value_parser!(::std::string::String))
                    .required(true)
                    .help("The RFD number (examples: 1 or 123)"),
            )
            .arg(
                ::clap::Arg::new("revision")
                    .long("revision")
                    .value_parser(::clap::value_parser!(types::TypedUuidForRfdRevisionId))
                    .required(true)
                    .help("The revision id of the RFD"),
            )
            .about("Get the PDF locations of a revision of a RFD")
    }

    pub fn cli_view_rfd_revision() -> ::clap::Command {
        ::clap::Command::new("")
            .arg(
                ::clap::Arg::new("number")
                    .long("number")
                    .value_parser(::clap::value_parser!(::std::string::String))
                    .required(true)
                    .help("The RFD number (examples: 1 or 123)"),
            )
            .arg(
                ::clap::Arg::new("revision")
                    .long("revision")
                    .value_parser(::clap::value_parser!(types::TypedUuidForRfdRevisionId))
                    .required(true)
                    .help("The revision id of the RFD"),
            )
            .about("Get the raw contents of a revision of a RFD")
    }

    pub fn cli_discuss_rfd() -> ::clap::Command {
        ::clap::Command::new("")
            .arg(
                ::clap::Arg::new("number")
                    .long("number")
                    .value_parser(::clap::value_parser!(::std::string::String))
                    .required(true)
                    .help("The RFD number (examples: 1 or 123)"),
            )
            .about("Open a RFD for discussion")
    }

    pub fn cli_publish_rfd() -> ::clap::Command {
        ::clap::Command::new("")
            .arg(
                ::clap::Arg::new("number")
                    .long("number")
                    .value_parser(::clap::value_parser!(::std::string::String))
                    .required(true)
                    .help("The RFD number (examples: 1 or 123)"),
            )
            .about("Publish a RFD")
    }

    pub fn cli_update_rfd_visibility() -> ::clap::Command {
        ::clap::Command::new("")
            .arg(
                ::clap::Arg::new("number")
                    .long("number")
                    .value_parser(::clap::value_parser!(::std::string::String))
                    .required(true)
                    .help("The RFD number (examples: 1 or 123)"),
            )
            .arg(
                ::clap::Arg::new("visibility")
                    .long("visibility")
                    .value_parser(::clap::builder::TypedValueParser::map(
                        ::clap::builder::PossibleValuesParser::new([
                            types::Visibility::Public.to_string(),
                            types::Visibility::Private.to_string(),
                        ]),
                        |s| types::Visibility::try_from(s).unwrap(),
                    ))
                    .required_unless_present("json-body"),
            )
            .arg(
                ::clap::Arg::new("json-body")
                    .long("json-body")
                    .value_name("JSON-FILE")
                    .required(false)
                    .value_parser(::clap::value_parser!(std::path::PathBuf))
                    .help("Path to a file that contains the full json body."),
            )
            .arg(
                ::clap::Arg::new("json-body-template")
                    .long("json-body-template")
                    .action(::clap::ArgAction::SetTrue)
                    .help("XXX"),
            )
            .about("Modify the visibility of a RFD")
    }

    pub fn cli_search_rfds() -> ::clap::Command {
        ::clap::Command::new("")
            .arg(
                ::clap::Arg::new("attributes-to-crop")
                    .long("attributes-to-crop")
                    .value_parser(::clap::value_parser!(::std::string::String))
                    .required(false),
            )
            .arg(
                ::clap::Arg::new("highlight-post-tag")
                    .long("highlight-post-tag")
                    .value_parser(::clap::value_parser!(::std::string::String))
                    .required(false),
            )
            .arg(
                ::clap::Arg::new("highlight-pre-tag")
                    .long("highlight-pre-tag")
                    .value_parser(::clap::value_parser!(::std::string::String))
                    .required(false),
            )
            .arg(
                ::clap::Arg::new("limit")
                    .long("limit")
                    .value_parser(::clap::value_parser!(u32))
                    .required(false),
            )
            .arg(
                ::clap::Arg::new("offset")
                    .long("offset")
                    .value_parser(::clap::value_parser!(u32))
                    .required(false),
            )
            .arg(
                ::clap::Arg::new("q")
                    .long("q")
                    .value_parser(::clap::value_parser!(::std::string::String))
                    .required(true),
            )
            .about("Search the RFD index and get a list of results")
    }

    pub fn cli_get_self() -> ::clap::Command {
        ::clap::Command::new("").about("View details for the calling user")
    }

    pub async fn execute(
        &self,
        cmd: CliCommand,
        matches: &::clap::ArgMatches,
    ) -> anyhow::Result<()> {
        match cmd {
            CliCommand::JwksJson => self.execute_jwks_json(matches).await,
            CliCommand::OpenidConfiguration => self.execute_openid_configuration(matches).await,
            CliCommand::ListApiUsers => self.execute_list_api_users(matches).await,
            CliCommand::CreateApiUser => self.execute_create_api_user(matches).await,
            CliCommand::GetApiUser => self.execute_get_api_user(matches).await,
            CliCommand::UpdateApiUser => self.execute_update_api_user(matches).await,
            CliCommand::SetApiUserContactEmail => {
                self.execute_set_api_user_contact_email(matches).await
            }
            CliCommand::AddApiUserToGroup => self.execute_add_api_user_to_group(matches).await,
            CliCommand::RemoveApiUserFromGroup => {
                self.execute_remove_api_user_from_group(matches).await
            }
            CliCommand::LinkProvider => self.execute_link_provider(matches).await,
            CliCommand::ListApiUserTokens => self.execute_list_api_user_tokens(matches).await,
            CliCommand::CreateApiUserToken => self.execute_create_api_user_token(matches).await,
            CliCommand::GetApiUserToken => self.execute_get_api_user_token(matches).await,
            CliCommand::DeleteApiUserToken => self.execute_delete_api_user_token(matches).await,
            CliCommand::CreateLinkToken => self.execute_create_link_token(matches).await,
            CliCommand::GithubWebhook => self.execute_github_webhook(matches).await,
            CliCommand::GetGroups => self.execute_get_groups(matches).await,
            CliCommand::CreateGroup => self.execute_create_group(matches).await,
            CliCommand::UpdateGroup => self.execute_update_group(matches).await,
            CliCommand::DeleteGroup => self.execute_delete_group(matches).await,
            CliCommand::GetGroupMembers => self.execute_get_group_members(matches).await,
            CliCommand::ListJobs => self.execute_list_jobs(matches).await,
            CliCommand::MagicLinkExchange => self.execute_magic_link_exchange(matches).await,
            CliCommand::MagicLinkSend => self.execute_magic_link_send(matches).await,
            CliCommand::AuthzCodeRedirect => self.execute_authz_code_redirect(matches).await,
            CliCommand::AuthzCodeCallback => self.execute_authz_code_callback(matches).await,
            CliCommand::AuthzCodeExchange => self.execute_authz_code_exchange(matches).await,
            CliCommand::GetDeviceProvider => self.execute_get_device_provider(matches).await,
            CliCommand::ExchangeDeviceToken => self.execute_exchange_device_token(matches).await,
            CliCommand::ListMagicLinks => self.execute_list_magic_links(matches).await,
            CliCommand::CreateMagicLink => self.execute_create_magic_link(matches).await,
            CliCommand::GetMagicLink => self.execute_get_magic_link(matches).await,
            CliCommand::CreateMagicLinkRedirectUri => {
                self.execute_create_magic_link_redirect_uri(matches).await
            }
            CliCommand::DeleteMagicLinkRedirectUri => {
                self.execute_delete_magic_link_redirect_uri(matches).await
            }
            CliCommand::CreateMagicLinkSecret => {
                self.execute_create_magic_link_secret(matches).await
            }
            CliCommand::DeleteMagicLinkSecret => {
                self.execute_delete_magic_link_secret(matches).await
            }
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
            CliCommand::ListRfds => self.execute_list_rfds(matches).await,
            CliCommand::ReserveRfd => self.execute_reserve_rfd(matches).await,
            CliCommand::ViewRfdMeta => self.execute_view_rfd_meta(matches).await,
            CliCommand::ViewRfdAttr => self.execute_view_rfd_attr(matches).await,
            CliCommand::SetRfdAttr => self.execute_set_rfd_attr(matches).await,
            CliCommand::SetRfdContent => self.execute_set_rfd_content(matches).await,
            CliCommand::ViewRfdDiscussion => self.execute_view_rfd_discussion(matches).await,
            CliCommand::ViewRfdPdf => self.execute_view_rfd_pdf(matches).await,
            CliCommand::ViewRfd => self.execute_view_rfd(matches).await,
            CliCommand::SetRfdDocument => self.execute_set_rfd_document(matches).await,
            CliCommand::ListRfdRevisions => self.execute_list_rfd_revisions(matches).await,
            CliCommand::ViewRfdRevisionMeta => self.execute_view_rfd_revision_meta(matches).await,
            CliCommand::UpdateRfdRevision => self.execute_update_rfd_revision(matches).await,
            CliCommand::ViewRfdRevisionAttr => self.execute_view_rfd_revision_attr(matches).await,
            CliCommand::ViewRfdRevisionDiscussion => {
                self.execute_view_rfd_revision_discussion(matches).await
            }
            CliCommand::ViewRfdRevisionPdf => self.execute_view_rfd_revision_pdf(matches).await,
            CliCommand::ViewRfdRevision => self.execute_view_rfd_revision(matches).await,
            CliCommand::DiscussRfd => self.execute_discuss_rfd(matches).await,
            CliCommand::PublishRfd => self.execute_publish_rfd(matches).await,
            CliCommand::UpdateRfdVisibility => self.execute_update_rfd_visibility(matches).await,
            CliCommand::SearchRfds => self.execute_search_rfds(matches).await,
            CliCommand::GetSelf => self.execute_get_self(matches).await,
        }
    }

    pub async fn execute_jwks_json(&self, matches: &::clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.jwks_json();
        self.config.execute_jwks_json(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_openid_configuration(
        &self,
        matches: &::clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.openid_configuration();
        self.config
            .execute_openid_configuration(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_list_api_users(&self, matches: &::clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.list_api_users();
        self.config.execute_list_api_users(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_create_api_user(
        &self,
        matches: &::clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.create_api_user();
        if let Some(value) = matches.get_one::<std::path::PathBuf>("json-body") {
            let body_txt = std::fs::read_to_string(value).unwrap();
            let body_value =
                serde_json::from_str::<types::ApiUserUpdateParamsForRfdPermission>(&body_txt)
                    .unwrap();
            request = request.body(body_value);
        }

        self.config.execute_create_api_user(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_get_api_user(&self, matches: &::clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.get_api_user();
        if let Some(value) = matches.get_one::<types::TypedUuidForUserId>("user-id") {
            request = request.user_id(value.clone());
        }

        self.config.execute_get_api_user(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_update_api_user(
        &self,
        matches: &::clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.update_api_user();
        if let Some(value) = matches.get_one::<types::TypedUuidForUserId>("user-id") {
            request = request.user_id(value.clone());
        }

        if let Some(value) = matches.get_one::<std::path::PathBuf>("json-body") {
            let body_txt = std::fs::read_to_string(value).unwrap();
            let body_value =
                serde_json::from_str::<types::ApiUserUpdateParamsForRfdPermission>(&body_txt)
                    .unwrap();
            request = request.body(body_value);
        }

        self.config.execute_update_api_user(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_set_api_user_contact_email(
        &self,
        matches: &::clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.set_api_user_contact_email();
        if let Some(value) = matches.get_one::<::std::string::String>("email") {
            request = request.body_map(|body| body.email(value.clone()))
        }

        if let Some(value) = matches.get_one::<types::TypedUuidForUserId>("user-id") {
            request = request.user_id(value.clone());
        }

        if let Some(value) = matches.get_one::<std::path::PathBuf>("json-body") {
            let body_txt = std::fs::read_to_string(value).unwrap();
            let body_value =
                serde_json::from_str::<types::ApiUserEmailUpdateParams>(&body_txt).unwrap();
            request = request.body(body_value);
        }

        self.config
            .execute_set_api_user_contact_email(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_add_api_user_to_group(
        &self,
        matches: &::clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.add_api_user_to_group();
        if let Some(value) = matches.get_one::<types::TypedUuidForAccessGroupId>("group-id") {
            request = request.body_map(|body| body.group_id(value.clone()))
        }

        if let Some(value) = matches.get_one::<types::TypedUuidForUserId>("user-id") {
            request = request.user_id(value.clone());
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
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_remove_api_user_from_group(
        &self,
        matches: &::clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.remove_api_user_from_group();
        if let Some(value) = matches.get_one::<types::TypedUuidForAccessGroupId>("group-id") {
            request = request.group_id(value.clone());
        }

        if let Some(value) = matches.get_one::<types::TypedUuidForUserId>("user-id") {
            request = request.user_id(value.clone());
        }

        self.config
            .execute_remove_api_user_from_group(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_link_provider(&self, matches: &::clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.link_provider();
        if let Some(value) = matches.get_one::<::std::string::String>("token") {
            request = request.body_map(|body| body.token(value.clone()))
        }

        if let Some(value) = matches.get_one::<types::TypedUuidForUserId>("user-id") {
            request = request.user_id(value.clone());
        }

        if let Some(value) = matches.get_one::<std::path::PathBuf>("json-body") {
            let body_txt = std::fs::read_to_string(value).unwrap();
            let body_value =
                serde_json::from_str::<types::ApiUserProviderLinkPayload>(&body_txt).unwrap();
            request = request.body(body_value);
        }

        self.config.execute_link_provider(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_no_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_list_api_user_tokens(
        &self,
        matches: &::clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.list_api_user_tokens();
        if let Some(value) = matches.get_one::<types::TypedUuidForUserId>("user-id") {
            request = request.user_id(value.clone());
        }

        self.config
            .execute_list_api_user_tokens(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_create_api_user_token(
        &self,
        matches: &::clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.create_api_user_token();
        if let Some(value) =
            matches.get_one::<::chrono::DateTime<::chrono::offset::Utc>>("expires-at")
        {
            request = request.body_map(|body| body.expires_at(value.clone()))
        }

        if let Some(value) = matches.get_one::<types::TypedUuidForUserId>("user-id") {
            request = request.user_id(value.clone());
        }

        if let Some(value) = matches.get_one::<std::path::PathBuf>("json-body") {
            let body_txt = std::fs::read_to_string(value).unwrap();
            let body_value =
                serde_json::from_str::<types::ApiKeyCreateParamsForRfdPermission>(&body_txt)
                    .unwrap();
            request = request.body(body_value);
        }

        self.config
            .execute_create_api_user_token(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_get_api_user_token(
        &self,
        matches: &::clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.get_api_user_token();
        if let Some(value) = matches.get_one::<types::TypedUuidForApiKeyId>("api-key-id") {
            request = request.api_key_id(value.clone());
        }

        if let Some(value) = matches.get_one::<types::TypedUuidForUserId>("user-id") {
            request = request.user_id(value.clone());
        }

        self.config
            .execute_get_api_user_token(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_delete_api_user_token(
        &self,
        matches: &::clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.delete_api_user_token();
        if let Some(value) = matches.get_one::<types::TypedUuidForApiKeyId>("api-key-id") {
            request = request.api_key_id(value.clone());
        }

        if let Some(value) = matches.get_one::<types::TypedUuidForUserId>("user-id") {
            request = request.user_id(value.clone());
        }

        self.config
            .execute_delete_api_user_token(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_create_link_token(
        &self,
        matches: &::clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.create_link_token();
        if let Some(value) = matches.get_one::<types::TypedUuidForUserProviderId>("provider-id") {
            request = request.provider_id(value.clone());
        }

        if let Some(value) = matches.get_one::<types::TypedUuidForUserId>("user-id") {
            request = request.body_map(|body| body.user_id(value.clone()))
        }

        if let Some(value) = matches.get_one::<std::path::PathBuf>("json-body") {
            let body_txt = std::fs::read_to_string(value).unwrap();
            let body_value =
                serde_json::from_str::<types::ApiUserLinkRequestPayload>(&body_txt).unwrap();
            request = request.body(body_value);
        }

        self.config
            .execute_create_link_token(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_github_webhook(&self, matches: &::clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.github_webhook();
        if let Some(value) = matches.get_one::<::std::string::String>("ref") {
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
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_get_groups(&self, matches: &::clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.get_groups();
        self.config.execute_get_groups(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_create_group(&self, matches: &::clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.create_group();
        if let Some(value) = matches.get_one::<::std::string::String>("name") {
            request = request.body_map(|body| body.name(value.clone()))
        }

        if let Some(value) = matches.get_one::<std::path::PathBuf>("json-body") {
            let body_txt = std::fs::read_to_string(value).unwrap();
            let body_value =
                serde_json::from_str::<types::AccessGroupUpdateParamsForRfdPermission>(&body_txt)
                    .unwrap();
            request = request.body(body_value);
        }

        self.config.execute_create_group(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_update_group(&self, matches: &::clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.update_group();
        if let Some(value) = matches.get_one::<types::TypedUuidForAccessGroupId>("group-id") {
            request = request.group_id(value.clone());
        }

        if let Some(value) = matches.get_one::<::std::string::String>("name") {
            request = request.body_map(|body| body.name(value.clone()))
        }

        if let Some(value) = matches.get_one::<std::path::PathBuf>("json-body") {
            let body_txt = std::fs::read_to_string(value).unwrap();
            let body_value =
                serde_json::from_str::<types::AccessGroupUpdateParamsForRfdPermission>(&body_txt)
                    .unwrap();
            request = request.body(body_value);
        }

        self.config.execute_update_group(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_delete_group(&self, matches: &::clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.delete_group();
        if let Some(value) = matches.get_one::<types::TypedUuidForAccessGroupId>("group-id") {
            request = request.group_id(value.clone());
        }

        self.config.execute_delete_group(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_get_group_members(
        &self,
        matches: &::clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.get_group_members();
        if let Some(value) = matches.get_one::<types::TypedUuidForAccessGroupId>("group-id") {
            request = request.group_id(value.clone());
        }

        self.config
            .execute_get_group_members(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_list_jobs(&self, matches: &::clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.list_jobs();
        if let Some(value) = matches.get_one::<i64>("limit") {
            request = request.limit(value.clone());
        }

        if let Some(value) = matches.get_one::<i64>("offset") {
            request = request.offset(value.clone());
        }

        if let Some(value) = matches.get_one::<::std::string::String>("rfd") {
            request = request.rfd(value.clone());
        }

        self.config.execute_list_jobs(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_magic_link_exchange(
        &self,
        matches: &::clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.magic_link_exchange();
        if let Some(value) = matches.get_one::<types::TypedUuidForMagicLinkAttemptId>("attempt-id")
        {
            request = request.body_map(|body| body.attempt_id(value.clone()))
        }

        if let Some(value) = matches.get_one::<::std::string::String>("channel") {
            request = request.channel(value.clone());
        }

        if let Some(value) = matches.get_one::<::std::string::String>("recipient") {
            request = request.body_map(|body| body.recipient(value.clone()))
        }

        if let Some(value) = matches.get_one::<::std::string::String>("secret") {
            request = request.body_map(|body| body.secret(value.clone()))
        }

        if let Some(value) = matches.get_one::<std::path::PathBuf>("json-body") {
            let body_txt = std::fs::read_to_string(value).unwrap();
            let body_value =
                serde_json::from_str::<types::MagicLinkExchangeRequest>(&body_txt).unwrap();
            request = request.body(body_value);
        }

        self.config
            .execute_magic_link_exchange(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_magic_link_send(
        &self,
        matches: &::clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.magic_link_send();
        if let Some(value) = matches.get_one::<::std::string::String>("channel") {
            request = request.channel(value.clone());
        }

        if let Some(value) = matches.get_one::<i64>("expires-in") {
            request = request.body_map(|body| body.expires_in(value.clone()))
        }

        if let Some(value) = matches.get_one::<types::MagicLinkMedium>("medium") {
            request = request.body_map(|body| body.medium(value.clone()))
        }

        if let Some(value) = matches.get_one::<::std::string::String>("recipient") {
            request = request.body_map(|body| body.recipient(value.clone()))
        }

        if let Some(value) = matches.get_one::<::std::string::String>("redirect-uri") {
            request = request.body_map(|body| body.redirect_uri(value.clone()))
        }

        if let Some(value) = matches.get_one::<::std::string::String>("scope") {
            request = request.body_map(|body| body.scope(value.clone()))
        }

        if let Some(value) = matches.get_one::<::std::string::String>("secret") {
            request = request.body_map(|body| body.secret(value.clone()))
        }

        if let Some(value) = matches.get_one::<std::path::PathBuf>("json-body") {
            let body_txt = std::fs::read_to_string(value).unwrap();
            let body_value =
                serde_json::from_str::<types::MagicLinkSendRequest>(&body_txt).unwrap();
            request = request.body(body_value);
        }

        self.config.execute_magic_link_send(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_authz_code_redirect(
        &self,
        matches: &::clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.authz_code_redirect();
        if let Some(value) = matches.get_one::<types::TypedUuidForOAuthClientId>("client-id") {
            request = request.client_id(value.clone());
        }

        if let Some(value) = matches.get_one::<types::OAuthProviderName>("provider") {
            request = request.provider(value.clone());
        }

        if let Some(value) = matches.get_one::<::std::string::String>("redirect-uri") {
            request = request.redirect_uri(value.clone());
        }

        if let Some(value) = matches.get_one::<::std::string::String>("response-type") {
            request = request.response_type(value.clone());
        }

        if let Some(value) = matches.get_one::<::std::string::String>("scope") {
            request = request.scope(value.clone());
        }

        if let Some(value) = matches.get_one::<::std::string::String>("state") {
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
        matches: &::clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.authz_code_callback();
        if let Some(value) = matches.get_one::<::std::string::String>("code") {
            request = request.code(value.clone());
        }

        if let Some(value) = matches.get_one::<::std::string::String>("error") {
            request = request.error(value.clone());
        }

        if let Some(value) = matches.get_one::<types::OAuthProviderName>("provider") {
            request = request.provider(value.clone());
        }

        if let Some(value) = matches.get_one::<::std::string::String>("state") {
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
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_authz_code_exchange(
        &self,
        matches: &::clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.authz_code_exchange();
        if let Some(value) = matches.get_one::<types::TypedUuidForOAuthClientId>("client-id") {
            request = request.body_map(|body| body.client_id(value.clone()))
        }

        if let Some(value) = matches.get_one::<types::SecretString>("client-secret") {
            request = request.body_map(|body| body.client_secret(value.clone()))
        }

        if let Some(value) = matches.get_one::<::std::string::String>("code") {
            request = request.body_map(|body| body.code(value.clone()))
        }

        if let Some(value) = matches.get_one::<::std::string::String>("grant-type") {
            request = request.body_map(|body| body.grant_type(value.clone()))
        }

        if let Some(value) = matches.get_one::<::std::string::String>("pkce-verifier") {
            request = request.body_map(|body| body.pkce_verifier(value.clone()))
        }

        if let Some(value) = matches.get_one::<types::OAuthProviderName>("provider") {
            request = request.provider(value.clone());
        }

        if let Some(value) = matches.get_one::<::std::string::String>("redirect-uri") {
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
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_get_device_provider(
        &self,
        matches: &::clap::ArgMatches,
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
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_exchange_device_token(
        &self,
        matches: &::clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.exchange_device_token();
        if let Some(value) = matches.get_one::<::std::string::String>("device-code") {
            request = request.body_map(|body| body.device_code(value.clone()))
        }

        if let Some(value) =
            matches.get_one::<::chrono::DateTime<::chrono::offset::Utc>>("expires-at")
        {
            request = request.body_map(|body| body.expires_at(value.clone()))
        }

        if let Some(value) = matches.get_one::<::std::string::String>("grant-type") {
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

    pub async fn execute_list_magic_links(
        &self,
        matches: &::clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.list_magic_links();
        self.config
            .execute_list_magic_links(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_create_magic_link(
        &self,
        matches: &::clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.create_magic_link();
        self.config
            .execute_create_magic_link(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_get_magic_link(&self, matches: &::clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.get_magic_link();
        if let Some(value) = matches.get_one::<types::TypedUuidForMagicLinkId>("client-id") {
            request = request.client_id(value.clone());
        }

        self.config.execute_get_magic_link(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_create_magic_link_redirect_uri(
        &self,
        matches: &::clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.create_magic_link_redirect_uri();
        if let Some(value) = matches.get_one::<types::TypedUuidForMagicLinkId>("client-id") {
            request = request.client_id(value.clone());
        }

        if let Some(value) = matches.get_one::<::std::string::String>("redirect-uri") {
            request = request.body_map(|body| body.redirect_uri(value.clone()))
        }

        if let Some(value) = matches.get_one::<std::path::PathBuf>("json-body") {
            let body_txt = std::fs::read_to_string(value).unwrap();
            let body_value =
                serde_json::from_str::<types::AddMagicLinkRedirectBody>(&body_txt).unwrap();
            request = request.body(body_value);
        }

        self.config
            .execute_create_magic_link_redirect_uri(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_delete_magic_link_redirect_uri(
        &self,
        matches: &::clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.delete_magic_link_redirect_uri();
        if let Some(value) = matches.get_one::<types::TypedUuidForMagicLinkId>("client-id") {
            request = request.client_id(value.clone());
        }

        if let Some(value) =
            matches.get_one::<types::TypedUuidForMagicLinkRedirectUriId>("redirect-uri-id")
        {
            request = request.redirect_uri_id(value.clone());
        }

        self.config
            .execute_delete_magic_link_redirect_uri(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_create_magic_link_secret(
        &self,
        matches: &::clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.create_magic_link_secret();
        if let Some(value) = matches.get_one::<types::TypedUuidForMagicLinkId>("client-id") {
            request = request.client_id(value.clone());
        }

        self.config
            .execute_create_magic_link_secret(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_delete_magic_link_secret(
        &self,
        matches: &::clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.delete_magic_link_secret();
        if let Some(value) = matches.get_one::<types::TypedUuidForMagicLinkId>("client-id") {
            request = request.client_id(value.clone());
        }

        if let Some(value) = matches.get_one::<types::TypedUuidForMagicLinkSecretId>("secret-id") {
            request = request.secret_id(value.clone());
        }

        self.config
            .execute_delete_magic_link_secret(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_get_mappers(&self, matches: &::clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.get_mappers();
        if let Some(value) = matches.get_one::<bool>("include-depleted") {
            request = request.include_depleted(value.clone());
        }

        self.config.execute_get_mappers(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_create_mapper(&self, matches: &::clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.create_mapper();
        if let Some(value) = matches.get_one::<i32>("max-activations") {
            request = request.body_map(|body| body.max_activations(value.clone()))
        }

        if let Some(value) = matches.get_one::<::std::string::String>("name") {
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
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_delete_mapper(&self, matches: &::clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.delete_mapper();
        if let Some(value) = matches.get_one::<types::TypedUuidForMapperId>("mapper-id") {
            request = request.mapper_id(value.clone());
        }

        self.config.execute_delete_mapper(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_list_oauth_clients(
        &self,
        matches: &::clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.list_oauth_clients();
        self.config
            .execute_list_oauth_clients(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_create_oauth_client(
        &self,
        matches: &::clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.create_oauth_client();
        self.config
            .execute_create_oauth_client(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_get_oauth_client(
        &self,
        matches: &::clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.get_oauth_client();
        if let Some(value) = matches.get_one::<types::TypedUuidForOAuthClientId>("client-id") {
            request = request.client_id(value.clone());
        }

        self.config
            .execute_get_oauth_client(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_create_oauth_client_redirect_uri(
        &self,
        matches: &::clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.create_oauth_client_redirect_uri();
        if let Some(value) = matches.get_one::<types::TypedUuidForOAuthClientId>("client-id") {
            request = request.client_id(value.clone());
        }

        if let Some(value) = matches.get_one::<::std::string::String>("redirect-uri") {
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
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_delete_oauth_client_redirect_uri(
        &self,
        matches: &::clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.delete_oauth_client_redirect_uri();
        if let Some(value) = matches.get_one::<types::TypedUuidForOAuthClientId>("client-id") {
            request = request.client_id(value.clone());
        }

        if let Some(value) =
            matches.get_one::<types::TypedUuidForOAuthRedirectUriId>("redirect-uri-id")
        {
            request = request.redirect_uri_id(value.clone());
        }

        self.config
            .execute_delete_oauth_client_redirect_uri(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_create_oauth_client_secret(
        &self,
        matches: &::clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.create_oauth_client_secret();
        if let Some(value) = matches.get_one::<types::TypedUuidForOAuthClientId>("client-id") {
            request = request.client_id(value.clone());
        }

        self.config
            .execute_create_oauth_client_secret(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_delete_oauth_client_secret(
        &self,
        matches: &::clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.delete_oauth_client_secret();
        if let Some(value) = matches.get_one::<types::TypedUuidForOAuthClientId>("client-id") {
            request = request.client_id(value.clone());
        }

        if let Some(value) = matches.get_one::<types::TypedUuidForOAuthSecretId>("secret-id") {
            request = request.secret_id(value.clone());
        }

        self.config
            .execute_delete_oauth_client_secret(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_list_rfds(&self, matches: &::clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.list_rfds();
        self.config.execute_list_rfds(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_reserve_rfd(&self, matches: &::clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.reserve_rfd();
        if let Some(value) = matches.get_one::<::std::string::String>("content") {
            request = request.body_map(|body| body.content(value.clone()))
        }

        if let Some(value) = matches.get_one::<::std::string::String>("title") {
            request = request.body_map(|body| body.title(value.clone()))
        }

        if let Some(value) = matches.get_one::<std::path::PathBuf>("json-body") {
            let body_txt = std::fs::read_to_string(value).unwrap();
            let body_value = serde_json::from_str::<types::ReserveRfdBody>(&body_txt).unwrap();
            request = request.body(body_value);
        }

        self.config.execute_reserve_rfd(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_view_rfd_meta(&self, matches: &::clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.view_rfd_meta();
        if let Some(value) = matches.get_one::<::std::string::String>("number") {
            request = request.number(value.clone());
        }

        self.config.execute_view_rfd_meta(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_view_rfd_attr(&self, matches: &::clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.view_rfd_attr();
        if let Some(value) = matches.get_one::<types::RfdAttrName>("attr") {
            request = request.attr(value.clone());
        }

        if let Some(value) = matches.get_one::<::std::string::String>("number") {
            request = request.number(value.clone());
        }

        self.config.execute_view_rfd_attr(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_set_rfd_attr(&self, matches: &::clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.set_rfd_attr();
        if let Some(value) = matches.get_one::<types::RfdAttrName>("attr") {
            request = request.attr(value.clone());
        }

        if let Some(value) = matches.get_one::<::std::string::String>("message") {
            request = request.body_map(|body| body.message(value.clone()))
        }

        if let Some(value) = matches.get_one::<::std::string::String>("number") {
            request = request.number(value.clone());
        }

        if let Some(value) = matches.get_one::<::std::string::String>("value") {
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
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_set_rfd_content(
        &self,
        matches: &::clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.set_rfd_content();
        if let Some(value) = matches.get_one::<::std::string::String>("content") {
            request = request.body_map(|body| body.content(value.clone()))
        }

        if let Some(value) = matches.get_one::<::std::string::String>("message") {
            request = request.body_map(|body| body.message(value.clone()))
        }

        if let Some(value) = matches.get_one::<::std::string::String>("number") {
            request = request.number(value.clone());
        }

        if let Some(value) = matches.get_one::<std::path::PathBuf>("json-body") {
            let body_txt = std::fs::read_to_string(value).unwrap();
            let body_value =
                serde_json::from_str::<types::RfdUpdateContentBody>(&body_txt).unwrap();
            request = request.body(body_value);
        }

        self.config.execute_set_rfd_content(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_view_rfd_discussion(
        &self,
        matches: &::clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.view_rfd_discussion();
        if let Some(value) = matches.get_one::<::std::string::String>("number") {
            request = request.number(value.clone());
        }

        self.config
            .execute_view_rfd_discussion(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_view_rfd_pdf(&self, matches: &::clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.view_rfd_pdf();
        if let Some(value) = matches.get_one::<::std::string::String>("number") {
            request = request.number(value.clone());
        }

        self.config.execute_view_rfd_pdf(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_view_rfd(&self, matches: &::clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.view_rfd();
        if let Some(value) = matches.get_one::<::std::string::String>("number") {
            request = request.number(value.clone());
        }

        self.config.execute_view_rfd(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_set_rfd_document(
        &self,
        matches: &::clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.set_rfd_document();
        if let Some(value) = matches.get_one::<::std::string::String>("document") {
            request = request.body_map(|body| body.document(value.clone()))
        }

        if let Some(value) = matches.get_one::<::std::string::String>("message") {
            request = request.body_map(|body| body.message(value.clone()))
        }

        if let Some(value) = matches.get_one::<::std::string::String>("number") {
            request = request.number(value.clone());
        }

        if let Some(value) = matches.get_one::<std::path::PathBuf>("json-body") {
            let body_txt = std::fs::read_to_string(value).unwrap();
            let body_value = serde_json::from_str::<types::RfdUpdateBody>(&body_txt).unwrap();
            request = request.body(body_value);
        }

        self.config
            .execute_set_rfd_document(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_list_rfd_revisions(
        &self,
        matches: &::clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.list_rfd_revisions();
        if let Some(value) = matches.get_one::<i64>("limit") {
            request = request.limit(value.clone());
        }

        if let Some(value) = matches.get_one::<::std::string::String>("number") {
            request = request.number(value.clone());
        }

        if let Some(value) = matches.get_one::<i64>("offset") {
            request = request.offset(value.clone());
        }

        self.config
            .execute_list_rfd_revisions(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_view_rfd_revision_meta(
        &self,
        matches: &::clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.view_rfd_revision_meta();
        if let Some(value) = matches.get_one::<::std::string::String>("number") {
            request = request.number(value.clone());
        }

        if let Some(value) = matches.get_one::<types::TypedUuidForRfdRevisionId>("revision") {
            request = request.revision(value.clone());
        }

        self.config
            .execute_view_rfd_revision_meta(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_update_rfd_revision(
        &self,
        matches: &::clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.update_rfd_revision();
        if let Some(value) = matches.get_one::<bool>("major-change") {
            request = request.body_map(|body| body.major_change(value.clone()))
        }

        if let Some(value) = matches.get_one::<::std::string::String>("number") {
            request = request.number(value.clone());
        }

        if let Some(value) = matches.get_one::<types::TypedUuidForRfdRevisionId>("revision") {
            request = request.revision(value.clone());
        }

        if let Some(value) = matches.get_one::<std::path::PathBuf>("json-body") {
            let body_txt = std::fs::read_to_string(value).unwrap();
            let body_value = serde_json::from_str::<types::UpdateRfdAttrBody>(&body_txt).unwrap();
            request = request.body(body_value);
        }

        self.config
            .execute_update_rfd_revision(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_view_rfd_revision_attr(
        &self,
        matches: &::clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.view_rfd_revision_attr();
        if let Some(value) = matches.get_one::<types::RfdAttrName>("attr") {
            request = request.attr(value.clone());
        }

        if let Some(value) = matches.get_one::<::std::string::String>("number") {
            request = request.number(value.clone());
        }

        if let Some(value) = matches.get_one::<types::TypedUuidForRfdRevisionId>("revision") {
            request = request.revision(value.clone());
        }

        self.config
            .execute_view_rfd_revision_attr(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_view_rfd_revision_discussion(
        &self,
        matches: &::clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.view_rfd_revision_discussion();
        if let Some(value) = matches.get_one::<::std::string::String>("number") {
            request = request.number(value.clone());
        }

        if let Some(value) = matches.get_one::<types::TypedUuidForRfdRevisionId>("revision") {
            request = request.revision(value.clone());
        }

        self.config
            .execute_view_rfd_revision_discussion(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_view_rfd_revision_pdf(
        &self,
        matches: &::clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.view_rfd_revision_pdf();
        if let Some(value) = matches.get_one::<::std::string::String>("number") {
            request = request.number(value.clone());
        }

        if let Some(value) = matches.get_one::<types::TypedUuidForRfdRevisionId>("revision") {
            request = request.revision(value.clone());
        }

        self.config
            .execute_view_rfd_revision_pdf(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_view_rfd_revision(
        &self,
        matches: &::clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.view_rfd_revision();
        if let Some(value) = matches.get_one::<::std::string::String>("number") {
            request = request.number(value.clone());
        }

        if let Some(value) = matches.get_one::<types::TypedUuidForRfdRevisionId>("revision") {
            request = request.revision(value.clone());
        }

        self.config
            .execute_view_rfd_revision(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_discuss_rfd(&self, matches: &::clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.discuss_rfd();
        if let Some(value) = matches.get_one::<::std::string::String>("number") {
            request = request.number(value.clone());
        }

        self.config.execute_discuss_rfd(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_publish_rfd(&self, matches: &::clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.publish_rfd();
        if let Some(value) = matches.get_one::<::std::string::String>("number") {
            request = request.number(value.clone());
        }

        self.config.execute_publish_rfd(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_update_rfd_visibility(
        &self,
        matches: &::clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.update_rfd_visibility();
        if let Some(value) = matches.get_one::<::std::string::String>("number") {
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
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_search_rfds(&self, matches: &::clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.search_rfds();
        if let Some(value) = matches.get_one::<::std::string::String>("attributes-to-crop") {
            request = request.attributes_to_crop(value.clone());
        }

        if let Some(value) = matches.get_one::<::std::string::String>("highlight-post-tag") {
            request = request.highlight_post_tag(value.clone());
        }

        if let Some(value) = matches.get_one::<::std::string::String>("highlight-pre-tag") {
            request = request.highlight_pre_tag(value.clone());
        }

        if let Some(value) = matches.get_one::<u32>("limit") {
            request = request.limit(value.clone());
        }

        if let Some(value) = matches.get_one::<u32>("offset") {
            request = request.offset(value.clone());
        }

        if let Some(value) = matches.get_one::<::std::string::String>("q") {
            request = request.q(value.clone());
        }

        self.config.execute_search_rfds(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_get_self(&self, matches: &::clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.get_self();
        self.config.execute_get_self(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }
}

pub trait CliConfig {
    fn success_item<T>(&self, value: &ResponseValue<T>)
    where
        T: schemars::JsonSchema + serde::Serialize + std::fmt::Debug;
    fn success_no_item(&self, value: &ResponseValue<()>);
    fn error<T>(&self, value: &Error<T>)
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
        matches: &::clap::ArgMatches,
        request: &mut builder::JwksJson,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_openid_configuration(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::OpenidConfiguration,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_list_api_users(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::ListApiUsers,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_create_api_user(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::CreateApiUser,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_get_api_user(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::GetApiUser,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_update_api_user(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::UpdateApiUser,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_set_api_user_contact_email(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::SetApiUserContactEmail,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_add_api_user_to_group(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::AddApiUserToGroup,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_remove_api_user_from_group(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::RemoveApiUserFromGroup,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_link_provider(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::LinkProvider,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_list_api_user_tokens(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::ListApiUserTokens,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_create_api_user_token(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::CreateApiUserToken,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_get_api_user_token(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::GetApiUserToken,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_delete_api_user_token(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::DeleteApiUserToken,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_create_link_token(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::CreateLinkToken,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_github_webhook(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::GithubWebhook,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_get_groups(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::GetGroups,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_create_group(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::CreateGroup,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_update_group(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::UpdateGroup,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_delete_group(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::DeleteGroup,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_get_group_members(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::GetGroupMembers,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_list_jobs(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::ListJobs,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_magic_link_exchange(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::MagicLinkExchange,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_magic_link_send(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::MagicLinkSend,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_authz_code_redirect(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::AuthzCodeRedirect,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_authz_code_callback(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::AuthzCodeCallback,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_authz_code_exchange(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::AuthzCodeExchange,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_get_device_provider(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::GetDeviceProvider,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_exchange_device_token(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::ExchangeDeviceToken,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_list_magic_links(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::ListMagicLinks,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_create_magic_link(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::CreateMagicLink,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_get_magic_link(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::GetMagicLink,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_create_magic_link_redirect_uri(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::CreateMagicLinkRedirectUri,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_delete_magic_link_redirect_uri(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::DeleteMagicLinkRedirectUri,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_create_magic_link_secret(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::CreateMagicLinkSecret,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_delete_magic_link_secret(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::DeleteMagicLinkSecret,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_get_mappers(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::GetMappers,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_create_mapper(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::CreateMapper,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_delete_mapper(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::DeleteMapper,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_list_oauth_clients(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::ListOauthClients,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_create_oauth_client(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::CreateOauthClient,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_get_oauth_client(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::GetOauthClient,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_create_oauth_client_redirect_uri(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::CreateOauthClientRedirectUri,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_delete_oauth_client_redirect_uri(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::DeleteOauthClientRedirectUri,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_create_oauth_client_secret(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::CreateOauthClientSecret,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_delete_oauth_client_secret(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::DeleteOauthClientSecret,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_list_rfds(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::ListRfds,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_reserve_rfd(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::ReserveRfd,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_view_rfd_meta(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::ViewRfdMeta,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_view_rfd_attr(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::ViewRfdAttr,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_set_rfd_attr(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::SetRfdAttr,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_set_rfd_content(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::SetRfdContent,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_view_rfd_discussion(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::ViewRfdDiscussion,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_view_rfd_pdf(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::ViewRfdPdf,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_view_rfd(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::ViewRfd,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_set_rfd_document(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::SetRfdDocument,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_list_rfd_revisions(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::ListRfdRevisions,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_view_rfd_revision_meta(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::ViewRfdRevisionMeta,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_update_rfd_revision(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::UpdateRfdRevision,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_view_rfd_revision_attr(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::ViewRfdRevisionAttr,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_view_rfd_revision_discussion(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::ViewRfdRevisionDiscussion,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_view_rfd_revision_pdf(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::ViewRfdRevisionPdf,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_view_rfd_revision(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::ViewRfdRevision,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_discuss_rfd(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::DiscussRfd,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_publish_rfd(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::PublishRfd,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_update_rfd_visibility(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::UpdateRfdVisibility,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_search_rfds(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::SearchRfds,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_get_self(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::GetSelf,
    ) -> anyhow::Result<()> {
        Ok(())
    }
}

#[derive(Copy, Clone, Debug)]
pub enum CliCommand {
    JwksJson,
    OpenidConfiguration,
    ListApiUsers,
    CreateApiUser,
    GetApiUser,
    UpdateApiUser,
    SetApiUserContactEmail,
    AddApiUserToGroup,
    RemoveApiUserFromGroup,
    LinkProvider,
    ListApiUserTokens,
    CreateApiUserToken,
    GetApiUserToken,
    DeleteApiUserToken,
    CreateLinkToken,
    GithubWebhook,
    GetGroups,
    CreateGroup,
    UpdateGroup,
    DeleteGroup,
    GetGroupMembers,
    ListJobs,
    MagicLinkExchange,
    MagicLinkSend,
    AuthzCodeRedirect,
    AuthzCodeCallback,
    AuthzCodeExchange,
    GetDeviceProvider,
    ExchangeDeviceToken,
    ListMagicLinks,
    CreateMagicLink,
    GetMagicLink,
    CreateMagicLinkRedirectUri,
    DeleteMagicLinkRedirectUri,
    CreateMagicLinkSecret,
    DeleteMagicLinkSecret,
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
    ListRfds,
    ReserveRfd,
    ViewRfdMeta,
    ViewRfdAttr,
    SetRfdAttr,
    SetRfdContent,
    ViewRfdDiscussion,
    ViewRfdPdf,
    ViewRfd,
    SetRfdDocument,
    ListRfdRevisions,
    ViewRfdRevisionMeta,
    UpdateRfdRevision,
    ViewRfdRevisionAttr,
    ViewRfdRevisionDiscussion,
    ViewRfdRevisionPdf,
    ViewRfdRevision,
    DiscussRfd,
    PublishRfd,
    UpdateRfdVisibility,
    SearchRfds,
    GetSelf,
}

impl CliCommand {
    pub fn iter() -> impl Iterator<Item = CliCommand> {
        vec![
            CliCommand::JwksJson,
            CliCommand::OpenidConfiguration,
            CliCommand::ListApiUsers,
            CliCommand::CreateApiUser,
            CliCommand::GetApiUser,
            CliCommand::UpdateApiUser,
            CliCommand::SetApiUserContactEmail,
            CliCommand::AddApiUserToGroup,
            CliCommand::RemoveApiUserFromGroup,
            CliCommand::LinkProvider,
            CliCommand::ListApiUserTokens,
            CliCommand::CreateApiUserToken,
            CliCommand::GetApiUserToken,
            CliCommand::DeleteApiUserToken,
            CliCommand::CreateLinkToken,
            CliCommand::GithubWebhook,
            CliCommand::GetGroups,
            CliCommand::CreateGroup,
            CliCommand::UpdateGroup,
            CliCommand::DeleteGroup,
            CliCommand::GetGroupMembers,
            CliCommand::ListJobs,
            CliCommand::MagicLinkExchange,
            CliCommand::MagicLinkSend,
            CliCommand::AuthzCodeRedirect,
            CliCommand::AuthzCodeCallback,
            CliCommand::AuthzCodeExchange,
            CliCommand::GetDeviceProvider,
            CliCommand::ExchangeDeviceToken,
            CliCommand::ListMagicLinks,
            CliCommand::CreateMagicLink,
            CliCommand::GetMagicLink,
            CliCommand::CreateMagicLinkRedirectUri,
            CliCommand::DeleteMagicLinkRedirectUri,
            CliCommand::CreateMagicLinkSecret,
            CliCommand::DeleteMagicLinkSecret,
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
            CliCommand::ListRfds,
            CliCommand::ReserveRfd,
            CliCommand::ViewRfdMeta,
            CliCommand::ViewRfdAttr,
            CliCommand::SetRfdAttr,
            CliCommand::SetRfdContent,
            CliCommand::ViewRfdDiscussion,
            CliCommand::ViewRfdPdf,
            CliCommand::ViewRfd,
            CliCommand::SetRfdDocument,
            CliCommand::ListRfdRevisions,
            CliCommand::ViewRfdRevisionMeta,
            CliCommand::UpdateRfdRevision,
            CliCommand::ViewRfdRevisionAttr,
            CliCommand::ViewRfdRevisionDiscussion,
            CliCommand::ViewRfdRevisionPdf,
            CliCommand::ViewRfdRevision,
            CliCommand::DiscussRfd,
            CliCommand::PublishRfd,
            CliCommand::UpdateRfdVisibility,
            CliCommand::SearchRfds,
            CliCommand::GetSelf,
        ]
        .into_iter()
    }
}
