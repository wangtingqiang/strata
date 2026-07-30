use strata::error::ErrorInfo;

#[derive(Debug, ErrorInfo)]
enum MissingAttr {
    NoAttr,
}

fn main() {}
