// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use async_trait::async_trait;
use base64::{prelude::BASE64_STANDARD, DecodeError, Engine};
use octorust::{repos::Repos, types::ContentFile, ClientError};
use tracing::instrument;

#[async_trait]
pub trait ReposExt {
    async fn get_content_blob(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
        file: &str,
    ) -> Result<ContentFile, ClientError>;
}

#[async_trait]
impl ReposExt for Repos {
    #[instrument(skip(self))]
    async fn get_content_blob(
        &self,
        owner: &str,
        repo: &str,
        ref_: &str,
        file: &str,
    ) -> Result<ContentFile, ClientError> {
        tracing::trace!("Fetching content from GitHub");
        let mut file = self.get_content_file(owner, repo, file, ref_).await?.body;

        // If the content is empty and the encoding is none then we likely hit a "too large" file case.
        // Try requesting the blob directly
        if file.content.is_empty() && file.encoding == "none" {
            let blob = self
                .client
                .git()
                .get_blob(owner, repo, &file.sha)
                .await?
                .body;

            // We are only interested in the copying over the content and encoding fields, everything
            // else from the original response should still be valid
            file.content = blob.content;
            file.encoding = blob.encoding;
        }

        Ok(file)
    }
}

pub trait ContentFileExt {
    fn is_empty(&self) -> bool;
    fn decode(&self) -> Result<Vec<u8>, DecodeError>;
}

impl ContentFileExt for ContentFile {
    fn is_empty(&self) -> bool {
        self.content.is_empty() && self.sha.is_empty()
    }

    fn decode(&self) -> Result<Vec<u8>, DecodeError> {
        BASE64_STANDARD
            .decode(self.content.replace('\n', ""))
            .map(|data| data.trim().to_vec())
    }
}

trait SliceExt {
    fn trim(&self) -> Self;
}

impl SliceExt for Vec<u8> {
    fn trim(&self) -> Vec<u8> {
        fn is_whitespace(c: &u8) -> bool {
            c == &b'\t' || c == &b' '
        }

        fn is_not_whitespace(c: &u8) -> bool {
            !is_whitespace(c)
        }

        if let Some(first) = self.iter().position(is_not_whitespace) {
            if let Some(last) = self.iter().rposition(is_not_whitespace) {
                self[first..last + 1].to_vec()
            } else {
                unreachable!();
            }
        } else {
            vec![]
        }
    }
}
