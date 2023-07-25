pub mod response {
    use dropshot::HttpError;
    use http::StatusCode;

    pub fn unauthorized() -> HttpError {
        client_error(StatusCode::UNAUTHORIZED, "Unauthorized")
    }

    pub fn client_error<S>(status_code: StatusCode, message: S) -> HttpError
    where
        S: ToString,
    {
        HttpError::for_client_error(None, status_code, message.to_string())
    }

    pub fn bad_request<S>(message: S) -> HttpError
    where
        S: ToString,
    {
        HttpError::for_bad_request(None, message.to_string())
    }

    pub fn not_found(internal_message: &str) -> HttpError {
        HttpError::for_not_found(None, internal_message.to_string())
    }

    pub fn internal_error<S>(internal_message: S) -> HttpError
    where
        S: ToString,
    {
        HttpError::for_internal_error(internal_message.to_string())
    }
}

#[cfg(test)]
pub mod tests {
    use dropshot::{HttpCodedResponse, HttpError};
    use http::StatusCode;

    use crate::email_validator::EmailValidator;

    pub fn get_status<T>(res: &Result<T, HttpError>) -> StatusCode
    where
        T: HttpCodedResponse,
    {
        match res {
            Ok(_) => T::STATUS_CODE,
            Err(err) => err.status_code,
        }
    }

    pub struct AnyEmailValidator;
    impl EmailValidator for AnyEmailValidator {
        fn validate(&self, _email: &str) -> bool {
            true
        }
    }
}
