// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use itertools::{EitherOrBoth, Itertools};
use owo_colors::{OwoColorize, Style};
use progenitor_client::ResponseValue;
use rfd_sdk::types::{
    self, AccessGroupForRfdPermission, ApiKeyResponseForRfdPermission, ApiUserContactEmail,
    ApiUserForRfdPermission, ApiUserLinkRequestResponse, Error, GetUserResponseForRfdPermission,
    InitialApiKeyResponseForRfdPermission, InitialMagicLinkSecretResponse,
    InitialOAuthClientSecretResponse, Job, Jwk, Jwks, MagicLink, MagicLinkExchangeResponse,
    MagicLinkRedirectUri, MagicLinkSecret, MagicLinkSendResponse, Mapper,
    OAuthAuthzCodeExchangeResponse, OAuthClient, OAuthClientRedirectUri, OAuthClientSecret,
    OAuthProviderInfo, OpenIdConfiguration, PermissionsForRfdPermission, ReserveRfdResponse,
    RfdAttr, RfdPdf, RfdRevisionMeta, RfdWithPdf, RfdWithRaw, RfdWithoutContent, SearchResultHit,
    SearchResults, Visibility,
};
use std::{collections::HashMap, fmt::Display, fs::File, io::Write, process::Command};
use tabwriter::TabWriter;

use crate::reserialize;

use super::CliOutput;

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
    fn output_api_user(&self, value: types::ApiUserForRfdPermission) {
        self.print_cli_output(&value, None);
    }

    fn output_api_user_list(&self, value: Vec<types::ApiUserForRfdPermission>) {
        self.print_cli_output(&value, Some("users".to_string()));
    }

    fn output_user(&self, value: types::GetUserResponseForRfdPermission) {
        self.print_cli_output(&value, None);
    }

    fn output_api_user_contact_email(&self, value: types::ApiUserContactEmail) {
        self.print_cli_output(&value, None);
    }

    fn output_api_user_link_request_response(&self, value: types::ApiUserLinkRequestResponse) {
        self.print_cli_output(&value, None);
    }

    fn output_api_key_list(&self, value: Vec<types::ApiKeyResponseForRfdPermission>) {
        self.print_cli_output(&value, Some("tokens".to_string()));
    }

    fn output_api_key_initial(&self, value: types::InitialApiKeyResponseForRfdPermission) {
        self.print_cli_output(&value, None);
    }

    fn output_api_key(&self, value: types::ApiKeyResponseForRfdPermission) {
        self.print_cli_output(&value, None);
    }

    fn output_group_list(&self, value: Vec<types::AccessGroupForRfdPermission>) {
        self.print_cli_output(&value, Some("groups".to_string()));
    }

    fn output_group(&self, value: types::AccessGroupForRfdPermission) {
        self.print_cli_output(&value, None);
    }

    fn output_mapper_list(&self, value: Vec<types::Mapper>) {
        self.print_cli_output(&value, Some("mappers".to_string()));
    }

    fn output_mapper(&self, value: types::Mapper) {
        self.print_cli_output(&value, None);
    }

    fn output_oauth_client_list(&self, value: Vec<types::OAuthClient>) {
        self.print_cli_output(&value, Some("clients".to_string()));
    }

    fn output_oauth_client(&self, value: types::OAuthClient) {
        self.print_cli_output(&value, None);
    }

    fn output_oauth_redirect_uri(&self, value: types::OAuthClientRedirectUri) {
        self.print_cli_output(&value, None);
    }

    fn output_oauth_secret_initial(&self, value: types::InitialOAuthClientSecretResponse) {
        self.print_cli_output(&value, None);
    }

    fn output_oauth_secret(&self, value: types::OAuthClientSecret) {
        self.print_cli_output(&value, None);
    }

    fn output_oauth_authz_code_exchange_response(
        &self,
        value: types::OAuthAuthzCodeExchangeResponse,
    ) {
        self.print_cli_output(&value, None);
    }

    fn output_oauth_provider_info(&self, value: types::OAuthProviderInfo) {
        self.print_cli_output(&value, None);
    }

    fn output_rfd_meta(&self, value: types::RfdWithoutContent) {
        self.print_cli_output(&value, None);
    }

    fn output_rfd_list(&self, value: Vec<types::RfdWithoutContent>) {
        self.print_cli_output(&value, Some("rfds".to_string()));
    }

    fn output_rfd_full(&self, value: types::RfdWithRaw) {
        self.print_cli_output(&value, None);
    }

    fn output_rfd_with_pdf(&self, value: types::RfdWithPdf) {
        self.print_cli_output(&value, None);
    }

    fn output_rfd(&self, value: types::Rfd) {
        self.print_cli_output(&value, None);
    }

    fn output_rfd_attr(&self, value: types::RfdAttr) {
        self.print_cli_output(&value, None);
    }

    fn output_rfd_revision_meta(&self, value: types::RfdRevisionMeta) {
        self.print_cli_output(&value, None);
    }

    fn output_rfd_revision_meta_list(&self, value: Vec<types::RfdRevisionMeta>) {
        self.print_cli_output(&value, Some("revisions".to_string()));
    }

    fn output_search_results(&self, value: types::SearchResults) {
        self.print_cli_output(&value, Some("results".to_string()));
    }

    fn output_reserved_rfd(&self, value: types::ReserveRfdResponse) {
        self.print_cli_output(&value, None);
    }

    fn output_job_list(&self, value: Vec<types::Job>) {
        self.print_cli_output(&value, Some("jobs".to_string()));
    }

    fn output_magic_link_client_list(&self, value: Vec<types::MagicLink>) {
        self.print_cli_output(&value, Some("clients".to_string()));
    }
    fn output_magic_link_client(&self, value: types::MagicLink) {
        self.print_cli_output(&value, None);
    }
    fn output_magic_link_redirect_uri(&self, value: types::MagicLinkRedirectUri) {
        self.print_cli_output(&value, None);
    }
    fn output_magic_link_secret_initial(&self, value: types::InitialMagicLinkSecretResponse) {
        self.print_cli_output(&value, None);
    }
    fn output_magic_link_secret(&self, value: types::MagicLinkSecret) {
        self.print_cli_output(&value, None);
    }

    fn output_magic_link_exchange_response(&self, value: types::MagicLinkExchangeResponse) {
        self.print_cli_output(&value, None);
    }

    fn output_magic_link_send_response(&self, value: types::MagicLinkSendResponse) {
        self.print_cli_output(&value, None);
    }

    fn output_jwks(&self, value: types::Jwks) {
        self.print_cli_output(&value, None);
    }

    fn output_openid_configuration(&self, value: types::OpenIdConfiguration) {
        self.print_cli_output(&value, None);
    }

    fn output_error<T>(&self, value: &progenitor_client::Error<T>)
    where
        T: schemars::JsonSchema + serde::Serialize + std::fmt::Debug,
    {
        self.print_error(value)
    }
}

