// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use async_trait::async_trait;
use rfd_data::content::{RfdAsciidoc, RfdContentError, RfdDocument};
use std::{path::PathBuf, process::Command};

use crate::util::write_file;

use super::{RenderableRfdError, RenderedPdf, RfdOutputError, RfdRenderedFormat};

pub(crate) async fn render_pdf_from_dir(
    content_dir: PathBuf,
) -> Result<RenderedPdf, RfdOutputError> {
    let content_dir = content_dir.canonicalize()?;
    let contents = std::fs::read_to_string(content_dir.join("contents.adoc"))?;
    let content = RfdAsciidoc::new(contents)
        .map_err(RfdContentError::Asciidoc)
        .map_err(RenderableRfdError::InvalidRfdContent)?;

    render_pdf(&content, content_dir).await
}

async fn render_pdf(
    content: &RfdAsciidoc<'_>,
    content_dir: PathBuf,
) -> Result<RenderedPdf, RfdOutputError> {
    let file_path = content_dir.join("contents.adoc");

    // Write the contents to a temporary file.
    write_file(&file_path, content.raw().as_bytes()).await?;
    tracing::info!("Wrote file to temp dir");

    let mut command = Command::new("asciidoctor-pdf");
    command.current_dir(content_dir.clone()).args([
        "-o",
        "-",
        "-r",
        "base64",
        "-r",
        "asciidoctor-mermaid/pdf",
        "-a",
        "reproducible",
        "-a",
        "source-highlighter=rouge",
        file_path.to_str().unwrap(),
    ]);

    let cmd_output = tokio::task::spawn_blocking(move || command.output()).await?;

    let cmd_output = cmd_output.map_err(|error| RenderableRfdError::ProcessStart {
        command: "asciidoctor-pdf",
        path: std::env::var("PATH").ok(),
        error,
    })?;

    if cmd_output.status.success() {
        Ok(cmd_output.stdout.into())
    } else {
        Err(RenderableRfdError::ParserFailed(String::from_utf8(
            cmd_output.stderr,
        )))?
    }
}

#[async_trait]
impl<'a> RfdRenderedFormat<RfdAsciidoc<'a>> for RenderedPdf {
    async fn render(content: &RfdAsciidoc, content_dir: PathBuf) -> Result<Self, RfdOutputError> {
        render_pdf(content, content_dir).await
    }
}
