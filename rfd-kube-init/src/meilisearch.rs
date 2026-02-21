// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use anyhow::{anyhow, Context, Result};
use clap::Args;
use meilisearch_sdk::client::Client;
use meilisearch_sdk::key::{Action, Key, KeyBuilder};
use reqwest_middleware::ClientBuilder;
use reqwest_retry::{policies::ExponentialBackoff, RetryTransientMiddleware};
use secrecy::ExposeSecret;
use std::fmt;
use std::time::Duration;
use time::OffsetDateTime;

use crate::kube;
use crate::meilisearch_client::RetryingMeilisearchClient;

const DEFAULT_SEARCH_API_KEY_NAME: &str = "Default Search API Key";
const DEFAULT_ADMIN_API_KEY_NAME: &str = "Default Admin API Key";

enum TokenType<'a> {
    /// Read-only tenant token derived from the search key
    ReadOnly { search_key: Option<Key> },
    /// Scoped write API key with access to specific indexes
    ReadWrite { indexes: &'a [String] },
}

impl fmt::Display for TokenType<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenType::ReadOnly { .. } => write!(f, "RO"),
            TokenType::ReadWrite { .. } => write!(f, "RW"),
        }
    }
}

#[derive(Args)]
pub struct MeilisearchArgs {
    /// Namespace where the Meilisearch master key secret is located.
    /// If set, --master-secret-name and --master-secret-key must also be provided.
    #[arg(
        long,
        env = "MEILI_MASTER_NAMESPACE",
        requires_all = ["master_secret_name", "master_secret_key"]
    )]
    master_namespace: Option<String>,

    /// Name of the Kubernetes secret containing the master key.
    /// If set, --master-namespace and --master-secret-key must also be provided.
    #[arg(
        long,
        env = "MEILI_MASTER_SECRET_NAME",
        requires_all = ["master_namespace", "master_secret_key"]
    )]
    master_secret_name: Option<String>,

    /// Key within the secret that contains the master key value.
    /// If set, --master-namespace and --master-secret-name must also be provided.
    #[arg(
        long,
        env = "MEILI_MASTER_SECRET_KEY",
        requires_all = ["master_namespace", "master_secret_name"]
    )]
    master_secret_key: Option<String>,

    /// Meilisearch host URL
    #[arg(long, env = "MEILI_HOST")]
    host: String,

    /// Name of the secret to create in target namespaces
    #[arg(long, env = "MEILI_SECRET_NAME", default_value = "meilisearch-token")]
    secret_name: String,

    /// Token expiration time in seconds
    #[arg(long, env = "MEILI_API_EXPIRATION_SECONDS")]
    expiration_seconds: Option<i64>,

    /// JSON search rules filter for the tenant token
    #[arg(long, env = "MEILI_TOKEN_FILTER")]
    token_filter: Option<String>,

    /// Indexes that the RW key can access (comma-separated). Supports wildcards (e.g., "rfd-*").
    /// Required when --rw-namespaces is provided.
    #[arg(long, env = "MEILI_RW_INDEXES", value_delimiter = ',')]
    rw_indexes: Vec<String>,

    #[command(flatten)]
    namespaces: NamespaceArgs,

    /// Maximum number of retry attempts for Meilisearch connection
    #[arg(long, env = "MEILI_MAX_RETRIES", default_value = "30")]
    max_retries: u32,
}

#[derive(Args)]
#[group(required = true, multiple = true)]
struct NamespaceArgs {
    /// Target namespaces for read-write tokens (comma-separated).
    /// Requires --rw-indexes to be specified.
    #[arg(
        long,
        env = "MEILI_RW_TOKEN_TARGET_NAMESPACES",
        value_delimiter = ',',
        requires = "rw_indexes"
    )]
    rw_namespaces: Vec<String>,

    /// Target namespaces for read-only tokens (comma-separated)
    #[arg(long, env = "MEILI_RO_TOKEN_TARGET_NAMESPACES", value_delimiter = ',')]
    ro_namespaces: Vec<String>,
}

struct FailedNamespace {
    namespace: String,
    token_type: String,
}

impl fmt::Display for FailedNamespace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}({})", self.namespace, self.token_type)
    }
}

