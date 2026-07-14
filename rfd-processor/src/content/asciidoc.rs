// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use async_trait::async_trait;
use rfd_data::content::{RfdAsciidoc, RfdDocument};
use std::{
    env,
    path::{Path, PathBuf},
    process::Command,
};

use crate::util::write_file;

use super::{RenderableRfdError, RenderedPdf, RfdOutputError, RfdRenderedFormat};

async fn render_pdf(
    content: &RfdAsciidoc<'_>,
    content_dir: PathBuf,
    path_prefix: Option<&Path>,
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

    if let Some(path_prefix) = path_prefix {
        let path = env::join_paths(
            std::iter::once(path_prefix.to_path_buf())
                .chain(env::split_paths(&env::var_os("PATH").unwrap_or_default())),
        )
        .expect("failed to construct PATH");
        command.env("PATH", path);
    }

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

#[async_trait]
impl<'a> RfdRenderedFormat<RfdAsciidoc<'a>> for RenderedPdf {
    async fn render(content: &RfdAsciidoc, content_dir: PathBuf) -> Result<Self, RfdOutputError> {
        render_pdf(content, content_dir, None).await
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

        fn write_mmdc_wrapper(&self) -> PathBuf {
            let wrapper_dir = self.path.join("mmdc-bin");
            fs::create_dir_all(&wrapper_dir).expect("failed to create mmdc wrapper directory");

            let real_mmdc = find_executable("mmdc");
            let puppeteer_config = wrapper_dir.join("puppeteer.json");
            fs::write(
                &puppeteer_config,
                r#"{"args":["--no-sandbox","--disable-setuid-sandbox"]}"#,
            )
            .expect("failed to write puppeteer config");

            let wrapper = wrapper_dir.join("mmdc");
            fs::write(
                &wrapper,
                format!(
                    r#"#!/usr/bin/env bash
real_mmdc={real_mmdc}
puppeteer_config={puppeteer_config}
exec "$real_mmdc" --puppeteerConfigFile "$puppeteer_config" "$@"
"#,
                    real_mmdc = shell_quote(real_mmdc.as_path()),
                    puppeteer_config = shell_quote(puppeteer_config.as_path()),
                ),
            )
            .expect("failed to write mmdc wrapper");

            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;

                let mut permissions = fs::metadata(&wrapper)
                    .expect("failed to stat mmdc wrapper")
                    .permissions();
                permissions.set_mode(0o755);
                fs::set_permissions(&wrapper, permissions)
                    .expect("failed to make mmdc wrapper executable");
            }

            wrapper_dir
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
        let mmdc_path_prefix = render_dir.write_mmdc_wrapper();

        let content = RfdAsciidoc::new(EXAMPLE_RFD).expect("example RFD should parse");
        let pdf = render_pdf(
            &content,
            render_dir.path().to_path_buf(),
            Some(mmdc_path_prefix.as_path()),
        )
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

    fn find_executable(name: &str) -> PathBuf {
        std::env::split_paths(&std::env::var_os("PATH").unwrap_or_default())
            .map(|path| path.join(name))
            .find(|path| path.is_file())
            .unwrap_or_else(|| panic!("failed to find {name} in PATH"))
    }

    fn shell_quote(path: &Path) -> String {
        let path = path.to_string_lossy();
        format!("'{}'", path.replace('\'', r#"'\''"#))
    }
}
