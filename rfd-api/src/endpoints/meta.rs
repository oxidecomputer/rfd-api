// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use dropshot::{endpoint, ClientErrorStatusCode, HttpError, HttpResponseOk, Path, RequestContext};
use schemars::JsonSchema;
use serde::Deserialize;
use trace_request::trace_request;
use tracing::instrument;
use v_api::ApiContext;
use v_model::permissions::Caller;

use crate::{
    context::{RfdContext, RfdWithoutContent},
    permissions::RfdPermission,
    util::response::client_error,
};

#[derive(Debug, Deserialize, JsonSchema)]
pub struct RfdPathParams {
    /// The RFD number (examples: 1 or 123)
    number: String,
}

/// Get the latest representation of a RFD
#[trace_request]
#[endpoint {
    method = GET,
    path = "/meta/rfd/{number}",
}]
#[instrument(skip(rqctx), fields(request_id = rqctx.request_id), err(Debug))]
pub async fn view_rfd_meta(
    rqctx: RequestContext<RfdContext>,
    path: Path<RfdPathParams>,
) -> Result<HttpResponseOk<RfdWithoutContent>, HttpError> {
    let ctx = rqctx.context();
    let caller = ctx.v_ctx().get_caller(&rqctx).await?;
    view_rfd_meta_op(ctx, &caller, path.into_inner().number).await
}

