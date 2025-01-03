// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use chrono::{DateTime, Utc};
use diesel::{prelude::QueryableByName, Insertable, Queryable, Selectable};
use partial_struct::partial;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    schema::{job, rfd, rfd_pdf, rfd_revision},
    schema_ext::{rfd_pdfs, ContentFormat, PdfSource, Visibility},
};

#[derive(Debug, Deserialize, Serialize, Queryable, Insertable, Selectable)]
#[diesel(table_name = rfd)]
pub struct RfdModel {
    pub id: Uuid,
    pub rfd_number: i32,
    pub link: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub visibility: Visibility,
}

#[derive(QueryableByName)]
#[diesel(table_name = rfd_pdfs)]
pub struct RfdPdfsRow {
    pub id: Uuid,
    pub rfd_number: i32,
    pub link: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub visibility: Visibility,
    pub revision_id: Uuid,
    pub revision_rfd_id: Uuid,
    pub revision_title: String,
    pub revision_state: Option<String>,
    pub revision_discussion: Option<String>,
    pub revision_authors: Option<String>,
    pub pdf_id: Uuid,
    pub pdf_rfd_revision_id: Uuid,
    pub pdf_source: PdfSource,
    pub pdf_link: String,
    pub pdf_created_at: DateTime<Utc>,
    pub pdf_updated_at: DateTime<Utc>,
    pub pdf_deleted_at: Option<DateTime<Utc>>,
    pub pdf_rfd_id: Uuid,
    pub pdf_external_id: String,
    pub revision_content_format: ContentFormat,
    pub revision_sha: String,
    pub revision_commit_sha: String,
    pub revision_committed_at: DateTime<Utc>,
    pub revision_created_at: DateTime<Utc>,
    pub revision_updated_at: DateTime<Utc>,
    pub revision_deleted_at: Option<DateTime<Utc>>,
    pub revision_labels: Option<String>,
}

#[partial(RfdRevisionMetaModel)]
#[partial(RfdRevisionPdfModel)]
#[derive(Debug, Deserialize, Serialize, Queryable, Selectable)]
#[diesel(table_name = rfd_revision)]
pub struct RfdRevisionModel {
    pub id: Uuid,
    pub rfd_id: Uuid,
    pub title: String,
    pub state: Option<String>,
    pub discussion: Option<String>,
    pub authors: Option<String>,
    #[partial(RfdRevisionMetaModel(skip))]
    #[partial(RfdRevisionPdfModel(retype = RfdPdfModel))]
    pub content: String,
    pub content_format: ContentFormat,
    pub sha: String,
    pub commit_sha: String,
    pub committed_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub labels: Option<String>,
}

impl From<RfdPdfsRow> for (RfdModel, RfdRevisionPdfModel) {
    fn from(value: RfdPdfsRow) -> Self {
        (
            RfdModel {
                id: value.id,
                rfd_number: value.rfd_number,
                link: value.link,
                created_at: value.created_at,
                updated_at: value.updated_at,
                deleted_at: value.deleted_at,
                visibility: value.visibility,
            },
            RfdRevisionPdfModel {
                id: value.revision_id,
                rfd_id: value.revision_rfd_id,
                title: value.revision_title,
                state: value.revision_state,
                discussion: value.revision_discussion,
                authors: value.revision_authors,
                content: RfdPdfModel {
                    id: value.pdf_id,
                    rfd_revision_id: value.pdf_rfd_revision_id,
                    source: value.pdf_source,
                    link: value.pdf_link,
                    created_at: value.pdf_created_at,
                    updated_at: value.pdf_updated_at,
                    deleted_at: value.pdf_deleted_at,
                    rfd_id: value.pdf_rfd_id,
                    external_id: value.pdf_external_id,
                },
                content_format: value.revision_content_format,
                sha: value.revision_sha,
                commit_sha: value.revision_commit_sha,
                committed_at: value.revision_committed_at,
                created_at: value.revision_created_at,
                updated_at: value.revision_updated_at,
                deleted_at: value.revision_deleted_at,
                labels: value.revision_labels,
            },
        )
    }
}

#[derive(Debug, Deserialize, Serialize, Queryable, Insertable)]
#[diesel(table_name = rfd_pdf)]
pub struct RfdPdfModel {
    pub id: Uuid,
    pub rfd_revision_id: Uuid,
    pub source: PdfSource,
    pub link: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub rfd_id: Uuid,
    pub external_id: String,
}

#[derive(Debug, Deserialize, Serialize, Queryable, Insertable)]
#[diesel(table_name = job)]
pub struct JobModel {
    pub id: i32,
    pub owner: String,
    pub repository: String,
    pub branch: String,
    pub sha: String,
    pub rfd: i32,
    pub webhook_delivery_id: Option<Uuid>,
    pub committed_at: DateTime<Utc>,
    pub processed: bool,
    pub created_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
}
