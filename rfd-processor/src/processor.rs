// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use futures::TryFutureExt;
use rfd_github::{GitHubRfdLocation, GitHubRfdUpdate};
use rfd_model::{
    storage::{JobFilter, JobStore},
    Job,
};
use v_model::storage::{ListPagination, StoreError};
use std::sync::Arc;
use tap::TapFallible;
use thiserror::Error;
use tokio::time::interval;
use tracing::instrument;

use crate::{context::Context, updater::RfdUpdater};

#[derive(Debug, Error)]
pub enum JobError {
    #[error(transparent)]
    Storage(#[from] StoreError),
}

pub async fn processor(ctx: Arc<Context>) -> Result<(), JobError> {
    let mut interval = interval(ctx.processor.interval);
    let pagination = ListPagination::default().limit(ctx.processor.batch_size);
    tracing::info!(?interval, ?pagination, "Starting processor");

    interval.tick().await;

    loop {
        if ctx.processor.enabled {
            let jobs = JobStore::list(
                &ctx.db.storage,
                JobFilter::default()
                    .processed(Some(false))
                    .started(Some(false)),
                &pagination,
            )
            .await?;

            tracing::info!(jobs = ?jobs.iter().map(|job| job.id).collect::<Vec<_>>(), "Spawning jobs");

            for job in jobs {
                let job_id = job.id;
                tracing::info!("Starting job processing");
                match JobStore::start(&ctx.db.storage, job.id).await {
                    Ok(Some(job)) => {
                        tracing::info!(job = ?job_id, "Spawning job");
                        tokio::spawn(run_job(ctx.clone(), job).or_else(move |err| async move {
                            tracing::error!(id = ?job_id, ?err, "Spawned job failed");
                            Err(err)
                        }));
                    }
                    Ok(None) => {
                        tracing::error!(?job, "Job that was scheduled to run has gone missing! Was it started by a different task?");
                    }
                    Err(err) => {
                        tracing::warn!(
                            ?job,
                            ?err,
                            "Failed to start job. Was it previously started?"
                        );
                    }
                }
            }
        }

        interval.tick().await;
    }
}

#[instrument(skip(ctx, job), fields(id = job.id, rfd = job.rfd, sha = ?job.sha, commited_at = ?job.committed_at))]
async fn run_job(ctx: Arc<Context>, job: Job) -> Result<(), JobError> {
    tracing::info!("Running job");

    let location = GitHubRfdLocation {
        client: ctx.github.client.clone(),
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
                .tap_err(|err| tracing::error!(?err, "Failed to mark job as completed"));
        }
        Err(err) => {
            tracing::error!(?err, "RFD update failed");

            // TODO: Mark job as failed or retry?
        }
    }

    Ok::<_, JobError>(())
}
