/// 错误类别，标识错误的性质。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorKind {
    /// 未认证。
    Unauthenticated,
    /// 访问被拒绝。
    AccessDenied,
    /// 参数校验失败。
    Validation,
    /// 业务规则不满足。
    Business,
    /// 资源不存在。
    NotFound,
    /// 资源冲突。
    Conflict,
    /// 超出频率限制。
    RateLimited,
    /// 内部错误。
    Internal,
    /// 技术性错误。
    Technical,
    /// 意外错误。
    Unexpected,
}

impl ErrorKind {
    /// 是否为严重错误（Internal / Technical / Unexpected）。
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
