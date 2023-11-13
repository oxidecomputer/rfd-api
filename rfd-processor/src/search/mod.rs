// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use hmac::{Hmac, Mac};
use md5::Md5;
use meilisearch_sdk::{
    errors::{Error as MeiliError, ErrorCode},
    indexes::Index,
    settings::Settings,
    Client,
};
use parse_rfd::{parse, ParsedDoc, ParserError, Section};
use rfd_data::RfdNumber;
use serde::{Deserialize, Serialize};
use std::{cmp::min, collections::HashMap};
use thiserror::Error;
use tracing::instrument;

#[derive(Debug, Error)]
pub enum SearchError {
    #[error(transparent)]
    Client(#[from] MeiliError),
    #[error(transparent)]
    Parse(#[from] ParserError),
}

#[derive(Debug)]
pub struct RfdSearchIndex {
    client: Client,
    index: String,
}

impl RfdSearchIndex {
    pub fn new(
        host: impl Into<String>,
        api_key: impl Into<String>,
        index: impl Into<String>,
    ) -> Self {
        Self {
            client: Client::new(host, Some(api_key)),
            index: index.into(),
        }
    }

    /// Trigger updating the search index for the RFD.
    #[instrument(skip(self, content), fields(index = ?self.index), err(Debug))]
    pub async fn index_rfd(
        &self,
        rfd_number: &RfdNumber,
        content: &str,
    ) -> Result<(), SearchError> {
        let index = self.client.index(&self.index);

        let lookup = self.find_rfd_ids(&index, rfd_number).await;

        // The index may not exist yet if this is the first RFD being indexed. In that case the err
        // returned from look up is noted and then discarded.
        match lookup {
            Ok(ids_to_delete) => {
                tracing::info!(?ids_to_delete, "Deleting documents for RFD");
                index.delete_documents(&ids_to_delete).await?;
            }
            Err(SearchError::Client(MeiliError::Meilisearch(err)))
                if err.error_code == ErrorCode::IndexNotFound
                    || err.error_code == ErrorCode::InvalidSearchFilter =>
            {
                tracing::info!(
                    ?err,
                    "Failed to find index during deletion lookup. Creating index and filters"
                );

                let settings = Settings::new().with_filterable_attributes(["rfd_number"]);
                index.set_settings(&settings).await?;
            }
            Err(err) => {
                return Err(err)?;
            }
        }

        let parsed = Self::parse_document(rfd_number, content)?;

        tracing::info!(count = parsed.len(), "Parsed RFD into sections to index");

        index.add_documents(&parsed, Some("objectID")).await?;

        Ok(())
    }

    #[instrument(skip(self, index))]
    pub async fn find_rfd_ids(
        &self,
        index: &Index,
        rfd_number: &RfdNumber,
    ) -> Result<Vec<String>, SearchError> {
        let mut query = index.search();
        let filter = format!("rfd_number = {}", rfd_number);
        query.with_array_filter(vec![&filter]);

        tracing::trace!(?filter, "Search for existing RFDs");

        let results = query.execute::<RfdId>().await?;

        Ok(results
            .hits
            .into_iter()
            .map(|search_result| search_result.result.object_id)
            .collect::<Vec<_>>())
    }

    #[instrument(skip(content), err(Debug))]
    pub fn parse_document(
        rfd_number: &RfdNumber,
        content: &str,
    ) -> Result<Vec<IndexDocument>, SearchError> {
        let ParsedDoc { title, sections } = parse(content)?;
        Ok(sections
            .into_iter()
            .map(|section| IndexDocument::new(section, rfd_number, &title))
            .collect::<Vec<_>>())
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RfdId {
    #[serde(rename = "objectID")]
    object_id: String,
}

type HmacMd5 = Hmac<Md5>;

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct IndexDocument {
    #[serde(rename = "objectID")]
    pub object_id: String,
    pub name: String,
    pub level: usize,
    pub anchor: String,
    pub content: String,
    pub rfd_number: i32,
    #[serde(flatten)]
    pub hierarchy: HashMap<String, String>,
    #[serde(flatten)]
    pub hierarchy_radio: HashMap<String, String>,
}

impl IndexDocument {
    pub fn new(section: Section, rfd_number: &RfdNumber, title: &str) -> Self {
        let level = section.parents.len() + 1;

        let mut hierarchy_radio = HashMap::new();
        if level == 1 {
            hierarchy_radio.insert("hierarchy_radio_lvl1".to_string(), section.name.clone());
        } else {
            hierarchy_radio.insert(
                format!("hierarchy_radio_lvl{}", min(5, level)),
                section.parents[section.parents.len() - 1].clone(),
            );
        }

        let mut hierarchy = HashMap::new();
        hierarchy.insert(
            "hierarchy_lvl0".to_string(),
            format!("RFD {} {}", rfd_number, title),
        );
        hierarchy.insert("hierarchy_lvl1".to_string(), section.name.to_string());

        for (i, section_name) in section.parents.into_iter().enumerate() {
            hierarchy.insert(format!("hierarchy_lvl{}", i + 2), section_name);
        }

        // The hash here is only intended to enforce uniqueness amongst documents. md5 and the
        // statically defined key is being used to maintain backward compatibility with previous
        // implementations. None of the key, the ids, nor hash are required to be kept secret
        let mut mac = HmacMd5::new_from_slice("dsflkajsdf".as_bytes())
            .expect("Statically defined key should always be valid");
        mac.update(rfd_number.as_number_string().as_bytes());
        mac.update(section.section_id.as_bytes());
        let object_id = hex::encode(&mac.finalize().into_bytes()[..]);

        Self {
            object_id,
            name: section.name,
            level,
            anchor: section.section_id,
            content: section.content,
            rfd_number: rfd_number.into(),
            hierarchy,
            hierarchy_radio,
        }
    }
}
