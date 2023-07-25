use async_trait::async_trait;
use base64::{prelude::BASE64_STANDARD, Engine};
use dropshot::{
    ApiEndpointBodyContentType, ExtensionMode, ExtractorMetadata, HttpError, RequestContext,
    ServerContext, SharedExtractor,
};

/// Credentials for basic authentication
pub struct BasicAuth {
    username: Option<String>,
    password: Option<String>,
}

impl BasicAuth {
    pub fn new(username: String, password: String) -> Self {
        Self {
            username: Some(username),
            password: Some(password),
        }
    }

    pub fn username(&self) -> Option<&String> {
        self.username.as_ref()
    }

    pub fn password(&self) -> Option<&String> {
        self.password.as_ref()
    }
}

/// Extracting a basic token should never fail, it should always return either `Ok(Some(BasicAuth))`
/// or `Ok(None)`. `None` will be returned in any of the cases that a valid string can not be extracted.
/// This extractor is not responsible for checking the value of the token.
#[async_trait]
impl SharedExtractor for BasicAuth {
    async fn from_request<Context: ServerContext>(
        rqctx: &RequestContext<Context>,
    ) -> Result<BasicAuth, HttpError> {
        // Similarly we only care about the presence of the Authorization header
        let header_value = rqctx
            .request
            .headers()
            .get("Authorization")
            .and_then(|header| {
                // If the value provided is not a readable string we will throw it out
                header
                    .to_str()
                    .map(|s| s.to_string())
                    .map_err(|err| {
                        tracing::info!("Failed to turn Authorization header into string");
                        err
                    })
                    .ok()
            });

        // Finally ensure that the value we found is properly formed
        let contents = header_value.and_then(|value| {
            let parts = value.split_once(' ');

            match parts {
                Some(("Basic", token)) => BASE64_STANDARD
                    .decode(token)
                    .map_err(|err| {
                        tracing::info!("Failed to decode basic auth header");
                        err
                    })
                    .ok()
                    .and_then(|decoded| {
                        String::from_utf8(decoded)
                            .map_err(|err| {
                                tracing::info!(
                                    "Failed to interpret decoded bytes from authorization header"
                                );
                                err
                            })
                            .ok()
                    })
                    .and_then(|parsed| match parsed.split_once(':') {
                        Some((username, password)) => {
                            Some((username.to_string(), password.to_string()))
                        }
                        _ => None,
                    }),
                _ => None,
            }
        });

        if let Some((username, password)) = contents {
            Ok(BasicAuth {
                username: Some(username),
                password: Some(password),
            })
        } else {
            Ok(BasicAuth {
                username: None,
                password: None,
            })
        }
    }

    fn metadata(_body_content_type: ApiEndpointBodyContentType) -> ExtractorMetadata {
        ExtractorMetadata {
            parameters: vec![],
            extension_mode: ExtensionMode::None,
        }
    }
}
