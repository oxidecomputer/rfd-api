use diesel::result::{DatabaseErrorKind, Error as DieselError};
use rfd_model::{
    storage::{JobStore, StoreError},
    NewJob,
};
use std::sync::Arc;
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
    let mut interval = interval(ctx.scanner.interval);
    interval.tick().await;

    loop {
        let updates = ctx
            .github
            .repository
            .get_rfd_sync_updates(&ctx.github.client)
            .await?;

        for update in updates {
            match JobStore::upsert(&ctx.db.storage, update.clone().into()).await {
                Ok(job) => tracing::trace!(?job.id, "Added job to the queue"),
                Err(err) => {
                    match err {
                        StoreError::Db(DieselError::DatabaseError(
                            DatabaseErrorKind::UniqueViolation,
                            _,
                        )) => {
                            // Nothing to do here, we expect uniqueness conflicts. It is expected
                            // that the scanner picks ups redundant jobs for RFDs that have not
                            // changed since the last scan
                        }
                        err => {
                            tracing::warn!(?err, ?update, "Failed to add job")
                        }
                    }
                }
            }
        }

        interval.tick().await;
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
