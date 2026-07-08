use strata::error::ErrorInfo;
use thiserror::Error;

#[derive(Debug, Error, ErrorInfo)]
enum PositionalOnNamed {
    #[info(kind = "Validation", code = "E001", message = "error: {0}")]
    #[error("error: {key}")]
    Variant { key: String },
}

fn main() {}
