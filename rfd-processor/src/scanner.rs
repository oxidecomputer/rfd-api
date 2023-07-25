use rfd_model::{
    storage::{JobStore, StoreError},
    NewJob,
};
use std::{sync::Arc, time::Duration};
use thiserror::Error;
use tokio::time::interval;

use crate::{
    context::Context,
    github::{GitHubError, GitHubRfdUpdate},
};

#[derive(Debug, Error)]
pub enum ScannerError {
    #[error(transparent)]
    GitHub(#[from] GitHubError),
    #[error(transparent)]
    Storage(#[from] StoreError),
}

pub async fn scanner(ctx: Arc<Context>) -> Result<(), ScannerError> {
    let mut interval = interval(Duration::from_secs(1));
    interval.tick().await;

    loop {
        let updates = ctx
            .github
            .repository
            .get_rfd_sync_updates(&ctx.github.client)
            .await?;

        for update in updates {
            JobStore::upsert(&ctx.db.storage, update.into()).await?;
        }
    }
}

impl From<GitHubRfdUpdate> for NewJob {
    fn from(value: GitHubRfdUpdate) -> Self {
        Self {
            owner: value.location.owner,
            repository: value.location.repo,
            branch: value.location.branch,
            sha: value.location.commit,
            rfd: value.number.into(),
            webhook_delivery_id: None,
            committed_at: value.committed_at,
        }
    }
}
