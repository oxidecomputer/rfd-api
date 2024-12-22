// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use chrono::{DateTime, Utc};
use db::{
    JobModel, RfdCommentModel, RfdCommentUserModel, RfdModel, RfdPdfModel, RfdReviewCommentModel,
    RfdReviewModel, RfdRevisionMetaModel, RfdRevisionModel,
};
use newtype_uuid::{GenericUuid, TypedUuid, TypedUuidKind, TypedUuidTag};
use partial_struct::partial;
use schema_ext::{ContentFormat, PdfSource, Visibility};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use thiserror::Error;

pub mod db;
pub mod schema;
pub mod schema_ext;
pub mod storage;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CommitSha(pub String);

impl Display for CommitSha {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for CommitSha {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<CommitSha> for String {
    fn from(value: CommitSha) -> Self {
        value.0
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct FileSha(pub String);

impl From<String> for FileSha {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<FileSha> for String {
    fn from(value: FileSha) -> Self {
        value.0
    }
}

#[derive(JsonSchema)]
pub enum RfdId {}
impl TypedUuidKind for RfdId {
    fn tag() -> TypedUuidTag {
        const TAG: TypedUuidTag = TypedUuidTag::new("rfd");
        TAG
    }
}

#[partial(NewRfd)]
#[partial(RfdMeta)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct Rfd {
    pub id: TypedUuid<RfdId>,
    pub rfd_number: i32,
    pub link: Option<String>,
    #[partial(NewRfd(skip))]
    #[partial(RfdMeta(retype = RfdRevisionMeta))]
    pub content: RfdRevision,
    #[partial(NewRfd(skip))]
    pub created_at: DateTime<Utc>,
    #[partial(NewRfd(skip))]
    pub updated_at: DateTime<Utc>,
    #[partial(NewRfd(skip))]
    pub deleted_at: Option<DateTime<Utc>>,
    pub visibility: Visibility,
}

impl From<(RfdModel, RfdRevisionModel)> for Rfd {
    fn from((rfd, revision): (RfdModel, RfdRevisionModel)) -> Self {
        Self {
            id: TypedUuid::from_untyped_uuid(rfd.id),
            rfd_number: rfd.rfd_number,
            link: rfd.link,
            content: revision.into(),
            created_at: rfd.created_at,
            updated_at: rfd.updated_at,
            deleted_at: rfd.deleted_at,
            visibility: rfd.visibility,
        }
    }
}

impl From<(RfdModel, RfdRevisionMetaModel)> for RfdMeta {
    fn from((rfd, revision): (RfdModel, RfdRevisionMetaModel)) -> Self {
        Self {
            id: TypedUuid::from_untyped_uuid(rfd.id),
            rfd_number: rfd.rfd_number,
            link: rfd.link,
            content: revision.into(),
            created_at: rfd.created_at,
            updated_at: rfd.updated_at,
            deleted_at: rfd.deleted_at,
            visibility: rfd.visibility,
        }
    }
}

impl From<RfdMeta> for NewRfd {
    fn from(value: RfdMeta) -> Self {
        Self {
            id: value.id,
            rfd_number: value.rfd_number,
            link: value.link,
            visibility: value.visibility,
        }
    }
}

#[derive(JsonSchema)]
pub enum RfdRevisionId {}
impl TypedUuidKind for RfdRevisionId {
    fn tag() -> TypedUuidTag {
        const TAG: TypedUuidTag = TypedUuidTag::new("rfd-revision");
        TAG
    }
}

#[partial(NewRfdRevision)]
#[partial(RfdRevisionMeta)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct RfdRevision {
    pub id: TypedUuid<RfdRevisionId>,
    pub rfd_id: TypedUuid<RfdId>,
    pub title: String,
    pub state: Option<String>,
    pub discussion: Option<String>,
    pub authors: Option<String>,
    pub labels: Option<String>,
    #[partial(RfdRevisionMeta(skip))]
    pub content: String,
    pub content_format: ContentFormat,
    pub sha: FileSha,
    pub commit: CommitSha,
    pub committed_at: DateTime<Utc>,
    #[partial(NewRfdRevision(skip))]
    pub created_at: DateTime<Utc>,
    #[partial(NewRfdRevision(skip))]
    pub updated_at: DateTime<Utc>,
    #[partial(NewRfdRevision(skip))]
    pub deleted_at: Option<DateTime<Utc>>,
}

impl From<RfdRevisionModel> for RfdRevision {
    fn from(value: RfdRevisionModel) -> Self {
        Self {
            id: TypedUuid::from_untyped_uuid(value.id),
            rfd_id: TypedUuid::from_untyped_uuid(value.rfd_id),
            title: value.title,
            state: value.state,
            discussion: value.discussion,
            authors: value.authors,
            labels: value.labels,
            content: value.content,
            content_format: value.content_format,
            sha: value.sha.into(),
            commit: value.commit_sha.into(),
            committed_at: value.committed_at,
            created_at: value.created_at,
            updated_at: value.updated_at,
            deleted_at: value.deleted_at,
        }
    }
}

impl From<RfdRevisionMetaModel> for RfdRevisionMeta {
    fn from(value: RfdRevisionMetaModel) -> Self {
        Self {
            id: TypedUuid::from_untyped_uuid(value.id),
            rfd_id: TypedUuid::from_untyped_uuid(value.rfd_id),
            title: value.title,
            state: value.state,
            discussion: value.discussion,
            authors: value.authors,
            labels: value.labels,
            content_format: value.content_format,
            sha: value.sha.into(),
            commit: value.commit_sha.into(),
            committed_at: value.committed_at,
            created_at: value.created_at,
            updated_at: value.updated_at,
            deleted_at: value.deleted_at,
        }
    }
}

#[derive(JsonSchema)]
pub enum RfdPdfId {}
impl TypedUuidKind for RfdPdfId {
    fn tag() -> TypedUuidTag {
        const TAG: TypedUuidTag = TypedUuidTag::new("rfd-pdf");
        TAG
    }
}

#[partial(NewRfdPdf)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct RfdPdf {
    pub id: TypedUuid<RfdPdfId>,
    pub rfd_revision_id: TypedUuid<RfdRevisionId>,
    pub source: PdfSource,
    pub link: String,
    #[partial(NewRfdPdf(skip))]
    pub created_at: DateTime<Utc>,
    #[partial(NewRfdPdf(skip))]
    pub updated_at: DateTime<Utc>,
    #[partial(NewRfdPdf(skip))]
    pub deleted_at: Option<DateTime<Utc>>,
    pub rfd_id: TypedUuid<RfdId>,
    pub external_id: String,
}

impl From<RfdPdfModel> for RfdPdf {
    fn from(value: RfdPdfModel) -> Self {
        Self {
            id: TypedUuid::from_untyped_uuid(value.id),
            rfd_revision_id: TypedUuid::from_untyped_uuid(value.rfd_revision_id),
            source: value.source,
            link: value.link,
            created_at: value.created_at,
            updated_at: value.updated_at,
            deleted_at: value.deleted_at,
            rfd_id: TypedUuid::from_untyped_uuid(value.rfd_id),
            external_id: value.external_id,
        }
    }
}

#[derive(JsonSchema)]
pub enum WebhookDeliveryId {}
impl TypedUuidKind for WebhookDeliveryId {
    fn tag() -> TypedUuidTag {
        const TAG: TypedUuidTag = TypedUuidTag::new("webhook-delivery");
        TAG
    }
}

#[partial(NewJob)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct Job {
    #[partial(NewJob(skip))]
    pub id: i32,
    pub owner: String,
    pub repository: String,
    pub branch: String,
    pub sha: CommitSha,
    pub rfd: i32,
    pub webhook_delivery_id: Option<TypedUuid<WebhookDeliveryId>>,
    pub committed_at: DateTime<Utc>,
    #[partial(NewJob(skip))]
    pub processed: bool,
    #[partial(NewJob(skip))]
    pub created_at: DateTime<Utc>,
    #[partial(NewJob(skip))]
    pub started_at: Option<DateTime<Utc>>,
}

impl From<JobModel> for Job {
    fn from(value: JobModel) -> Self {
        Self {
            id: value.id,
            owner: value.owner,
            repository: value.repository,
            branch: value.branch,
            sha: value.sha.into(),
            rfd: value.rfd,
            webhook_delivery_id: value.webhook_delivery_id.map(TypedUuid::from_untyped_uuid),
            committed_at: value.committed_at,
            processed: value.processed,
            created_at: value.created_at,
            started_at: value.started_at,
        }
    }
}

#[derive(Debug, Error)]
pub struct InvalidValueError {
    pub field: String,
    pub error: String,
}

impl Display for InvalidValueError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} has an invalid value: {}", self.field, self.error)
    }
}

