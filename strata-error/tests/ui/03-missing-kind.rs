use strata_error::ErrorInfo;

#[derive(Debug, ErrorInfo)]
enum MissingKind {
    #[info(code = "E001", message = "error")]
    Variant,
}

fn main() {}
