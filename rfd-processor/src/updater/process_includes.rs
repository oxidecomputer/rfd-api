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
        tracing::info!("Processing inculdes");
        let RfdUpdateActionContext { ctx, update, .. } = ctx;

        let content = new
            .content()
            .map_err(|err| RfdUpdateActionErr::Continue(Box::new(err)))?;
        let (format, new_content) = match &content.content {
            RfdContent::Asciidoc(adoc) => {
                let mut raw = adoc.raw().to_string();
                let includes = adoc.includes();

                // Ensure that we only do the work of downloading supporting documents if there
                // are include macros to process
                if includes.len() > 0 {
                    let documents = update
                        .location
                        .download_supporting_documents(&ctx.github.client, &update.number)
                        .await
                        .map_err(|err| RfdUpdateActionErr::Continue(Box::new(err)))?;

                    tracing::trace!(?documents, "Retrieved supporting documents from GitHub");
                    for include in includes {
                        tracing::info!(?include, "Handle include");

                        if let Some(document) = documents.iter().find(|document| {
                            let trimmed_path = document
                                .path
                                .trim_start_matches(&ctx.github.repository.path)
                                .trim_start_matches('/')
                                .trim_start_matches(&update.number.as_number_string())
                                .trim_start_matches('/');

                            tracing::debug!(path = ?document.path, ?trimmed_path, name = include.name(), "Test include name against path");
                            trimmed_path == include.name()
                        }) {
                            if let Some(content) = document.to_text() {
                                tracing::info!(name = include.name(), file = document.name, "Found include match. Replacing contents");
                                raw = include.perform_replacement(&raw, &content);
                            } else {
                                tracing::warn!(?include, "Non UTF-8 files can not be included")
                            }
                        }
                    }
                }

                (ContentFormat::Asciidoc, raw)
            }
            RfdContent::Markdown(_) => (ContentFormat::Markdown, content.raw().to_string()),
        };

        tracing::trace!("Processed all includes");
        new.set_content(format, &new_content);

        Ok(RfdUpdateActionResponse {
            requires_source_commit: false,
        })
    }
}
