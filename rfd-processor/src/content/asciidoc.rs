// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use async_trait::async_trait;
use rfd_data::content::{RfdAsciidoc, RfdDocument};
use std::{path::PathBuf, process::Command};
use uuid::Uuid;

use crate::util::write_file;

use super::{RenderableRfdError, RenderedPdf, RfdOutputError, RfdRenderedFormat};

pub(crate) async fn render_pdf_from_dir(
    content_dir: PathBuf,
) -> Result<RenderedPdf, RfdOutputError> {
    let content_dir = content_dir.canonicalize()?;
    let contents = std::fs::read_to_string(content_dir.join("contents.adoc"))?;
    let content = RfdAsciidoc::new(contents);

    render_pdf(&content, content_dir).await
}

/// Mermaid configuration handed to mmdc for every diagram, and the Ruby shim
/// that makes asciidoctor-mermaid pass it. See the shim for why this exists.
const MERMAID_CONFIG_JSON: &str = include_str!("../../asciidoctor/mermaid-config.json");
const MERMAID_CONFIG_SHIM: &str = include_str!("../../asciidoctor/mermaid_config.rb");

async fn render_pdf(
    content: &RfdAsciidoc<'_>,
    content_dir: PathBuf,
) -> Result<RenderedPdf, RfdOutputError> {
    let file_path = content_dir.join("contents.adoc");

    // Write the contents to a temporary file.
    write_file(&file_path, content.raw().as_bytes()).await?;
    tracing::info!("Wrote file to temp dir");

    // asciidoctor-mermaid has no way to configure mmdc, so the shim is loaded
    // after it to add `--configFile`. The files go in their own directory so
    // the content directory is left untouched.
    let support_dir = std::env::temp_dir().join(format!("rfd-mermaid-{}", Uuid::new_v4()));
    let shim_path = support_dir.join("mermaid_config.rb");
    write_file(
        &support_dir.join("mermaid-config.json"),
        MERMAID_CONFIG_JSON.as_bytes(),
    )
    .await?;
    write_file(&shim_path, MERMAID_CONFIG_SHIM.as_bytes()).await?;

    let mut command = Command::new("asciidoctor-pdf");
    command.current_dir(content_dir.clone()).args([
        "-o",
        "-",
        "-r",
        "base64",
        "-r",
        "asciidoctor-mermaid/pdf",
        "-r",
        shim_path.to_str().unwrap(),
        "-a",
        "reproducible",
        "-a",
        "source-highlighter=rouge",
        file_path.to_str().unwrap(),
    ]);

    let cmd_output = tokio::task::spawn_blocking(move || command.output()).await?;

    if let Err(error) = tokio::fs::remove_dir_all(&support_dir).await {
        tracing::warn!(?error, ?support_dir, "Failed to remove mermaid config dir");
    }

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
