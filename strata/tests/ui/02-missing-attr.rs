use strata::error::ErrorInfo;
use thiserror::Error;

#[derive(Debug, Error, ErrorInfo)]
enum MissingAttr {
    NoAttr,
}

fn main() {}
