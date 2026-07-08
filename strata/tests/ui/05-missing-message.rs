use strata::error::ErrorInfo;
use thiserror::Error;

#[derive(Debug, Error, ErrorInfo)]
enum MissingMessage {
    #[info(kind = "Validation", code = "E001")]
    #[error("error")]
    Variant,
}

fn main() {}