/// Initialize meilisearch secrets across target namespaces.
pub async fn init(kube_client: &::kube::Client, args: &MeilisearchArgs) -> Result<()> {
    // Check if master key configuration is provided (clap ensures all three are set together)
    let (master_namespace, master_secret_name, master_secret_key) = match (
        &args.master_namespace,
        &args.master_secret_name,
        &args.master_secret_key,
    ) {
        (Some(ns), Some(name), Some(key)) => (ns, name, key),
        _ => {
            tracing::info!("Meilisearch master key not configured, skipping initialization");
            return Ok(());
        }
    };

    // Read master key from kubernetes secret
    let master_key = kube::read_secret_key(
        kube_client,
        master_namespace,
        master_secret_name,
        master_secret_key,
    )
    .await
    .context("Failed to read Meilisearch master key from Kubernetes secret")?;

    // Calculate expiration time if provided
    let expires_at = args
        .expiration_seconds
        .map(|secs| OffsetDateTime::now_utc() + time::Duration::seconds(secs));

    // Parse search rules filter (empty string or missing means wildcard access to all indexes)
    let search_rules: serde_json::Value = match &args.token_filter {
        Some(filter) if !filter.is_empty() => {
            serde_json::from_str(filter).context("token-filter must be valid JSON")?
        }
        _ => serde_json::json!(["*"]),
    };

    // Create meilisearch client with retry middleware
    let retry_policy = ExponentialBackoff::builder()
        .retry_bounds(Duration::from_secs(1), Duration::from_secs(30))
        .build_with_max_retries(args.max_retries);

    let http_client = RetryingMeilisearchClient::new(
        ClientBuilder::new(reqwest::Client::new())
            .with(RetryTransientMiddleware::new_with_policy(retry_policy))
            .build(),
        master_key.expose_secret().to_string(),
    );

    let client = Client::new_with_client(&args.host, Some(master_key.expose_secret()), http_client);
    let (search_key, _admin_key) = get_api_keys(&client).await?;

    let mut failures = Vec::new();

    // Generate and distribute RW key (scoped write API key)
    failures.extend(
        generate_and_distribute_token(
            &client,
            kube_client,
            &args.namespaces.rw_namespaces,
            &args.secret_name,
            search_rules.clone(),
            expires_at,
            TokenType::ReadWrite {
                indexes: &args.rw_indexes,
            },
        )
        .await?,
    );

    // Generate and distribute RO token (tenant token)
    failures.extend(
        generate_and_distribute_token(
            &client,
            kube_client,
            &args.namespaces.ro_namespaces,
            &args.secret_name,
            search_rules,
            expires_at,
            TokenType::ReadOnly { search_key },
        )
        .await?,
    );

    if !failures.is_empty() {
        let failed_list: Vec<String> = failures.iter().map(|f| f.to_string()).collect();
        tracing::error!(
            failed_count = failures.len(),
            failed_namespaces = %failed_list.join(", "),
            "Failed to write secrets to some namespaces"
        );
        return Err(anyhow!(
            "Failed to write secrets to namespaces: {}",
            failed_list.join(", ")
        ));
    }

    Ok(())
}

/// Fetch API keys from Meilisearch and return the search and admin keys.
async fn get_api_keys(
    client: &Client<RetryingMeilisearchClient>,
) -> Result<(Option<Key>, Option<Key>)> {
    tracing::debug!("Fetching API keys from Meilisearch");
    let keys_result = client
        .get_keys()
        .await
        .context("Failed to fetch API keys from Meilisearch")?;

    tracing::debug!(
        key_count = keys_result.results.len(),
        "Retrieved API keys from Meilisearch"
    );

    let mut search_key = None;
    let mut admin_key = None;

    for key in keys_result.results {
        match key.name.as_deref() {
            Some(DEFAULT_SEARCH_API_KEY_NAME) => {
                tracing::debug!("Found default search API key");
                search_key = Some(key);
            }
            Some(DEFAULT_ADMIN_API_KEY_NAME) => {
                tracing::debug!("Found default admin API key");
                admin_key = Some(key);
            }
            _ => {}
        }
    }

    if search_key.is_none() {
        tracing::warn!("Default search API key not found");
    }
    if admin_key.is_none() {
        tracing::warn!("Default admin API key not found");
    }

    Ok((search_key, admin_key))
}

/// Create a scoped API key with write permissions limited to specific indexes.
async fn create_scoped_write_key(
    client: &Client<RetryingMeilisearchClient>,
    indexes: &[String],
    expires_at: Option<OffsetDateTime>,
) -> Result<Key> {
    let timestamp = OffsetDateTime::now_utc().unix_timestamp();
    let key_name = format!("rfd-kube-init-rw-{}", timestamp);

    tracing::info!(
        key_name = key_name.as_str(),
        indexes = ?indexes,
        "Creating scoped write API key"
    );

    let mut key_builder = KeyBuilder::new();
    key_builder
        .with_name(&key_name)
        .with_description("Scoped write key created by rfd-kube-init")
        .with_indexes(indexes)
        .with_actions([
            Action::DocumentsAdd,
            Action::DocumentsGet,
            Action::DocumentsDelete,
            Action::IndexesGet,
            Action::IndexesCreate,
            Action::SettingsGet,
            Action::SettingsUpdate,
            Action::Search,
        ]);

    if let Some(expires) = expires_at {
        key_builder.with_expires_at(expires);
    }

    let key = client
        .create_key(&key_builder)
        .await
        .context("Failed to create scoped write API key")?;

    tracing::info!(
        key_name = key.name,
        key_uid = %key.uid,
        "Created scoped write API key"
    );

    Ok(key)
}

