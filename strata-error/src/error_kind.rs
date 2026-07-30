#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorKind {
    Unauthenticated,
    AccessDenied,
    Validation,
    Business,
    NotFound,
    Conflict,
    RateLimited,
    Internal,
    Technical,
    Unexpected,
}

impl ErrorKind {
    pub const fn is_severe(&self) -> bool {
        matches!(self, Self::Internal | Self::Technical | Self::Unexpected)
    }
}

impl std::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unauthenticated => write!(f, "unauthenticated"),
            Self::AccessDenied => write!(f, "access_denied"),
            Self::Validation => write!(f, "validation"),
            Self::Business => write!(f, "business"),
            Self::NotFound => write!(f, "not_found"),
            Self::Conflict => write!(f, "conflict"),
            Self::RateLimited => write!(f, "rate_limited"),
            Self::Internal => write!(f, "internal"),
            Self::Technical => write!(f, "technical"),
            Self::Unexpected => write!(f, "unexpected"),
        }
    }
}