#[derive(JsonSchema)]
pub enum RfdCommentUserId {}
impl TypedUuidKind for RfdCommentUserId {
    fn tag() -> TypedUuidTag {
        const TAG: TypedUuidTag = TypedUuidTag::new("rfd-comment-user");
        TAG
    }
}

#[partial(NewRfdCommentUser)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct RfdCommentUser {
    pub id: TypedUuid<RfdCommentUserId>,
    pub external_id: i32,
    pub node_id: String,
    pub user_username: Option<String>,
    pub user_avatar_url: Option<String>,
    pub user_type: String,
    #[partial(NewRfdCommentUser(skip))]
    pub created_at: DateTime<Utc>,
    #[partial(NewRfdCommentUser(skip))]
    pub updated_at: DateTime<Utc>,
    #[partial(NewRfdCommentUser(skip))]
    pub deleted_at: Option<DateTime<Utc>>,
}

impl From<RfdCommentUserModel> for RfdCommentUser {
    fn from(value: RfdCommentUserModel) -> Self {
        Self {
            id: TypedUuid::from_untyped_uuid(value.id),
            external_id: value.external_id,
            node_id: value.node_id,
            user_username: value.user_username,
            user_avatar_url: value.user_avatar_url,
            user_type: value.user_type,
            created_at: value.created_at,
            updated_at: value.updated_at,
            deleted_at: value.deleted_at,
        }
    }
}

