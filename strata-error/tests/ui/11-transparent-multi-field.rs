#[derive(Debug, strata_error::ErrorInfo)]
enum TransparentMultiField {
    #[info(transparent)]
    Variant(String, String),
}

fn main() {}
