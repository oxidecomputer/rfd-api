// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use dropshot::{endpoint, HttpError, HttpResponseOk, RequestContext};
use jsonwebtoken::jwk::{AlgorithmParameters, JwkSet, PublicKeyUse};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use trace_request::trace_request;
use tracing::instrument;

use crate::context::ApiContext;

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct OpenIdConfiguration {
    jwks_uri: String,
}

#[trace_request]
#[endpoint {
    method = GET,
    path = "/.well-known/openid-configuration",
}]
#[instrument(skip(rqctx), err(Debug))]
pub async fn openid_configuration(
    rqctx: RequestContext<ApiContext>,
) -> Result<HttpResponseOk<OpenIdConfiguration>, HttpError> {
    Ok(HttpResponseOk(OpenIdConfiguration {
        jwks_uri: format!("{}/.well-known/jwks.json", &rqctx.context().public_url),
    }))
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct Jwks {
    keys: Vec<Jwk>,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct Jwk {
    kty: String,
    kid: String,
    #[serde(rename = "use")]
    use_: String,
    n: String,
    e: String,
}

#[trace_request]
#[endpoint {
    method = GET,
    path = "/.well-known/jwks.json",
}]
#[instrument(skip(rqctx), err(Debug))]
pub async fn jwks_json(
    rqctx: RequestContext<ApiContext>,
) -> Result<HttpResponseOk<Jwks>, HttpError> {
    let jwks = rqctx.context().jwks().await;
    Ok(HttpResponseOk(jwks.into()))
}

impl From<&JwkSet> for Jwks {
    fn from(value: &JwkSet) -> Self {
        Self {
            keys: value
                .keys
                .iter()
                .map(|jwk| {
                    let (algo, n, e) = match &jwk.algorithm {
                        AlgorithmParameters::RSA(params) => {
                            ("RSA".to_string(), params.n.clone(), params.e.clone())
                        }
                        _ => panic!("Unexpected key type"),
                    };

                    Jwk {
                        kty: algo,
                        kid: jwk.common.key_id.as_ref().unwrap().clone(),
                        use_: match jwk.common.public_key_use {
                            Some(PublicKeyUse::Signature) => "sig".to_string(),
                            _ => panic!("Unexpected key use"),
                        },
                        n,
                        e,
                    }
                })
                .collect(),
        }
    }
}