trait TabDisplay {
    fn display(&self, tw: &mut TabWriter<Vec<u8>>, level: u8, printer: &RfdTabPrinter);
}

impl TabDisplay for types::Rfd {
    fn display(&self, tw: &mut TabWriter<Vec<u8>>, level: u8, printer: &RfdTabPrinter) {
        printer.print_field(tw, level, "id", &self.id);
        printer.print_field(tw, level, "rfd_number", &self.rfd_number);
        printer.print_field(tw, level, "visibility", &self.visibility.to_string());
        printer.print_field(
            tw,
            level,
            "latest_major_change_at",
            &self
                .latest_major_change_at
                .map(|d| d.to_string())
                .unwrap_or_else(|| "--".to_string()),
        );
        printer.print_field(tw, level, "link", &self.link.as_deref().unwrap_or(""));
    }
}

impl TabDisplay for ApiUserForRfdPermission {
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

impl TabDisplay for GetUserResponseForRfdPermission {
    fn display(&self, tw: &mut TabWriter<Vec<u8>>, level: u8, printer: &RfdTabPrinter) {
        self.info.display(tw, level, printer);

        printer.print_field(tw, level, "providers", &"");
        for provider in &self.providers {
            printer.print_field(tw, level + 1, "id", &provider.id);
            printer.print_field(tw, level + 1, "provider", &provider.provider);
            printer.print_field(tw, level + 1, "provider_id", &provider.provider_id);
            printer.print_list(tw, level + 1, "emails", &provider.emails);
            printer.print_list(tw, level + 1, "display_names", &provider.display_names);
            printer.print_field(tw, level + 1, "created_at", &provider.created_at);
            printer.print_field(tw, level + 1, "updated_at", &provider.updated_at);
            printer.print_field(
                tw,
                level + 1,
                "deleted_at",
                &self
                    .info
                    .deleted_at
                    .map(|d| d.to_string())
                    .unwrap_or_else(|| "--".to_string()),
            );
        }
    }
}

impl TabDisplay for ApiKeyResponseForRfdPermission {
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
                    .unwrap_or_else(|| PermissionsForRfdPermission(vec![])),
            )
            .iter()
            .map(|p| p.to_string())
            .collect_vec(),
        );
        printer.print_field(tw, level, "created_at", &self.created_at);
    }
}

