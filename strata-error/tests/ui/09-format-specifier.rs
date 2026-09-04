#[derive(Debug, strata_error::ErrorInfo)]
enum FormatSpecifier {
    #[info(kind = "Validation", code = "E001", message = "error: {key:?}")]
    Variant { key: String },
}

fn main() {}
