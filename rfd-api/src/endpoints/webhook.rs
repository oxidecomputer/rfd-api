use async_trait::async_trait;
use chrono::{DateTime, Utc};
use dropshot::{
    endpoint, ApiEndpointBodyContentType, ExtensionMode, ExtractorMetadata, HttpError,
    HttpResponseAccepted, RequestContext, ServerContext, SharedExtractor,
};
use dropshot_verified_body::{hmac::HmacVerifiedBody, services::github::GitHubWebhookVerification};
use http::HeaderName;
use regex::Regex;
use rfd_model::NewJob;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;
use uuid::Uuid;

use crate::{context::ApiContext, error::ApiError};

#[endpoint {
    method = POST,
    path = "/github",
    tags = ["hidden"],
}]
#[instrument(
    skip(rqctx, body),
    fields(
        request_id = rqctx.request_id,
        gh_delivery = ?rqctx.request.headers().get(GITHUB_DELIVERY_HEADER),
        gh_event = ?rqctx.request.headers().get(GITHUB_EVENT_HEADER)
    ),
    err(Debug)
)]
pub async fn github_webhook(
    rqctx: RequestContext<ApiContext>,
    DeliveryId(delivery_id): DeliveryId,
    EventType(event): EventType,
    body: HmacVerifiedBody<GitHubWebhookVerification, GitHubCommitPayload>,
) -> Result<HttpResponseAccepted<()>, HttpError> {
    let new_jobs = body.into_inner()?.create_jobs(delivery_id);

    for new_job in new_jobs {
        let job = rqctx
            .context()
            .register_job(new_job)
            .await
            .map_err(ApiError::Storage)?;
        tracing::info!(?job, "Registered job");
    }

    Ok(HttpResponseAccepted(()))
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct GitHubCommitPayload {
    #[serde(rename = "ref")]
    pub ref_: String,
    pub commits: Vec<GitHubCommit>,
    pub head_commit: Option<GitHubCommit>,
    pub repository: GitHubRepository,
    pub sender: GitHubSender,
    pub installation: GitHubInstallation,
}

impl GitHubCommitPayload {
    pub fn create_jobs(&self, delivery_id: Uuid) -> Vec<NewJob> {
        self.affected_rfds()
            .into_iter()
            .filter_map(|rfd| {
                if let Some(head_commit) = &self.head_commit {
                    Some(NewJob {
                        owner: self.repository.owner.login.clone(),
                        repository: self.repository.name.clone(),
                        branch: self.branch().to_string(),
                        sha: head_commit.id.clone(),
                        rfd,
                        webhook_delivery_id: Some(delivery_id),
                        committed_at: head_commit.timestamp.clone(),
                    })
                } else {
                    None
                }
            })
            .collect()
    }

    fn branch(&self) -> &str {
        self.ref_.trim_start_matches("refs/heads/")
    }

    fn affected_rfds(&self) -> Vec<i32> {
        // Check the committed files for changes to specific RFDs. Depending on the branch of the
        // commit, changes will be accepted to rejected. Changes on the default repository branch
        // are accepted for all RFDs, but on an RFD specific branch (i.e. 0123) on changes to
        // RFD 123 are accepted. Changes on non-default, non-rfd branches are always rejected
        let pattern = Regex::new(r#"^rfd/(\d{4})/"#).unwrap();
        let branch = self.branch();

        self.changed_files()
            .into_iter()
            .filter_map(|path| {
                pattern.captures(path).and_then(|captures| {
                    if self.is_on_default_branch() {
                        captures
                            .get(1)
                            .and_then(|number| number.as_str().parse::<i32>().ok())
                    } else {
                        captures.get(1).and_then(|number| {
                            if branch == number.as_str() {
                                number.as_str().parse::<i32>().ok()
                            } else {
                                None
                            }
                        })
                    }
                })
            })
            .collect()
    }

    fn is_on_default_branch(&self) -> bool {
        self.ref_ == format!("refs/heads/{}", self.repository.default_branch)
    }

    fn changed_files(&self) -> Vec<&str> {
        self.commits
            .iter()
            .flat_map(|commit| [&commit.added, &commit.modified, &commit.removed])
            .flatten()
            .map(|s| &**s)
            .collect::<Vec<_>>()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct GitHubCommit {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub added: Vec<String>,
    pub modified: Vec<String>,
    pub removed: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct GitHubRepository {
    pub id: u64,
    pub node_id: String,
    pub name: String,
    pub owner: GitHubRepositoryOwner,
    pub default_branch: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct GitHubRepositoryOwner {
    pub login: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct GitHubSender {
    pub id: u64,
    pub node_id: String,
    pub login: String,
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct GitHubInstallation {
    pub id: u64,
    pub node_id: String,
}

pub const GITHUB_DELIVERY_HEADER: HeaderName = HeaderName::from_static("x-github-delivery");

#[derive(Debug)]
pub struct DeliveryId(pub Uuid);

#[async_trait]
impl SharedExtractor for DeliveryId {
    async fn from_request<Context: ServerContext>(
        rqctx: &RequestContext<Context>,
    ) -> Result<DeliveryId, HttpError> {
        let delivery_id = rqctx
            .request
            .headers()
            .get(GITHUB_DELIVERY_HEADER)
            .ok_or(HttpError::for_bad_request(
                None,
                "Request did not present a GitHub delivery id".to_string(),
            ))?
            .to_str()
            .map_err(|err| {
                tracing::warn!(?err, "Failed to read GitHub delivery id header");
                HttpError::for_bad_request(
                    None,
                    "Request presented malformed GitHub delivery id".to_string(),
                )
            })
            .and_then(|header| {
                header.parse::<Uuid>().map_err(|err| {
                    tracing::warn!(?err, "Failed to parse GitHub delivery id header");
                    HttpError::for_bad_request(
                        None,
                        "Request presented invalid GitHub delivery id".to_string(),
                    )
                })
            })?;

        Ok(DeliveryId(delivery_id))
    }

    fn metadata(_body_content_type: ApiEndpointBodyContentType) -> ExtractorMetadata {
        ExtractorMetadata {
            extension_mode: ExtensionMode::None,
            parameters: vec![],
        }
    }
}

pub const GITHUB_EVENT_HEADER: HeaderName = HeaderName::from_static("x-github-event");

#[derive(Debug)]
pub struct EventType(pub String);

#[async_trait]
impl SharedExtractor for EventType {
    async fn from_request<Context: ServerContext>(
        rqctx: &RequestContext<Context>,
    ) -> Result<EventType, HttpError> {
        let event_type = rqctx
            .request
            .headers()
            .get(GITHUB_EVENT_HEADER)
            .ok_or(HttpError::for_bad_request(
                None,
                "Request did not present a GitHub event type".to_string(),
            ))?
            .to_str()
            .map_err(|err| {
                tracing::warn!(?err, "Failed to read GitHub event type header");
                HttpError::for_bad_request(
                    None,
                    "Request presented malformed GitHub event type".to_string(),
                )
            })?
            .to_string();

        Ok(EventType(event_type))
    }

    fn metadata(_body_content_type: ApiEndpointBodyContentType) -> ExtractorMetadata {
        ExtractorMetadata {
            extension_mode: ExtensionMode::None,
            parameters: vec![],
        }
    }
}
