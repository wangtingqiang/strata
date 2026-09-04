#[derive(Debug, strata_error::ErrorInfo)]
enum TransparentUnit {
    #[info(transparent)]
    Variant,
}

fn main() {}
