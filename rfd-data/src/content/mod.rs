// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

mod asciidoc;
mod markdown;
mod template;

pub use asciidoc::RfdAsciidoc;
pub use asciidoc::RfdAsciidocError;
pub use markdown::RfdMarkdown;
pub use template::{RenderableRfdTemplate, RfdTemplate, TemplateError};

use rfd_model::{schema_ext::ContentFormat, RfdRevision};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RfdContentError {
    #[error("Failed to parse Asciidoc content")]
    Asciidoc(#[from] RfdAsciidocError),
}

pub trait RfdDocument {
    type Error;

    /// Extract the title from the internal content
    fn get_title(&self) -> Option<&str>;

    /// Get the state value stored within the document
    fn get_state(&self) -> Option<&str>;

    // Update the state value stored within the document or add it if it does not exist
    fn update_state(&mut self, value: &str) -> Result<&mut Self, Self::Error>;

    /// Get the discussion link stored within the document
    fn get_discussion(&self) -> Option<&str>;

    // Update the discussion link stored within the document or add it if it does not exist
    fn update_discussion(&mut self, value: &str) -> Result<&mut Self, Self::Error>;

    /// Get the authors line stored within the document. The returned string may contain multiple
    /// names
    fn get_authors(&self) -> Option<&str>;

    /// Get the labels stored within the document
    fn get_labels(&self) -> Option<&str>;

    // Update the labels stored within the document or add them if they do not exist
    fn update_labels(&mut self, value: &str) -> Result<&mut Self, Self::Error>;

    // Get a reference to the contents of the RFD header
    fn header(&self) -> Option<&str>;

    /// Get a reference to the contents of the RFD body
    fn body(&self) -> Option<&str>;

    /// Get a reference to the contents of the RFD body
    fn update_body(&mut self, value: &str) -> Result<&mut Self, Self::Error>;

    /// Get a reference to the internal unparsed contents
    fn raw(&self) -> &str;

    /// Set the internal unparsed contents. This may incur a reparsing of the contents
    fn set_raw(&mut self, content: &str) -> Result<&mut Self, Self::Error>;
}

#[derive(Debug, Clone)]
pub enum RfdContent<'a> {
    Asciidoc(RfdAsciidoc<'a>),
    Markdown(RfdMarkdown<'a>),
}

impl<'a> RfdContent<'a> {
    pub fn format(&self) -> ContentFormat {
        match self {
            Self::Asciidoc(_) => ContentFormat::Asciidoc,
            Self::Markdown(_) => ContentFormat::Markdown,
        }
    }
}

impl<'a> RfdDocument for RfdContent<'a> {
    type Error = RfdContentError;

    fn get_title(&self) -> Option<&str> {
        match self {
            Self::Asciidoc(inner) => inner.get_title(),
            Self::Markdown(inner) => inner.get_title(),
        }
    }

    fn get_state(&self) -> Option<&str> {
        match self {
            Self::Asciidoc(inner) => inner.get_state(),
            Self::Markdown(inner) => inner.get_state(),
        }
    }

    fn update_state(&mut self, value: &str) -> Result<&mut Self, Self::Error> {
        match self {
            Self::Asciidoc(inner) => {
                inner.update_state(value)?;
            }
            Self::Markdown(inner) => {
                // Markdown returns an empty error so we can ignore it
                let _ = inner.update_state(value);
            }
        };
        Ok(self)
    }

    fn get_discussion(&self) -> Option<&str> {
        match self {
            Self::Asciidoc(inner) => inner.get_discussion(),
            Self::Markdown(inner) => inner.get_discussion(),
        }
    }

    fn update_discussion(&mut self, value: &str) -> Result<&mut Self, Self::Error> {
        match self {
            Self::Asciidoc(inner) => {
                inner.update_discussion(value)?;
            }
            Self::Markdown(inner) => {
                // Markdown returns an empty error so we can ignore it
                let _ = inner.update_discussion(value);
            }
        };
        Ok(self)
    }

    fn get_authors(&self) -> Option<&str> {
        match self {
            Self::Asciidoc(inner) => inner.get_authors(),
            Self::Markdown(inner) => inner.get_authors(),
        }
    }

    fn get_labels(&self) -> Option<&str> {
        match self {
            Self::Asciidoc(inner) => inner.get_labels(),
            Self::Markdown(inner) => inner.get_labels(),
        }
    }

    fn update_labels(&mut self, value: &str) -> Result<&mut Self, Self::Error> {
        match self {
            Self::Asciidoc(inner) => {
                inner.update_labels(value)?;
            }
            Self::Markdown(inner) => {
                // Markdown returns an empty error so we can ignore it
                let _ = inner.update_labels(value);
            }
        };
        Ok(self)
    }

    fn header(&self) -> Option<&str> {
        match self {
            Self::Asciidoc(inner) => inner.header(),
            Self::Markdown(inner) => inner.header(),
        }
    }

    fn body(&self) -> Option<&str> {
        match self {
            Self::Asciidoc(inner) => inner.body(),
            Self::Markdown(inner) => inner.body(),
        }
    }

    fn update_body(&mut self, value: &str) -> Result<&mut Self, Self::Error> {
        match self {
            Self::Asciidoc(inner) => {
                inner.update_body(value)?;
            }
            Self::Markdown(inner) => {
                // Markdown returns an empty error so we can ignore it
                let _ = inner.update_body(value);
            }
        };
        Ok(self)
    }

    fn raw(&self) -> &str {
        match self {
            Self::Asciidoc(inner) => inner.raw(),
            Self::Markdown(inner) => inner.raw(),
        }
    }

    fn set_raw(&mut self, content: &str) -> Result<&mut Self, Self::Error> {
        match self {
            Self::Asciidoc(inner) => {
                inner.set_raw(content)?;
            }
            Self::Markdown(inner) => {
                // Markdown returns an empty error so we can ignore it
                let _ = inner.set_raw(content);
            }
        };
        Ok(self)
    }
}

impl<'a> TryFrom<RfdRevision> for RfdContent<'a> {
    type Error = RfdContentError;
    fn try_from(value: RfdRevision) -> Result<Self, RfdContentError> {
        Ok(match value.content_format {
            ContentFormat::Asciidoc => RfdContent::Asciidoc(RfdAsciidoc::new(value.content)?),
            ContentFormat::Markdown => RfdContent::Markdown(RfdMarkdown::new(value.content)),
        })
    }
}
