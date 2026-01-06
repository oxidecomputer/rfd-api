// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use regex::Regex;
use std::borrow::Cow;
use thiserror::Error;

use super::RfdDocument;

static SUPPORTED_REFERENCES: [&str; 1] = ["authors"];

#[derive(Debug, Error)]
pub enum RfdAsciidocError {
    #[error("Failed to parse author line")]
    AuthorParse,
    #[error("Failed to resolve reference for {attribute}")]
    ReferenceResolver {
        attribute: String,
        error: regex::Error,
    },
}

/// The text data of an Asciidoc RFD
#[derive(Debug, Clone)]
pub struct RfdAsciidoc<'a> {
    content: Cow<'a, str>,
    resolved: String,
}

impl<'a> RfdAsciidoc<'a> {
    pub fn new<T>(content: T) -> Result<Self, RfdAsciidocError>
    where
        T: Into<Cow<'a, str>>,
    {
        let content = content.into();
        let mut resolved = Self::resolve_references(&content)?;
        Self::apply_author_line_attributes(&mut resolved);

        Ok(Self { content, resolved })
    }

    fn resolve_references(content: &str) -> Result<String, RfdAsciidocError> {
        let mut resolved = content.to_string();

        for attribute in SUPPORTED_REFERENCES {
            let pattern =
                Regex::new(&format!(r#"(?<b>[^\\]?)(\{{{}\}})"#, attribute)).map_err(|err| {
                    RfdAsciidocError::ReferenceResolver {
                        attribute: attribute.to_string(),
                        error: err,
                    }
                })?;
            let captures = pattern.captures(&resolved);

            if let Some(captures) = captures {
                let prefix = captures.get(1).map(|c| c.as_str()).unwrap_or_default();
                resolved = pattern
                    .replace_all(
                        &resolved,
                        format!(
                            "{}{}",
                            prefix,
                            Self::attr(attribute, &resolved).unwrap_or_default()
                        ),
                    )
                    .to_string();
            }
        }

        Ok(resolved)
    }

    fn apply_author_line_attributes(content: &mut String) {
        if let Some(author_line) = Self::author_line(content) {
            if let Some(authors) = RfdAuthors::parse(author_line) {
                if !authors.0.is_empty() {
                    *content = Self::set_attr(content, "authors", &authors.into_attr());
                }
            }
        }
    }

    fn attr<'b>(attr: &str, content: &'b str) -> Option<&'b str> {
        Self::attr_pattern(attr).find(content).map(|value| {
            value
                .as_str()
                .trim_start_matches(&format!(":{}:", attr))
                .trim()
        })
    }

    fn set_attr(content: &str, attr: &str, value: &str) -> String {
        let pattern = Self::attr_pattern(attr);

        if let Some(found) = pattern.find(content) {
            content.replacen(found.as_str(), &format!(":{}: {}\n", attr, value), 1)
        } else {
            let title = Self::title_line(content);

            if let Some(title) = title {
                let new_attr = format!(":{}: {}\n", attr, value);
                let header = Self::header(content);
                let body = Self::body(content);

                header.unwrap_or_default().to_string()
                    + "\n"
                    + &new_attr
                    + "\n\n"
                    + title
                    + body.unwrap_or_default()
            } else {
                content.to_string()
            }
        }
    }

    fn attr_pattern(attr: &str) -> Regex {
        Regex::new(&format!(r"(?m)^:{}:(.*)$\n", attr)).unwrap()
    }

    fn title_line(content: &str) -> Option<&str> {
        let title = Self::title_pattern().find(content);
        title.map(|m| m.as_str())
    }

    fn title_pattern() -> Regex {
        // This pattern also include markdown title handling fallbacks to handle malformed
        // documents
        Regex::new(r"(?m)^[=#][ ]+(?:RFD ?)?(?:\d+:? )?(.*)\n").unwrap()
    }

    fn author_line(content: &str) -> Option<&str> {
        Self::body(content).and_then(|body| {
            // After splitting the document at the title, the very first line (asuming it is non-empty)
            // is considered the author line
            body.split_inclusive('\n').nth(0).and_then(|line| {
                if line.is_empty() || line == "\n" {
                    None
                } else {
                    Some(line)
                }
            })
        })
    }

    fn header(content: &str) -> Option<&str> {
        Self::title_pattern()
            .splitn(content, 2)
            .nth(0)
            .map(|s| s.trim_end())
    }

    fn body(content: &str) -> Option<&str> {
        Self::title_pattern().splitn(content, 2).nth(1)
    }

    fn include_pattern() -> Regex {
        Regex::new(r"(?m)^include::(.*)\[\]$").unwrap()
    }

    pub fn includes(&'a self) -> Vec<AsciidocInclude<'a>> {
        Self::include_pattern()
            .captures_iter(&self.resolved)
            .map(|capture| {
                let extracted = capture.extract::<1>();
                AsciidocInclude {
                    file: extracted.1[0],
                    replacement: extracted.0,
                }
            })
            .collect::<Vec<_>>()
    }
}

