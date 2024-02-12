// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use anyhow::{anyhow, Error};
use rfd_sdk::{types::Error as ApiError, ProgenitorClientError};

use crate::{Context, VerbosityLevel};

pub fn format_api_err(ctx: &Context, client_err: ProgenitorClientError<ApiError>) -> Error {
    let mut err = anyhow!("API Request failed");

    match client_err {
        ProgenitorClientError::CommunicationError(inner) => {
            if ctx.verbosity >= VerbosityLevel::All {
                err = err.context("Communication Error").context(inner);
            }
        }
        ProgenitorClientError::ErrorResponse(response) => {
            if ctx.verbosity >= VerbosityLevel::All {
                err = err.context(format!("Status: {}", response.status()));
                err = err.context(format!("Headers {:?}", response.headers()));
            }

            let response_message = response.into_inner();

            if ctx.verbosity >= VerbosityLevel::All {
                err = err.context(format!("Request {}", response_message.request_id));
            }

            err = err.context(format!(
                "Code: {}",
                response_message
                    .error_code
                    .as_ref()
                    .map(|s| &**s)
                    .unwrap_or("")
            ));
            err = err.context(response_message.message);
        }
        ProgenitorClientError::InvalidRequest(message) => {
            err = err.context("Invalid request").context(message);
        }
        ProgenitorClientError::InvalidResponsePayload(_, inner) => {
            err = err.context("Invalid response").context(inner);
        }
        ProgenitorClientError::UnexpectedResponse(response) => {
            err = err
                .context("Unexpected response")
                .context(format!("Status: {}", response.status()));

            if ctx.verbosity >= VerbosityLevel::All {
                err = err.context(format!("Headers {:?}", response.headers()));
            }
        }
        ProgenitorClientError::ResponseBodyError(inner) => {
            err = err.context("Invalid response").context(inner);
        }
        ProgenitorClientError::InvalidUpgrade(inner) => {
            err = err.context("Invalid upgrade").context(inner)
        }
    }

    err
}
