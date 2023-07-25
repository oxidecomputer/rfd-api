use async_trait::async_trait;
use rfd_model::{
    schema_ext::PdfSource,
    storage::{ConnectionError, DbError, PoolError, RfdPdfStore, StoreError},
    NewRfdPdf,
};
use tracing::instrument;
use uuid::Uuid;

use crate::{
    content::RfdOutputError,
    context::Context,
    github::GitHubRfdUpdate,
    pdf::{PdfStorage, RfdPdfError},
    rfd::PersistedRfd,
};

use super::{RfdUpdateAction, RfdUpdateActionContext, RfdUpdateActionErr, RfdUpdateActionResponse};

#[derive(Debug)]
pub struct UpdatePdfs;

impl UpdatePdfs {
    async fn upload(
        ctx: &Context,
        update: &GitHubRfdUpdate,
        new: &mut PersistedRfd,
    ) -> Result<Vec<String>, RfdOutputError> {
        // Generate the PDFs for the RFD
        let pdf = match new
            .content()
            .to_pdf(&ctx.github.client, &update.number, &update.location)
            .await
        {
            Ok(pdf) => {
                tracing::info!("Uploaded RFD pdf");
                pdf
            }
            Err(err) => {
                match &err {
                    RfdOutputError::FormatNotSupported => {
                        tracing::info!("RFD is not in a format that supports PDF output");

                        // If an RFD does not support PDF output than we do not want to report an
                        // error. We return early instead
                        return Ok(vec![]);
                    }
                    inner => {
                        tracing::error!(?inner, "Failed trying to generate PDF for RFD");
                        return Err(err);
                    }
                }
            }
        };

        // Upload the generate PDF
        let store_results = ctx.pdf.store_rfd_pdf(&new.get_pdf_filename(), &pdf).await;

        Ok(store_results
            .into_iter()
            .enumerate()
            .filter_map(|(i, result)| match result {
                Ok(file) => file.url().map(|s| s.to_string()),
                Err(err) => {
                    tracing::error!(?err, storage_index = i, "Failed to store PDF");
                    None
                }
            })
            .collect::<Vec<_>>())
    }

    #[instrument(skip(ctx, previous, new), fields(id = ?new.rfd.id, number = new.rfd.rfd_number))]
    async fn delete_old(
        ctx: &Context,
        update: &GitHubRfdUpdate,
        previous: &Option<&PersistedRfd>,
        new: &mut PersistedRfd,
    ) -> Result<(), Vec<RfdPdfError>> {
        let old_pdf_filename = previous.map(|rfd| rfd.get_pdf_filename());

        // If the PDF filename has changed (likely due to a title change for an RFD), then ensure
        // that the old PDF files are deleted
        if let Some(old_pdf_filename) = old_pdf_filename {
            let new_pdf_filename = new.get_pdf_filename();

            if old_pdf_filename != new_pdf_filename {
                // Delete the old filename from drive. It is expected that the target drive and
                // folder already exist
                let results = ctx.pdf.remove_rfd_pdf(&old_pdf_filename).await;

                if !results.is_empty() {
                    return Err(results);
                }

                tracing::info!(
                    old_name = old_pdf_filename,
                    new_name = new_pdf_filename,
                    "Deleted old pdf file in Google Drive due to differing names",
                );
            }
        }

        Ok(())
    }
}

#[async_trait]
impl RfdUpdateAction for UpdatePdfs {
    #[instrument(skip(self, ctx, new), err(Debug))]
    async fn run(
        &self,
        ctx: &mut RfdUpdateActionContext,
        new: &mut PersistedRfd,
    ) -> Result<RfdUpdateActionResponse, RfdUpdateActionErr> {
        let RfdUpdateActionContext {
            ctx,
            previous,
            update,
            ..
        } = ctx;

        // On each update a PDF is uploaded (possibly overwriting an existing file)
        let urls = Self::upload(ctx, update, new)
            .await
            .map_err(|err| RfdUpdateActionErr::Continue(Box::new(err)))?;

        // Under the assumption that the PDF filename only changes if the revision id has also
        // changed, then this upsert will only create a new rows per revision. In any other case,
        // the upsert will hit a constraint conflict and drop the insert. The upsert call itself
        // should handle this case.
        for url in urls {
            tracing::trace!(?new.revision.id, source = ?PdfSource::Google, url, "Attempt to upsert PDF record");

            let response = RfdPdfStore::upsert(
                &ctx.db.storage,
                NewRfdPdf {
                    id: Uuid::new_v4(),
                    rfd_revision_id: new.revision.id,
                    source: PdfSource::Google,
                    link: url,
                },
            )
            .await;

            match response {
                Ok(_) =>
                /* Upsert succeeded, nothing to do */
                {
                    ()
                }

                // A not found error will be returned in the case of a conflict. This is expected
                // and should not cause the function to return
                Err(StoreError::Pool(PoolError::Connection(ConnectionError::Query(
                    DbError::NotFound,
                )))) => {
                    tracing::debug!("Dropping not found database response");
                }
                Err(err) => {
                    tracing::warn!(?err, "Updating RFD pdf link records failed");
                    return Err(RfdUpdateActionErr::Continue(Box::new(err)));
                }
            }
        }

        // Delete will remove any files that no longer match the new PDF filename. This is
        // problematic when handling updates out of order.

        // TODO: Fix deletes to ensure that current files are never deleted. Should files never be
        // deleted? There are also no guarantees that the files to be deleted exist at all. If PDFs
        // are not to be kept indefinitely, the correct option is likely to not run the PDF action
        Self::delete_old(ctx, update, previous, new)
            .await
            .map_err(|err| {
                RfdUpdateActionErr::Continue(Box::new(err.into_iter().nth(0).unwrap()))
            })?;

        // TODO: Do database records need to be deleted? If PDFs are not deleted then there is
        // nothing to do here. If they are deleted records should likely be removed as well.

        Ok(RfdUpdateActionResponse::default())
    }
}
