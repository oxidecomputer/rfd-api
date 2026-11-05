// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::collections::HashSet;

use rfd_model::CommitSha;

pub const IGNORE_REVS_PATH: &str = ".git-blame-ignore-revs";

/// Commit SHAs listed in `.git-blame-ignore-revs`
#[derive(Debug, Default, Clone, PartialEq)]
pub struct IgnoredRevs(HashSet<String>);

impl IgnoredRevs {
    /// One full SHA per line, `#` starts a comment
    pub fn parse(content: &str) -> Self {
        let mut revs = HashSet::new();

        for (index, line) in content.lines().enumerate() {
            let line = line.split('#').next().unwrap_or_default().trim();

            if line.is_empty() {
                continue;
            }

            if Self::is_full_sha(line) {
                revs.insert(line.to_ascii_lowercase());
            } else {
                tracing::warn!(line = index + 1, value = line, "Invalid ignore-revs entry");
            }
        }

        Self(revs)
    }

    fn is_full_sha(value: &str) -> bool {
        value.len() == 40 && value.chars().all(|c| c.is_ascii_hexdigit())
    }

    pub fn contains(&self, sha: &CommitSha) -> bool {
        self.contains_str(&sha.0)
    }

    pub fn contains_str(&self, sha: &str) -> bool {
        self.0.contains(&sha.to_ascii_lowercase())
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}
