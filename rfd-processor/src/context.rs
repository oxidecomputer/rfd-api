// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::{io::Cursor, time::Duration};

use async_trait::async_trait;
use google_drive3::{api::File, hyper_util::rt::TokioExecutor, DriveHub};
// use google_drive::{traits::FileOps, Client as GDriveClient};
use google_storage1::{
    hyper_rustls::{HttpsConnector, HttpsConnectorBuilder},
    hyper_util::client::legacy::{connect::HttpConnector, Client},
    Storage,
};
use octorust::{
    auth::{Credentials, InstallationTokenGenerator, JWTCredentials},
    http_cache::FileBasedCache,
    Client as GitHubClient, ClientError,
};
use reqwest::Error as ReqwestError;
use rfd_github::{GitHubError, GitHubRfdRepo};
use rfd_model::schema_ext::PdfSource;
use rsa::{
    pkcs1::{DecodeRsaPrivateKey, EncodeRsaPrivateKey},
    RsaPrivateKey,
};
use tap::TapFallible;
use thiserror::Error;
use tracing::instrument;
use v_model::storage::postgres::PostgresStore;

use crate::{
    pdf::{PdfFileLocation, PdfStorage, RfdPdf, RfdPdfError},
    search::{RfdSearchIndex, SearchError},
    updater::{BoxedAction, RfdUpdateMode, RfdUpdaterError},
    util::{gdrive_client, GDriveError},
    AppConfig, GitHubAuthConfig, PdfStorageConfig, SearchConfig, StaticStorageConfig,
};

pub struct Database {
    pub storage: PostgresStore,
}

impl Database {
    pub async fn new(database_url: &str) -> Self {
        Self {
            storage: PostgresStore::new(database_url)
                .await
                .map_err(|err| {
                    format!("Failed to establish initial database connection: {:?}", err)
                })
                .unwrap(),
        }
    }
}

