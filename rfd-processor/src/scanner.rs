// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use diesel::result::{DatabaseErrorKind, Error as DieselError};
use rfd_github::{GitHubError, GitHubRfdUpdate};
use rfd_model::{storage::JobStore, NewJob};
use std::sync::Arc;
use thiserror::Error;
use tokio::time::interval;
use v_model::storage::StoreError;

use crate::context::Context;

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
        if ctx.scanner.enabled {
            let updates = ctx
                .github
                .repository
                .get_rfd_sync_updates(&ctx.github.client)
                .await?;

            for update in updates {
                match JobStore::upsert(&ctx.db.storage, update.clone().into_job()).await {
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
        }

        interval.tick().await;
    }
}

pub trait IntoJob {
    fn into_job(self) -> NewJob;
}

impl IntoJob for GitHubRfdUpdate {
    fn into_job(self) -> NewJob {
        NewJob {
            owner: self.location.owner,
            repository: self.location.repo,
            branch: self.location.branch,
            sha: self.location.commit,
            rfd: self.number.into(),
            webhook_delivery_id: None,
            committed_at: self.committed_at,
        }
    }
}
