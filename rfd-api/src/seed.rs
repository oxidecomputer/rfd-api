use chrono::{Duration, Utc};
use rfd_model::{storage::StoreError, ApiUser, NewApiUser, NewApiKey};
use serde::Serialize;
use thiserror::Error;
use uuid::Uuid;

use crate::{
    authn::key::RawApiKey,
    context::ApiContext,
    endpoints::api_user::InitialApiKeyResponse,
    permissions::ApiPermission,
};

#[derive(Debug, Serialize)]
pub struct SeedApiUser {
    pub user: ApiUser<ApiPermission>,
    pub token: InitialApiKeyResponse,
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
                ApiPermission::CreateApiUserTokenAll,
                ApiPermission::GetApiUserAll,
                ApiPermission::GetApiUserTokenSelf,
                ApiPermission::DeleteApiUserTokenAll,
                ApiPermission::CreateApiUser,
                ApiPermission::UpdateApiUserAll,
                ApiPermission::GetAllRfds,
                ApiPermission::GetAllDiscussions,
                ApiPermission::CreateOAuthClient,
                ApiPermission::GetAssignedOAuthClients,
                ApiPermission::UpdateAssignedOAuthClients,
                ApiPermission::DeleteAssignedOAuthClients,
            ]
            .into(),
        })
        .await?;

    let token_id = Uuid::new_v4();
    let raw_key = RawApiKey::generate::<24>(&token_id);
    let encrypted_key = raw_key.sign(&*ctx.secrets.signer).await.unwrap();

    let stored_token = ctx
        .create_api_user_token(
            NewApiKey {
                id: token_id,
                api_user_id: user.id,
                key_signature: encrypted_key.signature().to_string(),
                permissions: user.permissions.clone(),
                expires_at: Utc::now() + Duration::seconds(7 * 24 * 60 * 60),
            },
            &user,
        )
        .await?;

    Ok(SeedApiUser {
        user,
        token: InitialApiKeyResponse {
            id: stored_token.id,
            key: encrypted_key.key(),
            permissions: stored_token.permissions,
            created_at: stored_token.created_at,
        },
    })
}
