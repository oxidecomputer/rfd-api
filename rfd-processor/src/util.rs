// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use base64::{prelude::BASE64_STANDARD, DecodeError, Engine};
use google_drive3::{
    hyper_rustls::{HttpsConnector, HttpsConnectorBuilder},
    hyper_util::{
        client::legacy::{connect::HttpConnector, Client},
        rt::TokioExecutor,
    },
    DriveHub,
};
use std::path::Path;
use thiserror::Error;
use tokio::{fs, io::AsyncWriteExt};

#[derive(Debug, Error)]
pub enum FileIoError {
    #[error("Failed to write file {0}")]
    Io(#[from] std::io::Error),
    #[error("Expected file path to have a parent path")]
    MissingParent,
}

pub async fn write_file(file: &Path, contents: &[u8]) -> Result<(), FileIoError> {
    if let Some(parent) = file.parent() {
        // create each directory.
        fs::create_dir_all(parent).await?;

        // Write to the file.
        let mut f = fs::File::create(file).await?;
        f.write_all(contents).await?;

        tracing::info!(?file, "Wrote file");

        Ok(())
    } else {
        Err(FileIoError::MissingParent)
    }
}

pub fn decode_base64(c: &str) -> Result<Vec<u8>, DecodeError> {
    let v = c.replace('\n', "");
    let decoded = BASE64_STANDARD.decode(&v)?;
    Ok(decoded.trim().to_vec())
}

trait SliceExt {
    fn trim(&self) -> Self;
}

impl SliceExt for Vec<u8> {
    fn trim(&self) -> Vec<u8> {
        fn is_whitespace(c: &u8) -> bool {
            c == &b'\t' || c == &b' '
        }

        fn is_not_whitespace(c: &u8) -> bool {
            !is_whitespace(c)
        }

        if let Some(first) = self.iter().position(is_not_whitespace) {
            if let Some(last) = self.iter().rposition(is_not_whitespace) {
                self[first..last + 1].to_vec()
            } else {
                unreachable!();
            }
        } else {
            vec![]
        }
    }
}

#[derive(Debug, Error)]
pub enum GDriveError {
    #[error("Failed to construct GDrive error: {0}")]
    RemoteKeyAuthMissing(#[source] std::io::Error),
}

pub async fn gdrive_client() -> Result<DriveHub<HttpsConnector<HttpConnector>>, GDriveError> {
    let opts = yup_oauth2::ApplicationDefaultCredentialsFlowOpts::default();

    tracing::trace!(?opts, "Request GCP credentials");

    let gcp_credentials =
        yup_oauth2::ApplicationDefaultCredentialsAuthenticator::builder(opts).await;

    tracing::trace!("Retrieved GCP credentials");

    let gcp_auth = match gcp_credentials {
        yup_oauth2::authenticator::ApplicationDefaultCredentialsTypes::ServiceAccount(auth) => {
            tracing::debug!("Create GCP service account based credentials");

            auth.build().await.map_err(|err| {
                tracing::error!(
                    ?err,
                    "Failed to construct Google Drive credentials from service account"
                );
                GDriveError::RemoteKeyAuthMissing(err)
            })?
        }
        yup_oauth2::authenticator::ApplicationDefaultCredentialsTypes::InstanceMetadata(auth) => {
            tracing::debug!("Create GCP instance based credentials");

            auth.build().await.map_err(|err| {
                tracing::error!(
                    ?err,
                    "Failed to construct Google Drive credentials from instance metadata"
                );
                GDriveError::RemoteKeyAuthMissing(err)
            })?
        }
    };

    Ok(DriveHub::new(
        Client::builder(TokioExecutor::new()).build(
            HttpsConnectorBuilder::new()
                .with_native_roots()
                .unwrap()
                .https_only()
                .enable_http2()
                .build(),
        ),
        gcp_auth,
    ))
}

#[cfg(test)]
pub mod test_util {
    use tracing_subscriber::EnvFilter;

    #[allow(dead_code)]
    pub fn start_tracing() {
        let _subscriber = tracing_subscriber::fmt()
            .with_file(false)
            .with_line_number(false)
            .with_env_filter(EnvFilter::from_default_env())
            .pretty()
            .init();
    }
}
