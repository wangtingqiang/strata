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

impl ErrorKind {
    pub const fn is_severe(&self) -> bool {
        matches!(self, Self::Technical | Self::Unexpected)
    }
}
