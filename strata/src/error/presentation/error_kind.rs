#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorKind {
    Validation,
    Unauthenticated,
    AccessDenied,
    NotFound,
    Conflict,
    Technical,
    Unexpected,
}
