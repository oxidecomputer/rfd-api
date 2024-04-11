// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

mod asciidoc;
mod markdown;

pub use asciidoc::RfdAsciidoc;
pub use markdown::RfdMarkdown;

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

    /// Get the labels stored within the document
    fn get_labels(&self) -> Option<&str>;

    // Update the labels stored within the document or add them if they do not exist
    fn update_labels(&mut self, value: &str);
}
