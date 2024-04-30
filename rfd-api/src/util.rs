// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use diesel::result::{DatabaseErrorKind, Error as DieselError};
use google_cloudkms1::{
    hyper_rustls::{self, HttpsConnector},
    CloudKMS,
};
use hyper::client::HttpConnector;
use rfd_model::storage::StoreError;

use crate::authn::CloudKmsError;

pub mod request {
    use cookie::Cookie;
    use dropshot::RequestInfo;
    use http::header::COOKIE;

    pub trait RequestCookies {
        fn cookie(&self, name: &str) -> Option<Cookie>;
    }

    impl RequestCookies for RequestInfo {
        fn cookie(&self, name: &str) -> Option<Cookie> {
            let cookie_header = self.headers().get(COOKIE)?;

            Cookie::split_parse(String::from_utf8(cookie_header.as_bytes().to_vec()).unwrap())
                .filter_map(|cookie| match cookie {
                    Ok(cookie) => {
                        if cookie.name() == name {
                            Some(cookie)
                        } else {
                            None
                        }
                    }
                    _ => None,
                })
                .nth(0)
        }
    }
}

pub mod response {
    use dropshot::HttpError;
    use http::StatusCode;
    use std::error::Error;
    use thiserror::Error;
    use tracing::instrument;

    pub fn conflict() -> HttpError {
        client_error(StatusCode::CONFLICT, "Already exists")
    }

    pub fn unauthorized() -> HttpError {
        client_error(StatusCode::UNAUTHORIZED, "Unauthorized")
    }

    pub fn forbidden() -> HttpError {
        client_error(StatusCode::FORBIDDEN, "Forbidden")
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

    pub type ResourceResult<T, E> = Result<T, ResourceError<E>>;

    #[derive(Debug, Error)]
    pub enum ResourceError<E> {
        #[error("Resource does not exist")]
        DoesNotExist,
        #[error("Caller does not have required access")]
        Restricted,
        #[error("Internal server error")]
        InternalError(#[source] E),
    }

    impl<E> ResourceError<E> {
        pub fn inner_into<F>(self) -> ResourceError<F>
        where
            F: From<E>,
        {
            match self {
                ResourceError::DoesNotExist => ResourceError::DoesNotExist,
                ResourceError::Restricted => ResourceError::Restricted,
                ResourceError::InternalError(inner) => ResourceError::InternalError(inner.into()),
            }
        }
    }

    pub trait ToResourceResultOpt<T, E> {
        fn opt_to_resource_result(self) -> ResourceResult<T, E>;
    }

    impl<T, E> ToResourceResultOpt<T, E> for Result<Option<T>, E> {
        fn opt_to_resource_result(self) -> ResourceResult<T, E> {
            match self {
                Ok(Some(value)) => ResourceResult::Ok(value),
                Ok(None) => resource_not_found(),
                Err(err) => resource_error(err),
            }
        }
    }

    pub trait ToResourceResult<T, E> {
        fn to_resource_result(self) -> ResourceResult<T, E>;
    }

    impl<T, E> ToResourceResult<T, E> for Result<T, E> {
        fn to_resource_result(self) -> ResourceResult<T, E> {
            match self {
                Ok(value) => ResourceResult::Ok(value),
                Err(err) => resource_error(err),
            }
        }
    }

    pub fn resource_not_found<T, E>() -> ResourceResult<T, E> {
        ResourceResult::Err(ResourceError::DoesNotExist)
    }

    pub fn resource_restricted<T, E>() -> ResourceResult<T, E> {
        ResourceResult::Err(ResourceError::Restricted)
    }

    pub fn resource_error<T, E>(err: E) -> ResourceResult<T, E> {
        ResourceResult::Err(ResourceError::InternalError(err))
    }

    impl<E> From<ResourceError<E>> for HttpError
    where
        E: Error,
    {
        fn from(value: ResourceError<E>) -> Self {
            match value {
                ResourceError::DoesNotExist => not_found(""),
                ResourceError::InternalError(err) => to_internal_error(err),
                ResourceError::Restricted => forbidden(),
            }
        }
    }
}

pub fn is_uniqueness_error(err: &StoreError) -> bool {
    match err {
        StoreError::Db(DieselError::DatabaseError(DatabaseErrorKind::UniqueViolation, _)) => true,
        _ => false,
    }
}

pub async fn cloud_kms_client() -> Result<CloudKMS<HttpsConnector<HttpConnector>>, CloudKmsError> {
    let opts = yup_oauth2::ApplicationDefaultCredentialsFlowOpts::default();

    tracing::trace!(?opts, "Request GCP credentials");

    let gcp_credentials =
        yup_oauth2::ApplicationDefaultCredentialsAuthenticator::builder(opts).await;

    tracing::trace!("Retrieved GCP credentials");

    let gcp_auth = match gcp_credentials {
        yup_oauth2::authenticator::ApplicationDefaultCredentialsTypes::ServiceAccount(auth) => {
            tracing::debug!("Create GCP service account based credentials");

            auth.build().await.map_err(|err| {
                tracing::error!(
                    ?err,
                    "Failed to construct Cloud KMS credentials from service account"
                );
                CloudKmsError::RemoteKeyAuthMissing(err)
            })?
        }
        yup_oauth2::authenticator::ApplicationDefaultCredentialsTypes::InstanceMetadata(auth) => {
            tracing::debug!("Create GCP instance based credentials");

            auth.build().await.map_err(|err| {
                tracing::error!(
                    ?err,
                    "Failed to construct Cloud KMS credentials from instance metadata"
                );
                CloudKmsError::RemoteKeyAuthMissing(err)
            })?
        }
    };

    let gcp_kms = CloudKMS::new(
        hyper::Client::builder().build(
            hyper_rustls::HttpsConnectorBuilder::new()
                .with_native_roots()
                .https_only()
                .enable_http2()
                .build(),
        ),
        gcp_auth,
    );

    Ok(gcp_kms)
}

#[cfg(test)]
pub mod tests {
    use dropshot::{HttpCodedResponse, HttpError};
    use http::StatusCode;
    use rand_core::RngCore;
    use rsa::{
        pkcs8::{EncodePrivateKey, EncodePublicKey, LineEnding},
        RsaPrivateKey, RsaPublicKey,
    };

    use crate::config::AsymmetricKey;

    pub fn get_status<T>(res: &Result<T, HttpError>) -> StatusCode
    where
        T: HttpCodedResponse,
    {
        match res {
            Ok(_) => T::STATUS_CODE,
            Err(err) => err.status_code,
        }
    }

    pub fn mock_key() -> AsymmetricKey {
        let mut rng = rand::thread_rng();
        let bits = 2048;
        let priv_key = RsaPrivateKey::new(&mut rng, bits).expect("Failed to generate a key");
        let pub_key = RsaPublicKey::from(&priv_key);

        let mut kid = [0; 24];
        rng.fill_bytes(&mut kid);

        AsymmetricKey::Local {
            kid: hex::encode(kid),
            private: String::from_utf8(
                priv_key
                    .to_pkcs8_pem(LineEnding::LF)
                    .unwrap()
                    .as_bytes()
                    .to_vec(),
            )
            .unwrap(),
            public: pub_key.to_public_key_pem(LineEnding::LF).unwrap(),
        }
    }
}
