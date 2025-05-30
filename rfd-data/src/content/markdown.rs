// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use regex::Regex;
use std::borrow::Cow;

use super::RfdDocument;

#[derive(Debug, Clone)]
pub struct RfdMarkdown<'a> {
    pub content: Cow<'a, str>,
}

impl<'a> RfdMarkdown<'a> {
    pub fn new<T>(content: T) -> Self
    where
        T: Into<Cow<'a, str>>,
    {
        Self {
            content: content.into(),
        }
    }

    fn attr(&self, attr: &str) -> Option<&str> {
        self.attr_pattern(attr).find(&self.content).map(|value| {
            value
                .as_str()
                .trim_start_matches(&format!("{}:", attr))
                .trim()
        })
    }

    fn set_attr(&mut self, attr: &str, value: &str) {
        let pattern = self.attr_pattern(attr);

        if let Some(found) = pattern.find(&self.content) {
            self.content = self
                .content
                .replacen(found.as_str(), &format!("{}: {}\n", attr, value), 1)
                .into()
        } else {
            let title = self.title_pattern().find(&self.content);

            if let Some(title) = title {
                let new_attr = format!("{}: {}\n", attr, value);
                let header = self.header();
                let body = self.body();

                self.content = (header.unwrap_or_default().to_string()
                    + "\n"
                    + &new_attr
                    + title.as_str()
                    + body.unwrap_or_default())
                .into()
            }
        }
    }

    fn attr_pattern(&self, attr: &str) -> Regex {
        Regex::new(&format!(r"(?m)^{}:(.*)$\n", attr)).unwrap()
    }

    fn title_pattern(&self) -> Regex {
        Regex::new(r"(?m)^[#].*$[\n\r]+").unwrap()
    }
}

impl<'a> RfdDocument for RfdMarkdown<'a> {
    type Error = ();

    fn get_title(&self) -> Option<&str> {
        let title_pattern = Regex::new(r"(?m)^[=# ]+(?:RFD ?)?(?:\d+:? )?(.*)$").unwrap();
        let fallback_title_pattern = Regex::new(r"(?m)^# (.*)$").unwrap();

        if let Some(caps) = title_pattern.captures(&self.content) {
            Some(caps.get(1).unwrap().as_str().trim())
        } else if let Some(caps) = fallback_title_pattern.captures(&self.content) {
            Some(caps.get(1).unwrap().as_str().trim())
        } else {
            None
        }
    }

    fn get_state(&self) -> Option<&str> {
        self.attr("state")
    }

    fn update_state(&mut self, value: &str) -> Result<&mut Self, Self::Error> {
        self.set_attr("state", value);
        Ok(self)
    }

    fn get_discussion(&self) -> Option<&str> {
        self.attr("discussion")
    }

    fn update_discussion(&mut self, value: &str) -> Result<&mut Self, Self::Error> {
        self.set_attr("discussion", value);
        Ok(self)
    }

    fn get_authors(&self) -> Option<&str> {
        self.attr("authors")
    }

    fn get_labels(&self) -> Option<&str> {
        self.attr("labels")
    }

    fn update_labels(&mut self, value: &str) -> Result<&mut Self, Self::Error> {
        self.set_attr("labels", value);
        Ok(self)
    }

    fn header(&self) -> Option<&str> {
        self.title_pattern()
            .splitn(&self.content, 2)
            .nth(0)
            .map(|s| s.trim_end())
    }

    fn body(&self) -> Option<&str> {
        self.title_pattern().splitn(&self.content, 2).nth(1)
    }

    fn update_body(&mut self, value: &str) -> Result<&mut Self, Self::Error> {
        self.content = Cow::Owned(
            self.title_pattern()
                .splitn(&self.content, 2)
                .nth(0)
                .unwrap_or_default()
                .to_string()
                + value,
        );
        Ok(self)
    }

    /// Get a reference to the internal unparsed contents
    fn raw(&self) -> &str {
        &self.content
    }