#[instrument(skip(ctx, caller), fields(caller = ?caller.id), err(Debug))]
async fn view_rfd_meta_op(
    ctx: &RfdContext,
    caller: &Caller<RfdPermission>,
    number: String,
) -> Result<HttpResponseOk<RfdWithoutContent>, HttpError> {
    if let Ok(rfd_number) = number.parse::<i32>() {
        Ok(HttpResponseOk(
            ctx.view_rfd_meta(caller, rfd_number, None).await?,
        ))
    } else {
        Err(client_error(
            ClientErrorStatusCode::BAD_REQUEST,
            "Malformed RFD number",
        ))
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use chrono::Utc;
    use dropshot::HttpResponseOk;
    use http::StatusCode;
    use newtype_uuid::{GenericUuid, TypedUuid};
    use rfd_model::{
        schema_ext::ContentFormat,
        storage::{mock::MockStorage, MockRfdRevisionMetaStore, MockRfdStore},
        CommitSha, FileSha, Rfd, RfdRevision, RfdRevisionMeta,
    };
    use uuid::Uuid;
    use v_api::ApiContext;
    use v_model::{permissions::Caller, Permissions};

    use crate::{
        context::{test_mocks::mock_context, RfdContext},
        endpoints::meta::view_rfd_meta_op,
        permissions::RfdPermission,
    };

    async fn ctx() -> RfdContext {
        let private_rfd_id_1 = Uuid::new_v4();
        let private_rfd_id_2 = Uuid::new_v4();
        let public_rfd_id = Uuid::new_v4();

        let mut rfd_store = MockRfdStore::new();
        rfd_store.expect_list().returning(move |filter, _| {
            let mut results = vec![
                Rfd {
                    id: TypedUuid::from_untyped_uuid(private_rfd_id_1),
                    rfd_number: 123,
                    link: None,
                    content: RfdRevision {
                        id: TypedUuid::new_v4(),
                        rfd_id: TypedUuid::from_untyped_uuid(private_rfd_id_1),
                        title: String::new(),
                        state: None,
                        discussion: None,
                        authors: None,
                        labels: None,
                        content: String::new(),
                        content_format: ContentFormat::Asciidoc,
                        sha: FileSha(String::new()),
                        commit: CommitSha(String::new()),
                        committed_at: Utc::now(),
                        created_at: Utc::now(),
                        updated_at: Utc::now(),
                        deleted_at: None,
                    },
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                    deleted_at: None,
                    visibility: rfd_model::schema_ext::Visibility::Private,
                },
                Rfd {
                    id: TypedUuid::from_untyped_uuid(public_rfd_id),
                    rfd_number: 456,
                    link: None,
                    content: RfdRevision {
                        id: TypedUuid::new_v4(),
                        rfd_id: TypedUuid::from_untyped_uuid(private_rfd_id_1),
                        title: String::new(),
                        state: None,
                        discussion: None,
                        authors: None,
                        labels: None,
                        content: String::new(),
                        content_format: ContentFormat::Asciidoc,
                        sha: FileSha(String::new()),
                        commit: CommitSha(String::new()),
                        committed_at: Utc::now(),
                        created_at: Utc::now(),
                        updated_at: Utc::now(),
                        deleted_at: None,
                    },
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                    deleted_at: None,
                    visibility: rfd_model::schema_ext::Visibility::Public,
                },
                Rfd {
                    id: TypedUuid::from_untyped_uuid(private_rfd_id_2),
                    rfd_number: 789,
                    link: None,
                    content: RfdRevision {
                        id: TypedUuid::new_v4(),
                        rfd_id: TypedUuid::from_untyped_uuid(private_rfd_id_1),
                        title: String::new(),
                        state: None,
                        discussion: None,
                        authors: None,
                        labels: None,
                        content: String::new(),
                        content_format: ContentFormat::Asciidoc,
                        sha: FileSha(String::new()),
                        commit: CommitSha(String::new()),
                        committed_at: Utc::now(),
                        created_at: Utc::now(),
                        updated_at: Utc::now(),
                        deleted_at: None,
                    },
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                    deleted_at: None,
                    visibility: rfd_model::schema_ext::Visibility::Private,
                },
            ];

            results.retain(|rfd| {
                filter.rfd_number.is_none()
                    || filter
                        .rfd_number
                        .as_ref()
                        .unwrap()
                        .contains(&rfd.rfd_number)
            });

            Ok(results)
        });

        let mut rfd_revision_meta_store = MockRfdRevisionMetaStore::new();
        rfd_revision_meta_store
            .expect_list()
            .returning(move |filter, _| {
                let mut results = vec![
                    RfdRevisionMeta {
                        id: TypedUuid::new_v4(),
                        rfd_id: TypedUuid::from_untyped_uuid(private_rfd_id_1),
                        title: "Private Test RFD 1".to_string(),
                        state: None,
                        discussion: None,
                        authors: None,
                        labels: None,
                        content_format: rfd_model::schema_ext::ContentFormat::Asciidoc,
                        sha: String::new().into(),
                        commit: String::new().into(),
                        committed_at: Utc::now(),
                        created_at: Utc::now(),
                        updated_at: Utc::now(),
                        deleted_at: None,
                    },
                    RfdRevisionMeta {
                        id: TypedUuid::new_v4(),
                        rfd_id: TypedUuid::from_untyped_uuid(public_rfd_id),
                        title: "Public Test RFD".to_string(),
                        state: None,
                        discussion: None,
                        authors: None,
                        labels: None,
                        content_format: rfd_model::schema_ext::ContentFormat::Asciidoc,
                        sha: String::new().into(),
                        commit: String::new().into(),
                        committed_at: Utc::now(),
                        created_at: Utc::now(),
                        updated_at: Utc::now(),
                        deleted_at: None,
                    },
                    RfdRevisionMeta {
                        id: TypedUuid::new_v4(),
                        rfd_id: TypedUuid::from_untyped_uuid(private_rfd_id_2),
                        title: "Private Test RFD 2".to_string(),
                        state: None,
                        discussion: None,
                        authors: None,
                        labels: None,
                        content_format: rfd_model::schema_ext::ContentFormat::Asciidoc,
                        sha: String::new().into(),
                        commit: String::new().into(),
                        committed_at: Utc::now(),
                        created_at: Utc::now(),
                        updated_at: Utc::now(),
                        deleted_at: None,
                    },
                ];

                results.retain(|revision| {
                    filter.rfd.is_none() || filter.rfd.as_ref().unwrap().contains(&revision.rfd_id)
                });

                Ok(results)
            });

        let mut storage = MockStorage::new();
        storage.rfd_store = Some(Arc::new(rfd_store));
        storage.rfd_revision_meta_store = Some(Arc::new(rfd_revision_meta_store));

        mock_context(storage).await
    }

    // Test RFD access via the global All RFDs permission

    #[tokio::test]
    async fn get_rfd_via_all_permission() {
        let ctx = ctx().await;
        let caller = Caller::from(Permissions::from(vec![RfdPermission::GetRfdsAll]));

        let HttpResponseOk(rfd) = view_rfd_meta_op(&ctx, &caller, "0123".to_string())
            .await
            .unwrap();
        assert_eq!(123, rfd.rfd_number);

        let HttpResponseOk(rfd) = view_rfd_meta_op(&ctx, &caller, "0456".to_string())
            .await
            .unwrap();
        assert_eq!(456, rfd.rfd_number);
    }

    // Test RFD access via the direct permission to a RFD

    #[tokio::test]
    async fn get_rfd_with_direct_permission() {
        let ctx = ctx().await;
        let caller = Caller::from(Permissions::from(vec![RfdPermission::GetRfd(123)]));

        let HttpResponseOk(rfd) = view_rfd_meta_op(&ctx, &caller, "0123".to_string())
            .await
            .unwrap();
        assert_eq!(123, rfd.rfd_number);

        let HttpResponseOk(rfd) = view_rfd_meta_op(&ctx, &caller, "0456".to_string())
            .await
            .unwrap();
        assert_eq!(456, rfd.rfd_number);
    }

    // Test RFD access fails when a caller does not have permission

    #[tokio::test]
    async fn get_rfd_without_permission() {
        let ctx = ctx().await;
        let caller = Caller::from(Permissions::<RfdPermission>::new());

        let result = view_rfd_meta_op(&ctx, &caller, "0123".to_string()).await;

        match result {
            Err(err) => assert_eq!(StatusCode::NOT_FOUND, err.status_code),
            Ok(response) => panic!(
                "Expected a 404 error, but instead found a RFD {:?}",
                response.0
            ),
        }
    }

    // Test RFD access to public RFDs as the unauthenticated user

    #[tokio::test]
    async fn get_rfd_as_unauthenticated() {
        let ctx = ctx().await;
        let caller = ctx.v_ctx().builtin_unauthenticated_caller();

        let result = view_rfd_meta_op(&ctx, &caller, "0123".to_string()).await;
        match result {
            Err(err) => assert_eq!(StatusCode::NOT_FOUND, err.status_code),
            Ok(response) => panic!(
                "Expected a 404 error, but instead found a RFD {:?}",
                response.0
            ),
        }
    }
}
