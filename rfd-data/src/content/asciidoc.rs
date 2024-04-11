// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use regex::Regex;
use std::borrow::Cow;

use super::RfdAttributes;

/// The text data of an Asciidoc RFD
#[derive(Debug, Clone)]
pub struct RfdAsciidoc<'a> {
    pub content: Cow<'a, str>,
}

impl<'a> RfdAsciidoc<'a> {
    pub fn new<T>(content: T) -> Self
    where
        T: Into<Cow<'a, str>>,
    {
        Self {
            content: content.into(),
        }
    }

    pub fn header(&self) -> Option<&str> {
        self.title_pattern().splitn(&self.content, 2).nth(0)
    }

    pub fn body(&self) -> Option<&str> {
        self.title_pattern().splitn(&self.content, 2).nth(1)
    }

    /// Get a reference to the internal unparsed contents
    pub fn raw(&self) -> &str {
        &self.content
    }

    fn attr(&self, attr: &str) -> Option<&str> {
        self.attr_pattern(attr).find(&self.content).map(|value| {
            value
                .as_str()
                .trim_start_matches(&format!(":{}:", attr))
                .trim()
        })
    }

    fn set_attr(&mut self, attr: &str, value: &str) {
        let pattern = self.attr_pattern(attr);

        if let Some(found) = pattern.find(&self.content) {
            self.content = self
                .content
                .replacen(found.as_str(), &format!(":{}: {}\n", attr, value), 1)
                .into()
        } else {
            let title = self.title_pattern().find(&self.content);

            if let Some(title) = title {
                let new_attr = format!(":{}: {}\n", attr, value);
                let header = self.header();
                let body = self.body();

                self.content = (header.unwrap_or_default().to_string()
                    + &new_attr
                    + title.as_str()
                    + body.unwrap_or_default())
                .into()
            }
        }
    }

    fn attr_pattern(&self, attr: &str) -> Regex {
        Regex::new(&format!(r"(?m)^:{}:(.*)$\n", attr)).unwrap()
    }

    fn title_pattern(&self) -> Regex {
        // This pattern also include markdown title handling fallbacks to handle malformed
        // documents
        Regex::new(r"(?m)^[=#].*$[\n\r]+").unwrap()
    }
}

impl<'a> RfdAttributes for RfdAsciidoc<'a> {
    fn get_title(&self) -> Option<&str> {
        let title_pattern = Regex::new(r"(?m)^[=# ]+(?:RFD ?)?(?:\d+:? )?(.*)$").unwrap();
        let fallback_title_pattern = Regex::new(r"(?m)^= (.*)$").unwrap();

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

    fn update_state(&mut self, value: &str) {
        self.set_attr("state", value.trim())
    }

    fn get_discussion(&self) -> Option<&str> {
        let link = self.attr("discussion")?;
        if link.starts_with("http") {
            Some(link)
        } else {
            None
        }
    }

    fn update_discussion(&mut self, value: &str) {
        self.set_attr("discussion", value.trim())
    }

    fn get_authors(&self) -> Option<&str> {
        self.body().and_then(|body| {
            body.lines().nth(0).and_then(|first_line| {
                // If {authors} is found, instead search the header for an authors attribute
                if first_line == "{authors}" {
                    self.attr("authors")
                } else {
                    // Given that we are in a fallback case we need to be slightly picky on what
                    // lines we allow. We require that the line at least include a *@*.* word to
                    // try and filter out lines that are not actually author lines
                    let author_fallback_pattern =
                        Regex::new(r"^.*?([\S]+@[\S]+.[\S]+).*?$").unwrap();
                    let fallback_matches = author_fallback_pattern.is_match(first_line);

                    if fallback_matches {
                        Some(first_line)
                    } else {
                        // If none of our attempts have found an author, we drop back to the
                        // attribute lookup. Eventually all of this logic should be removed and only
                        // the attribute version should be supported
                        self.attr("authors")
                    }
                }
            })
        })
    }

    fn get_labels(&self) -> Option<&str> {
        self.attr("labels")
    }

    fn update_labels(&mut self, value: &str) {
        self.set_attr("labels", value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Read authors tests

    #[test]
    fn test_get_asciidoc_fallback_authors() {
        let content = r#"sdfsdf
= sdfgsdfgsdfg
things <things@company>, firstname <email@company>
dsfsdf
sdf
authors: nope"#;
        let rfd = RfdAsciidoc::new(content);
        let authors = rfd.get_authors().unwrap();
        let expected = r#"things <things@company>, firstname <email@company>"#.to_string();
        assert_eq!(expected, authors);
    }

    #[test]
    fn test_get_asciidoc_attribute_authors() {
        let content = r#":authors: firstname <email@company>
= sdfgsdfgsdfg
{authors}
dsfsdf
sdf"#;
        let rfd = RfdAsciidoc::new(content);
        let authors = rfd.get_authors().unwrap();
        let expected = r#"firstname <email@company>"#.to_string();
        assert_eq!(expected, authors);
    }

    #[test]
    fn test_get_asciidoc_attribute_authors_without_marker() {
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
        let rfd = RfdAsciidoc::new(content);
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
        let rfd = RfdAsciidoc::new(content);
        let authors = rfd.get_authors().unwrap();
        let expected = r#"Author One <one@company>, Author Two <two@company>"#.to_string();
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
        let rfd = RfdAsciidoc::new(content);
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
        let rfd = RfdAsciidoc::new(content);
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
        let mut rfd = RfdAsciidoc::new(content);
        rfd.set_attr("xrefstyle", "short");
        assert_eq!(Some("short"), rfd.attr("xrefstyle"))
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

        let mut rfd = RfdAsciidoc::new(content);
        rfd.update_discussion(link);
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

        let mut rfd = RfdAsciidoc::new(content);
        rfd.update_discussion(link);
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
        let mut rfd = RfdAsciidoc::new(content);
        rfd.update_state(state);
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
        let mut rfd = RfdAsciidoc::new(content);
        rfd.update_state(state);
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
        let rfd = RfdAsciidoc::new(content);
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
        let rfd = RfdAsciidoc::new(content);
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
        let rfd = RfdAsciidoc::new(content);
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

        let rfd = RfdAsciidoc::new(Cow::Borrowed(content));
        let expected = "This should be the title";

        assert_eq!(expected, rfd.get_title().unwrap());
    }

    fn test_rfd_content() -> &'static str {
        r#"
:reproducible:
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
        let rfd = RfdAsciidoc::new(Cow::Borrowed(test_rfd_content()));
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
        let rfd = RfdAsciidoc::new(content);
        assert!(rfd.get_labels().is_none());
    }

    #[test]
    fn test_update_asciidoc_labels() {
        let mut rfd = RfdAsciidoc::new(Cow::Borrowed(test_rfd_content()));
        rfd.update_labels("newlabel1, newlabel2");
        let labels = rfd.get_labels().unwrap();
        let expected = "newlabel1, newlabel2".to_string();
        assert_eq!(expected, labels);
    }
}
