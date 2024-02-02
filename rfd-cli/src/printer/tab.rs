// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use itertools::{EitherOrBoth, Itertools};
use owo_colors::{OwoColorize, Style};
use rfd_sdk::types::{
    self, AccessGroupForApiPermissionResponse, ApiKeyResponse, ApiPermission,
    ApiUserForApiPermissionResponse, Error, FullRfd, FullRfdPdfEntry, GetUserResponse,
    InitialApiKeyResponse, InitialOAuthClientSecretResponse, ListRfd, Mapper, OAuthClient,
    OAuthClientRedirectUri, OAuthClientSecret, PermissionsForApiPermissionResponse,
    SearchResultHit, SearchResults, Visibility,
};
use std::{collections::HashMap, fmt::Display, fs::File, io::Write, process::Command};
use tabwriter::TabWriter;

use crate::generated::cli::CliOutput;

#[derive(Debug, Clone)]
pub struct Styles {
    label: Style,
    value: Style,
}

impl Default for Styles {
    fn default() -> Self {
        Styles {
            label: Style::new().bold(),
            value: Style::new(),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct RfdTabPrinter {
    stylesheet: Styles,
}

impl CliOutput for RfdTabPrinter {
    fn output_create_api_user(
        &self,
        response: Result<
            types::ApiUserForApiPermissionResponse,
            progenitor_client::Error<types::Error>,
        >,
    ) {
        self.print_cli_output(&response, None);
    }

    fn output_get_api_user(
        &self,
        response: Result<types::GetUserResponse, progenitor_client::Error<types::Error>>,
    ) {
        let mut tw = TabWriter::new(vec![]).ansi(true);

        match response {
            Ok(response) => {
                response.info.display(&mut tw, 0, self);

                self.print_field(&mut tw, 0, "providers", &"");
                for provider in response.providers {
                    self.print_field(&mut tw, 1, "id", &provider.id);
                    self.print_field(&mut tw, 1, "provider", &provider.provider);
                    self.print_field(&mut tw, 1, "provider_id", &provider.provider_id);
                    self.print_list(&mut tw, 1, "emails", &provider.emails);
                    self.print_field(&mut tw, 1, "created_at", &provider.created_at);
                    self.print_field(&mut tw, 1, "updated_at", &provider.updated_at);
                    self.print_field(
                        &mut tw,
                        1,
                        "deleted_at",
                        &response
                            .info
                            .deleted_at
                            .map(|d| d.to_string())
                            .unwrap_or_else(|| "--".to_string()),
                    );
                }
            }
            Err(err) => {
                self.print_error(&mut tw, &err);
            }
        }

        output_writer(tw);
    }

    fn output_update_api_user(
        &self,
        response: Result<
            types::ApiUserForApiPermissionResponse,
            progenitor_client::Error<types::Error>,
        >,
    ) {
        self.print_cli_output(&response, None);
    }

    fn output_add_api_user_to_group(
        &self,
        response: Result<
            types::ApiUserForApiPermissionResponse,
            progenitor_client::Error<types::Error>,
        >,
    ) {
        self.print_cli_output(&response, None);
    }

    fn output_remove_api_user_from_group(
        &self,
        response: Result<
            types::ApiUserForApiPermissionResponse,
            progenitor_client::Error<types::Error>,
        >,
    ) {
        self.print_cli_output(&response, None);
    }

    fn output_list_api_user_tokens(
        &self,
        response: Result<Vec<types::ApiKeyResponse>, progenitor_client::Error<types::Error>>,
    ) {
        self.print_cli_output(&response, Some("providers".to_string()));
    }

    fn output_create_api_user_token(
        &self,
        response: Result<types::InitialApiKeyResponse, progenitor_client::Error<types::Error>>,
    ) {
        self.print_cli_output(&response, None);
    }

    fn output_get_api_user_token(
        &self,
        response: Result<types::ApiKeyResponse, progenitor_client::Error<types::Error>>,
    ) {
        self.print_cli_output(&response, None);
    }

    fn output_delete_api_user_token(
        &self,
        response: Result<types::ApiKeyResponse, progenitor_client::Error<types::Error>>,
    ) {
        self.print_cli_output(&response, None);
    }

    fn output_github_webhook(&self, response: Result<(), progenitor_client::Error<types::Error>>) {}

    fn output_get_groups(
        &self,
        response: Result<
            Vec<types::AccessGroupForApiPermissionResponse>,
            progenitor_client::Error<types::Error>,
        >,
    ) {
        self.print_cli_output(&response, Some("groups".to_string()));
    }

    fn output_create_group(
        &self,
        response: Result<
            types::AccessGroupForApiPermissionResponse,
            progenitor_client::Error<types::Error>,
        >,
    ) {
        self.print_cli_output(&response, None);
    }

    fn output_update_group(
        &self,
        response: Result<
            types::AccessGroupForApiPermissionResponse,
            progenitor_client::Error<types::Error>,
        >,
    ) {
        self.print_cli_output(&response, None);
    }

    fn output_delete_group(
        &self,
        response: Result<
            types::AccessGroupForApiPermissionResponse,
            progenitor_client::Error<types::Error>,
        >,
    ) {
        self.print_cli_output(&response, None);
    }

    fn output_get_mappers(
        &self,
        response: Result<Vec<types::Mapper>, progenitor_client::Error<types::Error>>,
    ) {
        self.print_cli_output(&response, Some("mappers".to_string()));
    }

    fn output_create_mapper(
        &self,
        response: Result<types::Mapper, progenitor_client::Error<types::Error>>,
    ) {
        self.print_cli_output(&response, None);
    }

    fn output_delete_mapper(
        &self,
        response: Result<types::Mapper, progenitor_client::Error<types::Error>>,
    ) {
        self.print_cli_output(&response, None);
    }

    fn output_list_oauth_clients(
        &self,
        response: Result<Vec<types::OAuthClient>, progenitor_client::Error<types::Error>>,
    ) {
        self.print_cli_output(&response, Some("clients".to_string()));
    }

    fn output_create_oauth_client(
        &self,
        response: Result<types::OAuthClient, progenitor_client::Error<types::Error>>,
    ) {
        self.print_cli_output(&response, None);
    }

    fn output_get_oauth_client(
        &self,
        response: Result<types::OAuthClient, progenitor_client::Error<types::Error>>,
    ) {
        self.print_cli_output(&response, None);
    }

    fn output_create_oauth_client_redirect_uri(
        &self,
        response: Result<types::OAuthClientRedirectUri, progenitor_client::Error<types::Error>>,
    ) {
        self.print_cli_output(&response, None);
    }

    fn output_delete_oauth_client_redirect_uri(
        &self,
        response: Result<types::OAuthClientRedirectUri, progenitor_client::Error<types::Error>>,
    ) {
        self.print_cli_output(&response, None);
    }

    fn output_create_oauth_client_secret(
        &self,
        response: Result<
            types::InitialOAuthClientSecretResponse,
            progenitor_client::Error<types::Error>,
        >,
    ) {
        self.print_cli_output(&response, None);
    }

    fn output_delete_oauth_client_secret(
        &self,
        response: Result<types::OAuthClientSecret, progenitor_client::Error<types::Error>>,
    ) {
        self.print_cli_output(&response, None);
    }

    fn output_get_rfds(
        &self,
        response: Result<Vec<types::ListRfd>, progenitor_client::Error<types::Error>>,
    ) {
        self.print_cli_output(&response, Some("rfds".to_string()));
    }

    fn output_get_rfd(
        &self,
        response: Result<types::FullRfd, progenitor_client::Error<types::Error>>,
    ) {
        self.print_cli_output(&response, None);
    }

    fn output_search_rfds(
        &self,
        response: Result<types::SearchResults, progenitor_client::Error<types::Error>>,
    ) {
        self.print_cli_output(&response, Some("results".to_string()));
    }

    fn output_get_self(
        &self,
        response: Result<types::GetUserResponse, progenitor_client::Error<types::Error>>,
    ) {
        self.output_get_api_user(response);
    }
}

trait TabDisplay {
    fn display(&self, tw: &mut TabWriter<Vec<u8>>, level: u8, printer: &RfdTabPrinter);
}

impl TabDisplay for ApiUserForApiPermissionResponse {
    fn display(&self, tw: &mut TabWriter<Vec<u8>>, level: u8, printer: &RfdTabPrinter) {
        printer.print_field(tw, level, "id", &self.id);
        printer.print_list(
            tw,
            level,
            "permissions",
            &Vec::from(self.permissions.clone())
                .iter()
                .map(|p| p.to_string())
                .collect_vec(),
        );
        printer.print_list(
            tw,
            level,
            "groups",
            &self.groups.iter().map(|g| g.to_string()).collect_vec(),
        );
        printer.print_field(tw, level, "created_at", &self.created_at);
        printer.print_field(tw, level, "updated_at", &self.updated_at);
        printer.print_field(
            tw,
            level,
            "deleted_at",
            &self
                .deleted_at
                .map(|d| d.to_string())
                .unwrap_or_else(|| "--".to_string()),
        );
    }
}

impl TabDisplay for ApiKeyResponse {
    fn display(&self, tw: &mut TabWriter<Vec<u8>>, level: u8, printer: &RfdTabPrinter) {
        printer.print_field(tw, level, "id", &self.id);
        printer.print_list(
            tw,
            level,
            "permissions",
            &Vec::from(
                self.permissions
                    .as_ref()
                    .cloned()
                    .unwrap_or_else(|| PermissionsForApiPermissionResponse(vec![])),
            )
            .iter()
            .map(|p| p.to_string())
            .collect_vec(),
        );
        printer.print_field(tw, level, "created_at", &self.created_at);
    }
}

impl TabDisplay for InitialApiKeyResponse {
    fn display(&self, tw: &mut TabWriter<Vec<u8>>, level: u8, printer: &RfdTabPrinter) {
        printer.print_field(tw, level, "id", &self.id);
        printer.print_field(tw, level, "key", &self.key.0);
        printer.print_list(
            tw,
            level,
            "permissions",
            &Vec::from(
                self.permissions
                    .as_ref()
                    .cloned()
                    .unwrap_or_else(|| PermissionsForApiPermissionResponse(vec![])),
            )
            .iter()
            .map(|p| p.to_string())
            .collect_vec(),
        );
        printer.print_field(tw, level, "created_at", &self.created_at);
    }
}

impl TabDisplay for AccessGroupForApiPermissionResponse {
    fn display(&self, tw: &mut TabWriter<Vec<u8>>, level: u8, printer: &RfdTabPrinter) {
        printer.print_field(tw, level, "id", &self.id);
        printer.print_field(tw, level, "name", &self.name);
        printer.print_list(
            tw,
            level,
            "permissions",
            &Vec::from(self.permissions.clone())
                .iter()
                .map(|p| p.to_string())
                .collect_vec(),
        );
        printer.print_field(tw, level, "created_at", &self.created_at);
        printer.print_field(tw, level, "updated_at", &self.updated_at);
        printer.print_field(
            tw,
            level,
            "deleted_at",
            &self
                .deleted_at
                .map(|d| d.to_string())
                .unwrap_or_else(|| "--".to_string()),
        );
    }
}

impl TabDisplay for Mapper {
    fn display(&self, tw: &mut TabWriter<Vec<u8>>, level: u8, printer: &RfdTabPrinter) {
        printer.print_field(tw, level, "id", &self.id);
        printer.print_field(tw, level, "name", &self.name);
        printer.print_field(
            tw,
            level,
            "activations",
            &self
                .activations
                .map(|i| i.to_string())
                .unwrap_or_else(|| "--".to_string()),
        );
        printer.print_field(
            tw,
            level,
            "max_activations",
            &self
                .max_activations
                .map(|i| i.to_string())
                .unwrap_or_else(|| "--".to_string()),
        );
        printer.print_field(
            tw,
            level,
            "rule",
            &serde_json::to_string(&self.rule).unwrap(),
        );
        printer.print_field(tw, level, "created_at", &self.created_at);
        printer.print_field(
            tw,
            level,
            "depleted_at",
            &self
                .depleted_at
                .map(|d| d.to_string())
                .unwrap_or_else(|| "--".to_string()),
        );
        printer.print_field(
            tw,
            level,
            "deleted_at",
            &self
                .deleted_at
                .map(|d| d.to_string())
                .unwrap_or_else(|| "--".to_string()),
        );
    }
}

impl TabDisplay for OAuthClient {
    fn display(&self, tw: &mut TabWriter<Vec<u8>>, level: u8, printer: &RfdTabPrinter) {
        printer.print_field(tw, level, "id", &self.id);
        printer.print_field(tw, level, "secrets", &"");
        self.secrets.display(tw, level + 1, printer);
        printer.print_field(tw, level, "redirect_uris", &"");
        self.redirect_uris.display(tw, level + 1, printer);
        printer.print_field(tw, level, "created_at", &self.created_at);
        printer.print_field(
            tw,
            level,
            "deleted_at",
            &self
                .deleted_at
                .map(|d| d.to_string())
                .unwrap_or_else(|| "--".to_string()),
        );
    }
}

impl TabDisplay for OAuthClientRedirectUri {
    fn display(&self, tw: &mut TabWriter<Vec<u8>>, level: u8, printer: &RfdTabPrinter) {
        printer.print_field(tw, level, "id", &self.id);
        printer.print_field(tw, level, "oauth_client_id", &self.oauth_client_id);
        printer.print_field(tw, level, "redirect_uri", &self.redirect_uri);
        printer.print_field(tw, level, "created_at", &self.created_at);
        printer.print_field(
            tw,
            level,
            "deleted_at",
            &self
                .deleted_at
                .map(|d| d.to_string())
                .unwrap_or_else(|| "--".to_string()),
        );
    }
}

impl TabDisplay for OAuthClientSecret {
    fn display(&self, tw: &mut TabWriter<Vec<u8>>, level: u8, printer: &RfdTabPrinter) {
        printer.print_field(tw, level, "id", &self.id);
        printer.print_field(tw, level, "oauth_client_id", &self.oauth_client_id);
        printer.print_field(tw, level, "secret_signature", &self.secret_signature);
        printer.print_field(tw, level, "created_at", &self.created_at);
        printer.print_field(
            tw,
            level,
            "deleted_at",
            &self
                .deleted_at
                .map(|d| d.to_string())
                .unwrap_or_else(|| "--".to_string()),
        );
    }
}

impl TabDisplay for InitialOAuthClientSecretResponse {
    fn display(&self, tw: &mut TabWriter<Vec<u8>>, level: u8, printer: &RfdTabPrinter) {
        printer.print_field(tw, level, "id", &self.id);
        printer.print_field(tw, level, "key", &self.key.0);
        printer.print_field(tw, level, "created_at", &self.created_at);
    }
}

impl TabDisplay for ListRfd {
    fn display(&self, tw: &mut TabWriter<Vec<u8>>, level: u8, printer: &RfdTabPrinter) {
        printer.print_field(tw, level, "id", &self.id);
        printer.print_field(tw, level, "rfd_number", &self.rfd_number);
        printer.print_field(tw, level, "title", &self.title);
        printer.print_field(
            tw,
            level,
            "state",
            &self.state.as_ref().map(|s| s.as_str()).unwrap_or(""),
        );
        printer.print_field(
            tw,
            level,
            "visibility",
            match self.visibility {
                Visibility::Private => &"private",
                Visibility::Public => &"public",
            },
        );
        printer.print_field(
            tw,
            level,
            "authors",
            &self.authors.as_ref().map(|s| s.as_str()).unwrap_or(""),
        );
        printer.print_field(
            tw,
            level,
            "link",
            &self.link.as_ref().map(|s| s.as_str()).unwrap_or(""),
        );
        printer.print_field(
            tw,
            level,
            "discussion",
            &self.discussion.as_ref().map(|s| s.as_str()).unwrap_or(""),
        );
        printer.print_field(tw, level, "sha", &self.sha);
        printer.print_field(tw, level, "commit", &self.commit);
        printer.print_field(tw, level, "committed_at", &self.committed_at);
    }
}

impl TabDisplay for FullRfd {
    fn display(&self, tw: &mut TabWriter<Vec<u8>>, level: u8, printer: &RfdTabPrinter) {
        printer.print_field(tw, level, "id", &self.id);
        printer.print_field(tw, level, "rfd_number", &self.rfd_number);
        printer.print_field(tw, level, "title", &self.title);
        printer.print_field(
            tw,
            level,
            "state",
            &self.state.as_ref().map(|s| s.as_str()).unwrap_or(""),
        );
        printer.print_field(
            tw,
            level,
            "visibility",
            match self.visibility {
                Visibility::Private => &"private",
                Visibility::Public => &"public",
            },
        );
        printer.print_field(
            tw,
            level,
            "authors",
            &self.authors.as_ref().map(|s| s.as_str()).unwrap_or(""),
        );
        printer.print_field(
            tw,
            level,
            "link",
            &self.link.as_ref().map(|s| s.as_str()).unwrap_or(""),
        );
        printer.print_field(
            tw,
            level,
            "discussion",
            &self.discussion.as_ref().map(|s| s.as_str()).unwrap_or(""),
        );
        printer.print_field(tw, level, "pdfs", &"");
        self.pdfs.display(tw, level + 1, printer);
        printer.print_field(tw, level, "sha", &self.sha);
        printer.print_field(tw, level, "commit", &self.commit);
        printer.print_field(tw, level, "committed_at", &self.committed_at);
        writeln!(tw, "");
        writeln!(tw, "{}", self.content);
    }
}

impl TabDisplay for FullRfdPdfEntry {
    fn display(&self, tw: &mut TabWriter<Vec<u8>>, level: u8, printer: &RfdTabPrinter) {
        printer.print_field(tw, level, "link", &self.link);
        printer.print_field(tw, level, "source", &self.source);
    }
}

impl TabDisplay for SearchResults {
    fn display(&self, tw: &mut TabWriter<Vec<u8>>, level: u8, printer: &RfdTabPrinter) {
        printer.print_field(tw, level, "query", &self.query);
        printer.print_field(tw, level, "total hits", &self.hits.len());
        writeln!(tw, "");
        self.hits.display(tw, level, printer);
    }
}

impl TabDisplay for SearchResultHit {
    fn display(&self, tw: &mut TabWriter<Vec<u8>>, level: u8, printer: &RfdTabPrinter) {
        let mut heading_path = Vec::new();

        if let Some(lvl1) = &self.hierarchy[0] {
            heading_path.push(lvl1);
        }
        if let Some(lvl2) = &self.hierarchy[1] {
            heading_path.push(lvl2);
        }
        if let Some(lvl3) = &self.hierarchy[2] {
            heading_path.push(lvl3);
        }
        if let Some(lvl4) = &self.hierarchy[3] {
            heading_path.push(lvl4);
        }
        if let Some(lvl5) = &self.hierarchy[4] {
            heading_path.push(lvl5);
        }

        printer.print_field(tw, level, "rfd", &self.rfd_number.to_string());

        if let Some(lvl0) = &self.hierarchy_radio[0] {
            printer.print_field(tw, level, "title", lvl0);
        }

        if let Some(url) = &self.url {
            printer.print_field(tw, level, "url", url);
        }

        printer.print_field(tw, level, "location", &heading_path.iter().join(" > "));

        fn get_slice_of_string(s: &str, n: usize) -> &str {
            let char_boundary = s.char_indices().nth(n).map_or(s.len(), |(idx, _)| idx);
            &s[..char_boundary]
        }

        printer.print_field(tw, level, "content", &format!("{}...", get_slice_of_string(&self.content, 255)));
    }
}

impl<T> TabDisplay for Vec<T>
where
    T: TabDisplay,
{
    fn display(&self, tw: &mut TabWriter<Vec<u8>>, level: u8, printer: &RfdTabPrinter) {
        for entry in self {
            entry.display(tw, level, printer);
            writeln!(tw, "");
        }
    }
}

impl RfdTabPrinter {
    fn print_cli_output<T>(
        &self,
        response: &Result<T, progenitor_client::Error<types::Error>>,
        heading: Option<String>,
    ) where
        T: TabDisplay,
    {
        let mut tw = TabWriter::new(vec![]).ansi(true);

        match response {
            Ok(response) => {
                if let Some(heading) = &heading {
                    self.print_field(&mut tw, 0, heading, &"");
                }

                response.display(&mut tw, if heading.is_some() { 1 } else { 0 }, self);
            }
            Err(err) => self.print_error(&mut tw, err),
        }

        output_writer(tw);
    }

    fn print_field<T>(&self, tw: &mut TabWriter<Vec<u8>>, level: u8, key: &str, value: &T)
    where
        T: Display,
    {
        writeln!(
            tw,
            "{}{}:\t{}",
            prefix(level),
            key.style(self.stylesheet.label),
            value.style(self.stylesheet.value)
        );
    }

    fn print_list<T>(&self, tw: &mut TabWriter<Vec<u8>>, level: u8, key: &str, list: &Vec<T>)
    where
        T: Display,
    {
        let pre = prefix(level);
        for (i, entry) in list.iter().enumerate() {
            writeln!(
                tw,
                "{}{}{}\t{}",
                pre,
                if i == 0 { key } else { "" }.style(self.stylesheet.label),
                if i == 0 { ":" } else { "" },
                entry.style(self.stylesheet.value)
            );
        }
    }

    fn print_error(&self, tw: &mut TabWriter<Vec<u8>>, error: &progenitor_client::Error<Error>) {
        match error {
            progenitor_client::Error::CommunicationError(err) => {
                writeln!(tw, "Failed to reach the API server");
                writeln!(tw, "{:#?}", err);
            }
            progenitor_client::Error::ErrorResponse(err) => {
                writeln!(tw, "Received error from the remote API",);
                writeln!(tw, "Message\t{}", err.message);
                if let Some(code) = &err.error_code {
                    writeln!(tw, "Code\t{}", code);
                }
                writeln!(tw, "Request\t{}", err.request_id);
            }
            progenitor_client::Error::InvalidRequest(err) => {
                writeln!(tw, "Internal CLI error");
                writeln!(tw, "{:?}", err);
                writeln!(tw, "Please report this as a bug against the rfd-api");
            }
            progenitor_client::Error::InvalidResponsePayload(_, err) => {
                writeln!(tw, "Internal CLI error");
                writeln!(tw, "{:?}", err);
                writeln!(tw, "Please report this as a bug against the rfd-api");
            }
            progenitor_client::Error::UnexpectedResponse(err) => {
                writeln!(tw, "Internal CLI error");
                writeln!(tw, "{:?}", err);
                writeln!(tw, "Please report this as a bug against the rfd-api",);
            }
            progenitor_client::Error::ResponseBodyError(err) => {
                writeln!(tw, "Internal CLI error");
                writeln!(tw, "{:?}", err);
                writeln!(tw, "Please report this as a bug against the rfd-api");
            }
            progenitor_client::Error::InvalidUpgrade(err) => {
                writeln!(tw, "Internal CLI error");
                writeln!(tw, "{:?}", err);
                writeln!(tw, "Please report this as a bug against the rfd-api");
            }
        }
    }
}

fn prefix(level: u8) -> String {
    "    ".repeat(level as usize)
}

fn output_writer(mut tw: TabWriter<Vec<u8>>) {
    tw.flush().unwrap();
    let written = String::from_utf8(tw.into_inner().unwrap()).unwrap();
    println!("{}", written);
}

// fn state_color(state: &Option<String>) -> &'static str {
//     match state.as_deref() {
//         Some("published") => "\x1b[38;2;72;213;151m",
//         Some("committed") => "\x1b[38;2;72;213;151m",
//         Some("discussion") => "\x1b[38;2;139;161;255m",
//         Some("prediscussion") => "\x1b[38;2;163;128;203m",
//         Some("ideation") => "\x1b[38;2;245;185;68m",
//         Some("abandoned") => "\x1b[38;2;231;231;232m",
//         _ => "\x1b[38;2;231;231;232m",
//     }
// }
