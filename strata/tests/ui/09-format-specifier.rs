use strata::error::ErrorInfo;
use thiserror::Error;

#[derive(Debug, Error, ErrorInfo)]
enum FormatSpecifier {
    #[info(kind = "Validation", code = "E001", message = "error: {key:?}")]
    #[error("error: {key}")]
    Variant { key: String },
}

fn main() {}
