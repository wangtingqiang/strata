#[derive(Debug, strata_error::ErrorInfo)]
enum MissingMessage {
    #[info(kind = "Validation", code = "E001")]
    Variant,
}

fn main() {}
