// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use core::panic;

use anyhow::Result;
use clap::{Parser, Subcommand};
use jsonwebtoken::{Algorithm, DecodingKey, Validation};
use oauth2::TokenResponse;
use rfd_sdk::types::OAuthProviderName;
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    cmd::auth::{login::AuthenticationMode, oauth},
    Context,
};

use super::login::LoginProvider;

// Authenticates and generates an access token for interacting with the api
#[derive(Parser, Debug, Clone)]
#[clap(name = "link")]
pub struct Link {
    #[arg(value_enum)]
    provider: LoginProvider,
}

#[derive(Debug, Deserialize)]
struct Claims {
    prv: Uuid,
}

impl Link {
    pub async fn run(&self, ctx: &mut Context) -> Result<()> {
        // Determine the user id of the currently authenticated user
        let self_id = ctx.client()?.get_self().send().await?.info.id;

        let access_token = self
            .provider
            .run(ctx, &AuthenticationMode::Identity)
            .await?;

        // Fetch the public JWKS from the remote API
        let jwks = ctx.client()?.jwks_json().send().await?.into_inner();
        let jwk = &jwks.keys[0];

        // Decode the access token to extract the provider token
        let jwt = jsonwebtoken::decode::<Claims>(
            &access_token,
            &DecodingKey::from_rsa_components(&jwk.n, &jwk.e)?,
            &Validation::new(Algorithm::RS256),
        )?;

        // An account linking request can only be generated by the owning account. Therefore we
        // need to use the sdk to generate a new client
        let client = ctx.new_client(Some(&access_token))?;

        // This needs to be the id of the provider the client just logged in with
        let link_token = client
            .create_link_token()
            .identifier(jwt.claims.prv)
            .body_map(|body| body.user_identifier(self_id))
            .send()
            .await?
            .into_inner()
            .token;

        ctx.client()?
            .link_provider()
            .identifier(self_id)
            .body_map(|body| body.token(link_token))
            .send()
            .await?;

        println!("Successfully linked provider");

        Ok(())
    }
}
