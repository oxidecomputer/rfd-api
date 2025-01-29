// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use async_trait::async_trait;
use rfd_data::content::{RfdAsciidoc, RfdDocument};
use std::{path::PathBuf, process::Command};

use crate::util::write_file;

use super::{RenderableRfdError, RenderedPdf, RfdOutputError, RfdRenderedFormat};

#[async_trait]
impl<'a> RfdRenderedFormat<RfdAsciidoc<'a>> for RenderedPdf {
    async fn render(content: &RfdAsciidoc, content_dir: PathBuf) -> Result<Self, RfdOutputError> {
        let file_path = content_dir.join("contents.adoc");

        // Write the contents to a temporary file.
        write_file(&file_path, content.raw().as_bytes()).await?;
        tracing::info!("Wrote file to temp dir");

        let mut command = Command::new("asciidoctor-pdf");
        command.current_dir(content_dir.clone()).args([
            "-o",
            "-",
            "-r",
            "asciidoctor-mermaid/pdf",
            "-a",
            "source-highlighter=rouge",
            file_path.to_str().unwrap(),
        ]);

        let cmd_output = tokio::task::spawn_blocking(move || {
            tracing::info!(?file_path, "Shelling out to asciidoctor");

            // Verify the expected resources exist
            tracing::info!(?file_path, exists = file_path.exists(), "Check document");

            let out = command.output();

            match &out {
                Ok(_) => tracing::info!(?file_path, "Command succeeded"),
                Err(err) => tracing::info!(?file_path, ?err, "Command failed"),
            };

            out
        })
        .await??;

        tracing::info!("Completed asciidoc rendering");

        if cmd_output.status.success() {
            Ok(cmd_output.stdout.into())
        } else {
            Err(RenderableRfdError::ParserFailed(String::from_utf8(
                cmd_output.stderr,
            )))?
        }
    }
}
