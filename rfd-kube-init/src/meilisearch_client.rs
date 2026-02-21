// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::pin::pin;

use futures_io::AsyncRead;
use futures_util::io::AsyncReadExt;
use meilisearch_sdk::errors::Error;
use meilisearch_sdk::request::{parse_response, HttpClient, Method};
use reqwest_middleware::ClientWithMiddleware;
use serde::{de::DeserializeOwned, Serialize};

#[derive(Clone)]
pub struct RetryingMeilisearchClient {
    client: ClientWithMiddleware,
    api_key: String,
}

impl RetryingMeilisearchClient {
    pub fn new(client: ClientWithMiddleware, api_key: String) -> Self {
        Self { client, api_key }
    }
}

#[async_trait::async_trait]
impl HttpClient for RetryingMeilisearchClient {
    async fn stream_request<
        Query: Serialize + Send + Sync,
        Body: AsyncRead + Send + Sync + 'static,
        Output: DeserializeOwned + 'static,
    >(
        &self,
        url: &str,
        method: Method<Query, Body>,
        content_type: &str,
        expected_status_code: u16,
    ) -> Result<Output, Error> {
        let url = format!("{url}?{}", yaup::to_string(method.query())?);

        let request = match method {
            Method::Get { .. } => self.client.get(&url),
            Method::Delete { .. } => self.client.delete(&url),
            Method::Post { body, .. } => self.client.post(&url).body(to_body(body).await),
            Method::Put { body, .. } => self.client.put(&url).body(to_body(body).await),
            Method::Patch { body, .. } => self.client.patch(&url).body(to_body(body).await),
        }
        .header(reqwest::header::CONTENT_TYPE, content_type);

        let request = request.header(reqwest::header::AUTHORIZATION, format!("Bearer {}", self.api_key));

        let response = request.send().await.map_err(|e| Error::Other(e.into()))?;
        let status = response.status().as_u16();
        let body = response.text().await.map_err(|e| Error::Other(e.into()))?;
        let body = if body.is_empty() { "null" } else { &body };

        parse_response(status, expected_status_code, body, url.to_string())
    }

    fn is_tokio(&self) -> bool {
        true
    }
}

async fn to_body(reader: impl AsyncRead + Send + Sync + 'static) -> reqwest::Body {
    let mut buf = Vec::new();
    let mut pinned = pin!(reader);
    // Buffer the body so it can be cloned by retry middleware
    let _ = pinned.read_to_end(&mut buf).await;
    reqwest::Body::from(buf)
}
