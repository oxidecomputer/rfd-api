// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#![allow(unused)]

use anyhow::{anyhow, Result};
use clap::{value_parser, Arg, ArgAction, Command, CommandFactory, FromArgMatches, ValueEnum};
use generated::cli::{CliConfig as ProgenitorCliConfig, *};
use printer::{CliOutput, Printer, RfdJsonPrinter, RfdTabPrinter};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use rfd_sdk::Client;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::any::Any;
use std::fmt::{Debug, Display};
use std::ops::Deref;
use std::time::Duration;
use std::{collections::HashMap, error::Error};
use store::CliConfig;

mod cmd;
mod err;
mod generated;
mod printer;
mod store;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum VerbosityLevel {
    None,
    All,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Clone, Serialize, Deserialize)]
pub enum FormatStyle {
    #[value(name = "json")]
    Json,
    #[value(name = "tab")]
    Tab,
}

impl Display for FormatStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Json => write!(f, "json"),
            Self::Tab => write!(f, "tab"),
        }
    }
}

#[derive(Debug)]
pub struct Context {
    config: CliConfig,
    client: Option<Client>,
    printer: Option<Printer>,
    verbosity: VerbosityLevel,
}

impl Context {
    pub fn new() -> Result<Self> {
        let config = CliConfig::new()?;

        Ok(Self {
            config,
            client: None,
            printer: None,
            verbosity: VerbosityLevel::None,
        })
    }

    pub fn new_client(&self, token: Option<&str>) -> Result<Client> {
        let mut default_headers = HeaderMap::new();

        if let Some(token) = token {
            let mut auth_header = HeaderValue::from_str(&format!("Bearer {}", token))?;
            auth_header.set_sensitive(true);
            default_headers.insert(AUTHORIZATION, auth_header);
        }

        let http_client = reqwest::Client::builder()
            .default_headers(default_headers)
            .connect_timeout(Duration::from_secs(5))
            .timeout(Duration::from_secs(10))
            .build()?;

        Ok(Client::new_with_client(self.config.host()?, http_client))
    }

    pub fn client(&mut self) -> Result<&Client> {
        if self.client.is_none() {
            self.client = Some(Self::new_client(self, self.config.token().ok())?);
        }

        self.client
            .as_ref()
            .ok_or_else(|| anyhow!("Failed to construct client"))
    }

    pub fn printer(&self) -> Result<&Printer> {
        self.printer
            .as_ref()
            .ok_or_else(|| anyhow!("No printer configured"))
    }
}

#[derive(Debug, Default)]
struct Tree<'a> {
    children: HashMap<&'a str, Tree<'a>>,
    cmd: Option<CliCommand>,
}

impl<'a> Tree<'a> {
    fn cmd(&self, name: &str) -> Command {
        let mut cmd = if let Some(op) = self.cmd {
            // Command node
            // TODO
            Cli::<Context>::get_command(op).name(name.to_owned())
        } else {
            // Internal node
            Command::new(name.to_owned()).subcommand_required(true)
        };

        for (sub_name, sub_tree) in self.children.iter() {
            cmd = cmd.subcommand(sub_tree.cmd(sub_name));
        }

        cmd
    }
}

