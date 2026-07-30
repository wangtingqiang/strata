use strata_error::ErrorInfo;

#[derive(Debug, ErrorInfo)]
enum PositionalOnUnit {
    #[info(kind = "Validation", code = "E001", message = "error: {0}")]
    Variant,
}

fn main() {}