    fn set_raw(&mut self, content: &str) -> Result<&mut Self, Self::Error> {
        self.content = Cow::Owned(content.to_string());
        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use crate::content::{markdown::RfdMarkdown, RfdDocument};

    // Read authors tests

    #[test]
    fn test_get_markdown_authors() {
        let content = r#"sdfsdf
sdfsdf
authors: things, joe
dsfsdf
sdf
authors: nope"#;
        let rfd = RfdMarkdown::new(content);
        let authors = rfd.get_authors().unwrap();
        let expected = "things, joe".to_string();
        assert_eq!(expected, authors);
    }

    #[test]
    fn test_markdown_ignores_asciidoc_authors() {
        let content = r#"sdfsdf
= sdfgsdfgsdfg
things, joe
dsfsdf
sdf
:authors: nope"#;
        let rfd = RfdMarkdown::new(content);
        let authors = rfd.get_authors();
        assert!(authors.is_none());
    }

    #[test]
    fn test_set_nonexistent_attribute() {
        let content = r#"sdfsdf
# sdfgsdfgsdfg
discussion: https://github.com/org/repo/pulls/1
dsfsdf
sdf
discussion: nope"#;
        let mut rfd = RfdMarkdown::new(content);
        rfd.set_attr("xrefstyle", "short");
        assert_eq!(Some("short"), rfd.attr("xrefstyle"))
    }

    // Read state tests

    #[test]
    fn test_get_markdown_state() {
        let content = r#"sdfsdf
sdfsdf
state: discussion
dsfsdf
sdf
authors: nope"#;
        let rfd = RfdMarkdown::new(content);
        let state = rfd.get_state().unwrap();
        let expected = "discussion".to_string();
        assert_eq!(expected, state);
    }

    // Read discussion link tests

    #[test]
    fn test_get_markdown_discussion_link() {
        let content = r#"sdfsdf
sdfsdf
discussion: https://github.com/org/repo/pulls/1
dsfsdf
sdf
authors: nope"#;
        let rfd = RfdMarkdown::new(content);
        let discussion = rfd.get_discussion().unwrap();
        let expected = "https://github.com/org/repo/pulls/1".to_string();
        assert_eq!(expected, discussion);
    }

    // Update discussion link tests

    #[test]
    fn test_update_existing_markdown_discussion_link() {
        let link = "https://github.com/org/repo/pulls/2019";

        let content = r#"sdfsdf
sdfsdf
discussion:   https://github.com/org/repo/pulls/1
dsfsdf
sdf
authors: nope"#;
        let mut rfd = RfdMarkdown::new(content);
        rfd.update_discussion(link).unwrap();

        let expected = r#"sdfsdf
sdfsdf
discussion: https://github.com/org/repo/pulls/2019
dsfsdf
sdf
authors: nope"#;

        assert_eq!(expected, rfd.raw());
    }

    // Update state tests

    #[test]
    fn test_update_existing_markdown_state() {
        let state = "discussion";
        let content = r#"sdfsdf
sdfsdf
state:   sdfsdfsdf
dsfsdf
sdf
authors: nope"#;
        let mut rfd = RfdMarkdown::new(content);
        rfd.update_state(state).unwrap();

        let expected = r#"sdfsdf
sdfsdf
state: discussion
dsfsdf
sdf
authors: nope"#;
        assert_eq!(expected, rfd.raw());
    }

    // Read title tests

    #[test]
    fn test_get_markdown_title() {
        let content = r#"things
# RFD 43 Identity and Access Management (IAM)
sdfsdf
title: https://github.com/org/repo/pulls/1
dsfsdf
sdf
authors: nope"#;
        let rfd = RfdMarkdown::new(content);
        let expected = "Identity and Access Management (IAM)".to_string();
        assert_eq!(expected, rfd.get_title().unwrap());
    }

    #[test]
    fn test_get_markdown_title_colon() {
        let content = r#"things
# RFD 43: Identity and Access Management (IAM)
sdfsdf
title: https://github.com/org/repo/pulls/1
dsfsdf
sdf
authors: nope"#;
        let rfd = RfdMarkdown::new(content);
        let expected = "Identity and Access Management (IAM)".to_string();
        assert_eq!(expected, rfd.get_title().unwrap());
    }
}
