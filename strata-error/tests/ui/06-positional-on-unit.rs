#[derive(Debug, strata_error::ErrorInfo)]
enum PositionalOnUnit {
    #[info(kind = "Validation", code = "E001", message = "error: {0}")]
    Variant,
}

fn main() {}
