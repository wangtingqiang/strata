#[derive(Debug, strata_error::ErrorInfo)]
enum MissingCode {
    #[info(kind = "Validation", message = "error")]
    Variant,
}

fn main() {}