#[derive(Debug, Error)]
pub enum ContextError {
    #[error(transparent)]
    ClientConstruction(ReqwestError),
    #[error(transparent)]
    FailedToCreateGitHubClient(#[from] ClientError),
    #[error("Failed to find GCP credentials {0}")]
    FailedToFindGcpCredentials(std::io::Error),
    #[error(transparent)]
    GDrive(#[from] GDriveError),
    #[error(transparent)]
    GitHub(#[from] GitHubError),
    #[error(transparent)]
    InvalidAction(#[from] RfdUpdaterError),
    #[error(transparent)]
    InvalidGitHubPrivateKey(#[from] rsa::pkcs1::Error),
    #[error(transparent)]
    Search(#[from] SearchError),
}

pub struct Context {
    pub processor: ProcessorCtx,
    pub scanner: ScannerCtx,
    pub db: Database,
    pub github: GitHubCtx,
    pub actions: Vec<BoxedAction>,
    pub assets: StaticAssetStorageCtx,
    pub pdf: PdfStorageCtx,
    pub search: SearchCtx,
}

impl Context {
    pub async fn new(db: Database, config: &AppConfig) -> Result<Self, ContextError> {
        let http = reqwest::Client::builder()
            .build()
            .map_err(ContextError::ClientConstruction)?;
        let retry_policy =
            reqwest_retry::policies::ExponentialBackoff::builder().build_with_max_retries(3);
        let client = reqwest_middleware::ClientBuilder::new(http)
            // Trace HTTP requests. See the tracing crate to make use of these traces.
            .with(reqwest_tracing::TracingMiddleware::default())
            // Retry failed requests.
            .with(reqwest_retry::RetryTransientMiddleware::new_with_policy(
                retry_policy,
            ))
            .build();
        let http_cache = Box::new(FileBasedCache::new("/tmp/.cache/github"));

        let github_client = match &config.auth.github {
            GitHubAuthConfig::Installation {
                app_id,
                installation_id,
                private_key,
            } => GitHubClient::custom(
                "rfd-processor",
                Credentials::InstallationToken(InstallationTokenGenerator::new(
                    *installation_id,
                    JWTCredentials::new(
                        *app_id,
                        RsaPrivateKey::from_pkcs1_pem(private_key)?
                            .to_pkcs1_der()?
                            .to_bytes()
                            .to_vec(),
                    )?,
                )),
                client,
                http_cache,
            ),
            GitHubAuthConfig::User { token } => GitHubClient::custom(
                "rfd-processor",
                Credentials::Token(token.to_string()),
                client,
                http_cache,
            ),
        };

        let repository = GitHubRfdRepo::new(
            &github_client,
            config.source.owner.clone(),
            config.source.repo.clone(),
            config.source.path.clone(),
            config.source.default_branch.clone(),
        )
        .await?;

        Ok(Self {
            processor: ProcessorCtx {
                enabled: config.processor_enabled,
                batch_size: config.processor_batch_size,
                interval: Duration::from_secs(config.processor_interval),
                update_mode: config.processor_update_mode,
                capacity: config.processor_capacity,
            },
            scanner: ScannerCtx {
                enabled: config.scanner_enabled,
                interval: Duration::from_secs(config.scanner_interval),
            },
            db,
            github: GitHubCtx {
                client: github_client,
                repository,
            },
            actions: config
                .actions
                .iter()
                .map(|action| action.as_str().try_into())
                .collect::<Result<Vec<_>, RfdUpdaterError>>()?,
            assets: StaticAssetStorageCtx::new(&config.static_storage).await?,
            pdf: PdfStorageCtx::new(&config.pdf_storage).await?,
            search: SearchCtx::new(&config.search_storage)?,
        })
    }
}

pub struct ProcessorCtx {
    pub enabled: bool,
    pub batch_size: i64,
    pub interval: Duration,
    pub update_mode: RfdUpdateMode,
    pub capacity: u64,
}

pub struct ScannerCtx {
    pub enabled: bool,
    pub interval: Duration,
}

pub struct GitHubCtx {
    pub client: GitHubClient,
    pub repository: GitHubRfdRepo,
}

pub struct StaticAssetStorageCtx {
    pub client: Storage<HttpsConnector<HttpConnector>>,
    pub locations: Vec<StaticAssetLocation>,
}

impl StaticAssetStorageCtx {
    pub async fn new(entries: &[StaticStorageConfig]) -> Result<Self, ContextError> {
        let opts = yup_oauth2::ApplicationDefaultCredentialsFlowOpts::default();
        let gcp_auth = match yup_oauth2::ApplicationDefaultCredentialsAuthenticator::builder(opts)
            .await
        {
            yup_oauth2::authenticator::ApplicationDefaultCredentialsTypes::ServiceAccount(auth) => {
                tracing::debug!("Service account based credentials");

                auth.build().await.map_err(|err| {
                    tracing::error!(
                        ?err,
                        "Failed to construct Cloud Storage credentials from service account"
                    );
                    ContextError::FailedToFindGcpCredentials(err)
                })?
            }
            yup_oauth2::authenticator::ApplicationDefaultCredentialsTypes::InstanceMetadata(
                auth,
            ) => {
                tracing::debug!("Create instance based credentials");

                auth.build().await.map_err(|err| {
                    tracing::error!(
                        ?err,
                        "Failed to construct Cloud Storage credentials from instance metadata"
                    );
                    ContextError::FailedToFindGcpCredentials(err)
                })?
            }
        };

        let storage = Storage::new(
            Client::builder(TokioExecutor::new()).build(
                HttpsConnectorBuilder::new()
                    .with_native_roots()
                    .unwrap()
                    .https_only()
                    .enable_http2()
                    .build(),
            ),
            gcp_auth,
        );

        Ok(Self {
            client: storage,
            locations: entries
                .iter()
                .map(|e| StaticAssetLocation {
                    bucket: e.bucket.to_string(),
                })
                .collect(),
        })
    }
}

pub struct StaticAssetLocation {
    pub bucket: String,
}

pub type GDriveClient = DriveHub<HttpsConnector<HttpConnector>>;

pub struct PdfStorageCtx {
    client: GDriveClient,
    locations: Vec<PdfStorageLocation>,
}

impl PdfStorageCtx {
    pub async fn new(config: &Option<PdfStorageConfig>) -> Result<Self, GDriveError> {
        Ok(Self {
            // A client is only needed if files are going to be written
            client: gdrive_client().await?,
            locations: config
                .as_ref()
                .map(|config| {
                    vec![PdfStorageLocation {
                        folder_id: config.folder.clone(),
                    }]
                })
                .unwrap_or_default(),
        })
    }
}

#[async_trait]
impl PdfStorage for PdfStorageCtx {
    #[instrument(skip(self, pdf), fields(locations = ?self.locations))]
    async fn store_rfd_pdf(
        &self,
        external_id: Option<&str>,
        filename: &str,
        pdf: &RfdPdf,
    ) -> Vec<Result<PdfFileLocation, RfdPdfError>> {
        tracing::info!("Attempt to store PFD");

        if let Some(location) = self.locations.get(0) {
            let mut req = File {
                name: Some(filename.to_string()),
                ..Default::default()
            };

            let stream = Cursor::new(pdf.contents.clone());

            let response = match external_id {
                Some(file_id) => {
                    tracing::info!(?req, "Updating existing PDF with new version");
                    self.client
                        .files()
                        .update(req, file_id)
                        .supports_all_drives(true)
                        .upload_resumable(
                            stream,
                            "application/pdf".parse().expect("Failed to parse mimetype"),
                        )
                        .await
                        .map_err(RfdPdfError::Remote)
                }
                None => {
                    req.parents = Some(vec![location.folder_id.to_string()]);
                    tracing::info!(?req, "Creating new PDF file");
                    self.client
                        .files()
                        .create(req)
                        .supports_all_drives(true)
                        .upload_resumable(
                            stream,
                            "application/pdf".parse().expect("Failed to parse mimetype"),
                        )
                        .await
                        .map_err(RfdPdfError::Remote)
                }
            }
            .tap_ok(|_| {
                tracing::info!("Sucessfully uploaded PDF");
            })
            .tap_err(|err| {
                tracing::error!(?err, "Failed to upload PDF");
            });

            vec![response.and_then(|(_, file)| {
                file.id
                    .ok_or_else(|| RfdPdfError::FileIdMissing(filename.to_string()))
                    .map(|id| PdfFileLocation {
                        source: PdfSource::Google,
                        url: format!("https://drive.google.com/open?id={}", id),
                        external_id: id,
                    })
            })]
        } else {
            vec![]
        }
    }
}

#[derive(Debug)]
pub struct PdfStorageLocation {
    pub folder_id: String,
}

pub struct SearchCtx {
    pub indexes: Vec<RfdSearchIndex>,
}

impl SearchCtx {
    pub fn new(entries: &[SearchConfig]) -> Result<Self, SearchError> {
        Ok(Self {
            indexes: entries
                .iter()
                .map(|c| RfdSearchIndex::new(&c.host, &c.key, &c.index))
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}
