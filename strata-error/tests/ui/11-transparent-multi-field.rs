use strata_error::ErrorInfo;

#[derive(Debug, ErrorInfo)]
enum TransparentMultiField {
    #[info(transparent)]
    Variant(String, String),
}

fn main() {}
