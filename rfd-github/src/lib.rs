// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::{
    borrow::Cow,
    collections::HashMap,
    fmt::Debug,
    future::Future,
    pin::Pin,
    str::{from_utf8, Utf8Error},
};

use base64::{prelude::BASE64_STANDARD, DecodeError, Engine};
use chrono::{DateTime, ParseError, Utc};
use http::StatusCode;
use octorust::{
    types::{GitCreateRefRequest, PullRequestSimple, ReposCreateUpdateFileContentsRequest},
    Client, ClientError, Response,
};
use regex::Regex;
use rfd_data::{
    content::{RfdAsciidoc, RfdContent, RfdMarkdown},
    RfdNumber,
};
use rfd_model::{CommitSha, FileSha};
use thiserror::Error;
use tracing::{instrument, Instrument};

pub mod ext;
mod util;

use ext::ReposExt;
use util::decode_base64;

#[derive(Debug, Error)]
pub enum GitHubError {
    #[error("Internal client error")]
    ClientError(#[from] ClientError),
    #[error("Failed to decode blob")]
    FailedToDecodeData(#[from] DecodeError),
    #[error("Could not find a committer for a commit")]
    FailedToFindCommitter,
    #[error("Failed to parse decoded contents")]
    InvalidData(#[from] Utf8Error),
    #[error("Failed to parse date")]
    InvalidDate(#[from] ParseError),
    #[error("Expected to find at least one commit")]
    NoCommitsFound,
    #[error("Expected to find at least one committer")]
    NoCommitterFound,
}

#[derive(Clone)]
pub struct GitHubRfdRepo {
    pub client: Client,
    pub owner: String,
    pub repo: String,
    pub path: String,
    pub default_branch: String,
}

impl Debug for GitHubRfdRepo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GitHubRfdRepo")
            .field("owner", &self.owner)
            .field("repo", &self.repo)
            .field("path", &self.path)
            .field("default_branch", &self.default_branch)
            .finish()
    }
}

impl GitHubRfdRepo {
    pub async fn new(
        client: &Client,
        owner: String,
        repo: String,
        path: String,
        default_branch: String,
    ) -> Result<Self, GitHubError> {
        Ok(Self {
            client: client.clone(),
            default_branch,
            owner,
            repo,
            path,
        })
    }

    #[instrument(skip(self))]
    pub async fn next_rfd_number(&self) -> Result<GitHubNewRfdNumber, GitHubError> {
        // We need to use two separate calls to try to determine the next available RFD number:
        //  `1. The list of published RFD files that exist on the default branch
        //   2. The list of open branches in the RFD repo that are not yet published

        // Get the latest commit of the default branch
        let default = self
            .client
            .repos()
            .get_branch(&self.owner, &self.repo, &self.default_branch)
            .await?
            .body;

        tracing::debug!(?default.commit.sha, "Fetched default branch");

        // List all of the files that are in the RFD path of the repo
        let rfd_files = match self
            .client
            .repos()
            .get_content_vec_entries(&self.owner, &self.repo, &self.path, &default.commit.sha)
            .await
        {
            Ok(entries) => entries.body,
            Err(ClientError::HttpError { status, .. }) if status == StatusCode::NOT_FOUND => {
                vec![]
            }
            Err(err) => return Err(err)?,
        };

        tracing::info!(count = ?rfd_files.len(), "Fetched repo files");

        let mut rfd_numbers_on_default = vec![];

        // Each RFD should exist at a path that looks like rfd/{number}/README.adoc
        for entry in rfd_files {
            let path_parts = entry.path.split('/').collect::<Vec<&str>>();

            // There should always be exactly 2 parts "rfd" "{number}"
            if path_parts.len() == 2 {
                if let Ok(number) = path_parts[1].parse::<i32>() {
                    rfd_numbers_on_default.push(number);
                } else {
                    tracing::warn!(?path_parts, "Failed to parse RFD number from file path");
                }
            } else {
                tracing::warn!(?path_parts, path = ?entry.path, "Found RFD file with an invalid path");
            }
        }

        let max_rfd_number_on_default = rfd_numbers_on_default.into_iter().max().unwrap_or(0);

        let branches = self.branches().await?;
        let max_rfd_number_on_branch = branches
            .iter()
            .filter_map(|location| location.branch.parse::<i32>().ok())
            .max()
            .unwrap_or(0);

        let next_rfd_number: RfdNumber =
            (max_rfd_number_on_default.max(max_rfd_number_on_branch) + 1).into();

        Ok(GitHubNewRfdNumber {
            number: next_rfd_number,
            commit: default.commit.sha.into(),
        })
    }

    pub async fn branches(&self) -> Result<Vec<GitHubRfdLocation>, GitHubError> {
        let branch_pattern = Regex::new(r#"^\d{4}$"#).unwrap();
        let responses = self
            .client
            .repos()
            .list_all_branches(&self.owner, &self.repo, false)
            .await?;
        Ok(responses
            .body
            .into_iter()
            .filter_map(|branch| {
                if branch_pattern.is_match(&branch.name) || branch.name == self.default_branch {
                    Some(self.location(branch.name, branch.commit.sha.into()))
                } else {
                    None
                }
            })
            .collect())
    }

    #[instrument(skip(self), fields(owner = self.owner, repo = self.repo))]
    pub async fn create_branch(
        &self,
        number: &RfdNumber,
        commit: &CommitSha,
    ) -> Result<GitHubRfdLocation, GitHubError> {
        let ref_ = format!("refs/heads/{}", number.as_number_string());

        tracing::debug!(ref_, "Creating GitHub ref");

        self.client
            .git()
            .create_ref(
                &self.owner,
                &self.repo,
                &GitCreateRefRequest {
                    key: String::new(),
                    ref_,
                    sha: commit.to_string(),
                },
            )
            .await?;

        Ok(self.location(number.as_number_string(), commit.clone()))
    }

    pub fn location(&self, branch: String, commit: CommitSha) -> GitHubRfdLocation {
        GitHubRfdLocation {
            client: self.client.clone(),
            owner: self.owner.clone(),
            repo: self.repo.clone(),
            default_branch: self.default_branch.clone(),
            branch,
            commit,
        }
    }

    pub async fn locations_for_commit(
        &self,
        commit: CommitSha,
    ) -> Result<Vec<GitHubRfdLocation>, GitHubError> {
        let branches = self
            .client
            .repos()
            .list_branches_for_head_commit(&self.owner, &self.repo, commit.0.as_str())
            .await?;
        Ok(branches
            .body
            .into_iter()
            .map(|branch| self.location(branch.name, commit.clone()))
            .collect::<Vec<_>>())
    }

    pub async fn get_rfd_sync_updates(
        &self,
        client: &Client,
    ) -> Result<Vec<GitHubRfdUpdate>, GitHubError> {
        // List of updates to build up
        let mut updates = HashMap::new();

        let default = client
            .repos()
            .get_branch(&self.owner, &self.repo, &self.default_branch)
            .await?
            .body;

        // To get the date of the commit we need to fetch the commit itself
        let default_commit = client
            .repos()
            .get_commit(&self.owner, &self.repo, 0, 1, &default.commit.sha)
            .await?
            .body;
        let default_committed_at = default_commit
            .commit
            .committer
            .ok_or(GitHubError::NoCommitterFound)
            .and_then(|committer| {
                committer
                    .date
                    .parse::<DateTime<Utc>>()
                    .map_err(GitHubError::InvalidDate)
            })?;

        // Get a list of all RFDs that are present on the default repository branch. These should be
        // in a final state (published, committed, or abandoned) and exist in under the rfd/ path
        let rfd_files = client
            .repos()
            .get_content_vec_entries(&self.owner, &self.repo, &self.path, &default.commit.sha)
            .await?
            .body;

        tracing::info!(?rfd_files, "Fetched repo files");

        // Each RFD should exist at a path that looks like rfd/{number}/README.adoc
        for entry in rfd_files {
            tracing::trace!(?entry.name, ?entry.path, ?entry, ?entry.sha, "Processing file on default branch");

            let path_parts = entry.path.split('/').collect::<Vec<&str>>();

            // There should always be exactly 2 parts "rfd" "{number}"
            if path_parts.len() == 2 {
                tracing::trace!(?path_parts, "Handle RFD on default branch");

                if let Ok(number) = path_parts[1].parse::<i32>() {
                    tracing::trace!(
                        ?number,
                        branch = self.default_branch,
                        "Add RFD to update batch"
                    );

                    let update = GitHubRfdUpdate {
                        number: number.into(),
                        location: self.location(
                            self.default_branch.clone(),
                            default.commit.sha.clone().into(),
                        ),
                        committed_at: default_committed_at,
                    };

                    updates.insert(number, update);
                } else {
                    tracing::warn!(?path_parts, "Failed to parse RFD number from file path");
                }
            } else {
                tracing::warn!(?path_parts, path = ?entry.path, "Found RFD file with an invalid path");
            }
        }

        // After collecting the list of RFDs that exist on the default branch, the next step is to
        // iterate through the remote branches and extract RFDs from each one. RFD branches may
        // contain changes to multiple RFDs, but for update processing, only the RFD that matches
        // the branch name is considered

        // TODO: Fix the underlying client so that "protected" argument is Option<bool>
        let branches = client
            .repos()
            .list_all_branches(&self.owner, &self.repo, false)
            .await?
            .body
            .into_iter();

        for branch in branches {
            // Skip the default brach
            if branch.name == self.default_branch {
                tracing::trace!("Skip default branch during branch iteration");
                continue;
            }

            // To get the date of the commit we need to fetch the commit itself
            let commit = client
                .repos()
                .get_commit(&self.owner, &self.repo, 0, 1, &branch.commit.sha)
                .await?
                .body;
            let committed_at = commit
                .commit
                .committer
                .ok_or(GitHubError::NoCommitterFound)
                .and_then(|committer| {
                    committer
                        .date
                        .parse::<DateTime<Utc>>()
                        .map_err(GitHubError::InvalidDate)
                })?;

            let span = tracing::info_span!("Process branch", branch = ?branch.name, commit = ?branch.commit.sha);

            async {
                // There will be many branches that are not RFD branches, and they are ignored
                if let Ok(number) = branch.name.parse::<i32>() {

                    // RFDs that are in final state in the dedebugfault branch AND have a branch are ignored
                    if !updates.contains_key(&number) {

                        let rfd_number = RfdNumber::from(number);

                        // Only interested in exactly the RFD file that matches the branch name
                        let mut response = client.repos().get_content_file(&self.owner, &self.repo, &format!("rfd/{}/README.adoc", rfd_number.as_number_string()), &branch.commit.sha).await;

                        // If we fail to find an Asciidoc readme, try to fall back to a Markdown version
                        if match response {
                            Err(ClientError::HttpError { status, .. }) if status == StatusCode::NOT_FOUND => true,
                            _ => false
                        } {
                            response = client.repos().get_content_file(&self.owner, &self.repo, &format!("rfd/{}/README.md", rfd_number.as_number_string()), &branch.commit.sha).await;
                        }

                        // 404s are returned as errors, but that should not stop processing. This only
                        // means that the branch should be skipped
                        match response {
                            Ok(Response { body: file, status, .. }) if status == StatusCode::OK || status == StatusCode::NOT_MODIFIED => {
                                tracing::debug!(?file.path, "Retrieved RFD file contents");

                                let path_parts = file.path.split('/').collect::<Vec<&str>>();

                                // There should always be exactly 3 parts "rfd" "{number}" "README.adoc"
                                if path_parts.len() == 3 {
                                    tracing::trace!(?number, branch = ?branch.name, "Add RFD to update batch");

                                    // Only RFDs that have a number matching the branch name are considered
                                    if let Ok(number) = path_parts[1].parse::<i32>() {
                                        let update = GitHubRfdUpdate {
                                            number: rfd_number,
                                            location: self.location(branch.name, branch.commit.sha.into()),
                                            committed_at,
                                        };

                                        updates.insert(number, update);
                                    } else {
                                        tracing::warn!(?number, ?path_parts, "Failed to parse RFD number from file path");
                                    }
                                } else {
                                    tracing::warn!(?number, ?path_parts, path = ?file.path, "Found RFD file with an invalid path");
                                }
                            }
                            _ => {
                                tracing::info!(?number, ?response, "Skipping branch due to non-200 response during lookup");
                            }
                        }
                    } else {
                        tracing::debug!(?number, "Ignoring branch for RFD number that is in a final state on the default branch");
                    }
                } else {
                    tracing::trace!("Ignoring branch with non-number name");
                }
            }.instrument(span).await
        }

        Ok(updates.into_iter().map(|(_, v)| v).collect())
    }
}

#[derive(Clone)]
pub struct GitHubRfdLocation {
    pub client: Client,
    pub owner: String,
    pub repo: String,
    pub default_branch: String,
    pub branch: String,
    pub commit: CommitSha,
}

impl Debug for GitHubRfdLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GitHubRfdRepo")
            .field("owner", &self.owner)
            .field("repo", &self.repo)
            .field("default_branch", &self.default_branch)
            .field("branch", &self.branch)
            .field("commit", &self.commit)
            .finish()
    }
}

impl GitHubRfdLocation {
    pub fn is_default(&self) -> bool {
        self.branch == self.default_branch
    }

