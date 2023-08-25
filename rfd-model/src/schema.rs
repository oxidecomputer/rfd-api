// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "dispatch_mode"))]
    pub struct DispatchMode;

    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "dispatch_status"))]
    pub struct DispatchStatus;

    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "rfd_content_format"))]
    pub struct RfdContentFormat;

    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "rfd_pdf_source"))]
    pub struct RfdPdfSource;
}

diesel::table! {
    allow_list (id) {
        id -> Int4,
        username -> Varchar,
        #[sql_name = "type"]
        type_ -> Varchar,
        rules -> Array<Nullable<Int4>>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    api_user (id) {
        id -> Uuid,
        permissions -> Jsonb,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        deleted_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    api_user_access_token (id) {
        id -> Uuid,
        api_user_id -> Uuid,
        revoked_at -> Nullable<Timestamptz>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    api_user_provider (id) {
        id -> Uuid,
        api_user_id -> Uuid,
        provider -> Varchar,
        provider_id -> Varchar,
        emails -> Array<Nullable<Text>>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        deleted_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    api_user_token (id) {
        id -> Uuid,
        api_user_id -> Uuid,
        token -> Text,
        permissions -> Jsonb,
        expires_at -> Timestamptz,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        deleted_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::DispatchMode;
    use super::sql_types::DispatchStatus;

    dispatch (id) {
        id -> Int4,
        dispatch_id -> Uuid,
        mode -> DispatchMode,
        pattern -> Varchar,
        workflow -> Int8,
        owner -> Varchar,
        repository -> Varchar,
        #[sql_name = "ref"]
        ref_ -> Varchar,
        response_status -> Int4,
        duration -> Int8,
        created_at -> Timestamptz,
        source -> Nullable<Uuid>,
        requires_token -> Bool,
        status -> DispatchStatus,
        scheduled_for -> Timestamptz,
    }
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
    }
}

diesel::table! {
    rfd (id) {
        id -> Uuid,
        rfd_number -> Int4,
        link -> Nullable<Varchar>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        deleted_at -> Nullable<Timestamptz>,
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
        labels -> Nullable<Varchar>,
        content -> Varchar,
        content_format -> RfdContentFormat,
        sha -> Varchar,
        commit_sha -> Varchar,
        committed_at -> Timestamptz,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        deleted_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    rule (id) {
        id -> Int4,
        pattern -> Varchar,
        target_repository -> Varchar,
        target_owner -> Varchar,
        target_ref -> Varchar,
        target_workflow -> Int8,
        enabled -> Bool,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        requires_token -> Bool,
        debounce -> Int4,
        conditions -> Nullable<Jsonb>,
    }
}

diesel::joinable!(api_user_access_token -> api_user (api_user_id));
diesel::joinable!(api_user_provider -> api_user (api_user_id));
diesel::joinable!(api_user_token -> api_user (api_user_id));
diesel::joinable!(rfd_pdf -> rfd_revision (rfd_revision_id));
diesel::joinable!(rfd_revision -> rfd (rfd_id));

diesel::allow_tables_to_appear_in_same_query!(
    allow_list,
    api_user,
    api_user_access_token,
    api_user_provider,
    api_user_token,
    dispatch,
    job,
    rfd,
    rfd_pdf,
    rfd_revision,
    rule,
);