#[derive(JsonSchema)]
pub enum RfdReviewId {}
impl TypedUuidKind for RfdReviewId {
    fn tag() -> TypedUuidTag {
        const TAG: TypedUuidTag = TypedUuidTag::new("rfd-review");
        TAG
    }
}

#[partial(NewRfdReview)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct RfdReview {
    pub id: TypedUuid<RfdReviewId>,
    pub rfd_id: TypedUuid<RfdId>,
    #[partial(NewRfdReview(retype = TypedUuid<RfdCommentUserId>))]
    pub comment_user: RfdCommentUser,
    pub external_id: i32,
    pub node_id: String,
    pub body: String,
    pub state: String,
    pub commit_id: String,
    pub review_created_at: DateTime<Utc>,
    #[partial(NewRfdReview(skip))]
    pub created_at: DateTime<Utc>,
    #[partial(NewRfdReview(skip))]
    pub updated_at: DateTime<Utc>,
    #[partial(NewRfdReview(skip))]
    pub deleted_at: Option<DateTime<Utc>>,
}

impl From<(RfdReviewModel, RfdCommentUserModel)> for RfdReview {
    fn from((review, user): (RfdReviewModel, RfdCommentUserModel)) -> Self {
        Self {
            id: TypedUuid::from_untyped_uuid(review.id),
            rfd_id: TypedUuid::from_untyped_uuid(review.rfd_id),
            comment_user: user.into(),
            external_id: review.external_id,
            node_id: review.node_id,
            body: review.body,
            state: review.state,
            commit_id: review.commit_id,
            review_created_at: review.review_created_at,
            created_at: review.created_at,
            updated_at: review.updated_at,
            deleted_at: review.deleted_at,
        }
    }
}

#[derive(JsonSchema)]
pub enum RfdReviewCommentId {}
impl TypedUuidKind for RfdReviewCommentId {
    fn tag() -> TypedUuidTag {
        const TAG: TypedUuidTag = TypedUuidTag::new("rfd-review-comment");
        TAG
    }
}

