use strata::error::ErrorInfo;
use thiserror::Error;

#[derive(Debug, Error, ErrorInfo)]
enum OutOfRange {
    #[info(kind = "Validation", code = "E001", message = "error: {1}")]
    #[error("error: {0}")]
    Variant(String),
}

fn main() {}