#[derive(Debug, PartialEq)]
pub struct AsciidocInclude<'a> {
    file: &'a str,
    replacement: &'a str,
}

impl<'a> AsciidocInclude<'a> {
    pub fn name(&self) -> &str {
        self.file
    }

    pub fn perform_replacement(&self, body: &str, new_content: &str) -> String {
        tracing::trace!(self.replacement, "Replacing include");
        body.replace(self.replacement, new_content)
    }
}

impl<'a> RfdDocument for RfdAsciidoc<'a> {
    type Error = RfdAsciidocError;

    fn get_title(&self) -> Option<&str> {
        let fallback_title_pattern = Regex::new(r"(?m)^= (.*)$").unwrap();

        if let Some(caps) = Self::title_pattern().captures(&self.content) {
            Some(caps.get(1).unwrap().as_str().trim())
        } else if let Some(caps) = fallback_title_pattern.captures(&self.content) {
            Some(caps.get(1).unwrap().as_str().trim())
        } else {
            None
        }
    }

    fn get_state(&self) -> Option<&str> {
        Self::attr("state", &self.resolved)
    }

    fn update_state(&mut self, value: &str) -> Result<&mut Self, Self::Error> {
        self.set_raw(&Self::set_attr(&self.content, "state", value.trim()))
    }

    fn get_discussion(&self) -> Option<&str> {
        let link = Self::attr("discussion", &self.resolved)?;
        if link.starts_with("http") {
            Some(link)
        } else {
            None
        }
    }

    fn update_discussion(&mut self, value: &str) -> Result<&mut Self, Self::Error> {
        self.set_raw(&Self::set_attr(&self.content, "discussion", value.trim()))
    }

    fn get_authors(&self) -> Option<&str> {
        Self::attr("authors", &self.resolved)
    }

    fn get_labels(&self) -> Option<&str> {
        Self::attr("labels", &self.resolved)
    }

    fn update_labels(&mut self, value: &str) -> Result<&mut Self, Self::Error> {
        self.set_raw(&Self::set_attr(&self.content, "labels", value.trim()))
    }

    fn header(&self) -> Option<&str> {
        RfdAsciidoc::header(&self.content)
    }

    fn body(&self) -> Option<&str> {
        RfdAsciidoc::body(&self.content)
    }

    fn update_body(&mut self, value: &str) -> Result<&mut Self, Self::Error> {
        self.set_raw(&format!(
            "{}\n\n{}{}\n{}",
            self.header().unwrap_or_default(),
            Self::title_line(&self.content).unwrap_or_default(),
            Self::author_line(&self.content).unwrap_or_default(),
            value
        ))
    }

    fn raw(&self) -> &str {
        &self.content
    }

    fn set_raw(&mut self, content: &str) -> Result<&mut Self, Self::Error> {
        let mut resolved = Self::resolve_references(content)?;
        Self::apply_author_line_attributes(&mut resolved);
        self.resolved = resolved;
        self.content = Cow::Owned(content.to_string());
        Ok(self)
    }
}

