// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use futures_io::AsyncRead;
use meilisearch_sdk::errors::Error;
use meilisearch_sdk::request::{parse_response, HttpClient, Method};
use reqwest_middleware::ClientWithMiddleware;
use serde::{de::DeserializeOwned, Serialize};
use tokio_util::compat::FuturesAsyncReadCompatExt;
use tokio_util::io::ReaderStream;

#[derive(Clone)]
pub struct RetryingMeilisearchClient {
    client: ClientWithMiddleware,
}

impl RetryingMeilisearchClient {
    pub fn new(client: ClientWithMiddleware) -> Self {
        Self { client }
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
            Method::Post { body, .. } => self.client.post(&url).body(to_body(body)),
            Method::Put { body, .. } => self.client.put(&url).body(to_body(body)),
            Method::Patch { body, .. } => self.client.patch(&url).body(to_body(body)),
        }
        .header(reqwest::header::CONTENT_TYPE, content_type);

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

fn to_body(reader: impl AsyncRead + Send + Sync + 'static) -> reqwest::Body {
    reqwest::Body::wrap_stream(ReaderStream::new(reader.compat()))
}
