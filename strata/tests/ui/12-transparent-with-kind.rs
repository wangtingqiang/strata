use strata::error::ErrorInfo;
use thiserror::Error;

#[derive(Debug, Error, ErrorInfo)]
enum TransparentWithKind {
    #[error("error: {0}")]
    #[info(transparent, kind = "Validation", code = "E001", message = "error")]
    Variant(String),
}

fn main() {}
