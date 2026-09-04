#[derive(Debug, strata_error::ErrorInfo)]
enum PositionalOnNamed {
    #[info(kind = "Validation", code = "E001", message = "error: {0}")]
    Variant { key: String },
}

fn main() {}