/// Generate a tenant token from the given search key.
fn generate_tenant_token(
    client: &Client<RetryingMeilisearchClient>,
    search_key: &Key,
    search_rules: serde_json::Value,
    expires_at: Option<OffsetDateTime>,
) -> Result<String> {
    let token = client.generate_tenant_token(
        search_key.uid.clone(),
        search_rules,
        Some(&search_key.key),
        expires_at,
    )?;

    tracing::info!("Generated Meilisearch tenant token");
    Ok(token)
}

/// Generate an API key or token and distribute it to the specified namespaces.
/// Returns a list of namespaces that failed to write.
async fn generate_and_distribute_token(
    client: &Client<RetryingMeilisearchClient>,
    kube_client: &::kube::Client,
    namespaces: &[String],
    secret_name: &str,
    search_rules: serde_json::Value,
    expires_at: Option<OffsetDateTime>,
    token_type: TokenType<'_>,
) -> Result<Vec<FailedNamespace>> {
    if namespaces.is_empty() {
        tracing::debug!(token_type = %token_type, "No namespaces configured, skipping");
        return Ok(Vec::new());
    }

    tracing::info!(
        token_type = %token_type,
        namespace_count = namespaces.len(),
        "Distributing tokens to namespaces"
    );

    let token_type_str = token_type.to_string();

    let api_key = match token_type {
        TokenType::ReadWrite { indexes } => {
            let key = create_scoped_write_key(client, indexes, expires_at).await?;
            key.key
        }
        TokenType::ReadOnly { search_key } => {
            let key = search_key.ok_or_else(|| {
                tracing::error!(
                    key_name = DEFAULT_SEARCH_API_KEY_NAME,
                    "API key not found in Meilisearch but namespaces are configured"
                );
                anyhow!(
                    "Could not find '{}' in Meilisearch keys",
                    DEFAULT_SEARCH_API_KEY_NAME
                )
            })?;

            tracing::info!(
                key_name = key.name,
                token_type = token_type_str.as_str(),
                "Found API key for token",
            );

            generate_tenant_token(client, &key, search_rules, expires_at)?
        }
    };

    let mut failures = Vec::new();

    for ns in namespaces {
        match kube::write_secret(
            kube_client,
            ns,
            secret_name,
            &[("MEILISEARCH_API_KEY", &api_key)],
        )
        .await
        {
            Ok(()) => {
                tracing::info!(
                    namespace = ns.as_str(),
                    secret = secret_name,
                    token_type = token_type_str.as_str(),
                    "Wrote meilisearch secret"
                );
            }
            Err(err) => {
                failures.push(FailedNamespace {
                    namespace: ns.clone(),
                    token_type: token_type_str.clone(),
                });
                tracing::error!(
                    namespace = ns.as_str(),
                    secret = secret_name,
                    token_type = token_type_str.as_str(),
                    error = %err,
                    "Failed to write meilisearch secret"
                );
            }
        }
    }

    Ok(failures)
}

#[cfg(test)]
mod tests {
    use meilisearch_sdk::client::Client;

    /// Test that crypto providers are properly configured.
    /// This exercises both jsonwebtoken (JWT signing) and rustls (TLS) crypto backends.
    #[test]
    fn test_crypto_providers_configured() {
        // Create a meilisearch client - this validates rustls crypto provider is available
        let client = Client::new("http://localhost:7700", Some("test_master_key"))
            .expect("Failed to create meilisearch client");

        // Generate a tenant token - this exercises jsonwebtoken crypto
        // We use a fake UID but the signing operation still runs
        let search_rules = serde_json::json!(["*"]);
        let fake_uid = "550e8400-e29b-41d4-a716-446655440000".to_string();

        let result = client.generate_tenant_token(fake_uid, search_rules, None, None);

        // The token generation should succeed (crypto works) even though
        // the key UID is fake - we're testing the signing, not validation
        assert!(
            result.is_ok(),
            "JWT token generation failed: {:?}",
            result.err()
        );

        let token = result.unwrap();
        // JWT tokens have 3 parts separated by dots
        assert_eq!(
            token.split('.').count(),
            3,
            "Generated token should be a valid JWT format"
        );
    }
}
