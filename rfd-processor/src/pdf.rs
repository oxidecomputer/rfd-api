use async_trait::async_trait;
use google_drive::ClientError;
use rfd_data::RfdNumber;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RfdPdfError {
    #[error(transparent)]
    Remote(#[from] ClientError),
}

pub struct PdfFileLocation {
    pub url: Option<String>,
}

impl PdfFileLocation {
    pub fn url(&self) -> Option<&str> {
        self.url.as_ref().map(|s| &**s)
    }
}

#[async_trait]
pub trait PdfStorage {
    async fn store_rfd_pdf(
        &self,
        filename: &str,
        pdf: &RfdPdf,
    ) -> Vec<Result<PdfFileLocation, RfdPdfError>>;

    async fn remove_rfd_pdf(&self, filename: &str) -> Vec<RfdPdfError>;
}

#[derive(Debug)]
pub struct RfdPdf {
    pub number: RfdNumber,
    pub contents: Vec<u8>,
}
