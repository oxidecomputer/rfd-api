use std::time::Duration;

use async_trait::async_trait;
use google_drive::{traits::FileOps, Client as GDriveClient};
use google_storage1::{
    hyper, hyper::client::HttpConnector, hyper_rustls, hyper_rustls::HttpsConnector, Storage,
};
use octorust::{
    auth::{Credentials, JWTCredentials, InstallationTokenGenerator}, http_cache::FileBasedCache, Client as GitHubClient, ClientError,
};
use reqwest::Error as ReqwestError;
use rfd_model::storage::postgres::PostgresStore;
use rsa::{RsaPrivateKey, pkcs1::{DecodeRsaPrivateKey, EncodeRsaPrivateKey}};
use thiserror::Error;

use crate::{
    features::Feature,
    github::{GitHubError, GitHubRfdRepo},
    pdf::{PdfFileLocation, PdfStorage, RfdPdf, RfdPdfError},
    search::RfdSearchIndex,
    updater::{BoxedAction, RfdUpdaterError},
    AppConfig, PdfStorageConfig, SearchConfig, StaticStorageConfig, GitHubAuthConfig,
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
    GitHub(#[from] GitHubError),
    #[error(transparent)]
    InvalidAction(#[from] RfdUpdaterError),
    #[error(transparent)]
    InvalidGitHubPrivateKey(#[from] rsa::pkcs1::Error),
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
            GitHubAuthConfig::Installation { app_id, installation_id, private_key } => {
                GitHubClient::custom(
                    "rfd-processor",
                    Credentials::InstallationToken(InstallationTokenGenerator::new(
                        *installation_id,
                        JWTCredentials::new(*app_id, RsaPrivateKey::from_pkcs1_pem(private_key)?.to_pkcs1_der()?.to_bytes().to_vec())?,
                    )),
                    client,
                    http_cache,
                )
            },
            GitHubAuthConfig::User { token } => {
                GitHubClient::custom(
                    "rfd-processor",
                    Credentials::Token(token.to_string()),
                    client,
                    http_cache,
                )
            }
        };

        let repository = GitHubRfdRepo::new(
            &github_client,
            config.source.owner.clone(),
            config.source.repo.clone(),
            config.source.path.clone(),
        )
        .await?;

        Ok(Self {
            processor: ProcessorCtx {
                batch_size: config.processor_batch_size,
                interval: Duration::from_secs(config.processor_interval),
            },
            scanner: ScannerCtx {
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
            pdf: PdfStorageCtx::new(&config.pdf_storage),
            search: SearchCtx::new(&config.search_storage),
        })
    }
}

pub struct ProcessorCtx {
    pub batch_size: i64,
    pub interval: Duration,
}

pub struct ScannerCtx {
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
            hyper::Client::builder().build(
                hyper_rustls::HttpsConnectorBuilder::new()
                    .with_native_roots()
                    .https_or_http()
                    .enable_http1()
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

pub struct PdfStorageCtx {
    client: Option<GDriveClient>,
    locations: Vec<PdfStorageLocation>,
}

impl PdfStorageCtx {
    pub fn new(entries: &[PdfStorageConfig]) -> Self {
        Self {
            // A client is only needed if files are going to be written
            client: Feature::WritePdfToDrive
                .enabled()
                .then(|| GDriveClient::new("", "", "", "", "")),
            locations: entries
                .iter()
                .map(|e| PdfStorageLocation {
                    drive_id: e.drive.to_string(),
                    folder_id: e.folder.to_string(),
                })
                .collect(),
        }
    }
}

#[async_trait]
impl PdfStorage for PdfStorageCtx {
    async fn store_rfd_pdf(
        &self,
        filename: &str,
        pdf: &RfdPdf,
    ) -> Vec<Result<PdfFileLocation, RfdPdfError>> {
        let mut results = vec![];

        for location in &self.locations {
            // Write the pdf to storage if it is enabled
            if let Some(client) = &self.client {
                results.push(
                    client
                        .files()
                        .create_or_update(
                            &location.drive_id,
                            &location.folder_id,
                            &filename,
                            "application/pdf",
                            &pdf.contents,
                        )
                        .await
                        .map(|file| PdfFileLocation {
                            url: Some(format!("https://drive.google.com/open?id={}", file.body.id)),
                        })
                        .map_err(|err| RfdPdfError::from(err)),
                );
            } else {
                // Otherwise just mark the write as a success. The id argument reported will not be
                // a real id
                results.push(Ok(PdfFileLocation {
                    url: Some(format!("https://drive.google.com/open?id={}", filename)),
                }));
            }
        }

        results
    }

    async fn remove_rfd_pdf(&self, filename: &str) -> Vec<RfdPdfError> {
        let mut results = vec![];

        for location in &self.locations {
            if let Some(client) = &self.client {
                // Delete the old filename from drive. It is expected that the target drive and
                // folder already exist
                if let Err(err) = client
                    .files()
                    .delete_by_name(&location.drive_id, &location.folder_id, &filename)
                    .await
                {
                    tracing::warn!(
                        ?err,
                        ?location,
                        ?filename,
                        "Faileid to remove PDF from drive"
                    );
                    results.push(err.into());
                }
            }
        }

        results
    }
}

#[derive(Debug)]
pub struct PdfStorageLocation {
    pub drive_id: String,
    pub folder_id: String,
}

pub struct SearchCtx {
    pub indexes: Vec<RfdSearchIndex>,
}

impl SearchCtx {
    pub fn new(entries: &[SearchConfig]) -> Self {
        Self {
            indexes: entries
                .iter()
                .map(|c| RfdSearchIndex::new(&c.host, &c.key, &c.index))
                .collect(),
        }
    }
}
