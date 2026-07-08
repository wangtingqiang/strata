use strata::error::ErrorInfo;

#[derive(ErrorInfo)]
struct NotAnEnum {
    _x: i32,
}

fn main() {}
