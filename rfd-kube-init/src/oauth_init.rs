// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use anyhow::{anyhow, Context, Result};
use clap::Args;
use reqwest_middleware::ClientBuilder;
use reqwest_retry::{policies::ExponentialBackoff, RetryTransientMiddleware};
use serde::{Deserialize, Serialize};
use std::time::Duration;

use crate::kube;

/// Request body for the /init endpoint
#[derive(Debug, Serialize)]
struct InitRequest {
    redirect_uris: Vec<String>,
}

/// Response from the /init endpoint
#[derive(Debug, Deserialize)]
struct InitResponse {
    client_id: String,
    secret: String,
    redirect_uris: Vec<String>,
}

/// Result of calling the /init endpoint
enum InitResult {
    /// Successfully initialized, contains the response
    Success(InitResponse),
    /// System was already initialized (409 Conflict)
    AlreadyInitialized,
}

#[derive(Args)]
pub struct OAuthInitArgs {
    /// RFD API host URL (e.g., http://rfd-api:8080)
    #[arg(long, env = "RFD_API_HOST")]
    host: String,

    /// Redirect URIs for the OAuth client (comma-separated)
    #[arg(long, env = "OAUTH_REDIRECT_URIS", value_delimiter = ',')]
    redirect_uris: Vec<String>,

    /// Target namespaces to write the OAuth client credentials (comma-separated)
    #[arg(long, env = "OAUTH_TARGET_NAMESPACES", value_delimiter = ',')]
    target_namespaces: Vec<String>,

    /// Name of the secret to create in target namespaces
    #[arg(long, env = "OAUTH_SECRET_NAME", default_value = "rfd-oauth-client")]
    secret_name: String,

    /// Maximum number of retry attempts for the /init endpoint
    #[arg(long, env = "OAUTH_MAX_RETRIES", default_value = "30")]
    max_retries: u32,
}

/// Initialize OAuth client and distribute credentials to target namespaces.
pub async fn init(kube_client: &::kube::Client, args: &OAuthInitArgs) -> Result<()> {
    if args.redirect_uris.is_empty() {
        return Err(anyhow!("At least one redirect URI must be provided"));
    }

    if args.target_namespaces.is_empty() {
        return Err(anyhow!("At least one target namespace must be provided"));
    }

    tracing::info!(
        host = %args.host,
        redirect_uri_count = args.redirect_uris.len(),
        target_namespace_count = args.target_namespaces.len(),
        "Initializing OAuth client"
    );

    // Build retry-enabled HTTP client
    let retry_policy = ExponentialBackoff::builder()
        .retry_bounds(Duration::from_secs(1), Duration::from_secs(30))
        .build_with_max_retries(args.max_retries);

    let client = ClientBuilder::new(reqwest::Client::new())
        .with(RetryTransientMiddleware::new_with_policy(retry_policy))
        .build();

    let init_url = format!("{}/init", args.host.trim_end_matches('/'));

    let request_body = InitRequest {
        redirect_uris: args.redirect_uris.clone(),
    };

    tracing::debug!(url = %init_url, "Calling /init endpoint");

    let response = client
        .post(&init_url)
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&request_body)?)
        .send()
        .await
        .context("Failed to send request to /init endpoint")?;

    let init_response = match handle_init_response(response).await? {
        InitResult::Success(response) => response,
        InitResult::AlreadyInitialized => return Ok(()),
    };

    tracing::info!(
        client_id = %init_response.client_id,
        redirect_uri_count = init_response.redirect_uris.len(),
        "OAuth client created successfully"
    );

    // Distribute credentials to target namespaces
    let mut failures = Vec::new();

    for ns in &args.target_namespaces {
        match kube::write_secret(
            kube_client,
            ns,
            &args.secret_name,
            &[
                ("OAUTH_CLIENT_ID", &init_response.client_id),
                ("OAUTH_CLIENT_SECRET", &init_response.secret),
            ],
        )
        .await
        {
            Ok(()) => {
                tracing::info!(
                    namespace = ns.as_str(),
                    secret = args.secret_name.as_str(),
                    "Wrote OAuth client credentials"
                );
            }
            Err(err) => {
                failures.push(ns.clone());
                tracing::error!(
                    namespace = ns.as_str(),
                    secret = args.secret_name.as_str(),
                    error = %err,
                    "Failed to write OAuth client credentials"
                );
            }
        }
    }

    if !failures.is_empty() {
        return Err(anyhow!(
            "Failed to write secrets to namespaces: {}",
            failures.join(", ")
        ));
    }

    Ok(())
}

/// Process the HTTP response from the /init endpoint.
async fn handle_init_response(response: reqwest::Response) -> Result<InitResult> {
    let status = response.status();

    match status {
        reqwest::StatusCode::CONFLICT => {
            tracing::warn!("System already initialized (409 Conflict), skipping");
            Ok(InitResult::AlreadyInitialized)
        }
        reqwest::StatusCode::OK | reqwest::StatusCode::CREATED => {
            let parsed = response
                .json()
                .await
                .context("Failed to parse /init response")?;
            Ok(InitResult::Success(parsed))
        }
        _ => {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            Err(anyhow!(
                "Failed to initialize OAuth client: {} - {}",
                status,
                error_text
            ))
        }
    }
}
