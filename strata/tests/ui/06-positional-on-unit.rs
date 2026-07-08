use strata::error::ErrorInfo;
use thiserror::Error;

#[derive(Debug, Error, ErrorInfo)]
enum PositionalOnUnit {
    #[info(kind = "Validation", code = "E001", message = "error: {0}")]
    #[error("error")]
    Variant,
}

fn main() {}
