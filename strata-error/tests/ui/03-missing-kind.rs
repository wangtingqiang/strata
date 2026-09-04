#[derive(Debug, strata_error::ErrorInfo)]
enum MissingKind {
    #[info(code = "E001", message = "error")]
    Variant,
}

fn main() {}
