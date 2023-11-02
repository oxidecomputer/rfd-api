use rfd_model::storage::{JobFilter, JobStore, ListPagination, StoreError};
use std::sync::Arc;
use tap::TapFallible;
use thiserror::Error;
use tokio::time::interval;

use crate::{
    context::Context,
    github::{GitHubRfdLocation, GitHubRfdUpdate},
    updater::RfdUpdater,
};

#[derive(Debug, Error)]
pub enum JobError {
    #[error(transparent)]
    Storage(#[from] StoreError),
}

pub async fn processor(ctx: Arc<Context>) -> Result<(), JobError> {
    let mut interval = interval(ctx.processor.interval);
    interval.tick().await;

    let pagination = ListPagination::default().limit(ctx.processor.batch_size);

    loop {
        let jobs = JobStore::list(
            &ctx.db.storage,
            JobFilter::default().processed(Some(false)).started(Some(false)),
            &pagination,
        )
        .await?;

        for job in jobs {
            let ctx = ctx.clone();
            tokio::spawn(async move {
                // Make the job as started
                match JobStore::start(&ctx.db.storage, job.id).await {
                    Ok(Some(job)) => {

                        let location = GitHubRfdLocation {
                            owner: job.owner.clone(),
                            repo: job.repository.clone(),
                            branch: job.branch.clone(),
                            commit: job.sha.clone(),
                            default_branch: ctx.github.repository.default_branch.clone(),
                        };

                        let update = GitHubRfdUpdate {
                            location,
                            number: job.rfd.into(),
                            committed_at: job.committed_at,
                        };

                        let updater = RfdUpdater::new(&ctx.actions, ctx.processor.update_mode);

                        match updater.handle(&ctx, &[update]).await {
                            Ok(_) => {
                                let _ = JobStore::complete(&ctx.db.storage, job.id)
                                    .await
                                    .tap_err(|err| {
                                        tracing::error!(?err, "Failed to mark job as completed")
                                    });
                            }
                            Err(err) => {
                                tracing::error!(?err, "RFD update failed");

                                // TODO: Mark job as failed or retry?
                            }
                        }
                    }
                    Ok(None) => {
                        tracing::error!(?job, "Job that was scheduled to run has gone missing! Was it started by a different task?");
                    }
                    Err(err) => {
                        tracing::warn!(?job, ?err, "Failed to start job. Was it previously started?");
                    }
                }

                Ok::<_, JobError>(())
            });
        }

        interval.tick().await;
    }
}
