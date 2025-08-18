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

diesel::joinable!(rfd_pdf -> rfd (rfd_id));
diesel::joinable!(rfd_pdf -> rfd_revision (rfd_revision_id));
diesel::joinable!(rfd_revision -> rfd (rfd_id));

diesel::allow_tables_to_appear_in_same_query!(
    job,
    rfd,
    rfd_pdf,
    rfd_revision,
);