    pub async fn readme_path(&self, client: &Client, rfd_number: &RfdNumber) -> String {
        // Use the supplied RFD number to determine the location in the RFD repo to read from
        let dir = rfd_number.repo_path();

        // Get the contents of the file
        let mut path = format!("{}/README.adoc", dir);
        match self.fetch_content(&client, &path, &self.commit).await {
            Ok(_) => path,
            Err(err) => {
                tracing::trace!(
                    ?err,
                    "Failed to find asciidoc README, falling back to markdown"
                );

                path = format!("{}/README.md", dir);
                match self.fetch_content(&client, &path, &self.commit).await {
                    Ok(_) => path,
                    Err(err) => {
                        tracing::trace!(
                            ?err,
                            "Failed to find markdown README. With no README detected, defaulting to asciidoc"
                        );

                        format!("{}/README.adoc", dir)
                    }
                }
            }
        }
    }

    /// Checks if this branch actually exists in the remote system (GitHub)
    pub async fn exists_in_remote(&self, client: &Client) -> bool {
        client
            .repos()
            .get_branch(&self.owner, &self.repo, &self.branch)
            .await
            .is_ok()
    }

    /// Try to get the markdown or asciidoc contents from the repo.
    #[instrument(skip(self, client), fields(repo = self.repo, branch = self.branch))]
    pub async fn get_readme_contents<'a>(
        &self,
        client: &Client,
        rfd_number: &RfdNumber,
    ) -> Result<GitHubRfdReadme<'a>, GitHubError> {
        let readme_path = self.readme_path(client, rfd_number).await;
        tracing::info!(?readme_path, "Fetch readme contents");

