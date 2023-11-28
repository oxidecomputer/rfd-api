// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use meilisearch_sdk::SearchResults;
use reqwest::Client;
use secrecy::{ExposeSecret, SecretString};
use serde::{de::DeserializeOwned, Serialize};
use std::fmt::Display;
use thiserror::Error;

pub struct SearchClient {
    client: Client,
    endpoint: String,
    secret: SecretString,
}

#[derive(Debug, Error)]
pub enum SearchError {
    #[error("Internal client error {0}")]
    Client(#[from] reqwest::Error),
    #[error("Failed to deserialize response {0}")]
    Deserialize(#[from] serde_json::Error),
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

impl SearchClient {
    pub fn new<T, U, V>(endpoint: T, index: U, secret: V) -> Self
    where
        T: Display,
        U: Display,
        V: ToString,
    {
        Self {
            client: Client::new(),
            endpoint: format!("{}/indexes/{}/search", endpoint, index),
            secret: SecretString::new(secret.to_string()),
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
