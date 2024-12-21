// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

pub mod sql_types {
    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "rfd_content_format"))]
    pub struct RfdContentFormat;

    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "rfd_pdf_source"))]
    pub struct RfdPdfSource;

    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "rfd_visibility"))]
    pub struct RfdVisibility;
}

diesel::table! {
    job (id) {
        id -> Int4,
        owner -> Varchar,
        repository -> Varchar,
        branch -> Varchar,
        sha -> Varchar,
        rfd -> Int4,
        webhook_delivery_id -> Nullable<Uuid>,
        committed_at -> Timestamptz,
        processed -> Bool,
        created_at -> Timestamptz,
        started_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::RfdVisibility;

    rfd (id) {
        id -> Uuid,
        rfd_number -> Int4,
        link -> Nullable<Varchar>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        deleted_at -> Nullable<Timestamptz>,
        visibility -> RfdVisibility,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::RfdPdfSource;

    rfd_pdf (id) {
        id -> Uuid,
        rfd_revision_id -> Uuid,
        source -> RfdPdfSource,
        link -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        deleted_at -> Nullable<Timestamptz>,
        rfd_id -> Uuid,
        external_id -> Varchar,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::RfdContentFormat;

    rfd_revision (id) {
        id -> Uuid,
        rfd_id -> Uuid,
        title -> Varchar,
        state -> Nullable<Varchar>,
        discussion -> Nullable<Varchar>,
        authors -> Nullable<Varchar>,
        content -> Varchar,
        content_format -> RfdContentFormat,
        sha -> Varchar,
        commit_sha -> Varchar,
        committed_at -> Timestamptz,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        deleted_at -> Nullable<Timestamptz>,
        labels -> Nullable<Varchar>,
    }
}

diesel::table! {
    rfd_comment_user (id) {
        id -> Uuid,
        github_user_id -> Integer,
        github_user_node_id -> Varchar,
        github_user_username -> Nullable<Varchar>,
        github_user_avatar_url -> Nullable<Varchar>,
        github_user_type -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        deleted_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    rfd_comment (id) {
        id -> Uuid,
        rfd_id -> Uuid,
        comment_user_id -> Uuid,
        external_id -> Integer,
        node_id -> Varchar,
        discussion_number -> Nullable<Integer>,
        diff_hunk -> Varchar,
        path -> Varchar,
        body -> Nullable<Varchar>,
        commit_id -> Varchar,
        original_commit_id -> Varchar,
        line -> Nullable<Integer>,
        original_line -> Nullable<Integer>,
        start_line -> Nullable<Integer>,
        original_start_line -> Nullable<Integer>,
        side -> Nullable<Varchar>,
        start_side -> Nullable<Varchar>,
        subject -> Varchar,
        in_reply_to -> Nullable<Integer>,
        comment_created_at -> Nullable<Timestamptz>,
        comment_updated_at -> Nullable<Timestamptz>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        deleted_at -> Nullable<Timestamptz>,
    }
}

diesel::joinable!(rfd_pdf -> rfd (rfd_id));
diesel::joinable!(rfd_pdf -> rfd_revision (rfd_revision_id));
diesel::joinable!(rfd_revision -> rfd (rfd_id));
diesel::joinable!(rfd_comment -> rfd (rfd_id));
diesel::joinable!(rfd_comment -> rfd_comment_user (comment_user_id));

diesel::allow_tables_to_appear_in_same_query!(
    job,
    rfd,
    rfd_pdf,
    rfd_revision,
    rfd_comment,
    rfd_comment_user
);