        let is_markdown = readme_path.ends_with(".md");
        let FetchedRfdContent {
            parsed, sha, url, ..
        } = self
            .fetch_content(&client, &readme_path, &self.commit)
            .await?;

        let content = if is_markdown {
            RfdContent::Markdown(RfdMarkdown::new(Cow::Owned(parsed)))
        } else {
            RfdContent::Asciidoc(RfdAsciidoc::new(Cow::Owned(parsed)))
        };

        // The html_url for the README.* file will look something like:
        //   https://github.com/<owner>/<repo>/blob/<number>/rfd/<number>/README.*
        // and it needs to be in the form:
        //   https://github.com/<owner>/<repo>/tree/<number>/rfd/<number>
        let tree_link = url
            .rsplit_once('/')
            .map(|(dir, _)| dir.replace("blob", "tree"));

        Ok(GitHubRfdReadme {
            content,
            sha: sha.into(),
            location: GitHubRfdReadmeLocation {
                file: readme_path,
                blob_link: url,
                tree_link,
                branch: self.clone(),
            },
        })
    }

    async fn fetch_content(
        &self,
        client: &Client,
        path: &str,
        ref_: &CommitSha,
    ) -> Result<FetchedRfdContent, GitHubError> {
        let file = client
            .repos()
            .get_content_blob(&self.owner, &self.repo, ref_.0.as_str(), path)
            .await?;

        let decoded = decode_base64(&file.content)?;
        let parsed = from_utf8(&decoded)?.to_string();

        Ok(FetchedRfdContent {
            decoded,
            parsed,
            sha: file.sha.into(),
            url: file.html_url,
        })
    }

    /// Get a list of images that are store in this branch
    pub async fn get_images(
        &self,
        client: &Client,
        rfd_number: &RfdNumber,
    ) -> Result<Vec<octorust::types::ContentFile>, GitHubError> {
        let dir = rfd_number.repo_path();
        Self::get_images_internal(client, &self.owner, &self.repo, &self.commit, dir).await
    }

    #[instrument(skip(client, dir))]
    fn get_images_internal<'a>(
        client: &'a Client,
        owner: &'a String,
        repo: &'a String,
        ref_: &'a CommitSha,
        dir: String,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<Vec<octorust::types::ContentFile>, GitHubError>> + Send + 'a,
        >,
    > {
        Box::pin(async move {
            let mut files: Vec<octorust::types::ContentFile> = Default::default();

            let resp = client
                .repos()
                .get_content_vec_entries(owner, repo, &dir, ref_.0.as_str())
                .await?;

            for file in resp.body {
                tracing::info!(file.path, file.type_, "Processing git entry");

                if file.type_ == "dir" {
                    let images =
                        Self::get_images_internal(client, owner, repo, ref_, file.path).await?;

                    for image in images {
                        files.push(image)
                    }
                } else if is_image(&file.name) {
                    let file = client
                        .repos()
                        .get_content_blob(owner, repo, ref_.0.as_str(), &file.path)
                        .await?;
                    files.push(file);
                }
            }

            Ok(files)
        })
    }

    /// Find any existing pull request coming from the branch for this RFD
    pub async fn find_pull_requests(
        &self,
        client: &Client,
    ) -> Result<Vec<PullRequestSimple>, GitHubError> {
        // If this is an update occurring on the default branch than it can skip the look up as
        // only want pull requests that are coming from a non-default branch are relevant
        let prs = if self.branch != self.default_branch {
            let pulls = client
                .pulls()
                .list_all(
                    &self.owner,
                    &self.repo,
                    octorust::types::IssuesListState::All,
                    // Only pull requests that are targeting the default branch should be used for
                    // updating
                    &format!("{}:{}", self.owner, self.branch),
                    &self.default_branch,
                    // Pull requests are expected to come from within the organization
                    Default::default(),
                    Default::default(),
                )
                .await?
                .body;

            pulls
                .into_iter()
                .filter_map(|pull| {
                    let pull_branch = pull.head.ref_.trim_start_matches("refs/heads/");

                    // These should never differ as long as the GitHub API is returning correct
                    // results
                    if pull_branch == self.branch {
                        Some(pull.into())
                    } else {
                        tracing::warn!(?pull, "Detected invalid pull request");
                        None
                    }
                })
                .collect::<Vec<_>>()
        } else {
            vec![]
        };

        Ok(prs)
    }

    pub async fn get_commit_date(
        &self,
        client: &Client,
        rfd_number: &RfdNumber,
    ) -> Result<DateTime<Utc>, GitHubError> {
        let commits = client
            .repos()
            .list_commits(
                &self.owner,
                &self.repo,
                &self.commit.0.as_str(),
                &rfd_number.repo_path(),
                "",
                None,
                None,
                0,
                0,
            )
            .await?
            .body;
        let latest_commit = commits
            .into_iter()
            .next()
            .ok_or_else(|| GitHubError::NoCommitsFound)?;

        Ok(latest_commit
            .commit
            .committer
            .as_ref()
            .ok_or_else(|| GitHubError::FailedToFindCommitter)?
            .date
            .parse()?)
    }

    #[instrument(skip(self, content))]
    pub async fn upsert(
        &self,
        rfd_number: &RfdNumber,
        content: &[u8],
        message: &str,
    ) -> Result<Option<CommitSha>, GitHubError> {
        let readme_path = self.readme_path(&self.client, rfd_number).await;
        let (decoded, sha) = match self
            .fetch_content(&self.client, &readme_path, &self.commit)
            .await
        {
            Ok(FetchedRfdContent { decoded, sha, .. }) => (decoded, sha.into()),
            Err(err) => match err {
                GitHubError::ClientError(ClientError::HttpError { status, .. })
                    if status == StatusCode::NOT_FOUND =>
                {
                    (vec![], String::new())
                }
                other => return Err(other),
            },
        };

        // We can short circuit if the new and old content are the same
        if content == &decoded {
            tracing::info!("File contents are the same. Skipping commit");
            return Ok(None);
        }

        tracing::info!(
            old = decoded.len(),
            new = content.len(),
            "Writing file to GitHub"
        );

        let response = self
            .client
            .repos()
            .create_or_update_file_contents(
                &self.owner,
                &self.repo,
                &readme_path.trim_start_matches('/'),
                &ReposCreateUpdateFileContentsRequest {
                    message: format!("{}\nCommitted via rfd-api", message),
                    sha,
                    branch: self.branch.clone(),
                    content: BASE64_STANDARD.encode(content),
                    committer: Default::default(),
                    author: Default::default(),
                },
            )
            .await?;

        Ok(Some(response.body.commit.sha.into()))
    }
}

