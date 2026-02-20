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

use rfd_secret::SecretResolutionError;

use crate::{
    pdf::{PdfFileLocation, PdfStorage, RfdPdf, RfdPdfError},
    search::{RfdSearchIndex, SearchError},
    updater::{BoxedAction, RfdUpdateMode, RfdUpdaterError},
    util::{gdrive_client, GDriveError},
    AppConfig, GcsStorageConfig, GitHubAuthConfig, PdfStorageConfig, S3StorageConfig, SearchConfig,
};

pub type StaticStorageError = Box<dyn std::error::Error + Send + Sync>;

#[async_trait]
pub trait StaticStorage: Send + Sync {
    async fn put(
        &self,
        key: &str,
        data: Vec<u8>,
        content_type: &str,
    ) -> Result<(), StaticStorageError>;
    fn name(&self) -> &str;
}

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
    #[error(transparent)]
    SecretResolution(#[from] SecretResolutionError),
}

pub struct Context {
    pub processor: ProcessorCtx,
    pub scanner: ScannerCtx,
    pub db: Database,
    pub github: GitHubCtx,
    pub actions: Vec<BoxedAction>,
    pub static_storage: Vec<Box<dyn StaticStorage>>,
    pub pdf: Option<PdfStorageCtx>,
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
            } => {
                let resolved_key = private_key.resolve()?;
                GitHubClient::custom(
                    "rfd-processor",
                    Credentials::InstallationToken(InstallationTokenGenerator::new(
                        *installation_id,
                        JWTCredentials::new(
                            *app_id,
                            RsaPrivateKey::from_pkcs1_pem(&resolved_key)?
                                .to_pkcs1_der()?
                                .to_bytes()
                                .to_vec(),
                        )?,
                    )),
                    client,
                    http_cache,
                )
            }
            GitHubAuthConfig::User { token } => GitHubClient::custom(
                "rfd-processor",
                Credentials::Token(token.resolve()?),
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
            static_storage: build_static_storage(&config.gcs_storage, &config.s3_storage).await?,
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

async fn build_static_storage(
    gcs_entries: &[GcsStorageConfig],
    s3_entries: &[S3StorageConfig],
) -> Result<Vec<Box<dyn StaticStorage>>, ContextError> {
    let mut storage: Vec<Box<dyn StaticStorage>> = Vec::new();

    // Build GCS storage instances
    for entry in gcs_entries {
        let client = build_gcs_client().await?;
        storage.push(Box::new(GcsStorage {
            client,
            bucket: entry.bucket.clone(),
        }));
    }

    // Build S3 storage instances
    for entry in s3_entries {
        let mut config_loader = aws_config::defaults(aws_config::BehaviorVersion::latest())
            .region(aws_config::Region::new(entry.region.clone()));

        if let Some(endpoint) = &entry.endpoint {
            config_loader = config_loader.endpoint_url(endpoint);
        }

        let sdk_config = config_loader.load().await;
        let client = aws_sdk_s3::Client::new(&sdk_config);

        storage.push(Box::new(S3Storage {
            client,
            bucket: entry.bucket.clone(),
        }));
    }

    Ok(storage)
}

async fn build_gcs_client() -> Result<Storage<HttpsConnector<HttpConnector>>, ContextError> {
    let opts = yup_oauth2::ApplicationDefaultCredentialsFlowOpts::default();
    let gcp_auth = match yup_oauth2::ApplicationDefaultCredentialsAuthenticator::builder(opts).await
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
        yup_oauth2::authenticator::ApplicationDefaultCredentialsTypes::InstanceMetadata(auth) => {
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

    Ok(Storage::new(
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

pub struct GcsStorage {
    client: Storage<HttpsConnector<HttpConnector>>,
    bucket: String,
}

#[async_trait]
impl StaticStorage for GcsStorage {
    async fn put(
        &self,
        key: &str,
        data: Vec<u8>,
        content_type: &str,
    ) -> Result<(), StaticStorageError> {
        use google_storage1::api::Object;

        let mime_type: mime_guess::Mime = content_type
            .parse()
            .unwrap_or(mime_guess::mime::APPLICATION_OCTET_STREAM);
        let cursor = std::io::Cursor::new(data);

        self.client
            .objects()
            .insert(Object::default(), &self.bucket)
            .name(key)
            .upload(cursor, mime_type)
            .await?;

        Ok(())
    }

    fn name(&self) -> &str {
        &self.bucket
    }
}

pub struct S3Storage {
    client: aws_sdk_s3::Client,
    bucket: String,
}

#[async_trait]
impl StaticStorage for S3Storage {
    async fn put(
        &self,
        key: &str,
        data: Vec<u8>,
        content_type: &str,
    ) -> Result<(), StaticStorageError> {
        use aws_sdk_s3::primitives::ByteStream;

        let body = ByteStream::from(data);

        self.client
            .put_object()
            .bucket(&self.bucket)
            .key(key)
            .content_type(content_type)
            .body(body)
            .send()
            .await?;

        Ok(())
    }

    fn name(&self) -> &str {
        &self.bucket
    }
}

pub type GDriveClient = DriveHub<HttpsConnector<HttpConnector>>;

pub struct PdfStorageCtx {
    client: GDriveClient,
    locations: Vec<PdfStorageLocation>,
}

impl PdfStorageCtx {
    pub async fn new(config: &Option<PdfStorageConfig>) -> Result<Option<Self>, GDriveError> {
        match config {
            Some(config) => Ok(Some(Self {
                client: gdrive_client().await?,
                locations: vec![PdfStorageLocation {
                    folder_id: config.folder.clone(),
                }],
            })),
            None => Ok(None),
        }
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

        if let Some(location) = self.locations.first() {
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
                tracing::info!("Successfully uploaded PDF");
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
    pub fn new(entries: &[SearchConfig]) -> Result<Self, ContextError> {
        Ok(Self {
            indexes: entries
                .iter()
                .map(|c| {
                    let key = c.key.resolve()?;
                    RfdSearchIndex::new(&c.host, key, &c.index).map_err(ContextError::from)
                })
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    use mockall::mock;
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};

    mock! {
        pub StaticStorage {}

        #[async_trait]
        impl StaticStorage for StaticStorage {
            async fn put(
                &self,
                key: &str,
                data: Vec<u8>,
                content_type: &str,
            ) -> Result<(), StaticStorageError>;
            fn name(&self) -> &str;
        }
    }

    /// In-memory storage implementation for testing
    pub struct InMemoryStorage {
        name: String,
        objects: Arc<Mutex<HashMap<String, StoredObject>>>,
    }

    #[derive(Clone)]
    pub struct StoredObject {
        pub data: Vec<u8>,
        pub content_type: String,
    }

    impl InMemoryStorage {
        pub fn new(name: &str) -> Self {
            Self {
                name: name.to_string(),
                objects: Arc::new(Mutex::new(HashMap::new())),
            }
        }

        pub fn get(&self, key: &str) -> Option<StoredObject> {
            self.objects.lock().unwrap().get(key).cloned()
        }

        pub fn len(&self) -> usize {
            self.objects.lock().unwrap().len()
        }
    }

    #[async_trait]
    impl StaticStorage for InMemoryStorage {
        async fn put(
            &self,
            key: &str,
            data: Vec<u8>,
            content_type: &str,
        ) -> Result<(), StaticStorageError> {
            self.objects.lock().unwrap().insert(
                key.to_string(),
                StoredObject {
                    data,
                    content_type: content_type.to_string(),
                },
            );
            Ok(())
        }

        fn name(&self) -> &str {
            &self.name
        }
    }

    #[tokio::test]
    async fn in_memory_storage_stores_and_retrieves_objects() {
        let storage = InMemoryStorage::new("test-bucket");

        let data = b"hello world".to_vec();
        storage
            .put("test/key.txt", data.clone(), "text/plain")
            .await
            .unwrap();

        let obj = storage.get("test/key.txt").unwrap();
        assert_eq!(obj.data, data);
        assert_eq!(obj.content_type, "text/plain");
    }

    #[tokio::test]
    async fn in_memory_storage_returns_correct_name() {
        let storage = InMemoryStorage::new("my-bucket");
        assert_eq!(storage.name(), "my-bucket");
    }

    #[tokio::test]
    async fn in_memory_storage_overwrites_existing_objects() {
        let storage = InMemoryStorage::new("test-bucket");

        storage
            .put("key", b"first".to_vec(), "text/plain")
            .await
            .unwrap();
        storage
            .put("key", b"second".to_vec(), "text/plain")
            .await
            .unwrap();

        let obj = storage.get("key").unwrap();
        assert_eq!(obj.data, b"second".to_vec());
        assert_eq!(storage.len(), 1);
    }

    #[tokio::test]
    async fn mock_storage_can_simulate_failure() {
        let mut mock = MockStaticStorage::new();
        mock.expect_put()
            .returning(|_, _, _| Err("simulated failure".into()));
        mock.expect_name().return_const("mock-bucket".to_string());

        let result = mock.put("key", vec![], "text/plain").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn mock_storage_can_verify_calls() {
        let mut mock = MockStaticStorage::new();
        mock.expect_put()
            .withf(|key, data, content_type| {
                key == "expected/key.png" && data == b"image data" && content_type == "image/png"
            })
            .times(1)
            .returning(|_, _, _| Ok(()));
        mock.expect_name().return_const("mock-bucket".to_string());

        mock.put("expected/key.png", b"image data".to_vec(), "image/png")
            .await
            .unwrap();
    }
}
