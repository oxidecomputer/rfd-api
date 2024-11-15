// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use meilisearch_sdk::{SearchResult, SearchResults};
use reqwest::Client;
use secrecy::{ExposeSecret, SecretString};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::fmt::Display;
use thiserror::Error;

use crate::endpoints::rfd::{FormattedSearchResultHit, RfdSearchQuery, SearchResultHit};

#[derive(Debug, Error)]
pub enum SearchError {
    #[error("Internal client error {0}")]
    Client(#[from] reqwest::Error),
    #[error("Failed to deserialize response {0}")]
    Deserialize(#[from] serde_json::Error),
}

pub struct SearchClient {
    client: Client,
    endpoint: String,
    secret: SecretString,
}

impl SearchClient {
    pub fn new<T, U, V>(endpoint: T, index: U, secret: V) -> Self
    where
        T: Display,
        U: Display,
        V: AsRef<str>,
    {
        Self {
            client: Client::new(),
            endpoint: format!("{}/indexes/{}/search", endpoint, index),
            secret: SecretString::from(secret.as_ref()),
        }
    }

    pub async fn search<T>(&self, request: &SearchRequest) -> Result<SearchResults<T>, SearchError>
    where
        T: DeserializeOwned,
    {
        let response = self
            .client
            .post(&self.endpoint)
            .bearer_auth(self.secret.expose_secret())
            .json(request)
            .send()
            .await?;
        let body = response.text().await?;
        Ok(serde_json::from_str(&body)?)
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchRequest {
    pub q: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub attributes_to_highlight: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub highlight_pre_tag: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub highlight_post_tag: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub attributes_to_crop: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,
}

impl From<RfdSearchQuery> for SearchRequest {
    fn from(query: RfdSearchQuery) -> Self {
        SearchRequest {
            q: query.q,
            filter: None,
            // TODO: Make this configurable
            attributes_to_highlight: vec!["*".to_string()],
            highlight_pre_tag: query.highlight_pre_tag,
            highlight_post_tag: query.highlight_post_tag,
            attributes_to_crop: query
                .attributes_to_crop
                .unwrap_or_default()
                .split(",")
                .map(|s| s.to_string())
                .collect::<Vec<_>>(),
            limit: query.limit,
            offset: query.offset,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct MeiliSearchResult {
    hierarchy_radio_lvl0: Option<String>,
    hierarchy_radio_lvl1: Option<String>,
    hierarchy_radio_lvl2: Option<String>,
    hierarchy_radio_lvl3: Option<String>,
    hierarchy_radio_lvl4: Option<String>,
    hierarchy_radio_lvl5: Option<String>,
    hierarchy_lvl0: Option<String>,
    hierarchy_lvl1: Option<String>,
    hierarchy_lvl2: Option<String>,
    hierarchy_lvl3: Option<String>,
    hierarchy_lvl4: Option<String>,
    hierarchy_lvl5: Option<String>,
    content: String,
    #[serde(rename = "objectID")]
    object_id: String,
    rfd_number: u64,
    anchor: Option<String>,
    url: Option<String>,
    public: Option<bool>,
}

impl From<SearchResult<MeiliSearchResult>> for SearchResultHit {
    fn from(hit: SearchResult<MeiliSearchResult>) -> Self {
        SearchResultHit {
            hierarchy_radio: [
                hit.result.hierarchy_radio_lvl0,
                hit.result.hierarchy_radio_lvl1,
                hit.result.hierarchy_radio_lvl2,
                hit.result.hierarchy_radio_lvl3,
                hit.result.hierarchy_radio_lvl4,
                hit.result.hierarchy_radio_lvl5,
            ],
            hierarchy: [
                hit.result.hierarchy_lvl0,
                hit.result.hierarchy_lvl1,
                hit.result.hierarchy_lvl2,
                hit.result.hierarchy_lvl3,
                hit.result.hierarchy_lvl4,
                hit.result.hierarchy_lvl5,
            ],
            content: hit.result.content,
            object_id: hit.result.object_id.clone(),
            rfd_number: hit.result.rfd_number.clone(),
            anchor: hit.result.anchor,
            url: hit.result.url,
            formatted: hit
                .formatted_result
                .map(|formatted| FormattedSearchResultHit {
                    hierarchy_radio: [
                        formatted
                            .get("hierarchy_radio_lvl0")
                            .and_then(|v| v.as_str().map(|s| s.to_string())),
                        formatted
                            .get("hierarchy_radio_lvl1")
                            .and_then(|v| v.as_str().map(|s| s.to_string())),
                        formatted
                            .get("hierarchy_radio_lvl2")
                            .and_then(|v| v.as_str().map(|s| s.to_string())),
                        formatted
                            .get("hierarchy_radio_lvl3")
                            .and_then(|v| v.as_str().map(|s| s.to_string())),
                        formatted
                            .get("hierarchy_radio_lvl4")
                            .and_then(|v| v.as_str().map(|s| s.to_string())),
                        formatted
                            .get("hierarchy_radio_lvl5")
                            .and_then(|v| v.as_str().map(|s| s.to_string())),
                    ],
                    hierarchy: [
                        formatted
                            .get("hierarchy_lvl0")
                            .and_then(|v| v.as_str().map(|s| s.to_string())),
                        formatted
                            .get("hierarchy_lvl1")
                            .and_then(|v| v.as_str().map(|s| s.to_string())),
                        formatted
                            .get("hierarchy_lvl2")
                            .and_then(|v| v.as_str().map(|s| s.to_string())),
                        formatted
                            .get("hierarchy_lvl3")
                            .and_then(|v| v.as_str().map(|s| s.to_string())),
                        formatted
                            .get("hierarchy_lvl4")
                            .and_then(|v| v.as_str().map(|s| s.to_string())),
                        formatted
                            .get("hierarchy_lvl5")
                            .and_then(|v| v.as_str().map(|s| s.to_string())),
                    ],
                    content: formatted
                        .get("content")
                        .and_then(|v| v.as_str().map(|s| s.to_string())),
                    object_id: hit.result.object_id,
                    rfd_number: hit.result.rfd_number,
                    anchor: formatted
                        .get("anchor")
                        .and_then(|v| v.as_str().map(|s| s.to_string())),
                    url: formatted
                        .get("url")
                        .and_then(|v| v.as_str().map(|s| s.to_string())),
                }),
        }
    }
}
