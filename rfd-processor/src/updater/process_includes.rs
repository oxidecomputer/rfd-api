// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use async_trait::async_trait;
use rfd_data::content::{RfdContent, RfdDocument};
use rfd_github::ext::ContentFileExt;
use rfd_model::schema_ext::ContentFormat;
use tracing::instrument;

use crate::rfd::PersistedRfd;

use super::{
    RfdUpdateAction, RfdUpdateActionContext, RfdUpdateActionErr, RfdUpdateActionResponse,
    RfdUpdateMode,
};

#[derive(Debug)]
pub struct ProcessIncludes;

#[async_trait]
impl RfdUpdateAction for ProcessIncludes {
    #[instrument(skip(self, ctx, new), err(Debug))]
    async fn run(
        &self,
        ctx: &mut RfdUpdateActionContext,
        new: &mut PersistedRfd,
        _mode: RfdUpdateMode,
    ) -> Result<RfdUpdateActionResponse, RfdUpdateActionErr> {
        let RfdUpdateActionContext { ctx, update, .. } = ctx;

        let documents = update
            .location
            .download_supporting_documents(&ctx.github.client, &update.number)
            .await
            .map_err(|err| RfdUpdateActionErr::Continue(Box::new(err)))?;

        let content = new
            .content()
            .map_err(|err| RfdUpdateActionErr::Continue(Box::new(err)))?;
        let (format, new_content) = match &content.content {
            RfdContent::Asciidoc(adoc) => {
                let mut raw = adoc.raw().to_string();
                let includes = adoc.includes();
                for include in includes {
                    if let Some(document) = documents
                        .iter()
                        .find(|document| document.name == include.name())
                    {
                        if let Some(content) = document.to_text() {
                            raw = include.perform_replacement(&raw, &content);
                        } else {
                            tracing::warn!(?include, "Non UTF-8 files can not be included")
                        }
                    }
                }
                (ContentFormat::Asciidoc, raw)
            }
            RfdContent::Markdown(_) => (ContentFormat::Markdown, content.raw().to_string()),
        };
        new.set_content(format, &new_content);

        Ok(RfdUpdateActionResponse {
            requires_source_commit: false,
        })
    }
}
