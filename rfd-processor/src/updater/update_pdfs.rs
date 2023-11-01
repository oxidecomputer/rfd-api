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
    pdf::{PdfFileLocation, PdfStorage},
    rfd::PersistedRfd,
};

use super::{
    RfdUpdateAction, RfdUpdateActionContext, RfdUpdateActionErr, RfdUpdateActionResponse,
    RfdUpdateMode,
};

#[derive(Debug)]
pub struct UpdatePdfs;

impl UpdatePdfs {
    async fn upload(
        ctx: &Context,
        update: &GitHubRfdUpdate,
        new: &mut PersistedRfd,
        mode: RfdUpdateMode,
    ) -> Result<Vec<PdfFileLocation>, RfdOutputError> {
        // Generate the PDFs for the RFD
        let pdf = match new
            .content()
            .to_pdf(&ctx.github.client, &update.number, &update.location)
            .await
        {
            Ok(pdf) => {
                tracing::info!("Generated RFD pdf");
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
        let store_results = match mode {
            RfdUpdateMode::Read => Vec::new(),
            RfdUpdateMode::Write => {
                ctx.pdf
                    .store_rfd_pdf(
                        new.pdf_external_id.as_ref().map(|s| s.as_str()),
                        &new.get_pdf_filename(),
                        &pdf,
                    )
                    .await
            }
        };

        Ok(store_results
            .into_iter()
            .enumerate()
            .filter_map(|(i, result)| match result {
                Ok(file) => Some(file),
                Err(err) => {
                    tracing::error!(?err, storage_index = i, "Failed to store PDF");
                    None
                }
            })
            .collect::<Vec<_>>())
    }
}

#[async_trait]
impl RfdUpdateAction for UpdatePdfs {
    #[instrument(skip(self, ctx, new), err(Debug))]
    async fn run(
        &self,
        ctx: &mut RfdUpdateActionContext,
        new: &mut PersistedRfd,
        mode: RfdUpdateMode,
    ) -> Result<RfdUpdateActionResponse, RfdUpdateActionErr> {
        let RfdUpdateActionContext { ctx, update, .. } = ctx;

        // On each update a PDF is uploaded (possibly overwriting an existing file)
        let pdf_locations = Self::upload(ctx, update, new, mode)
            .await
            .map_err(|err| RfdUpdateActionErr::Continue(Box::new(err)))?;

        // Under the assumption that the PDF filename only changes if the revision id has also
        // changed, then this upsert will only create a new rows per revision. In any other case,
        // the upsert will hit a constraint conflict and drop the insert. The upsert call itself
        // should handle this case.
        for pdf_location in pdf_locations {
            tracing::trace!(?new.revision.id, source = ?PdfSource::Google, ?pdf_location, "Attempt to upsert PDF record");

            let response = RfdPdfStore::upsert(
                &ctx.db.storage,
                NewRfdPdf {
                    id: Uuid::new_v4(),
                    rfd_revision_id: new.revision.id,
                    source: PdfSource::Google,
                    link: pdf_location.url,
                    rfd_id: new.rfd.id,
                    external_id: pdf_location.external_id,
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

        Ok(RfdUpdateActionResponse::default())
    }
}
