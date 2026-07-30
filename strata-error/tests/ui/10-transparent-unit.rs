use strata_error::ErrorInfo;

#[derive(Debug, ErrorInfo)]
enum TransparentUnit {
    #[info(transparent)]
    Variant,
}

fn main() {}
