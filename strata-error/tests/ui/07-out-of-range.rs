use strata_error::ErrorInfo;

#[derive(Debug, ErrorInfo)]
enum OutOfRange {
    #[info(kind = "Validation", code = "E001", message = "error: {1}")]
    Variant(String),
}

fn main() {}
