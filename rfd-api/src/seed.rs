use chrono::{Duration, Utc};
use rfd_model::{storage::StoreError, ApiUser, NewApiUser, NewApiKey};
use serde::Serialize;
use thiserror::Error;
use uuid::Uuid;

use crate::{
    authn::key::NewApiKey,
    context::ApiContext,
    endpoints::api_user::InitialApiKKeyResponse,
    permissions::{ApiPermission, ApiUserPermission, RfdPermission},
};

#[derive(Debug, Serialize)]
pub struct SeedApiUser {
    pub user: ApiUser<ApiPermission>,
    pub token: InitialApiKKeyResponse,
}

#[derive(Debug, Error)]
pub enum SeedError {
    #[error("Unable create a seed user in a storage system that already contains users")]
    ExistingData,
    #[error("Storage failed when creating seed user: {0}")]
    Storage(#[from] StoreError),
}

pub async fn seed(ctx: &ApiContext) -> Result<SeedApiUser, SeedError> {
    if !ctx.is_empty().await? {
        tracing::error!("Failed to start service with seeded user. Data in the configured database already exists");
        return Err(SeedError::ExistingData);
    }

    let user_id = Uuid::new_v4();

    let user = ctx
        .update_api_user(NewApiUser {
            id: user_id,
            permissions: vec![
                RfdPermission::GetAllRfds.into(),
                ApiUserPermission::CreateApiUser.into(),
                ApiUserPermission::UpdateAllApiUser.into(),
                ApiUserPermission::CreateApiUserToken(user_id).into(),
                ApiUserPermission::GetApiUserToken(user_id).into(),
                ApiUserPermission::DeleteApiUserToken(user_id).into(),
            ]
            .into(),
        })
        .await?;

    let token_id = Uuid::new_v4();
    let (token, hash) = NewApiKey::generate::<24>(&token_id).consume();

    let stored_token = ctx
        .create_api_user_token(
            NewApiKey {
                id: token_id,
                api_user_id: user.id,
                token: hash,
                permissions: user.permissions.clone(),
                expires_at: Utc::now() + Duration::seconds(7 * 24 * 60 * 60),
            },
            &user,
        )
        .await?;

    Ok(SeedApiUser {
        user,
        token: InitialApiKKeyResponse {
            id: stored_token.id,
            token,
            permissions: stored_token.permissions,
            created_at: stored_token.created_at,
        },
    })
}
