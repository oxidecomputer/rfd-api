// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use dropshot::{endpoint, HttpError, RequestContext, TypedBody};
use http::Response;
use hyper::Body;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::context::ApiContext;

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct LocalLogin {
    pub external_id: String,
    pub email: String,
}

#[endpoint {
    method = POST,
    path = "/login/local"
}]
#[instrument(skip(rqctx), fields(request_id = rqctx.request_id), err(Debug))]
pub async fn local_login(
    rqctx: RequestContext<ApiContext>,
    body: TypedBody<LocalLogin>,
) -> Result<Response<Body>, HttpError> {
    #[cfg(feature = "local-dev")]
    {
        use chrono::Utc;
        use http::{header, StatusCode};

        use crate::endpoints::login::{DeviceTokenResponse, UserInfo};

        let ctx = rqctx.context();
        let body = body.into_inner();

        let info = UserInfo {
            external_id: super::ExternalUserId::Local(body.external_id),
            verified_emails: vec![body.email],
            display_name: Some("Local Dev".to_string()),
        };

        let (api_user, api_user_provider) = ctx
            .register_api_user(ctx.builtin_registration_user(), info)
            .await?;

        tracing::info!(api_user_id = ?api_user.id, api_user_provider_id = ?api_user_provider.id, "Retrieved api user to generate device token for");

        let token = ctx
            .register_access_token(
                ctx.builtin_registration_user(),
                &api_user,
                &api_user_provider,
                None,
            )
            .await?;

        tracing::info!(provider = "local", api_user_id = ?api_user.id, "Generated access token");

        Ok(Response::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, "application/json")
            .body(
                serde_json::to_string(&DeviceTokenResponse {
                    access_token: token.signed_token,
                    token_type: "Bearer".to_string(),
                    expires_in: Some(
                        (token.expires_at - Utc::now())
                            .num_seconds()
                            .try_into()
                            .unwrap_or(0),
                    ),
                    refresh_token: None,
                    scopes: None,
                })
                .unwrap()
                .into(),
            )?)
    }
    #[cfg(not(feature = "local-dev"))]
    {
        Err(HttpError::for_not_found(
            None,
            "Local login is not supported".to_string(),
        ))
    }
}
