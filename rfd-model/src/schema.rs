// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "attempt_state"))]
    pub struct AttemptState;

    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "rfd_content_format"))]
    pub struct RfdContentFormat;

    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "rfd_pdf_source"))]
    pub struct RfdPdfSource;
}

diesel::table! {
    api_key (id) {
        id -> Uuid,
        api_user_id -> Uuid,
        key -> Text,
        permissions -> Jsonb,
        expires_at -> Timestamptz,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        deleted_at -> Nullable<Timestamptz>,
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
    use diesel::sql_types::*;
    use super::sql_types::AttemptState;

    login_attempt (id) {
        id -> Uuid,
        attempt_state -> AttemptState,
        client_id -> Uuid,
        redirect_uri -> Varchar,
        state -> Nullable<Varchar>,
        pkce_challenge -> Nullable<Varchar>,
        pkce_challenge_method -> Nullable<Varchar>,
        authz_code -> Nullable<Varchar>,
        expires_at -> Nullable<Timestamptz>,
        error -> Nullable<Varchar>,
        provider -> Varchar,
        provider_state -> Varchar,
        provider_pkce_verifier -> Nullable<Varchar>,
        provider_authz_code -> Nullable<Varchar>,
        provider_error -> Nullable<Varchar>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    oauth_client (id) {
        id -> Uuid,
        created_at -> Timestamptz,
        deleted_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    oauth_client_redirect_uri (id) {
        id -> Uuid,
        oauth_client_id -> Uuid,
        redirect_uri -> Varchar,
        created_at -> Timestamptz,
        deleted_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    oauth_client_secret (id) {
        id -> Uuid,
        oauth_client_id -> Uuid,
        secret -> Varchar,
        created_at -> Timestamptz,
        deleted_at -> Nullable<Timestamptz>,
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

diesel::joinable!(api_key -> api_user (api_user_id));
diesel::joinable!(api_user_access_token -> api_user (api_user_id));
diesel::joinable!(api_user_provider -> api_user (api_user_id));
diesel::joinable!(oauth_client_redirect_uri -> oauth_client (oauth_client_id));
diesel::joinable!(oauth_client_secret -> oauth_client (oauth_client_id));
diesel::joinable!(rfd_pdf -> rfd_revision (rfd_revision_id));
diesel::joinable!(rfd_revision -> rfd (rfd_id));

diesel::allow_tables_to_appear_in_same_query!(
    api_key,
    api_user,
    api_user_access_token,
    api_user_provider,
    job,
    login_attempt,
    oauth_client,
    oauth_client_redirect_uri,
    oauth_client_secret,
    rfd,
    rfd_pdf,
    rfd_revision,
);
