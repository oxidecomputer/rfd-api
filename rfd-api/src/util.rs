// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

pub mod response {
    use dropshot::HttpError;
    use http::StatusCode;
    use std::error::Error;
    use tracing::instrument;

    pub fn unauthorized() -> HttpError {
        client_error(StatusCode::UNAUTHORIZED, "Unauthorized")
    }

    pub fn client_error<S>(status_code: StatusCode, message: S) -> HttpError
    where
        S: ToString,
    {
        HttpError::for_client_error(None, status_code, message.to_string())
    }

    #[instrument(skip(error))]
    pub fn to_internal_error<E>(error: E) -> HttpError
    where
        E: Error,
    {
        internal_error(error.to_string())
    }

    #[instrument(skip(internal_message))]
    pub fn internal_error<S>(internal_message: S) -> HttpError
    where
        S: ToString,
    {
        let internal_message = internal_message.to_string();
        tracing::error!(internal_message, "Request failed");
        HttpError::for_internal_error(internal_message)
    }
}
