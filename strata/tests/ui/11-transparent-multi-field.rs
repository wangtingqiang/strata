use strata::error::ErrorInfo;
use thiserror::Error;

#[derive(Debug, Error, ErrorInfo)]
enum TransparentMultiField {
    #[error("error: {0} {1}")]
    #[info(transparent)]
    Variant(String, String),
}

fn main() {}
