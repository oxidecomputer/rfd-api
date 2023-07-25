use base64::{prelude::BASE64_STANDARD, DecodeError, Engine};
use std::path::Path;
use thiserror::Error;
use tokio::{fs, io::AsyncWriteExt};

#[derive(Debug, Error)]
pub enum FileIoError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("Expected file path to have a parent path")]
    MissingParent,
}

pub async fn write_file(file: &Path, contents: &[u8]) -> Result<(), FileIoError> {
    if let Some(parent) = file.parent() {
        // create each directory.
        fs::create_dir_all(parent).await?;

        // Write to the file.
        let mut f = fs::File::create(file).await?;
        f.write_all(contents).await?;

        tracing::info!(?file, "Wrote file");

        Ok(())
    } else {
        Err(FileIoError::MissingParent)
    }
}

pub fn decode_base64(c: &str) -> Result<Vec<u8>, DecodeError> {
    let v = c.replace('\n', "");
    let decoded = BASE64_STANDARD.decode(&v)?;
    Ok(decoded.trim().to_vec())
}

trait SliceExt {
    fn trim(&self) -> Self;
}

impl SliceExt for Vec<u8> {
    fn trim(&self) -> Vec<u8> {
        fn is_whitespace(c: &u8) -> bool {
            c == &b'\t' || c == &b' '
        }

        fn is_not_whitespace(c: &u8) -> bool {
            !is_whitespace(c)
        }

        if let Some(first) = self.iter().position(is_not_whitespace) {
            if let Some(last) = self.iter().rposition(is_not_whitespace) {
                self[first..last + 1].to_vec()
            } else {
                unreachable!();
            }
        } else {
            vec![]
        }
    }
}
