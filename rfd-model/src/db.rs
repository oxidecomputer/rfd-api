// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use chrono::{DateTime, Utc};
use diesel::{Insertable, Queryable, Selectable};
use partial_struct::partial;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    schema::{job, rfd, rfd_pdf, rfd_revision},
    schema_ext::{ContentFormat, PdfSource, Visibility},
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

#[partial(RfdRevisionMetaModel)]
#[derive(Debug, Deserialize, Serialize, Queryable, Insertable, Selectable)]
#[diesel(table_name = rfd_revision)]
pub struct RfdRevisionModel {
    pub id: Uuid,
    pub rfd_id: Uuid,
    pub title: String,
    pub state: Option<String>,
    pub discussion: Option<String>,
    pub authors: Option<String>,
    #[partial(RfdRevisionMetaModel(skip))]
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
