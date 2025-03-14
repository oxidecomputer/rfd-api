// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use async_trait::async_trait;
use google_storage1::api::Object;
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

            let cursor = std::io::Cursor::new(data);

            for location in &ctx.assets.locations {
                tracing::info!(bucket = ?location.bucket, ?object_name, "Writing to location");

                if mode == RfdUpdateMode::Write {
                    // TODO: Move implementation to a trait and abstract over different storage systems
                    if let Err(err) = ctx
                        .assets
                        .client
                        .objects()
                        .insert(Object::default(), &location.bucket)
                        .name(&object_name)
                        .upload(cursor.clone(), mime_type.clone())
                        .await
                    {
                        tracing::error!(?err, "Failed to upload static file to GCP");
                    }
                }
            }
        }

        Ok(RfdUpdateActionResponse::default())
    }
}