#[derive(Clone)]
struct GitHubPullRequestComments {
    pub client: Client,
}

impl GitHubPullRequestComments {
    async fn comments(&self) {
        let pulls = self.client.pulls();
        let comments = pulls
            .list_all_review_comments(
                "owner",
                "repo",
                0,
                octorust::types::Sort::Created,
                octorust::types::Order::Desc,
                None,
            )
            .await
            .unwrap();
    }
}

struct FetchedRfdContent {
    decoded: Vec<u8>,
    parsed: String,
    sha: FileSha,
    url: String,
}

#[derive(Debug)]
pub struct GitHubRfdReadme<'a> {
    pub content: RfdContent<'a>,
    pub sha: FileSha,
    pub location: GitHubRfdReadmeLocation,
}

#[derive(Debug)]
pub struct GitHubRfdReadmeLocation {
    pub file: String,
    pub blob_link: String,
    pub tree_link: Option<String>,
    pub branch: GitHubRfdLocation,
}

#[derive(Debug, Clone)]
pub struct GitHubRfdUpdate {
    pub number: RfdNumber,
    pub location: GitHubRfdLocation,
    pub committed_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct GitHubNewRfdNumber {
    pub number: RfdNumber,
    pub commit: CommitSha,
}

// TODO: Expand this
pub fn is_image(file: &str) -> bool {
    file.ends_with(".svg")
        || file.ends_with(".png")
        || file.ends_with(".jpg")
        || file.ends_with(".jpeg")
}
