use async_trait::async_trait;
use dropshot::{
    ApiEndpointBodyContentType, ExtensionMode, ExtractorMetadata, HttpError, RequestContext,
    ServerContext, SharedExtractor,
};

/// A token used for bearer authentication
pub struct BearerAuth(Option<String>);

impl BearerAuth {
    pub fn new(token: String) -> Self {
        Self(Some(token))
    }

    pub fn key(&self) -> Option<&str> {
        self.0.as_ref().map(|s| s.as_str())
    }

    pub fn consume(self) -> Option<String> {
        self.0
    }
}

/// Extracting a bearer token should never fail, it should always return `Ok(BearerAuth(Some(token))))`
/// or `Ok(BearerAuth(None))`. `None` will be returned in any of the cases that a valid string can not
/// be extracted. This extractor is not responsible for checking the value of the token.
#[async_trait]
impl SharedExtractor for BearerAuth {
    async fn from_request<Context: ServerContext>(
        rqctx: &RequestContext<Context>,
    ) -> Result<BearerAuth, HttpError> {
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
                Some(("Bearer", token)) => Some(token.to_string()),
                _ => None,
            }
        });

        Ok(BearerAuth(contents))
    }

    fn metadata(_body_content_type: ApiEndpointBodyContentType) -> ExtractorMetadata {
        ExtractorMetadata {
            parameters: vec![],
            extension_mode: ExtensionMode::None,
        }
    }
}
