#[derive(Debug, strata_error::ErrorInfo)]
enum TransparentWithKind {
    #[info(transparent, kind = "Validation", code = "E001", message = "error")]
    Variant(String),
}

fn main() {}
