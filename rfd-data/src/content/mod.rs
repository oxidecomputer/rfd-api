// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

mod asciidoc;
mod markdown;

pub use asciidoc::RfdAsciidoc;
pub use markdown::RfdMarkdown;

pub trait RfdDocument {
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

    /// Get the labels stored within the document
    fn get_labels(&self) -> Option<&str>;

    // Update the labels stored within the document or add them if they do not exist
    fn update_labels(&mut self, value: &str);

    // Get a reference to the contents of the RFD header
    fn header(&self) -> Option<&str>;

    /// Get a reference to the contents of the RFD body
    fn body(&self) -> Option<&str>;

    /// Get a reference to the internal unparsed contents
    fn raw(&self) -> &str;
}

#[derive(Debug, Clone)]
pub enum RfdContent<'a> {
    Asciidoc(RfdAsciidoc<'a>),
    Markdown(RfdMarkdown<'a>),
}

impl<'a> RfdDocument for RfdContent<'a> {
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

    fn update_state(&mut self, value: &str) {
        match self {
            Self::Asciidoc(inner) => inner.update_state(value),
            Self::Markdown(inner) => inner.update_state(value),
        }
    }

    fn get_discussion(&self) -> Option<&str> {
        match self {
            Self::Asciidoc(inner) => inner.get_discussion(),
            Self::Markdown(inner) => inner.get_discussion(),
        }
    }

    fn update_discussion(&mut self, value: &str) {
        match self {
            Self::Asciidoc(inner) => inner.update_discussion(value),
            Self::Markdown(inner) => inner.update_discussion(value),
        }
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

    fn update_labels(&mut self, value: &str) {
        match self {
            Self::Asciidoc(inner) => inner.update_labels(value),
            Self::Markdown(inner) => inner.update_labels(value),
        }
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

    fn raw(&self) -> &str {
        match self {
            Self::Asciidoc(inner) => inner.raw(),
            Self::Markdown(inner) => inner.raw(),
        }
    }
}
