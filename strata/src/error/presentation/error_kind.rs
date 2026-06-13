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

impl std::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Validation => write!(f, "validation"),
            Self::Unauthenticated => write!(f, "unauthenticated"),
            Self::AccessDenied => write!(f, "access_denied"),
            Self::NotFound => write!(f, "not_found"),
            Self::Conflict => write!(f, "conflict"),
            Self::Technical => write!(f, "technical"),
            Self::Unexpected => write!(f, "unexpected"),
        }
    }
}
