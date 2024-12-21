// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use chrono::{DateTime, Utc};
use newtype_uuid::TypedUuid;
use octorust::{
    auth::{Credentials, InstallationTokenGenerator, JWTCredentials},
    http_cache::NoCache,
    Client as GitHubClient,
};
use partial_struct::partial;
use rfd_data::{
    content::{RfdContent, RfdDocument, RfdTemplate, TemplateError},
    RfdNumber,
};
use rfd_github::{GitHubError, GitHubNewRfdNumber, GitHubRfdRepo};
use rfd_model::{
    schema_ext::{ContentFormat, Visibility},
    storage::{JobStore, RfdFilter, RfdMetaStore, RfdStorage, RfdStore},
    CommitSha, FileSha, Job, NewJob, Rfd, RfdId, RfdMeta, RfdRevision, RfdRevisionId,
};
use rsa::{
    pkcs1::{DecodeRsaPrivateKey, EncodeRsaPrivateKey},
    RsaPrivateKey,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::{cmp::Ordering, sync::Arc};
use tap::TapFallible;
use thiserror::Error;
use tracing::instrument;
use v_api::{
    response::{resource_not_found, resource_restricted, ResourceResult, ToResourceResult},
    ApiContext, VContext,
};
use v_model::{
    permissions::Caller,
    storage::{ApiUserProviderFilter, ListPagination, StoreError},
};

use crate::{
    config::{ContentConfig, GitHubAuthConfig, SearchConfig, ServicesConfig},
    error::AppError,
    permissions::RfdPermission,
    search::SearchClient,
};

static UNLIMITED: i64 = 9999999;

pub struct RfdContext {
    pub public_url: String,
    pub storage: Arc<dyn RfdStorage>,
    pub search: SearchContext,
    pub content: ContentContext,
    pub github: GitHubRfdRepo,

    v_context: Arc<VContext<RfdPermission>>,
}

impl ApiContext for RfdContext {
    type AppPermissions = RfdPermission;
    fn v_ctx(&self) -> &VContext<Self::AppPermissions> {
        &self.v_context
    }
}

pub struct SearchContext {
    pub client: SearchClient,
}

pub struct ContentContext {
    pub placeholder_template: RfdTemplate,
    pub new_template: RfdTemplate,
}

#[derive(Debug, Error)]
pub enum UpdateRfdContentError {
    #[error(transparent)]
    GitHub(#[from] GitHubError),
    #[error("Internal GitHub state does not currently allow for update. This commit appears as the head commit on multiple branches.")]
    InternalState,
    #[error("Failed to construct new RFD template")]
    InvalidTemplate(#[from] TemplateError),
    #[error("Unable to perform action. Unable to find the default branch on GitHub.")]
    NoDefaultBranch,
    #[error(transparent)]
    Storage(#[from] StoreError),
}

#[partial(RfdWithoutContent)]
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct RfdWithContent {
    pub id: TypedUuid<RfdId>,
    pub rfd_number: i32,
    pub link: Option<String>,
    pub discussion: Option<String>,
    pub title: String,
    pub state: Option<String>,
    pub authors: Option<String>,
    pub labels: Option<String>,
    #[partial(RfdWithoutContent(skip))]
    pub content: String,
    pub format: ContentFormat,
    pub sha: FileSha,
    pub commit: CommitSha,
    pub committed_at: DateTime<Utc>,
    pub visibility: Visibility,
}

impl From<Rfd> for RfdWithContent {
    fn from(value: Rfd) -> Self {
        RfdWithContent {
            id: value.id,
            rfd_number: value.rfd_number,
            link: value.link,
            discussion: value.content.discussion,
            title: value.content.title,
            state: value.content.state,
            authors: value.content.authors,
            labels: value.content.labels,
            content: value.content.content,
            format: value.content.content_format,
            sha: value.content.sha,
            commit: value.content.commit.into(),
            committed_at: value.content.committed_at,
            visibility: value.visibility,
        }
    }
}

impl From<RfdMeta> for RfdWithoutContent {
    fn from(value: RfdMeta) -> Self {
        RfdWithoutContent {
            id: value.id,
            rfd_number: value.rfd_number,
            link: value.link,
            discussion: value.content.discussion,
            title: value.content.title,
            state: value.content.state,
            authors: value.content.authors,
            labels: value.content.labels,
            format: value.content.content_format,
            sha: value.content.sha,
            commit: value.content.commit.into(),
            committed_at: value.content.committed_at,
            visibility: value.visibility,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct PdfEntry {
    pub source: String,
    pub link: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub enum RfdRevisionIdentifier {
    Commit(CommitSha),
    Id(TypedUuid<RfdRevisionId>),
}

impl From<CommitSha> for RfdRevisionIdentifier {
    fn from(value: CommitSha) -> Self {
        Self::Commit(value)
    }
}

impl From<TypedUuid<RfdRevisionId>> for RfdRevisionIdentifier {
    fn from(value: TypedUuid<RfdRevisionId>) -> Self {
        Self::Id(value)
    }
}

impl RfdContext {
    pub async fn new(
        public_url: String,
        storage: Arc<dyn RfdStorage>,
        search: SearchConfig,
        content: ContentConfig,
        services: ServicesConfig,
        v_context: VContext<RfdPermission>,
    ) -> Result<Self, AppError> {
        let http = reqwest::Client::builder()
            .build()
            .map_err(AppError::ClientConstruction)?;
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

        Ok(Self {
            public_url,
            storage,
            search: SearchContext {
                client: SearchClient::new(search.host, search.index, search.key),
            },
            content: ContentContext {
                placeholder_template: content
                    .templates
                    .get("placeholder")
                    .cloned()
                    .ok_or(AppError::MissingNewRfdTemplate)?,
                new_template: content
                    .templates
                    .get("new")
                    .cloned()
                    .ok_or(AppError::MissingNewRfdTemplate)?,
            },
            github: GitHubRfdRepo::new(
                &match services.github.auth {
                    GitHubAuthConfig::Installation {
                        app_id,
                        installation_id,
                        private_key,
                    } => GitHubClient::custom(
                        "rfd-api",
                        Credentials::InstallationToken(InstallationTokenGenerator::new(
                            installation_id,
                            JWTCredentials::new(
                                app_id,
                                RsaPrivateKey::from_pkcs1_pem(&private_key)?
                                    .to_pkcs1_der()?
                                    .to_bytes()
                                    .to_vec(),
                            )?,
                        )),
                        client,
                        Box::new(NoCache),
                    ),
                    GitHubAuthConfig::User { token } => GitHubClient::custom(
                        "rfd-api",
                        Credentials::Token(token.to_string()),
                        client,
                        Box::new(NoCache),
                    ),
                },
                services.github.owner,
                services.github.repo,
                services.github.path,
                services.github.default_branch,
            )
            .await?,
            v_context: Arc::new(v_context),
        })
    }

    // RFD Operations

    #[instrument(skip(self, caller), err(Debug))]
    pub async fn create_rfd(
        &self,
        caller: &Caller<RfdPermission>,
        title: String,
        content: Option<String>,
    ) -> ResourceResult<RfdNumber, UpdateRfdContentError> {
        if caller.can(&RfdPermission::CreateRfd) {
            tracing::info!("Reserving new RFD");

            // We acknowledge that there are race conditions here, as there would be if an end user
            // were to attempt to reserve a RFD number manually
            let GitHubNewRfdNumber {
                number: next_rfd_number,
                commit,
            } = self
                .github
                .next_rfd_number()
                .await
                .map_err(UpdateRfdContentError::GitHub)
                .to_resource_result()?;

            let content = match content {
                Some(content) => self
                    .content
                    .new_template
                    .clone()
                    .field("number".to_string(), next_rfd_number.to_string())
                    .field("title".to_string(), title)
                    .field("body".to_string(), content),
                None => self
                    .content
                    .placeholder_template
                    .clone()
                    .field("number".to_string(), next_rfd_number.to_string())
                    .field("title".to_string(), title),
            }
            .build()
            .map_err(UpdateRfdContentError::InvalidTemplate)
            .to_resource_result()?;

            tracing::info!(?next_rfd_number, ?commit, "Creating new RFD branch");

            // Branch off of the default branch with a new branch with the padded form of the RFD number
            self.github
                .create_branch(&next_rfd_number, &commit)
                .await
                .map_err(UpdateRfdContentError::GitHub)
                .to_resource_result()?;

            tracing::info!(
                ?next_rfd_number,
                ?commit,
                "Created new branch for reserving RFD off of default branch"
            );

            self.commit_rfd_document(
                caller,
                next_rfd_number.into(),
                &content.render(),
                Some("Reserving RFD number"),
                commit,
                Some(&next_rfd_number.as_number_string()),
            )
            .await?;

            tracing::info!(
                ?next_rfd_number,
                "Pushed placeholder RFD to reserved branch"
            );

            Ok(next_rfd_number)
        } else {
            resource_restricted()
        }
    }

    pub async fn list_rfds(
        &self,
        caller: &Caller<RfdPermission>,
        filter: Option<RfdFilter>,
    ) -> ResourceResult<Vec<RfdWithoutContent>, StoreError> {
        // List all of the RFDs first and then perform filter. This should be be improved once
        // filters can be combined to support OR expressions. Effectively we need to be able to
        // express "Has access to OR is public" with a filter
        let mut rfds = RfdMetaStore::list(
            &*self.storage,
            filter.map(|filter| vec![filter]).unwrap_or_default(),
            &ListPagination::default().limit(UNLIMITED),
        )
        .await
        .tap_err(|err| tracing::error!(?err, "Failed to lookup RFDs"))
        .to_resource_result()?;

        // Filter the list of RFDs down to only those that the caller is allowed to access
        rfds.retain_mut(|rfd| {
            caller.can(&RfdPermission::GetRfdsAll)
                || caller.can(&RfdPermission::GetRfd(rfd.rfd_number))
                || rfd.visibility == Visibility::Public
        });

        let mut rfd_list = rfds
            .into_iter()
            .map(|rfd| RfdWithoutContent {
                id: rfd.id,
                rfd_number: rfd.rfd_number,
                link: rfd.link,
                discussion: rfd.content.discussion,
                title: rfd.content.title,
                state: rfd.content.state,
                authors: rfd.content.authors,
                labels: rfd.content.labels,
                format: rfd.content.content_format,
                sha: rfd.content.sha,
                commit: rfd.content.commit.into(),
                committed_at: rfd.content.committed_at,
                visibility: rfd.visibility,
            })
            .collect::<Vec<_>>();

        // Finally sort the RFD list by RFD number
        rfd_list.sort_by(|a, b| b.rfd_number.cmp(&a.rfd_number));

        Ok(rfd_list)
    }

    #[instrument(skip(self, caller))]
    async fn get_rfd(
        &self,
        caller: &Caller<RfdPermission>,
        rfd_number: i32,
        revision: Option<RfdRevisionIdentifier>,
    ) -> ResourceResult<Rfd, StoreError> {
        let mut filter = RfdFilter::default().rfd_number(Some(vec![rfd_number]));
        filter = match revision {
            Some(RfdRevisionIdentifier::Commit(commit_sha)) => {
                filter.commit_sha(Some(vec![commit_sha]))
            }
            Some(RfdRevisionIdentifier::Id(revision)) => filter.revision(Some(vec![revision])),
            None => filter,
        };

        let rfd = RfdStore::list(&*self.storage, vec![filter], &ListPagination::latest())
            .await
            .to_resource_result()?
            .pop();

        if let Some(rfd) = rfd {
            if caller.can(&RfdPermission::GetRfdsAll)
                || caller.can(&RfdPermission::GetRfd(rfd.rfd_number))
                || rfd.visibility == Visibility::Public
            {
                Ok(rfd)
            } else {
                resource_not_found()
            }
        } else {
            resource_not_found()
        }
    }

    #[instrument(skip(self, caller))]
    async fn get_rfd_meta(
        &self,
        caller: &Caller<RfdPermission>,
        rfd_number: i32,
        revision: Option<RfdRevisionIdentifier>,
    ) -> ResourceResult<RfdMeta, StoreError> {
        let mut filter = RfdFilter::default().rfd_number(Some(vec![rfd_number]));
        filter = match revision {
            Some(RfdRevisionIdentifier::Commit(commit_sha)) => {
                filter.commit_sha(Some(vec![commit_sha]))
            }
            Some(RfdRevisionIdentifier::Id(revision)) => filter.revision(Some(vec![revision])),
            None => filter,
        };

        let rfd = RfdMetaStore::list(&*self.storage, vec![filter], &ListPagination::latest())
            .await
            .to_resource_result()?
            .pop();

        if let Some(rfd) = rfd {
            if caller.can(&RfdPermission::GetRfdsAll)
                || caller.can(&RfdPermission::GetRfd(rfd.rfd_number))
                || rfd.visibility == Visibility::Public
            {
                Ok(rfd)
            } else {
                resource_not_found()
            }
        } else {
            resource_not_found()
        }
    }

    #[instrument(skip(self, caller))]
    pub async fn view_rfd(
        &self,
        caller: &Caller<RfdPermission>,
        rfd_number: i32,
        revision: Option<RfdRevisionIdentifier>,
    ) -> ResourceResult<RfdWithContent, StoreError> {
        let rfd = self.get_rfd(caller, rfd_number, revision).await?;
        Ok(rfd.into())
    }

    #[instrument(skip(self, caller))]
    pub async fn view_rfd_meta(
        &self,
        caller: &Caller<RfdPermission>,
        rfd_number: i32,
        revision: Option<RfdRevisionIdentifier>,
    ) -> ResourceResult<RfdWithoutContent, StoreError> {
        let rfd = self.get_rfd_meta(caller, rfd_number, revision).await?;
        Ok(rfd.into())
    }

    #[instrument(skip(self, caller))]
    pub async fn view_rfd_revision(
        &self,
        caller: &Caller<RfdPermission>,
        rfd_number: i32,
        revision: Option<RfdRevisionIdentifier>,
    ) -> ResourceResult<RfdRevision, StoreError> {
        let rfd = self.get_rfd(caller, rfd_number, revision).await?;
        Ok(rfd.content)
    }

    async fn get_latest_rfd_revision(
        &self,
        caller: &Caller<RfdPermission>,
        rfd_number: i32,
    ) -> ResourceResult<RfdRevision, StoreError> {
        self.view_rfd_revision(caller, rfd_number, None).await
    }

    #[instrument(skip(self, caller, content))]
    pub async fn update_rfd_content(
        &self,
        caller: &Caller<RfdPermission>,
        rfd_number: i32,
        content: &str,
        message: Option<&str>,
        branch_name: Option<&str>,
    ) -> ResourceResult<Option<CommitSha>, UpdateRfdContentError> {
        if caller.any(&[
            &RfdPermission::UpdateRfd(rfd_number),
            &RfdPermission::UpdateRfdsAll,
        ]) {
            let latest_revision = self
                .get_latest_rfd_revision(caller, rfd_number)
                .await
                .map_err(|err| err.inner_into())?;

            let sha = latest_revision.commit.clone();
            let mut updated_content: RfdContent = latest_revision.into();
            updated_content.update_body(content);

            self.commit_rfd_document(
                caller,
                rfd_number.into(),
                updated_content.raw(),
                message,
                sha,
                branch_name,
            )
            .await
        } else {
            resource_restricted()
        }
    }

    #[instrument(skip(self, caller, document))]
    pub async fn update_rfd_document(
        &self,
        caller: &Caller<RfdPermission>,
        rfd_number: i32,
        document: &str,
        message: Option<&str>,
        branch_name: Option<&str>,
    ) -> ResourceResult<Option<CommitSha>, UpdateRfdContentError> {
        if caller.any(&[
            &RfdPermission::UpdateRfd(rfd_number),
            &RfdPermission::UpdateRfdsAll,
        ]) {
            let latest_revision = self
                .get_latest_rfd_revision(caller, rfd_number)
                .await
                .map_err(|err| err.inner_into())?;
            let sha = latest_revision.commit;

            tracing::info!(?sha, "Found commit to update from");

            self.commit_rfd_document(
                caller,
                rfd_number.into(),
                document,
                message,
                sha,
                branch_name,
            )
            .await
        } else {
            resource_restricted()
        }
    }

    #[instrument(skip(self, caller, document), err(Debug))]
    async fn commit_rfd_document(
        &self,
        caller: &Caller<RfdPermission>,
        rfd_number: RfdNumber,
        document: &str,
        message: Option<&str>,
        head: CommitSha,
        branch_name: Option<&str>,
    ) -> ResourceResult<Option<CommitSha>, UpdateRfdContentError> {
        tracing::info!("Pushing update to GitHub");

        let mut github_locations = self
            .github
            .locations_for_commit(head.clone())
            .await
            .map_err(UpdateRfdContentError::GitHub)
            .to_resource_result()?
            .into_iter()
            .filter(|location| {
                branch_name
                    .as_ref()
                    .map(|branch_name| branch_name == &location.branch)
                    .unwrap_or(true)
            })
            .collect::<Vec<_>>();

        let mut filter = ApiUserProviderFilter::default();
        filter.api_user_id = Some(vec![caller.id]);
        let mut providers = self
            .v_ctx()
            .user
            .list_api_user_provider(caller, filter, &ListPagination::default())
            .await
            .map_err(|err| err.inner_into())?;

        // Prefer a GitHub identity provider, but we will use Google if we can not find one
        providers.sort_by(|a, b| {
            if a.provider == b.provider {
                Ordering::Equal
            } else if a.provider == "github" {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        });

        let display_name = providers
            .into_iter()
            .filter(|provider| !provider.display_names.is_empty())
            .nth(0)
            .and_then(|p| p.display_names.get(0).map(|s| s.clone()))
            .unwrap_or_else(|| caller.id.to_string());

        match github_locations.len() {
            0 => {
                tracing::warn!(
                    ?head,
                    ?rfd_number,
                    "Failed to find a GitHub location for most recent revision"
                );
                resource_not_found()
            }
            1 => {
                tracing::info!("Pushing RFD commit to GitHub");

                let message = format!(
                    "{}\n\nSubmitted by {}",
                    message.unwrap_or("RFD API update"),
                    display_name,
                );

                // Unwrap is checked by the location length
                let location = github_locations.pop().unwrap();
                let commit = location
                    .upsert(&rfd_number, document.as_bytes(), &message)
                    .await
                    .map_err(UpdateRfdContentError::GitHub)
                    .to_resource_result()?;

                // If we committed a change, immediately register a job as well. This may conflict with
                // a job already added by a webhook, this is fine and we can ignore the error
                if let Some(commit) = commit.clone() {
                    let new_job = NewJob {
                        owner: self.github.owner.clone(),
                        repository: self.github.repo.clone(),
                        branch: rfd_number.as_number_string(),
                        sha: commit.clone(),
                        rfd: rfd_number.into(),
                        // This job is not being triggered by a webhook
                        webhook_delivery_id: None,
                        committed_at: Utc::now(), // Use the current time or extract from commit if available
                    };

                    if let Err(err) = self.register_job(new_job).await {
                        tracing::info!(?err, "Failed to register job for RFD update");
                    } else {
                        tracing::debug!(?rfd_number, ?commit, "Registered job for RFD update");
                    }
                }

                Ok(commit)
            }
            _ => Err(UpdateRfdContentError::InternalState).to_resource_result(),
        }
    }

    #[instrument(skip(self, caller))]
    pub async fn update_rfd_visibility(
        &self,
        caller: &Caller<RfdPermission>,
        rfd_number: i32,
        visibility: Visibility,
    ) -> ResourceResult<Rfd, StoreError> {
        if caller.any(&[
            &RfdPermission::GetRfd(rfd_number),
            &RfdPermission::GetRfdsAll,
        ]) && caller.any(&[
            &RfdPermission::ManageRfdVisibility(rfd_number),
            &RfdPermission::ManageRfdsVisibilityAll,
        ]) {
            let mut rfd = self.get_rfd_meta(caller, rfd_number, None).await?;
            rfd.visibility = visibility;
            RfdStore::upsert(&*self.storage, rfd.into())
                .await
                .to_resource_result()
        } else {
            resource_restricted()
        }
    }

    // Webhook Operations

    pub async fn register_job(&self, new_job: NewJob) -> Result<Job, StoreError> {
        JobStore::upsert(&*self.storage, new_job).await
    }
}

#[cfg(test)]
pub(crate) mod test_mocks {
    use rand::RngCore;
    use rfd_data::content::RfdTemplate;
    use rfd_model::storage::mock::MockStorage;
    use rsa::{
        pkcs8::{EncodePrivateKey, EncodePublicKey, LineEnding},
        RsaPrivateKey, RsaPublicKey,
    };
    use std::sync::Arc;
    use v_api::{
        config::{AsymmetricKey, JwtConfig},
        endpoints::login::oauth::{google::GoogleOAuthProvider, OAuthProviderName},
        VContext,
    };
    use v_model::storage::postgres::PostgresStore;

    use crate::config::{
        ContentConfig, GitHubAuthConfig, GitHubConfig, SearchConfig, ServicesConfig,
    };

    use super::RfdContext;

    // Construct a mock context that can be used in tests
    pub async fn mock_context(storage: MockStorage) -> RfdContext {
        let mut content = ContentConfig::default();
        content
            .templates
            .insert("new".to_string(), RfdTemplate::default());
        content
            .templates
            .insert("placeholder".to_string(), RfdTemplate::default());

        let mut rng = rand::thread_rng();
        let bits = 2048;
        let priv_key = RsaPrivateKey::new(&mut rng, bits).expect("Failed to generate a key");
        let pub_key = RsaPublicKey::from(&priv_key);

        let mut kid = [0; 24];
        rng.fill_bytes(&mut kid);

        let key = AsymmetricKey::Local {
            kid: hex::encode(kid),
            private: String::from_utf8(
                priv_key
                    .to_pkcs8_pem(LineEnding::LF)
                    .unwrap()
                    .as_bytes()
                    .to_vec(),
            )
            .unwrap(),
            public: pub_key.to_public_key_pem(LineEnding::LF).unwrap(),
        };

        let mut v_context = VContext::new(
            String::new(),
            Arc::new(PostgresStore::new("").await.unwrap()),
            JwtConfig {
                default_expiration: 0,
            },
            vec![key],
        )
        .await
        .unwrap();
        v_context.insert_oauth_provider(
            OAuthProviderName::Google,
            Box::new(move || {
                Box::new(GoogleOAuthProvider::new(
                    "google_device_client_id".to_string(),
                    "google_device_client_secret".to_string().into(),
                    "google_web_client_id".to_string(),
                    "google_web_client_secret".to_string().into(),
                    None,
                ))
            }),
        );

        let ctx = RfdContext::new(
            "".to_string(),
            Arc::new(storage),
            SearchConfig::default(),
            content,
            ServicesConfig {
                github: GitHubConfig {
                    owner: String::new(),
                    repo: String::new(),
                    path: String::new(),
                    default_branch: String::new(),
                    auth: GitHubAuthConfig::User {
                        token: String::default(),
                    },
                },
            },
            v_context,
        )
        .await
        .unwrap();

        ctx
    }
}
