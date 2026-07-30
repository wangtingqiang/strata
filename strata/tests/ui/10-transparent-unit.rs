use strata::error::ErrorInfo;
use thiserror::Error;

#[derive(Debug, Error, ErrorInfo)]
enum TransparentUnit {
    #[error("error")]
    #[info(transparent)]
    Variant,
}

fn main() {}
