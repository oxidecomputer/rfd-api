use std::{borrow::Cow, io, path::PathBuf, str::Utf8Error, string::FromUtf8Error};

use async_trait::async_trait;
use base64::DecodeError;
use octorust::Client;
use rfd_data::RfdNumber;
use rfd_model::schema_ext::ContentFormat;
use thiserror::Error;
use tokio::task::JoinError;

use crate::{
    github::{GitHubError, GitHubRfdLocation},
    pdf::RfdPdf,
    util::FileIoError,
};

mod asciidoc;
mod markdown;

use asciidoc::RfdAsciidoc;

use self::markdown::RfdMarkdown;

#[derive(Debug, Error)]
pub enum RfdContentError {
    #[error("Failed to decode content file {0}")]
    Decode(#[from] DecodeError),
    #[error("Failed communication with GitHub API {0}")]
    GitHub(#[from] GitHubError),
    #[error("Failed to convert content string {0}")]
    InvalidContent(#[from] Utf8Error),
    #[error("General io failure {0}")]
    Io(#[from] io::Error),
    #[error("File io failure {0}")]
    File(#[from] FileIoError),
    #[error("Failed to parse content")]
    ParserFailed(Result<String, FromUtf8Error>),
    #[error("Failed to run output generator to completion {0}")]
    TaskFailure(#[from] JoinError),
}

pub trait RfdAttributes {
    /// Extract the title from the internal content
    fn get_title<'a>(&'a self) -> Option<&'a str>;

    /// Get the state value stored within the document
    fn get_state(&self) -> Option<&str>;

    // Update the state value stored within the document or add it if it does not exist
    fn update_state(&mut self, value: &str);

    /// Get the discussion link stored within the document
    fn get_discussion(&self) -> Option<&str>;

    // Update the discussion link stored within the document or add it if it does not exist
    fn update_discussion(&mut self, value: &str);

    /// Get the authors line stored within the document. The returned string may contain multiple
    /// names
    fn get_authors(&self) -> Option<&str>;
}

#[derive(Debug, Clone)]
pub enum RfdContent<'a> {
    Asciidoc(RfdAsciidoc<'a>),
    Markdown(RfdMarkdown<'a>),
}

impl<'a> RfdContent<'a> {
    /// Construct a new RfdContent wrapper that contains Asciidoc content
    pub fn new_asciidoc<T>(content: T) -> Self
    where
        T: Into<Cow<'a, str>>,
    {
        Self::Asciidoc(RfdAsciidoc::new(content.into()))
    }

    /// Construct a new RfdContent wrapper that contains Markdown content
    pub fn new_markdown<T>(content: T) -> Self
    where
        T: Into<Cow<'a, str>>,
    {
        Self::Markdown(RfdMarkdown::new(content.into()))
    }

    /// Get a reference to the internal unparsed contents
    pub fn raw(&self) -> &str {
        match self {
            Self::Asciidoc(adoc) => &adoc.content,
            Self::Markdown(md) => &md.content,
        }
    }

    /// Fetch the content that is above the title line
    pub fn header(&self) -> Option<&str> {
        match self {
            Self::Asciidoc(adoc) => adoc.header(),
            Self::Markdown(md) => md.header(),
        }
    }

    /// Fetch the content that is below the title line
    pub fn body(&self) -> Option<&str> {
        match self {
            Self::Asciidoc(adoc) => adoc.body(),
            Self::Markdown(md) => md.body(),
        }
    }

    /// Get an indicator of the inner content format
    pub fn format(&self) -> ContentFormat {
        match self {
            Self::Asciidoc(_) => ContentFormat::Asciidoc,
            Self::Markdown(_) => ContentFormat::Markdown,
        }
    }

    /// Consume this wrapper and return the internal unparsed contents
    pub fn into_inner_content(self) -> String {
        match self {
            Self::Asciidoc(adoc) => adoc.content.into_owned(),
            Self::Markdown(md) => md.content.into_owned(),
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
        match self {
            Self::Asciidoc(adoc) => Ok(adoc.to_pdf(client, number, branch).await?),
            _ => Err(RfdOutputError::FormatNotSupported),
        }
    }
}

impl<'a> RfdAttributes for RfdContent<'a> {
    fn get_title(&self) -> Option<&str> {
        match self {
            Self::Asciidoc(adoc) => adoc.get_title(),
            Self::Markdown(md) => md.get_title(),
        }
    }

    fn get_state(&self) -> Option<&str> {
        match self {
            Self::Asciidoc(adoc) => adoc.get_state(),
            Self::Markdown(md) => md.get_state(),
        }
    }

    fn update_state(&mut self, value: &str) {
        match self {
            Self::Asciidoc(adoc) => adoc.update_state(value),
            Self::Markdown(md) => md.update_state(value),
        }
    }

    fn get_discussion(&self) -> Option<&str> {
        match self {
            Self::Asciidoc(adoc) => adoc.get_discussion(),
            Self::Markdown(md) => md.get_discussion(),
        }
    }

    fn update_discussion(&mut self, value: &str) {
        match self {
            Self::Asciidoc(adoc) => adoc.update_discussion(value),
            Self::Markdown(md) => md.update_discussion(value),
        }
    }

    fn get_authors(&self) -> Option<&str> {
        match self {
            Self::Asciidoc(adoc) => adoc.get_authors(),
            Self::Markdown(md) => md.get_authors(),
        }
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
    ContentFailure(#[from] RfdContentError),
    #[error(transparent)]
    File(#[from] FileIoError),
    #[error("Output format is not supported")]
    FormatNotSupported,
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
