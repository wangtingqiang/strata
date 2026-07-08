use strata::error::ErrorInfo;
use thiserror::Error;

#[derive(Debug, Error, ErrorInfo)]
enum MissingCode {
    #[info(kind = "Validation", message = "error")]
    #[error("error")]
    Variant,
}

fn main() {}
