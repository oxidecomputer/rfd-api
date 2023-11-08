#![allow(unused)]

use anyhow::{anyhow, Result};
use clap::{Arg, ArgAction, Command, CommandFactory, FromArgMatches};
use generated::cli::*;
use printer::{Printer, RfdJsonPrinter, RfdTabPrinter};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use rfd_sdk::types::MappingRules;
use rfd_sdk::Client;
use std::time::Duration;
use std::{collections::HashMap, error::Error};
use store::CliConfig;

mod auth;
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

#[derive(Debug)]
pub struct Context {
    config: CliConfig,
    client: Option<Client>,
    verbosity: VerbosityLevel,
}

impl Context {
    pub fn new() -> Result<Self> {
        let config = CliConfig::new()?;

        Ok(Self {
            config,
            client: None,
            verbosity: VerbosityLevel::None,
        })
    }

    pub fn new_client(token: Result<&str>, host: &str) -> Result<Client> {
        let mut default_headers = HeaderMap::new();

        if let Ok(token) = token {
            let mut auth_header = HeaderValue::from_str(&format!("Bearer {}", token))?;
            auth_header.set_sensitive(true);
            default_headers.insert(AUTHORIZATION, auth_header);
        }

        let http_client = reqwest::Client::builder()
            .default_headers(default_headers)
            .connect_timeout(Duration::from_secs(5))
            .timeout(Duration::from_secs(10))
            .build()?;

        Ok(Client::new_with_client(host, http_client))
    }

    pub fn client(&mut self) -> Result<&Client> {
        if self.client.is_none() {
            self.client = Some(Self::new_client(self.config.token(), self.config.host()?)?);
        }

        self.client
            .as_ref()
            .ok_or_else(|| anyhow!("Failed to construct client"))
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
            Cli::get_command(op).name(name.to_owned())
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
        // User commands
        CliCommand::CreateApiUser => Some("user create"),
        CliCommand::CreateApiUserToken => Some("user token create"),
        CliCommand::DeleteApiUserToken => Some("user token delete"),
        CliCommand::GetApiUser => Some("user get"),
        CliCommand::GetApiUserToken => Some("user token get"),
        CliCommand::ListApiUserTokens => Some("user token list"),
        CliCommand::UpdateApiUser => Some("user update"),
        CliCommand::GetSelf => Some("user self"),

        // RFD commands
        CliCommand::GetRfd => Some("view"),
        CliCommand::GetRfds => Some("list"),
        CliCommand::SearchRfds => Some("search"),

        // Link commands are handled separately
        CliCommand::CreateLinkToken => None,
        CliCommand::LinkProvider => None,

        // Group commands
        CliCommand::GetGroups => Some("group list"),
        CliCommand::CreateGroup => Some("group create"),
        CliCommand::UpdateGroup => Some("group update"),
        CliCommand::DeleteGroup => Some("group delete"),

        // User admin commands
        CliCommand::AddApiUserToGroup => Some("group membership add"),
        CliCommand::RemoveApiUserFromGroup => Some("group membership remove"),

        // Mapper commands
        CliCommand::GetMappers => Some("mapper list"),
        CliCommand::CreateMapper => Some("mapper create"),
        CliCommand::DeleteMapper => Some("mapper delete"),

        // OAuth client commands
        CliCommand::ListOauthClients => Some("oauth list"),
        CliCommand::CreateOauthClient => Some("oauth create"),
        CliCommand::GetOauthClient => Some("oauth get"),
        CliCommand::CreateOauthClientRedirectUri => Some("oauth redirect create"),
        CliCommand::DeleteOauthClientRedirectUri => Some("oauth redirect delete"),
        CliCommand::CreateOauthClientSecret => Some("oauth secret create"),
        CliCommand::DeleteOauthClientSecret => Some("oauth secret delete"),

        // Authentication is handled separately
        CliCommand::ExchangeDeviceToken => None,
        CliCommand::GetDeviceProvider => None,

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
                .value_parser(["json", "tab"])
                .default_value("json")
                .action(ArgAction::Set),
        );

    cmd = cmd.subcommand(cmd::config::ConfigCmd::command());
    cmd = cmd.subcommand(auth::Auth::command());

    let mut ctx = Context::new()?;

    let matches = cmd.get_matches();

    if matches.try_get_one::<bool>("debug").ok().is_some() {
        ctx.verbosity = VerbosityLevel::All;
    }

    let format = matches.try_get_one::<String>("format").unwrap().unwrap();
    let printer = match format.as_str() {
        "json" => Printer::Json(RfdJsonPrinter),
        "tab" => Printer::Tab(RfdTabPrinter::default()),
        other => panic!("Unknown format {}", other),
    };

    let mut node = &root;
    let mut sm = &matches;

    match matches.subcommand() {
        Some(("config", sub_matches)) => {
            cmd::config::ConfigCmd::from_arg_matches(sub_matches)
                .unwrap()
                .run(&mut ctx)
                .await?;
        }
        Some(("auth", sub_matches)) => {
            let _ = auth::Auth::from_arg_matches(sub_matches)
                .unwrap()
                .run(&mut ctx)
                .await?;
        }
        _ => {
            while let Some((sub_name, sub_matches)) = sm.subcommand() {
                node = node.children.get(sub_name).unwrap();
                sm = sub_matches;
            }

            let cli = Cli::new_with_override(ctx.client()?.clone(), (), printer);
            cli.execute(node.cmd.unwrap(), sm).await;
        }
    };

    Ok(())
}
