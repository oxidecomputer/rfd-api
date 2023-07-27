use async_trait::async_trait;
use octorust::Client;
use regex::Regex;
use rfd_data::RfdNumber;
use std::{
    borrow::Cow,
    env,
    path::PathBuf,
    process::Command,
};
use tap::TapFallible;
use tracing::instrument;
use uuid::Uuid;

use crate::{
    github::GitHubRfdLocation,
    pdf::RfdPdf,
    util::{decode_base64, write_file},
};

use super::{RenderedPdf, RfdAttributes, RfdContentError, RfdOutputError, RfdRenderedFormat};

/// The text data of an Asciidoc RFD
#[derive(Debug, Clone)]
pub struct RfdAsciidoc<'a> {
    pub content: Cow<'a, str>,
    storage_id: Uuid,
}

impl<'a> RfdAsciidoc<'a> {
    pub fn new(content: Cow<'a, str>) -> Self {
        Self {
            content,
            storage_id: Uuid::new_v4(),
        }
    }

    pub fn header(&self) -> Option<&str> {
        self.title_pattern().splitn(&self.content, 2).nth(0)
    }

    pub fn body(&self) -> Option<&str> {
        self.title_pattern().splitn(&self.content, 2).nth(1)
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

    /// Generate a PDF by combining RFD contents with static resources that are stored for a given
    /// RFD number on a specific branch. Markdown documents do not support PDF generation
    pub async fn to_pdf(
        &self,
        client: &Client,
        number: &RfdNumber,
        branch: &GitHubRfdLocation,
    ) -> Result<RfdPdf, RfdOutputError> {
        self.download_images(client, number, branch).await?;

        let pdf = RenderedPdf::render(self, self.tmp_path()?).await?;

        self.cleanup_tmp_path()?;

        Ok(RfdPdf {
            contents: pdf.into_inner(),
            number: *number,
        })
    }

    /// Downloads images that are stored on the provided GitHub branch for the given RFD number.
    /// These are stored locally so in a tmp directory for use by asciidoctor
    #[instrument(skip(self, client), fields(storage_path = ?self.tmp_path()))]
    async fn download_images(
        &self,
        client: &Client,
        number: &RfdNumber,
        location: &GitHubRfdLocation,
    ) -> Result<(), RfdContentError> {
        let dir = number.repo_path();
        let storage_path = self.tmp_path()?;

        let images = location.get_images(client, number).await?;

        for image in images {
            let image_path = storage_path.join(
                image
                    .path
                    .replace(dir.trim_start_matches('/'), "")
                    .trim_start_matches('/'),
            );

            let path = PathBuf::from(image_path);
            write_file(&path, &decode_base64(&image.content)?).await?;

            tracing::info!(?path, "Wrote embedded image",);
        }

        Ok(())
    }

    /// Create a tmp directory for rendering this RFD
    fn tmp_path(&self) -> Result<PathBuf, RfdContentError> {
        let mut path = env::temp_dir();
        path.push("asciidoc-rfd-render/");
        path.push(&self.storage_id.to_string());

        // Ensure the path exists
        std::fs::create_dir_all(path.clone())?;

        Ok(path)
    }

    // Cleanup remaining images and local state that was used by asciidoctor
    #[instrument(skip(self), fields(storage_path = ?self.tmp_path()), err)]
    fn cleanup_tmp_path(&self) -> Result<(), RfdContentError> {
        let storage_path = self.tmp_path()?;

        if storage_path.exists() && storage_path.is_dir() {
            tracing::info!("Removing temporary content directory {:?}", storage_path);
            std::fs::remove_dir_all(storage_path)
                .tap_err(|err| tracing::warn!(?err, "Failed to clean up temporary files"))?
        }

        Ok(())
    }
}

impl<'a> RfdAttributes for RfdAsciidoc<'a> {
    fn get_title(&self) -> Option<&str> {
        let title_pattern = Regex::new(r"(?m)^[=# ]+(?:RFD ?)?(?:\d+ )?(.*)$").unwrap();
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
                    Some(first_line)
                }
            })
        })
    }
}

#[async_trait]
impl<'a> RfdRenderedFormat<RfdAsciidoc<'a>> for RenderedPdf {
    async fn render(content: &RfdAsciidoc, content_dir: PathBuf) -> Result<Self, RfdOutputError> {
        let file_path = content_dir.join("contents.adoc");

        // Write the contents to a temporary file.
        write_file(&file_path, content.content.as_bytes()).await?;
        tracing::info!("Wrote file to temp dir");

        let mut command = Command::new("asciidoctor-pdf");
        command.current_dir(content_dir.clone()).args([
            "-o",
            "-",
            "-r",
            "asciidoctor-mermaid/pdf",
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
            Err(RfdContentError::ParserFailed)?
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::content::RfdContent;

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
        let rfd = RfdContent::new_asciidoc(content);
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
        let rfd = RfdContent::new_asciidoc(content);
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
        let rfd = RfdContent::new_asciidoc(content);
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
        let rfd = RfdContent::new_asciidoc(content);
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
        let rfd = RfdContent::new_asciidoc(content);
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
        let mut rfd = RfdAsciidoc::new(content.into());
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

        let mut rfd = RfdContent::new_asciidoc(content);
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

        let mut rfd = RfdContent::new_asciidoc(content);
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
        let mut rfd = RfdContent::new_asciidoc(content);
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
        let mut rfd = RfdContent::new_asciidoc(content);
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
        let rfd = RfdContent::new_asciidoc(content);
        let expected = "Identity and Access Management (IAM)".to_string();
        assert_eq!(expected, rfd.get_title().unwrap());
    }

    #[test]
    fn test_get_asciidoc_title_without_rfd_prefix() {
        // Add a test to show what happens for rfd 31 where there is no "RFD" in
        // the title.
        let content = r#"sdfsdf
= Identity and Access Management (IAM)
:title: https://github.com/org/repo/pulls/1
dsfsdf
sdf
:title: nope"#;
        let rfd = RfdContent::new_asciidoc(content);
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

        let rfd = RfdContent::new_asciidoc(Cow::Borrowed(content));
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

    #[tokio::test]
    async fn test_asciidoc_to_pdf() {
        let rfd = RfdAsciidoc::new(Cow::Borrowed(test_rfd_content()));
        let pdf = RenderedPdf::render(&rfd, rfd.tmp_path().unwrap())
            .await
            .unwrap()
            .into_inner();

        let ref_path = format!(
            "{}/tests/ref/asciidoc_to_pdf.pdf",
            std::env::var("CARGO_MANIFEST_DIR").unwrap()
        );

        let store = std::env::var("TEST_PDF_UPDATE")
            .map(|var| &var == "true")
            .unwrap_or(false);

        if store {
            std::fs::write(&ref_path, &pdf).unwrap();
        }

        let expected = std::fs::read(&ref_path).unwrap();

        assert_eq!(expected, pdf);
    }
}
