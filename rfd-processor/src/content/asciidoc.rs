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
            "base64",
            "-r",
            "asciidoctor-mermaid/pdf",
            "-a",
            "reproducible",
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs, path::Path};
    use uuid::Uuid;

    const EXAMPLE_RFD: &str = include_str!("../../tests/content/example.adoc");
    const EXAMPLE_CSV: &str = include_str!("../../tests/content/example.csv");
    const OXIDE_LOGO: &str = include_str!("../../tests/content/oxide-logo.svg");

    struct RenderDir {
        path: PathBuf,
    }

    impl RenderDir {
        fn new() -> Self {
            let path = std::env::temp_dir().join(format!(
                "rfd-processor-asciidoc-to-pdf-test-{}",
                Uuid::new_v4()
            ));
            fs::create_dir_all(&path).expect("failed to create render test directory");
            Self { path }
        }

        fn path(&self) -> &Path {
            &self.path
        }
    }

    impl Drop for RenderDir {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.path);
        }
    }

    #[tokio::test]
    async fn render_pdf_example() {
        let render_dir = RenderDir::new();
        fs::write(render_dir.path().join("example.csv"), EXAMPLE_CSV)
            .expect("failed to write CSV include");
        fs::write(render_dir.path().join("oxide-logo.svg"), OXIDE_LOGO)
            .expect("failed to write supporting image");

        let content = RfdAsciidoc::new(EXAMPLE_RFD).expect("example RFD should parse");
        let pdf = RenderedPdf::render(&content, render_dir.path().to_path_buf())
            .await
            .expect("example RFD should render to PDF");
        let pdf = pdf.into_inner();

        fs::write(
            Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/content/example-local.pdf"),
            &pdf,
        )
        .expect("failed to write local example PDF");

        assert!(pdf.starts_with(b"%PDF-"), "rendered bytes should be a PDF");
        let repo_pdf_path =
            Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/content/example-repo.pdf");
        let expected_pdf = match fs::read(&repo_pdf_path) {
            Ok(pdf) => pdf,
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
                fs::write(&repo_pdf_path, &pdf).expect("failed to write repo example PDF");
                pdf.clone()
            }
            Err(err) => panic!("failed to read repo example PDF: {err}"),
        };

        if pdf != expected_pdf {
            let first_diff = pdf
                .iter()
                .zip(expected_pdf.iter())
                .position(|(actual, expected)| actual != expected);

            panic!(
                "rendered PDF should match tests/content/example-repo.pdf; actual len {}, expected len {}, first differing byte {:?}",
                pdf.len(),
                expected_pdf.len(),
                first_diff
            );
        }
    }
}
