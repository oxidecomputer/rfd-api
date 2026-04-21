// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::{borrow::Cow, env, io, path::PathBuf, str::Utf8Error, string::FromUtf8Error};

use async_trait::async_trait;
use base64::DecodeError;
use octorust::Client;
use rfd_data::{
    content::{RfdAsciidoc, RfdContent, RfdContentError, RfdDocument, RfdMarkdown},
    RfdNumber,
};
use rfd_github::{GitHubError, GitHubRfdLocation};
use rfd_model::schema_ext::ContentFormat;
use tap::TapFallible;
use thiserror::Error;
use tokio::task::JoinError;
use tracing::instrument;
use uuid::Uuid;

use crate::{
    pdf::RfdPdf,
    util::{decode_base64, write_file, FileIoError},
};

mod asciidoc;

#[derive(Debug, Error)]
pub enum RenderableRfdError {
    #[error("Failed to decode content file {0}")]
    Decode(#[from] DecodeError),
    #[error("Failed communication with GitHub API {0}")]
    GitHub(#[from] GitHubError),
    #[error("Failed to convert content string {0}")]
    InvalidContent(#[from] Utf8Error),
    #[error("Failed to parse Rfd contents")]
    InvalidRfdContent(#[from] RfdContentError),
    #[error("General io failure {0}")]
    Io(#[from] io::Error),
    #[error("File io failure {0}")]
    File(#[from] FileIoError),
    #[error("Failed to parse content")]
    ParserFailed(Result<String, FromUtf8Error>),
    #[error("Failed to run output generator to completion {0}")]
    TaskFailure(#[from] JoinError),
}

#[derive(Debug, Clone)]
pub struct RenderableRfd<'a> {
    pub content: RfdContent<'a>,
    render_id: Uuid,
}

impl<'a> RenderableRfd<'a> {
    pub fn new(content: RfdContent<'a>) -> Self {
        Self {
            content,
            render_id: Uuid::new_v4(),
        }
    }

    /// Construct a new RfdContent wrapper that contains Asciidoc content
    #[allow(clippy::result_large_err)]
    pub fn new_asciidoc<T>(content: T) -> Result<Self, RenderableRfdError>
    where
        T: Into<Cow<'a, str>>,
    {
        Ok(Self {
            content: RfdContent::Asciidoc(
                RfdAsciidoc::new(content.into()).map_err(RfdContentError::Asciidoc)?,
            ),
            render_id: Uuid::new_v4(),
        })
    }

    /// Construct a new RfdContent wrapper that contains Markdown content
    pub fn new_markdown<T>(content: T) -> Self
    where
        T: Into<Cow<'a, str>>,
    {
        Self {
            content: RfdContent::Markdown(RfdMarkdown::new(content.into())),
            render_id: Uuid::new_v4(),
        }
    }

    /// Get an indicator of the inner content format
    pub fn format(&self) -> ContentFormat {
        match self.content {
            RfdContent::Asciidoc(_) => ContentFormat::Asciidoc,
            RfdContent::Markdown(_) => ContentFormat::Markdown,
        }
    }

    /// Consume this wrapper and return the internal unparsed contents
    pub fn into_inner_content(self) -> String {
        match self.content {
            RfdContent::Asciidoc(adoc) => adoc.raw().to_string(),
            RfdContent::Markdown(md) => md.raw().to_string(),
        }
    }

    /// Generate a PDF by combining RFD contents with static resources that are stored for a given
    /// RFD number on a specific branch. Markdown documents do not support PDF generation
    pub async fn to_pdf(
        &self,
        client: &Client,
        number: &RfdNumber,
        branch: &GitHubRfdLocation,
    ) -> Result<RfdPdf, RfdOutputError> {
        match &self.content {
            RfdContent::Asciidoc(adoc) => {
                self.download_supporting_documents(client, number, branch)
                    .await?;

                let pdf = RenderedPdf::render(adoc, self.tmp_path()?).await?;

                self.cleanup_tmp_path()?;

                Ok(RfdPdf {
                    contents: pdf.into_inner(),
                    number: *number,
                })
                // Ok(adoc.to_pdf(client, number, branch).await?)
            }
            _ => Err(RfdOutputError::FormatNotSupported),
        }
    }

    /// Downloads images that are stored on the provided GitHub branch for the given RFD number.
    /// These are stored locally in a tmp directory for use by asciidoctor
    #[instrument(skip(self, client), fields(storage_path = ?self.tmp_path()))]
    async fn download_supporting_documents(
        &self,
        client: &Client,
        number: &RfdNumber,
        location: &GitHubRfdLocation,
    ) -> Result<(), RenderableRfdError> {
        let dir = number.repo_path();
        let storage_path = self.tmp_path()?;

        let documents = location
            .download_supporting_documents(client, number)
            .await?;

        for document in documents {
            let document_path = storage_path.join(
                document
                    .path
                    .replace(dir.trim_start_matches('/'), "")
                    .trim_start_matches('/'),
            );

            let path = document_path;
            write_file(&path, &decode_base64(&document.content)?).await?;

            tracing::info!(?path, "Wrote supporting document",);
        }

        Ok(())
    }

    /// Create a tmp directory for rendering this RFD
    #[allow(clippy::result_large_err)]
    fn tmp_path(&self) -> Result<PathBuf, RenderableRfdError> {
        let mut path = env::temp_dir();
        path.push("rfd-render/");
        path.push(self.render_id.to_string());

        // Ensure the path exists
        std::fs::create_dir_all(path.clone())?;

        Ok(path)
    }

    // Cleanup remaining images and local state that was used by asciidoctor
    #[allow(clippy::result_large_err)]
    #[instrument(skip(self), fields(storage_path = ?self.tmp_path()), err)]
    fn cleanup_tmp_path(&self) -> Result<(), RenderableRfdError> {
        let storage_path = self.tmp_path()?;

        if storage_path.exists() && storage_path.is_dir() {
            tracing::info!("Removing temporary content directory {:?}", storage_path);
            std::fs::remove_dir_all(storage_path)
                .tap_err(|err| tracing::warn!(?err, "Failed to clean up temporary files"))?
        }

        Ok(())
    }
}

impl<'a> RfdDocument for RenderableRfd<'a> {
    type Error = RenderableRfdError;

    fn get_title(&self) -> Option<&str> {
        RfdDocument::get_title(&self.content)
    }

    fn get_state(&self) -> Option<&str> {
        RfdDocument::get_state(&self.content)
    }

    fn update_state(&mut self, value: &str) -> Result<&mut Self, Self::Error> {
        RfdDocument::update_state(&mut self.content, value)?;
        Ok(self)
    }

    fn get_discussion(&self) -> Option<&str> {
        RfdDocument::get_discussion(&self.content)
    }

    fn update_discussion(&mut self, value: &str) -> Result<&mut Self, Self::Error> {
        RfdDocument::update_discussion(&mut self.content, value)?;
        Ok(self)
    }

    fn get_authors(&self) -> Option<&str> {
        RfdDocument::get_authors(&self.content)
    }

    fn get_labels(&self) -> Option<&str> {
        RfdDocument::get_labels(&self.content)
    }

    fn update_labels(&mut self, value: &str) -> Result<&mut Self, Self::Error> {
        RfdDocument::update_labels(&mut self.content, value)?;
        Ok(self)
    }

    fn header(&self) -> Option<&str> {
        RfdDocument::header(&self.content)
    }

    fn body(&self) -> Option<&str> {
        RfdDocument::body(&self.content)
    }

    fn update_body(&mut self, value: &str) -> Result<&mut Self, Self::Error> {
        RfdDocument::update_body(&mut self.content, value)?;
        Ok(self)
    }

    fn raw(&self) -> &str {
        RfdDocument::raw(&self.content)
    }

    fn set_raw(&mut self, content: &str) -> Result<&mut Self, Self::Error> {
        self.content.set_raw(content)?;
        Ok(self)
    }
}

#[async_trait]
pub trait RfdRenderedFormat<Source> {
    async fn render(content: &Source, content_dir: PathBuf) -> Result<Self, RfdOutputError>
    where
        Self: Sized;
}

pub struct RenderedPdf(Vec<u8>);

impl RenderedPdf {
    pub fn into_inner(self) -> Vec<u8> {
        self.0
    }
}

impl From<Vec<u8>> for RenderedPdf {
    fn from(value: Vec<u8>) -> Self {
        Self(value)
    }
}

#[derive(Debug, Error)]
pub enum RfdOutputError {
    #[error(transparent)]
    Command(#[from] JoinError),
    #[error("Failed to prepare content for output")]
    ContentFailure(#[from] RenderableRfdError),
    #[error(transparent)]
    File(#[from] FileIoError),
    #[error("Output format is not supported")]
    FormatNotSupported,
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