fn cmd_path<'a>(cmd: &CliCommand) -> Option<&'a str> {
    match cmd {
        // RFD commands
        CliCommand::GetRfd => Some("view"),
        CliCommand::GetRfdMeta => Some("meta"),
        CliCommand::GetRfds => Some("list"),
        CliCommand::GetRfdAttr => Some("attr"),
        CliCommand::SearchRfds => Some("search"),
        CliCommand::ReserveRfd => Some("reserve"),

        CliCommand::SetRfdAttr => Some("edit attr"),
        CliCommand::SetRfdContent => Some("edit content"),
        CliCommand::SetRfdDocument => Some("edit document"),
        CliCommand::UpdateRfdVisibility => Some("edit visibility"),
        CliCommand::PublishRfd => Some("edit publish"),
        CliCommand::DiscussRfd => Some("edit discuss"),

        // User commands
        CliCommand::CreateApiUser => Some("sys user create"),
        CliCommand::CreateApiUserToken => Some("sys user token create"),
        CliCommand::DeleteApiUserToken => Some("sys user token delete"),
        CliCommand::GetApiUser => Some("sys user get"),
        CliCommand::GetApiUserToken => Some("sys user token get"),
        CliCommand::ListApiUserTokens => Some("sys user token list"),
        CliCommand::UpdateApiUser => Some("sys user update"),
        CliCommand::GetSelf => Some("self"),

        // Set user email
        CliCommand::SetApiUserContactEmail => Some("sys user contact email set"),

        // Link commands are handled separately
        CliCommand::CreateLinkToken => None,
        CliCommand::LinkProvider => None,

        // Group commands
        CliCommand::GetGroups => Some("sys group list"),
        CliCommand::CreateGroup => Some("sys group create"),
        CliCommand::UpdateGroup => Some("sys group update"),
        CliCommand::DeleteGroup => Some("sys group delete"),

        // User admin commands
        CliCommand::AddApiUserToGroup => Some("sys group membership add"),
        CliCommand::RemoveApiUserFromGroup => Some("sys group membership remove"),

        // Mapper commands
        CliCommand::GetMappers => Some("sys mapper list"),
        CliCommand::CreateMapper => Some("sys mapper create"),
        CliCommand::DeleteMapper => Some("sys mapper delete"),

        // OAuth client commands
        CliCommand::ListOauthClients => Some("sys oauth list"),
        CliCommand::CreateOauthClient => Some("sys oauth create"),
        CliCommand::GetOauthClient => Some("sys oauth get"),
        CliCommand::CreateOauthClientRedirectUri => Some("sys oauth redirect create"),
        CliCommand::DeleteOauthClientRedirectUri => Some("sys oauth redirect delete"),
        CliCommand::CreateOauthClientSecret => Some("sys oauth secret create"),
        CliCommand::DeleteOauthClientSecret => Some("sys oauth secret delete"),

        // Magic link client commands
        CliCommand::ListMagicLinks => Some("sys mlink list"),
        CliCommand::CreateMagicLink => Some("sys mlink create"),
        CliCommand::GetMagicLink => Some("sys mlink get"),
        CliCommand::CreateMagicLinkRedirectUri => Some("sys mlink redirect create"),
        CliCommand::DeleteMagicLinkRedirectUri => Some("sys mlink redirect delete"),
        CliCommand::CreateMagicLinkSecret => Some("sys mlink secret create"),
        CliCommand::DeleteMagicLinkSecret => Some("sys mlink secret delete"),

        // Authentication is handled separately
        CliCommand::ExchangeDeviceToken => None,
        CliCommand::GetDeviceProvider => None,
        CliCommand::MagicLinkSend => None,
        CliCommand::MagicLinkExchange => None,
        #[cfg(feature = "local-dev")]
        CliCommand::LocalLogin => None,

        // Unsupported commands
        CliCommand::AuthzCodeRedirect => None,
        CliCommand::AuthzCodeCallback => None,
        CliCommand::AuthzCodeExchange => None,
        CliCommand::GithubWebhook => None,
        CliCommand::OpenidConfiguration => None,
        CliCommand::JwksJson => None,
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut root = Tree::default();

    for cmd in CliCommand::iter() {
        if let Some(path) = cmd_path(&cmd) {
            let mut node = &mut root;

            let mut parts = path.split(' ').peekable();
            while let Some(ss) = parts.next() {
                if parts.peek().is_some() {
                    node = node.children.entry(ss).or_default();
                } else {
                    assert!(
                        node.children.get(ss).is_none(),
                        "two identical subcommands {}",
                        path,
                    );
                    node.children.insert(
                        ss,
                        Tree {
                            cmd: Some(cmd),
                            ..Default::default()
                        },
                    );
                }
            }
        }
    }

    let mut cmd = root.cmd("rfd");
    cmd = cmd
        .bin_name("rfd")
        .arg(
            Arg::new("debug")
                .long("debug")
                .short('d')
                .help("Enable more verbose errors")
                .global(true)
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("format")
                .long("format")
                .short('f')
                .help("Specify the output format to display results in")
                .global(true)
                .value_parser(value_parser!(FormatStyle))
                .action(ArgAction::Set),
        );

    cmd = cmd.subcommand(cmd::auth::Auth::command());
    cmd = cmd.subcommand(cmd::config::ConfigCmd::command());
    cmd = cmd.subcommand(cmd::shortcut::ShortcutCmd::command());

    let mut ctx = Context::new()?;

    let matches = cmd.get_matches();

    if matches.try_get_one::<bool>("debug").ok().is_some() {
        ctx.verbosity = VerbosityLevel::All;
    }

    let format = matches
        .try_get_one::<FormatStyle>("format")
        .unwrap()
        .cloned()
        .unwrap_or_else(|| ctx.config.format_style());
    ctx.printer = Some(match format {
        FormatStyle::Json => Printer::Json(RfdJsonPrinter),
        FormatStyle::Tab => Printer::Tab(RfdTabPrinter::default()),
    });

    let mut node = &root;
    let mut sm = &matches;

    match matches.subcommand() {
        Some(("auth", sub_matches)) => {
            cmd::auth::Auth::from_arg_matches(sub_matches)
                .unwrap()
                .run(&mut ctx)
                .await?;
        }
        Some(("config", sub_matches)) => {
            cmd::config::ConfigCmd::from_arg_matches(sub_matches)
                .unwrap()
                .run(&mut ctx)
                .await?;
        }
        Some(("shortcut", sub_matches)) => {
            cmd::shortcut::ShortcutCmd::from_arg_matches(sub_matches)
                .unwrap()
                .run(&mut ctx)
                .await?;
        }
        _ => {
            while let Some((sub_name, sub_matches)) = sm.subcommand() {
                node = node.children.get(sub_name).unwrap();
                sm = sub_matches;
            }

            let cli = Cli::new(ctx.client()?.clone(), ctx);
            cli.execute(node.cmd.unwrap(), sm).await;
        }
    };

    Ok(())
}

pub fn reserialize<T, U>(value: &T) -> U
where
    T: Serialize + Debug,
    U: DeserializeOwned,
{
    serde_json::from_str::<U>(&serde_json::to_string::<T>(value).unwrap()).unwrap()
}

impl ProgenitorCliConfig for Context {
    fn success_item<T>(&self, value: &progenitor_client::ResponseValue<T>)
    where
        T: schemars::JsonSchema + serde::Serialize + std::fmt::Debug,
    {
        self.list_item(value.as_ref())
    }

    fn success_no_item(&self, value: &rfd_sdk::ResponseValue<()>) {
        self.list_item(value.as_ref())
    }

    fn error<T>(&self, value: &progenitor_client::Error<T>)
    where
        T: schemars::JsonSchema + serde::Serialize + std::fmt::Debug,
    {
        self.printer().unwrap().output_error(value)
    }

    fn list_start<T>(&self)
    where
        T: schemars::JsonSchema + serde::Serialize + std::fmt::Debug,
    {
    }

    fn list_item<T>(&self, value: &T)
    where
        T: schemars::JsonSchema + serde::Serialize + std::fmt::Debug,
    {
        match T::schema_name().as_str() {
            "ApiUserForApiPermissionResponse" => {
                self.printer().unwrap().output_api_user(reserialize(value))
            }
            "GetUserResponse" => self.printer().unwrap().output_user(reserialize(value)),
            "Vec<ApiKeyResponse>" => self
                .printer()
                .unwrap()
                .output_api_key_list(reserialize(value)),
            "InitialApiKeyResponse" => self
                .printer()
                .unwrap()
                .output_api_key_initial(reserialize(value)),
            "ApiKeyResponse" => self.printer().unwrap().output_api_key(reserialize(value)),
            "Array_of_AccessGroupForApiPermissionResponse" => self
                .printer()
                .unwrap()
                .output_group_list(reserialize(value)),
            "AccessGroupForApiPermissionResponse" => {
                self.printer().unwrap().output_api_user(reserialize(value))
            }
            "Array_of_AccessGroupForRfdPermission" => self
                .printer()
                .unwrap()
                .output_group_list(reserialize(value)),
            "Array_of_Mapper" => self
                .printer()
                .unwrap()
                .output_mapper_list(reserialize(value)),
            "Mapper" => self.printer().unwrap().output_mapper(reserialize(value)),
            "Array_of_OAuthClient" => self
                .printer()
                .unwrap()
                .output_oauth_client_list(reserialize(value)),
            "OAuthClient" => self
                .printer()
                .unwrap()
                .output_oauth_client(reserialize(value)),
            "OAuthClientRedirectUri" => self
                .printer()
                .unwrap()
                .output_oauth_redirect_uri(reserialize(value)),
            "InitialOAuthClientSecretResponse" => self
                .printer()
                .unwrap()
                .output_oauth_secret_initial(reserialize(value)),
            "OAuthClientSecret" => self
                .printer()
                .unwrap()
                .output_oauth_secret(reserialize(value)),
            "Array_of_ListRfd" => self.printer().unwrap().output_rfd_list(reserialize(value)),
            "FullRfd" => self.printer().unwrap().output_rfd_full(reserialize(value)),
            "Rfd" => self.printer().unwrap().output_rfd(reserialize(value)),
            "SearchResults" => self
                .printer()
                .unwrap()
                .output_search_results(reserialize(value)),
            "RfdAttr" => self.printer().unwrap().output_rfd_attr(reserialize(value)),
            "ReserveRfdResponse" => self
                .printer()
                .unwrap()
                .output_reserved_rfd(reserialize(value)),
            other => eprintln!(
                "Unhandled response type: {}. Please report this as a bug.",
                other
            ),
        }
    }

    fn list_end_success<T>(&self)
    where
        T: schemars::JsonSchema + serde::Serialize + std::fmt::Debug,
    {
    }

    fn list_end_error<T>(&self, value: &progenitor_client::Error<T>)
    where
        T: schemars::JsonSchema + serde::Serialize + std::fmt::Debug,
    {
    }
}