impl TabDisplay for InitialApiKeyResponseForRfdPermission {
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
                    .unwrap_or_else(|| PermissionsForRfdPermission(vec![])),
            )
            .iter()
            .map(|p| p.to_string())
            .collect_vec(),
        );
        printer.print_field(tw, level, "created_at", &self.created_at);
    }
}

impl TabDisplay for AccessGroupForRfdPermission {
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

impl TabDisplay for RfdWithoutContent {
    fn display(&self, tw: &mut TabWriter<Vec<u8>>, level: u8, printer: &RfdTabPrinter) {
        printer.print_field(tw, level, "id", &self.id);
        printer.print_field(tw, level, "rfd_number", &self.rfd_number);
        printer.print_field(tw, level, "title", &self.title.as_deref().unwrap_or(""));
        printer.print_field(tw, level, "state", &self.state.as_deref().unwrap_or(""));
        printer.print_field(
            tw,
            level,
            "visibility",
            match self.visibility {
                Visibility::Private => &"private",
                Visibility::Public => &"public",
            },
        );
        printer.print_field(tw, level, "authors", &self.authors.as_deref().unwrap_or(""));
        printer.print_field(tw, level, "labels", &self.labels.as_deref().unwrap_or(""));
        printer.print_field(tw, level, "link", &self.link.as_deref().unwrap_or(""));
        printer.print_field(
            tw,
            level,
            "discussion",
            &self.discussion.as_deref().unwrap_or(""),
        );
        printer.print_field(
            tw,
            level,
            "sha",
            &self.sha.as_ref().map(|s| s.as_str()).unwrap_or(""),
        );
        printer.print_field(
            tw,
            level,
            "commit",
            &self.commit.as_ref().map(|s| s.as_str()).unwrap_or(""),
        );
        printer.print_field(
            tw,
            level,
            "committed_at",
            &self
                .committed_at
                .map(|d| d.to_string())
                .unwrap_or_else(|| "--".to_string()),
        );
        printer.print_field(
            tw,
            level,
            "latest_major_change_at",
            &self
                .latest_major_change_at
                .map(|d| d.to_string())
                .unwrap_or_else(|| "--".to_string()),
        );
    }
}

impl TabDisplay for RfdWithRaw {
    fn display(&self, tw: &mut TabWriter<Vec<u8>>, level: u8, printer: &RfdTabPrinter) {
        printer.print_field(tw, level, "id", &self.id);
        printer.print_field(tw, level, "rfd_number", &self.rfd_number);
        printer.print_field(tw, level, "title", &self.title.as_deref().unwrap_or(""));
        printer.print_field(tw, level, "state", &self.state.as_deref().unwrap_or(""));
        printer.print_field(
            tw,
            level,
            "visibility",
            match self.visibility {
                Visibility::Private => &"private",
                Visibility::Public => &"public",
            },
        );
        printer.print_field(tw, level, "authors", &self.authors.as_deref().unwrap_or(""));
        printer.print_field(tw, level, "labels", &self.labels.as_deref().unwrap_or(""));
        printer.print_field(tw, level, "link", &self.link.as_deref().unwrap_or(""));
        printer.print_field(
            tw,
            level,
            "discussion",
            &self.discussion.as_deref().unwrap_or(""),
        );
        printer.print_field(
            tw,
            level,
            "sha",
            &self.sha.as_ref().map(|s| s.as_str()).unwrap_or(""),
        );
        printer.print_field(
            tw,
            level,
            "commit",
            &self.commit.as_ref().map(|s| s.as_str()).unwrap_or(""),
        );
        printer.print_field(
            tw,
            level,
            "committed_at",
            &self
                .committed_at
                .map(|d| d.to_string())
                .unwrap_or_else(|| "--".to_string()),
        );
        printer.print_field(
            tw,
            level,
            "latest_major_change_at",
            &self
                .latest_major_change_at
                .map(|d| d.to_string())
                .unwrap_or_else(|| "--".to_string()),
        );
        writeln!(tw);
        if let Some(content) = &self.content {
            writeln!(tw, "{}", content);
        }
    }
}

impl TabDisplay for RfdRevisionMeta {
    fn display(&self, tw: &mut TabWriter<Vec<u8>>, level: u8, printer: &RfdTabPrinter) {
        printer.print_field(tw, level, "revision_id", &self.id.to_string());
        printer.print_field(tw, level, "commit_sha", &self.commit_sha.to_string());
        printer.print_field(tw, level, "committed_at", &self.committed_at.to_string());
        printer.print_field(tw, level, "major_change", &self.major_change.to_string());
    }
}

impl TabDisplay for RfdAttr {
    fn display(&self, tw: &mut TabWriter<Vec<u8>>, level: u8, printer: &RfdTabPrinter) {
        let (key, value) = match self {
            Self::Discussion(value) => ("discussion", value.to_string()),
            Self::Labels(value) => ("labels", value.to_string()),
            Self::State(value) => ("state", value.to_string()),
        };
        printer.print_field(tw, level, key, &value);
    }
}

impl TabDisplay for SearchResults {
    fn display(&self, tw: &mut TabWriter<Vec<u8>>, level: u8, printer: &RfdTabPrinter) {
        printer.print_field(tw, level, "query", &self.query);
        printer.print_field(tw, level, "total hits", &self.hits.len());
        writeln!(tw);
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

        printer.print_field(
            tw,
            level,
            "content",
            &format!("{}...", get_slice_of_string(&self.content, 255)),
        );
    }
}

impl TabDisplay for ReserveRfdResponse {
    fn display(&self, tw: &mut TabWriter<Vec<u8>>, level: u8, printer: &RfdTabPrinter) {
        printer.print_field(tw, level, "number", &self.number);
    }
}

impl TabDisplay for Job {
    fn display(&self, tw: &mut TabWriter<Vec<u8>>, level: u8, printer: &RfdTabPrinter) {
        printer.print_field(tw, level, "id", &self.id);
        printer.print_field(tw, level, "owner", &self.owner);
        printer.print_field(tw, level, "repository", &self.repository);
        printer.print_field(tw, level, "branch", &self.branch);
        printer.print_field(tw, level, "sha", &self.sha);
        printer.print_field(tw, level, "rfd", &self.rfd);
        printer.print_field(
            tw,
            level,
            "webhook_delivery_id",
            &self
                .webhook_delivery_id
                .as_ref()
                .map(|id| id.0.to_string())
                .unwrap_or_default(),
        );
        printer.print_field(tw, level, "committed_at", &self.committed_at);
        printer.print_field(tw, level, "processed", &self.processed);
        printer.print_field(tw, level, "created_at", &self.created_at);
        printer.print_field(
            tw,
            level,
            "started_at",
            &self
                .started_at
                .map(|d| d.to_string())
                .unwrap_or_else(|| "--".to_string()),
        );
    }
}

impl TabDisplay for MagicLink {
    fn display(&self, tw: &mut TabWriter<Vec<u8>>, level: u8, printer: &RfdTabPrinter) {
        printer.print_field(tw, level, "id", &*self.id);
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

impl TabDisplay for MagicLinkRedirectUri {
    fn display(&self, tw: &mut TabWriter<Vec<u8>>, level: u8, printer: &RfdTabPrinter) {
        printer.print_field(tw, level, "id", &*self.id);
        printer.print_field(
            tw,
            level,
            "magic_link_client_id",
            &*self.magic_link_client_id,
        );
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

impl TabDisplay for MagicLinkSecret {
    fn display(&self, tw: &mut TabWriter<Vec<u8>>, level: u8, printer: &RfdTabPrinter) {
        printer.print_field(tw, level, "id", &*self.id);
        printer.print_field(
            tw,
            level,
            "magic_link_client_id",
            &*self.magic_link_client_id,
        );
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

impl TabDisplay for InitialMagicLinkSecretResponse {
    fn display(&self, tw: &mut TabWriter<Vec<u8>>, level: u8, printer: &RfdTabPrinter) {
        printer.print_field(tw, level, "id", &*self.id);
        printer.print_field(tw, level, "key", &self.key.0);
        printer.print_field(tw, level, "created_at", &self.created_at);
    }
}

impl TabDisplay for ApiUserContactEmail {
    fn display(&self, tw: &mut TabWriter<Vec<u8>>, level: u8, printer: &RfdTabPrinter) {
        printer.print_field(tw, level, "id", &self.id);
        printer.print_field(tw, level, "user_id", &self.user_id);
        printer.print_field(tw, level, "email", &self.email);
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

impl TabDisplay for ApiUserLinkRequestResponse {
    fn display(&self, tw: &mut TabWriter<Vec<u8>>, level: u8, printer: &RfdTabPrinter) {
        printer.print_field(tw, level, "token", &self.token.0);
    }
}

impl TabDisplay for OAuthAuthzCodeExchangeResponse {
    fn display(&self, tw: &mut TabWriter<Vec<u8>>, level: u8, printer: &RfdTabPrinter) {
        printer.print_field(tw, level, "access_token", &self.access_token);
        printer.print_field(tw, level, "token_type", &self.token_type);
        printer.print_field(tw, level, "expires_in", &self.expires_in);
    }
}

impl TabDisplay for OAuthProviderInfo {
    fn display(&self, tw: &mut TabWriter<Vec<u8>>, level: u8, printer: &RfdTabPrinter) {
        printer.print_field(tw, level, "provider", &self.provider.to_string());
        printer.print_field(tw, level, "client_id", &self.client_id);
        printer.print_field(tw, level, "auth_url_endpoint", &self.auth_url_endpoint);
        printer.print_field(tw, level, "token_endpoint", &self.token_endpoint);
        printer.print_field(
            tw,
            level,
            "device_code_endpoint",
            &self.device_code_endpoint,
        );
        printer.print_list(tw, level, "scopes", &self.scopes);
    }
}

impl TabDisplay for RfdWithPdf {
    fn display(&self, tw: &mut TabWriter<Vec<u8>>, level: u8, printer: &RfdTabPrinter) {
        printer.print_field(tw, level, "id", &self.id);
        printer.print_field(tw, level, "rfd_number", &self.rfd_number);
        printer.print_field(tw, level, "title", &self.title.as_deref().unwrap_or(""));
        printer.print_field(tw, level, "state", &self.state.as_deref().unwrap_or(""));
        printer.print_field(
            tw,
            level,
            "visibility",
            match self.visibility {
                Visibility::Private => &"private",
                Visibility::Public => &"public",
            },
        );
        printer.print_field(tw, level, "authors", &self.authors.as_deref().unwrap_or(""));
        printer.print_field(tw, level, "labels", &self.labels.as_deref().unwrap_or(""));
        printer.print_field(tw, level, "link", &self.link.as_deref().unwrap_or(""));
        printer.print_field(
            tw,
            level,
            "discussion",
            &self.discussion.as_deref().unwrap_or(""),
        );
        printer.print_field(
            tw,
            level,
            "sha",
            &self.sha.as_ref().map(|s| s.as_str()).unwrap_or(""),
        );
        printer.print_field(
            tw,
            level,
            "commit",
            &self.commit.as_ref().map(|s| s.as_str()).unwrap_or(""),
        );
        printer.print_field(
            tw,
            level,
            "committed_at",
            &self
                .committed_at
                .map(|d| d.to_string())
                .unwrap_or_else(|| "--".to_string()),
        );
        printer.print_field(
            tw,
            level,
            "latest_major_change_at",
            &self
                .latest_major_change_at
                .map(|d| d.to_string())
                .unwrap_or_else(|| "--".to_string()),
        );
        printer.print_field(tw, level, "pdfs", &"");
        self.content.display(tw, level + 1, printer);
    }
}

impl TabDisplay for RfdPdf {
    fn display(&self, tw: &mut TabWriter<Vec<u8>>, level: u8, printer: &RfdTabPrinter) {
        printer.print_field(tw, level, "id", &self.id);
        printer.print_field(tw, level, "source", &self.source.to_string());
        printer.print_field(tw, level, "link", &self.link);
        printer.print_field(tw, level, "external_id", &self.external_id);
        printer.print_field(tw, level, "rfd_id", &self.rfd_id);
        printer.print_field(tw, level, "rfd_revision_id", &self.rfd_revision_id);
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

impl TabDisplay for MagicLinkExchangeResponse {
    fn display(&self, tw: &mut TabWriter<Vec<u8>>, level: u8, printer: &RfdTabPrinter) {
        printer.print_field(tw, level, "access_token", &self.access_token);
        printer.print_field(tw, level, "token_type", &self.token_type);
        printer.print_field(tw, level, "expires_in", &self.expires_in);
    }
}

impl TabDisplay for MagicLinkSendResponse {
    fn display(&self, tw: &mut TabWriter<Vec<u8>>, level: u8, printer: &RfdTabPrinter) {
        printer.print_field(tw, level, "attempt_id", &*self.attempt_id);
    }
}

impl TabDisplay for Jwks {
    fn display(&self, tw: &mut TabWriter<Vec<u8>>, level: u8, printer: &RfdTabPrinter) {
        printer.print_field(tw, level, "keys", &"");
        self.keys.display(tw, level + 1, printer);
    }
}

impl TabDisplay for Jwk {
    fn display(&self, tw: &mut TabWriter<Vec<u8>>, level: u8, printer: &RfdTabPrinter) {
        printer.print_field(tw, level, "kid", &self.kid);
        printer.print_field(tw, level, "kty", &self.kty);
        printer.print_field(tw, level, "use", &self.use_);
        printer.print_field(tw, level, "n", &self.n);
        printer.print_field(tw, level, "e", &self.e);
    }
}

impl TabDisplay for OpenIdConfiguration {
    fn display(&self, tw: &mut TabWriter<Vec<u8>>, level: u8, printer: &RfdTabPrinter) {
        printer.print_field(tw, level, "jwks_uri", &self.jwks_uri);
    }
}

impl<T> TabDisplay for Vec<T>
where
    T: TabDisplay,
{
    fn display(&self, tw: &mut TabWriter<Vec<u8>>, level: u8, printer: &RfdTabPrinter) {
        for entry in self {
            entry.display(tw, level, printer);
            writeln!(tw);
        }
    }
}

impl RfdTabPrinter {
    fn print_cli_output<T>(&self, value: &T, heading: Option<String>)
    where
        T: TabDisplay,
    {
        let mut tw = TabWriter::new(vec![]).ansi(true);

        if let Some(heading) = &heading {
            self.print_field(&mut tw, 0, heading, &"");
        }

        value.display(&mut tw, if heading.is_some() { 1 } else { 0 }, self);

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

    fn print_list<T>(&self, tw: &mut TabWriter<Vec<u8>>, level: u8, key: &str, list: &[T])
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

    fn print_error<T>(&self, error: &progenitor_client::Error<T>)
    where
        T: schemars::JsonSchema + serde::Serialize + std::fmt::Debug,
    {
        let mut tw = TabWriter::new(vec![]).ansi(true);

        match error {
            progenitor_client::Error::CommunicationError(err) => {
                writeln!(tw, "Failed to reach the API server");
                writeln!(tw, "{:#?}", err);
            }
            progenitor_client::Error::ErrorResponse(err) => {
                let err: types::Error = reserialize(err.as_ref());
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
            progenitor_client::Error::Custom(err) => {
                writeln!(tw, "Internal CLI error");
                writeln!(tw, "{:?}", err);
                writeln!(tw, "Please report this as a bug against the rfd-api");
            }
        }

        output_writer(tw);
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
