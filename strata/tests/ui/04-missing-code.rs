use strata::error::ErrorInfo;

#[derive(Debug, ErrorInfo)]
enum MissingCode {
    #[info(kind = "Validation", message = "error")]
    Variant,
}

fn main() {}
