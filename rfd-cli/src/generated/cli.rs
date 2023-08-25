// The contents of this file are generated; do not modify them.

use rfd_sdk::*;

pub struct Cli<T: CliOverride = ()> {
    client: rfd_sdk::Client,
    over: T,
}

impl Cli {
    pub fn new(client: rfd_sdk::Client) -> Self {
        Self { client, over: () }
    }

    pub fn get_command(cmd: CliCommand) -> clap::Command {
        match cmd {
            CliCommand::CreateApiUser => Self::cli_create_api_user(),
            CliCommand::GetApiUser => Self::cli_get_api_user(),
            CliCommand::UpdateApiUser => Self::cli_update_api_user(),
            CliCommand::ListApiUserTokens => Self::cli_list_api_user_tokens(),
            CliCommand::CreateApiUserToken => Self::cli_create_api_user_token(),
            CliCommand::GetApiUserToken => Self::cli_get_api_user_token(),
            CliCommand::DeleteApiUserToken => Self::cli_delete_api_user_token(),
            CliCommand::GithubWebhook => Self::cli_github_webhook(),
            CliCommand::AccessTokenLogin => Self::cli_access_token_login(),
            CliCommand::JwtLogin => Self::cli_jwt_login(),
            CliCommand::GetDeviceProvider => Self::cli_get_device_provider(),
            CliCommand::ExchangeDeviceToken => Self::cli_exchange_device_token(),
            CliCommand::GetRfd => Self::cli_get_rfd(),
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

    pub fn cli_access_token_login() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("expiration")
                    .long("expiration")
                    .value_parser(clap::value_parser!(chrono::DateTime<chrono::offset::Utc>))
                    .required(false),
            )
            .arg(
                clap::Arg::new("provider")
                    .long("provider")
                    .value_parser(clap::builder::TypedValueParser::map(
                        clap::builder::PossibleValuesParser::new([
                            types::AccessTokenProviderName::Github.to_string(),
                        ]),
                        |s| types::AccessTokenProviderName::try_from(s).unwrap(),
                    ))
                    .required(true),
            )
            .arg(
                clap::Arg::new("token")
                    .long("token")
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

    pub fn cli_jwt_login() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("expiration")
                    .long("expiration")
                    .value_parser(clap::value_parser!(chrono::DateTime<chrono::offset::Utc>))
                    .required(false),
            )
            .arg(
                clap::Arg::new("provider")
                    .long("provider")
                    .value_parser(clap::builder::TypedValueParser::map(
                        clap::builder::PossibleValuesParser::new([
                            types::JwtProviderName::Google.to_string()
                        ]),
                        |s| types::JwtProviderName::try_from(s).unwrap(),
                    ))
                    .required(true),
            )
            .arg(
                clap::Arg::new("token")
                    .long("token")
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

    pub fn cli_get_device_provider() -> clap::Command {
        clap::Command::new("").arg(
            clap::Arg::new("provider")
                .long("provider")
                .value_parser(clap::builder::TypedValueParser::map(
                    clap::builder::PossibleValuesParser::new([
                        types::OAuthProviderName::Google.to_string()
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

    pub fn cli_get_rfd() -> clap::Command {
        clap::Command::new("").arg(
            clap::Arg::new("number")
                .long("number")
                .value_parser(clap::value_parser!(String))
                .required(true),
        )
    }

    pub fn cli_get_self() -> clap::Command {
        clap::Command::new("").about("Retrieve the user information of the calling user")
    }
}

impl<T: CliOverride> Cli<T> {
    pub fn new_with_override(client: rfd_sdk::Client, over: T) -> Self {
        Self { client, over }
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
            CliCommand::AccessTokenLogin => {
                self.execute_access_token_login(matches).await;
            }
            CliCommand::JwtLogin => {
                self.execute_jwt_login(matches).await;
            }
            CliCommand::GetDeviceProvider => {
                self.execute_get_device_provider(matches).await;
            }
            CliCommand::ExchangeDeviceToken => {
                self.execute_exchange_device_token(matches).await;
            }
            CliCommand::GetRfd => {
                self.execute_get_rfd(matches).await;
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
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
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
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
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
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
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
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
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
            let body_value =
                serde_json::from_str::<types::ApiUserTokenCreateParams>(&body_txt).unwrap();
            request = request.body(body_value);
        }

        self.over
            .execute_create_api_user_token(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
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
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
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
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
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
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_access_token_login(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.access_token_login();
        if let Some(value) = matches.get_one::<chrono::DateTime<chrono::offset::Utc>>("expiration")
        {
            request = request.body_map(|body| body.expiration(value.clone()))
        }

        if let Some(value) = matches.get_one::<types::AccessTokenProviderName>("provider") {
            request = request.provider(value.clone());
        }

        if let Some(value) = matches.get_one::<String>("token") {
            request = request.body_map(|body| body.token(value.clone()))
        }

        if let Some(value) = matches.get_one::<std::path::PathBuf>("json-body") {
            let body_txt = std::fs::read_to_string(value).unwrap();
            let body_value =
                serde_json::from_str::<types::AccessTokenProviderLogin>(&body_txt).unwrap();
            request = request.body(body_value);
        }

        self.over
            .execute_access_token_login(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_jwt_login(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.jwt_login();
        if let Some(value) = matches.get_one::<chrono::DateTime<chrono::offset::Utc>>("expiration")
        {
            request = request.body_map(|body| body.expiration(value.clone()))
        }

        if let Some(value) = matches.get_one::<types::JwtProviderName>("provider") {
            request = request.provider(value.clone());
        }

        if let Some(value) = matches.get_one::<String>("token") {
            request = request.body_map(|body| body.token(value.clone()))
        }

        if let Some(value) = matches.get_one::<std::path::PathBuf>("json-body") {
            let body_txt = std::fs::read_to_string(value).unwrap();
            let body_value = serde_json::from_str::<types::JwtProviderLogin>(&body_txt).unwrap();
            request = request.body(body_value);
        }

        self.over.execute_jwt_login(matches, &mut request).unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
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
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
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

    pub async fn execute_get_rfd(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.get_rfd();
        if let Some(value) = matches.get_one::<String>("number") {
            request = request.number(value.clone());
        }

        self.over.execute_get_rfd(matches, &mut request).unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_get_self(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.get_self();
        self.over.execute_get_self(matches, &mut request).unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
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

    fn execute_access_token_login(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::AccessTokenLogin,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_jwt_login(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::JwtLogin,
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

    fn execute_get_rfd(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::GetRfd,
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

#[derive(Copy, Clone, Debug)]
pub enum CliCommand {
    CreateApiUser,
    GetApiUser,
    UpdateApiUser,
    ListApiUserTokens,
    CreateApiUserToken,
    GetApiUserToken,
    DeleteApiUserToken,
    GithubWebhook,
    AccessTokenLogin,
    JwtLogin,
    GetDeviceProvider,
    ExchangeDeviceToken,
    GetRfd,
    GetSelf,
}

impl CliCommand {
    pub fn iter() -> impl Iterator<Item = CliCommand> {
        vec![
            CliCommand::CreateApiUser,
            CliCommand::GetApiUser,
            CliCommand::UpdateApiUser,
            CliCommand::ListApiUserTokens,
            CliCommand::CreateApiUserToken,
            CliCommand::GetApiUserToken,
            CliCommand::DeleteApiUserToken,
            CliCommand::GithubWebhook,
            CliCommand::AccessTokenLogin,
            CliCommand::JwtLogin,
            CliCommand::GetDeviceProvider,
            CliCommand::ExchangeDeviceToken,
            CliCommand::GetRfd,
            CliCommand::GetSelf,
        ]
        .into_iter()
    }
}
