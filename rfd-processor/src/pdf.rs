// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use async_trait::async_trait;
use google_drive3::Error as ClientError;
use rfd_data::RfdNumber;
use rfd_model::schema_ext::PdfSource;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RfdPdfError {
    #[error("Upload failed to return a valid file id for {0}")]
    FileIdMissing(String),
    #[error(transparent)]
    Remote(#[from] ClientError),
}

#[derive(Debug)]
pub struct PdfFileLocation {
    pub source: PdfSource,
    pub url: String,
    pub external_id: String,
}

#[async_trait]
pub trait PdfStorage {
    async fn store_rfd_pdf(
        &self,
        external_id: Option<&str>,
        filename: &str,
        pdf: &RfdPdf,
    ) -> Vec<Result<PdfFileLocation, RfdPdfError>>;
}

#[derive(Debug)]
pub struct RfdPdf {
    pub number: RfdNumber,
    pub contents: Vec<u8>,
}