#[derive(Debug)]
struct RfdAuthors(Vec<RfdAuthor>);

#[derive(Debug)]
struct RfdAuthor {
    first_name: String,
    last_name: Option<String>,
    middle_name: Option<String>,
    email: Option<String>,
}

impl RfdAuthors {
    fn parse(line: &str) -> Option<Self> {
        // Author parsing is defined by the Asciidoc documentation https://docs.asciidoctor.org/asciidoc/latest/document/author-information/#multiple-author-attributes

        // Ensure that the line has a single newline at its end. Otherwise it is considered invalid
        if line.chars().filter(|c| c == &'\n').count() > 1 {
            return None;
        }

        // Start by splitting the line in the supported separators. We diverge here from the
        // documentation for backwards compatibility, but only a single separator can be used
        let authors = if let Some(separator) = [';', ','].iter().find(|s| line.contains(**s)) {
            // We are in the multiple author case. Iterate over each case. All authors must be
            // valid to return and authors line
            let authors = line
                .trim()
                // Similar to the separators we trim a trailing comma and semicolon for
                // backwords compatability
                .trim_end_matches(';')
                .trim_end_matches(',')
                .split(*separator)
                .map(|part| RfdAuthor::parse(part.trim()))
                .collect::<Vec<_>>();

            Some(
                authors
                    .into_iter()
                    .filter_map(|author| {
                        if author.is_some() && !author.as_ref().unwrap().is_empty() {
                            author
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>(),
            )
        } else {
            // The single or no author case. Continue attempting to parse
            let author = RfdAuthor::parse(line.trim())?;
            if author.is_empty() {
                None
            } else {
                Some(vec![author])
            }
        }?;

        Some(Self(authors))
    }

    fn into_attr(self) -> String {
        self.0
            .into_iter()
            .map(RfdAuthor::into_attr)
            .collect::<Vec<_>>()
            .join("; ")
    }
}

impl RfdAuthor {
    fn parse(source: &str) -> Option<Self> {
        // Split on the first occurence of < which denotes the start of an email or url
        let parts = source.split('<').collect::<Vec<_>>();
        match parts.len() {
            1 => {
                // Without the presence of a < we are in the name only branch. Split the name
                // on spaces
                let (first, middle, last) = Self::parse_name(parts.first().unwrap().trim());

                Some(Self {
                    first_name: first.unwrap_or_default(),
                    last_name: last,
                    middle_name: middle,
                    email: None,
                })
            }
            2 => {
                // A single < indicates that this author has an email or url associated
                let (first, middle, last) = Self::parse_name(parts.first().unwrap().trim());
                let email = Self::parse_email(format!("<{}", parts.get(1).unwrap().trim()).trim())?;

                Some(Self {
                    first_name: first.unwrap_or_default(),
                    last_name: last,
                    middle_name: middle,
                    email: Some(email),
                })
            }
            n => {
                tracing::info!(
                    count = n,
                    "Invalid author line contained more than a single <"
                );
                None
            }
        }
    }

    fn parse_name(part: &str) -> (Option<String>, Option<String>, Option<String>) {
        let name_parts = part.split(' ').collect::<Vec<_>>();
        match name_parts.len() {
            2 => {
                // Firstname Lastname
                (
                    Some(name_parts.first().unwrap().to_string()),
                    None,
                    Some(name_parts.get(1).unwrap().to_string()),
                )
            }
            3 => {
                // Firstname Middlename Lastname
                (
                    Some(name_parts.first().unwrap().to_string()),
                    Some(name_parts.get(1).unwrap().to_string()),
                    Some(name_parts.get(2).unwrap().to_string()),
                )
            }
            _ => {
                // Anything else assigns only to Firstname
                (Some(name_parts.join(" ")), None, None)
            }
        }
    }

    fn parse_email(part: &str) -> Option<String> {
        // An email must start with a < and end with a >
        if part.starts_with('<') && part.ends_with('>') {
            // Asciidoc does not specify what an email or URL should look like
            // TODO: Should we do parsing / validation here?
            Some(
                part.trim_start_matches('<')
                    .trim_end_matches('>')
                    .to_string(),
            )
        } else {
            None
        }
    }

    fn is_empty(&self) -> bool {
        self.first_name.is_empty()
            && self.middle_name.is_none()
            && self.last_name.is_none()
            && self.email.is_none()
    }

    fn into_attr(self) -> String {
        let Self {
            first_name,
            middle_name,
            last_name,
            email,
        } = self;
        [
            Some(first_name),
            middle_name,
            last_name,
            email.map(|email| format!("<{}>", email)),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>()
        .join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // RFD Author tests

    #[test]
    fn test_single_author() {
        let line = "firstname";
        let authors = RfdAuthors::parse(line).unwrap();
        assert_eq!(authors.0.get(0).unwrap().first_name, "firstname");
        assert_eq!(authors.0.get(0).unwrap().middle_name, None);
        assert_eq!(authors.0.get(0).unwrap().last_name, None);
        assert_eq!(authors.0.get(0).unwrap().email, None);
        let line = "firstname <email@company.com>";
        let authors = RfdAuthors::parse(line).unwrap();
        assert_eq!(authors.0.get(0).unwrap().first_name, "firstname");
        assert_eq!(authors.0.get(0).unwrap().middle_name, None);
        assert_eq!(authors.0.get(0).unwrap().last_name, None);
        assert_eq!(
            authors.0.get(0).unwrap().email.as_deref(),
            Some("email@company.com")
        );

        let line = "firstname lastname";
        let authors = RfdAuthors::parse(line).unwrap();
        assert_eq!(authors.0.get(0).unwrap().first_name, "firstname");
        assert_eq!(authors.0.get(0).unwrap().middle_name, None);
        assert_eq!(
            authors.0.get(0).unwrap().last_name.as_deref(),
            Some("lastname")
        );
        assert_eq!(authors.0.get(0).unwrap().email, None);
        let line = "firstname lastname <email@company.com>";
        let authors = RfdAuthors::parse(line).unwrap();
        assert_eq!(authors.0.get(0).unwrap().first_name, "firstname");
        assert_eq!(authors.0.get(0).unwrap().middle_name, None);
        assert_eq!(
            authors.0.get(0).unwrap().last_name.as_deref(),
            Some("lastname")
        );
        assert_eq!(
            authors.0.get(0).unwrap().email.as_deref(),
            Some("email@company.com")
        );

        let line = "firstname middlename lastname";
        let authors = RfdAuthors::parse(line).unwrap();
        assert_eq!(authors.0.get(0).unwrap().first_name, "firstname");
        assert_eq!(
            authors.0.get(0).unwrap().middle_name.as_deref(),
            Some("middlename")
        );
        assert_eq!(
            authors.0.get(0).unwrap().last_name.as_deref(),
            Some("lastname")
        );
        assert_eq!(authors.0.get(0).unwrap().email, None);
        let line = "firstname middlename lastname <email@company.com>";
        let authors = RfdAuthors::parse(line).unwrap();
        assert_eq!(authors.0.get(0).unwrap().first_name, "firstname");
        assert_eq!(
            authors.0.get(0).unwrap().middle_name.as_deref(),
            Some("middlename")
        );
        assert_eq!(
            authors.0.get(0).unwrap().last_name.as_deref(),
            Some("lastname")
        );
        assert_eq!(
            authors.0.get(0).unwrap().email.as_deref(),
            Some("email@company.com")
        );
    }

    #[test]
    fn test_author_trailing_semicolon() {
        let line = "firstname middlename lastname <email@company.com>;";
        let authors = RfdAuthors::parse(line).unwrap();
        assert_eq!(authors.0.get(0).unwrap().first_name, "firstname");
        assert_eq!(
            authors.0.get(0).unwrap().middle_name.as_deref(),
            Some("middlename")
        );
        assert_eq!(
            authors.0.get(0).unwrap().last_name.as_deref(),
            Some("lastname")
        );
        assert_eq!(
            authors.0.get(0).unwrap().email.as_deref(),
            Some("email@company.com")
        );
    }

    #[test]
    fn test_author_excess_names_assigned_to_fname() {
        let line = r#"First2 M2 Last2 \"#;

        let authors = RfdAuthors::parse(line).unwrap();
        let expected = r#"First2 M2 Last2 \"#.to_string();
        assert_eq!(expected, authors.0.get(0).unwrap().first_name);
    }

    // Reference resolution tests

    #[test]
    fn test_resolves_internally() {
        let content = r#":authors: firstname <email@company>
= sdfgsdfgsdfg
{authors}
dsfsdf
sdf"#;
        let rfd = RfdAsciidoc::new(content).unwrap();
        assert!(rfd.content.to_string() != rfd.resolved);
        assert!(!rfd.resolved.contains("{authors}"));
    }

    // Read authors tests

    #[test]
    fn test_get_asciidoc_authors_from_authors_line() {
        let content = r#"sdfsdf
= sdfgsdfgsdfg
things <things@company>, firstname <email@company>
dsfsdf
sdf
authors: nope"#;
        let rfd = RfdAsciidoc::new(content).unwrap();
        let authors = rfd.get_authors().unwrap();
        let expected = r#"things <things@company>; firstname <email@company>"#.to_string();
        assert_eq!(expected, authors);
    }

    #[test]
    fn test_get_asciidoc_authors_via_reference() {
        let content = r#":authors: firstname <email@company>
= sdfgsdfgsdfg
{authors}
dsfsdf
sdf"#;
        let rfd = RfdAsciidoc::new(content).unwrap();
        let authors = rfd.get_authors().unwrap();
        let expected = r#"firstname <email@company>"#.to_string();
        assert_eq!(expected, authors);
    }

    #[test]
    fn test_get_asciidoc_attribute_authors_without_reference() {
        let content = r#":showtitle:
:toc: left
:numbered:
:icons: font
:state: published
:revremark: State: {state} | {discussion}
:authors: firstname <email@company>

= RFD 123 Test Rfd

dsfsdfdsfsdfdsfsdfdsfsdfdsfsdfdsfsdfdsfsdfdsfsdfdsfsdfdsfsdfdsfsdf
"#;
        let rfd = RfdAsciidoc::new(content).unwrap();
        let authors = rfd.get_authors().unwrap();
        let expected = r#"firstname <email@company>"#.to_string();
        assert_eq!(expected, authors);
    }

    #[test]
    fn test_get_asciidoc_attribute_authors_with_markdown_sections() {
        let content = r#"
:authors: Author One <one@company>, Author Two <two@company>

# Asciidoc with Markdown Section Indicators
{authors}

[bibliography]
## External References
* https://company.com[link to company] - An external reference
"#;
        let rfd = RfdAsciidoc::new(content).unwrap();
        let authors = rfd.get_authors().unwrap();
        let expected = r#"Author One <one@company>; Author Two <two@company>"#.to_string();
        assert_eq!(expected, authors);
    }

    #[test]
    fn test_get_asciidoc_attribute_authors_under_title_parses_nonsensically() {
        let content = r#"
= RFD 363 Minibar
:authors: Author One <one@company>, Author Two <two@company>
:state: discussion
:discussion: https://github.com/org/repo/pull/123
:revremark: State: {state} | {discussion}
:imagesdir: figures
:sectnums: |,all|
:sectnumlevels: 5
:xrefstyle: short

== Introduction
"#;

        let rfd = RfdAsciidoc::new(content).unwrap();
        let authors = rfd.get_authors().unwrap();
        let expected =
            r#":authors: Author One <one@company>; Author Two <two@company>"#.to_string();
        assert_eq!(expected, authors);
    }

    #[test]
    fn test_get_asciidoc_authors_from_author_line_with_later_attribute() {
        let content = r#"
= RFD 363 Minibar
Author One <one@company>, Author Two <two@company>

== Introduction

:authors: Author One <one@company>
"#;

        let rfd = RfdAsciidoc::new(content).unwrap();
        let authors = rfd.get_authors().unwrap();
        let expected = r#"Author One <one@company>; Author Two <two@company>"#.to_string();
        assert_eq!(expected, authors);
    }

    // Read state tests

    #[test]
    fn test_get_asciidoc_state() {
        let content = r#"sdfsdf
= sdfgsdfgsdfg
:state: prediscussion
dsfsdf
sdf
:state: nope"#;
        let rfd = RfdAsciidoc::new(content).unwrap();
        let state = rfd.get_state().unwrap();
        let expected = "prediscussion".to_string();
        assert_eq!(expected, state);
    }

    // Read discussion link tests

    #[test]
    fn test_get_asciidoc_discussion_link() {
        let content = r#"sdfsdf
= sdfgsdfgsdfg
:discussion: https://github.com/org/repo/pulls/1
dsfsdf
sdf
:discussion: nope"#;
        let rfd = RfdAsciidoc::new(content).unwrap();
        let discussion = rfd.get_discussion().unwrap();
        let expected = "https://github.com/org/repo/pulls/1".to_string();
        assert_eq!(expected, discussion);
    }

    #[test]
    fn test_set_nonexistent_attribute() {
        let content = r#"sdfsdf
= sdfgsdfgsdfg
:discussion: https://github.com/org/repo/pulls/1
dsfsdf
sdf
:discussion: nope"#;
        let mut rfd = RfdAsciidoc::new(content).unwrap();
        rfd.set_raw(&RfdAsciidoc::set_attr(rfd.raw(), "xrefstyle", "short"))
            .unwrap();
        assert_eq!(Some("short"), RfdAsciidoc::attr("xrefstyle", &rfd.content))
    }

    // Update discussion link tests

    #[test]
    fn test_update_existing_asciidoc_discussion_link_and_ignores_markdown_link() {
        let link = "https://github.com/org/repo/pulls/2019";

        let content = r#"sdfsdf
= sdfgsd
discussion: fgsdfg
:discussion: https://github.com/org/repo/pulls/1
dsfsdf
sdf
:discussion: nope"#;

        let mut rfd = RfdAsciidoc::new(content).unwrap();
        rfd.update_discussion(link).unwrap();
        let expected = r#"sdfsdf
= sdfgsd
discussion: fgsdfg
:discussion: https://github.com/org/repo/pulls/2019
dsfsdf
sdf
:discussion: nope"#;
        assert_eq!(expected, rfd.raw());
    }

    #[test]
    fn test_update_missing_asciidoc_discussion_link_and_ignores_markdown_link() {
        let link = "https://github.com/org/repo/pulls/2019";

        let content = r#"sdfsdf
= sdfgsd
discussion: fgsdfg
:discussion:
dsfsdf
sdf
:discussion: nope"#;

        let mut rfd = RfdAsciidoc::new(content).unwrap();
        rfd.update_discussion(link).unwrap();
        let expected = r#"sdfsdf
= sdfgsd
discussion: fgsdfg
:discussion: https://github.com/org/repo/pulls/2019
dsfsdf
sdf
:discussion: nope"#;
        assert_eq!(expected, rfd.raw());
    }

    // Update state tests

    #[test]
    fn test_update_existing_asciidoc_state() {
        let state = "discussion";
        let content = r#"sdfsdf
= sdfgsd
state: fgsdfg
:state: prediscussion
dsfsdf
sdf
:state: nope"#;
        let mut rfd = RfdAsciidoc::new(content).unwrap();
        rfd.update_state(state).unwrap();
        let expected = r#"sdfsdf
= sdfgsd
state: fgsdfg
:state: discussion
dsfsdf
sdf
:state: nope"#;
        assert_eq!(expected, rfd.raw());
    }

    #[test]
    fn test_update_empty_asciidoc_state() {
        let state = "discussion";
        let content = r#"sdfsdf
= sdfgsd
state: fgsdfg
:state:
dsfsdf
sdf
:state: nope"#;
        let mut rfd = RfdAsciidoc::new(content).unwrap();
        rfd.update_state(state).unwrap();
        let expected = r#"sdfsdf
= sdfgsd
state: fgsdfg
:state: discussion
dsfsdf
sdf
:state: nope"#;
        assert_eq!(expected, rfd.raw());
    }

    // Read title tests

    #[test]
    fn test_get_asciidoc_title() {
        let content = r#"sdfsdf
= RFD 43 Identity and Access Management (IAM)
:title: https://github.com/org/repo/pulls/1
dsfsdf
= RFD 53 Bye
sdf
:title: nope"#;
        let rfd = RfdAsciidoc::new(content).unwrap();
        let expected = "Identity and Access Management (IAM)".to_string();
        assert_eq!(expected, rfd.get_title().unwrap());
    }

    #[test]
    fn test_get_asciidoc_title_without_rfd_prefix() {
        // Add a test to show what happens when there is no "RFD" in
        // the title.
        let content = r#"sdfsdf
= Identity and Access Management (IAM)
:title: https://github.com/org/repo/pulls/1
dsfsdf
sdf
:title: nope"#;
        let rfd = RfdAsciidoc::new(content).unwrap();
        let expected = "Identity and Access Management (IAM)".to_string();
        assert_eq!(expected, rfd.get_title().unwrap());
    }

    #[test]
    fn test_get_asciidoc_title_colon() {
        // Add a test to show what happens when there is a colon following the digits
        let content = r#"sdfsdf
= RFD 123: Identity and Access Management (IAM)
:title: https://github.com/org/repo/pulls/1
dsfsdf
sdf
:title: nope"#;
        let rfd = RfdAsciidoc::new(content).unwrap();
        let expected = "Identity and Access Management (IAM)".to_string();
        assert_eq!(expected, rfd.get_title().unwrap());
    }

    #[test]
    fn test_get_asciidoc_title_with_rfd_in_preamble() {
        let content = r#"
:showtitle:
:toc: left
:numbered:
:icons: font
:state: published
:revremark: State: {state} | {discussion}
:authors: First Last <first@company.com>

// This is a preamble comment that contains the literal title prefix in the line below
// Previously an instance of the text RFD 123: would cause title parsing to fail

= RFD 123 This should be the title
{authors}
"#;

        let rfd = RfdAsciidoc::new(Cow::Borrowed(content)).unwrap();
        let expected = "This should be the title";

        assert_eq!(expected, rfd.get_title().unwrap());
    }

    fn test_rfd_content() -> &'static str {
        r#":reproducible:
:showtitle:
:toc: left
:numbered:
:icons: font
:state: prediscussion
:revremark: State: {state}
:docdatetime: 2019-01-04 19:26:06 UTC
:localdatetime: 2019-01-04 19:26:06 UTC
:labels: label1, label2

= RFD 123 Place
FirstName LastName <fname@company.org>

Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nunc et dignissim nisi. Donec ut libero in
dolor tempor aliquam quis quis nisl. Proin sit amet nunc in orci suscipit placerat. Mauris
pellentesque fringilla lacus id gravida. Donec in velit luctus, elementum mauris eu, pellentesque
massa. In lectus orci, vehicula at aliquet nec, elementum eu nisi. Vivamus viverra imperdiet
malesuada.

. Suspendisse blandit sem ligula, ac luctus metus condimentum non. Fusce enim purus, tincidunt ut
tortor eget, sollicitudin vestibulum sem. Proin eu velit orci.

. Proin eu finibus velit. Morbi eget blandit neque.

```mermaid
graph TD;
    A-->B;
    A-->C;
    B-->D;
    C-->D;
```

. Maecenas molestie, quam nec lacinia porta, lectus turpis molestie quam, at fringilla neque ipsum
in velit.

. Donec elementum luctus mauris.
"#
    }

    #[test]
    fn test_get_asciidoc_labels() {
        let rfd = RfdAsciidoc::new(Cow::Borrowed(test_rfd_content())).unwrap();
        let labels = rfd.get_labels().unwrap();
        let expected = "label1, label2".to_string();
        assert_eq!(expected, labels);
    }

    #[test]
    fn test_get_asciidoc_empty_labels() {
        let content = r#"sdfsdf
        = RFD 43 Identity and Access Management (IAM)
        No labels here
        "#;
        let rfd = RfdAsciidoc::new(content).unwrap();
        assert!(rfd.get_labels().is_none());
    }

    #[test]
    fn test_update_asciidoc_labels() {
        let mut rfd = RfdAsciidoc::new(Cow::Borrowed(test_rfd_content())).unwrap();
        rfd.update_labels("newlabel1, newlabel2").unwrap();
        let labels = rfd.get_labels().unwrap();
        let expected = "newlabel1, newlabel2".to_string();
        assert_eq!(expected, labels);
    }

    #[test]
    fn test_update_body() {
        let new_content = r#"This is the new body"#;
        let expected = r#":reproducible:
:showtitle:
:toc: left
:numbered:
:icons: font
:state: prediscussion
:revremark: State: {state}
:docdatetime: 2019-01-04 19:26:06 UTC
:localdatetime: 2019-01-04 19:26:06 UTC
:labels: label1, label2

= RFD 123 Place
FirstName LastName <fname@company.org>

This is the new body"#;
        let mut rfd = RfdAsciidoc::new(Cow::Borrowed(test_rfd_content())).unwrap();
        rfd.update_body(&new_content).unwrap();
        assert_eq!(expected, rfd.raw());
    }

    #[test]
    fn test_find_includes() {
        let contents = r#":reproducible:
:showtitle:
:toc: left
:numbered:
:icons: font
:state: prediscussion
:revremark: State: {state}
:docdatetime: 2019-01-04 19:26:06 UTC
:localdatetime: 2019-01-04 19:26:06 UTC
:labels: label1, label2

= RFD 123 Place
FirstName LastName <fname@company.org>

include::sub_doc1.adoc[]
This is the new body

include::sub_doc2.adoc[]"#;
        let rfd = RfdAsciidoc::new(Cow::Borrowed(contents)).unwrap();
        let includes = rfd.includes();

        let expected = vec![
            AsciidocInclude {
                file: "sub_doc1.adoc",
                replacement: "include::sub_doc1.adoc[]",
            },
            AsciidocInclude {
                file: "sub_doc2.adoc",
                replacement: "include::sub_doc2.adoc[]",
            },
        ];
        assert_eq!(expected, includes);
    }

    #[test]
    fn test_replace_include() {
        let original = r#":reproducible:
:showtitle:
:toc: left
:numbered:
:icons: font
:state: prediscussion
:revremark: State: {state}
:docdatetime: 2019-01-04 19:26:06 UTC
:localdatetime: 2019-01-04 19:26:06 UTC
:labels: label1, label2

= RFD 123 Place
FirstName LastName <fname@company.org>

include::sub_doc1.adoc[]
This is the new body

include::sub_doc1.adoc[]

include::sub_doc2.adoc[]"#;
        let include = AsciidocInclude {
            file: "sub_doc1.adoc",
            replacement: "include::sub_doc1.adoc[]",
        };

        let replacement_content = "Line 1
Line 2
Line 3";

        let expected = r#":reproducible:
:showtitle:
:toc: left
:numbered:
:icons: font
:state: prediscussion
:revremark: State: {state}
:docdatetime: 2019-01-04 19:26:06 UTC
:localdatetime: 2019-01-04 19:26:06 UTC
:labels: label1, label2

= RFD 123 Place
FirstName LastName <fname@company.org>

Line 1
Line 2
Line 3
This is the new body

Line 1
Line 2
Line 3

include::sub_doc2.adoc[]"#;

        assert_eq!(
            expected,
            include.perform_replacement(original, replacement_content)
        );
    }
}
