use strata::error::ErrorInfo;

#[derive(Debug, ErrorInfo)]
enum MissingMessage {
    #[info(kind = "Validation", code = "E001")]
    Variant,
}

fn main() {}