#[partial(NewRfdReviewComment)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct RfdReviewComment {
    pub id: TypedUuid<RfdReviewCommentId>,
    pub rfd_id: TypedUuid<RfdId>,
    #[partial(NewRfdReviewComment(retype = TypedUuid<RfdCommentUserId>))]
    pub comment_user: RfdCommentUser,
    pub external_id: i32,
    pub node_id: String,
    #[partial(NewRfdReviewComment(retype = Option<TypedUuid<RfdReviewId>>))]
    pub review: Option<RfdReview>,
    pub diff_hunk: String,
    pub path: String,
    pub body: String,
    pub commit_id: String,
    pub original_commit_id: String,
    pub line: Option<i32>,
    pub original_line: Option<i32>,
    pub start_line: Option<i32>,
    pub original_start_line: Option<i32>,
    pub side: Option<String>,
    pub start_side: Option<String>,
    pub subject: String,
    pub in_reply_to: Option<i32>,
    pub comment_created_at: DateTime<Utc>,
    pub comment_updated_at: DateTime<Utc>,
    #[partial(NewRfdReviewComment(skip))]
    pub created_at: DateTime<Utc>,
    #[partial(NewRfdReviewComment(skip))]
    pub updated_at: DateTime<Utc>,
    #[partial(NewRfdReviewComment(skip))]
    pub deleted_at: Option<DateTime<Utc>>,
}

impl
    From<(
        RfdReviewCommentModel,
        RfdCommentUserModel,
        Option<RfdReviewModel>,
    )> for RfdReviewComment
{
    fn from(
        (comment, user, review): (
            RfdReviewCommentModel,
            RfdCommentUserModel,
            Option<RfdReviewModel>,
        ),
    ) -> Self {
        Self {
            id: TypedUuid::from_untyped_uuid(comment.id),
            rfd_id: TypedUuid::from_untyped_uuid(comment.rfd_id),
            comment_user: user.clone().into(),
            external_id: comment.external_id,
            node_id: comment.node_id,
            review: review.map(|r| (r, user).into()),
            diff_hunk: comment.diff_hunk,
            path: comment.path,
            body: comment.body,
            commit_id: comment.commit_id,
            original_commit_id: comment.original_commit_id,
            line: comment.line,
            original_line: comment.original_line,
            start_line: comment.start_line,
            original_start_line: comment.original_start_line,
            side: comment.side,
            start_side: comment.start_side,
            subject: comment.subject,
            in_reply_to: comment.in_reply_to,
            comment_created_at: comment.comment_created_at,
            comment_updated_at: comment.comment_updated_at,
            created_at: comment.created_at,
            updated_at: comment.updated_at,
            deleted_at: comment.deleted_at,
        }
    }
}

#[derive(JsonSchema)]
pub enum RfdCommentId {}
impl TypedUuidKind for RfdCommentId {
    fn tag() -> TypedUuidTag {
        const TAG: TypedUuidTag = TypedUuidTag::new("rfd-review-comment");
        TAG
    }
}

#[partial(NewRfdComment)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct RfdComment {
    pub id: TypedUuid<RfdCommentId>,
    pub rfd_id: TypedUuid<RfdId>,
    #[partial(NewRfdComment(retype = TypedUuid<RfdCommentUserId>))]
    pub comment_user: RfdCommentUser,
    pub external_id: i32,
    pub node_id: String,
    pub body: String,
    pub comment_created_at: DateTime<Utc>,
    pub comment_updated_at: DateTime<Utc>,
    #[partial(NewRfdComment(skip))]
    pub created_at: DateTime<Utc>,
    #[partial(NewRfdComment(skip))]
    pub updated_at: DateTime<Utc>,
    #[partial(NewRfdComment(skip))]
    pub deleted_at: Option<DateTime<Utc>>,
}

impl From<(RfdCommentModel, RfdCommentUserModel)> for RfdComment {
    fn from((comment, user): (RfdCommentModel, RfdCommentUserModel)) -> Self {
        Self {
            id: TypedUuid::from_untyped_uuid(comment.id),
            rfd_id: TypedUuid::from_untyped_uuid(comment.rfd_id),
            comment_user: user.into(),
            external_id: comment.external_id,
            node_id: comment.node_id,
            body: comment.body,
            comment_created_at: comment.comment_created_at,
            comment_updated_at: comment.comment_updated_at,
            created_at: comment.created_at,
            updated_at: comment.updated_at,
            deleted_at: comment.deleted_at,
        }
    }
}
