pub enum Feature {
    CommitToGitHub,
    WritePdfToDrive,
}

impl Feature {
    pub fn enabled(&self) -> bool {
        match self {
            Self::CommitToGitHub => evar_enabled("COMMIT_GITHUB"),
            Self::WritePdfToDrive => evar_enabled("OUTPUT_PDFS"),
        }
    }
}

fn evar_enabled(var: &str) -> bool {
    std::env::var(var)
        .map(|value| value == "true")
        .unwrap_or(false)
}
