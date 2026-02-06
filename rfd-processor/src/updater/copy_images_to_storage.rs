// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use async_trait::async_trait;
use tracing::instrument;

use crate::{rfd::PersistedRfd, util::decode_base64};

use super::{
    RfdUpdateAction, RfdUpdateActionContext, RfdUpdateActionErr, RfdUpdateActionResponse,
    RfdUpdateMode,
};

#[derive(Debug)]
pub struct CopyImagesToStorage;

#[async_trait]
impl RfdUpdateAction for CopyImagesToStorage {
    #[instrument(skip(self, ctx, _new), err(Debug))]
    async fn run(
        &self,
        ctx: &mut RfdUpdateActionContext,
        _new: &mut PersistedRfd,
        mode: RfdUpdateMode,
    ) -> Result<RfdUpdateActionResponse, RfdUpdateActionErr> {
        let RfdUpdateActionContext { ctx, update, .. } = ctx;

        let images = update
            .location
            .download_supporting_documents(&ctx.github.client, &update.number)
            .await
            .map_err(|err| RfdUpdateActionErr::Continue(Box::new(err)))?;

        for image in images {
            let sub_path = image
                .path
                .replace(&format!("rfd/{}/", update.number.as_number_string()), "");
            let object_name = format!("rfd/{}/latest/{}", update.number, sub_path);
            let mime_type = mime_guess::from_path(&object_name).first_or_octet_stream();
            let data = decode_base64(&image.content)
                .map_err(|err| RfdUpdateActionErr::Continue(Box::new(err)))?;

            tracing::info!(
                ?object_name,
                ?mime_type,
                size = data.len(),
                "Writing file to storage buckets"
            );

            for storage in &ctx.static_storage {
                tracing::info!(name = storage.name(), ?object_name, "Writing to storage");

                if mode == RfdUpdateMode::Write {
                    if let Err(err) = storage
                        .put(&object_name, data.clone(), mime_type.as_ref())
                        .await
                    {
                        tracing::error!(
                            name = storage.name(),
                            ?err,
                            "Failed to upload static file"
                        );
                    }
                } else {
                    tracing::warn!(
                        "CopyImagesToStorage is enabled however RfdUpdateMode is not write: {:?}",
                        mode
                    );
                }
            }
        }

        Ok(RfdUpdateActionResponse::default())
    }
}
