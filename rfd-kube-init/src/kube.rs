// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use anyhow::{anyhow, Result};
use k8s_openapi::api::core::v1::Secret;
use kube::{
    api::{Api, ObjectMeta, Patch, PatchParams},
    Client,
};
use secrecy::SecretString;
use std::collections::BTreeMap;
use tracing::{debug, instrument};

/// Read a specific key from a Kubernetes secret.
/// Returns the value wrapped in a SecretString to protect it in memory.
#[instrument(skip(client), fields(namespace = %namespace, secret_name = %secret_name, key = %key))]
pub async fn read_secret_key(
    client: &Client,
    namespace: &str,
    secret_name: &str,
    key: &str,
) -> Result<SecretString> {
    debug!("Reading secret key from Kubernetes");
    let secrets: Api<Secret> = Api::namespaced(client.clone(), namespace);
    let secret = secrets.get(secret_name).await?;

    let data = secret
        .data
        .ok_or_else(|| anyhow!("Secret '{}' in namespace '{}' has no data", secret_name, namespace))?;

    let value_bytes = data
        .get(key)
        .ok_or_else(|| {
            anyhow!(
                "Key '{}' not found in secret '{}' in namespace '{}'",
                key,
                secret_name,
                namespace
            )
        })?;

    let value_str = String::from_utf8(value_bytes.0.clone())
        .map_err(|_| anyhow!("Secret key '{}' is not valid UTF-8", key))?;

    debug!("Successfully read secret key");
    Ok(SecretString::from(value_str))
}

/// Write key-value pairs to a Kubernetes secret.
/// Creates the secret if it doesn't exist, patches if it does.
/// Uses server-side apply for idempotent create-or-update behavior.
#[instrument(skip(client, data), fields(namespace = %namespace, secret_name = %secret_name, key_count = data.len()))]
pub async fn write_secret(
    client: &Client,
    namespace: &str,
    secret_name: &str,
    data: &[(&str, &str)],
) -> Result<()> {
    debug!("Writing secret to Kubernetes");
    let secrets: Api<Secret> = Api::namespaced(client.clone(), namespace);

    let mut string_data = BTreeMap::new();
    for (key, value) in data {
        string_data.insert(key.to_string(), value.to_string());
    }

    let secret = Secret {
        metadata: ObjectMeta {
            name: Some(secret_name.to_string()),
            namespace: Some(namespace.to_string()),
            ..Default::default()
        },
        string_data: Some(string_data),
        type_: Some("Opaque".to_string()),
        ..Default::default()
    };

    secrets
        .patch(
            secret_name,
            &PatchParams::apply("rfd-kube-init"),
            &Patch::Apply(&secret),
        )
        .await?;

    debug!("Successfully wrote secret");
    Ok(())
}
