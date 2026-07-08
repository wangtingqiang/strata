use strata::error::ErrorInfo;
use thiserror::Error;

#[derive(Debug, Error, ErrorInfo)]
enum MissingKind {
    #[info(code = "E001", message = "error")]
    #[error("error")]
    Variant,
}

fn main() {}
