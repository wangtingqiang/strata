use crate::error::presentation::ErrorKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ErrorInfo {
    kind: ErrorKind,
    code: &'static str,
    message: &'static str,
}

impl ErrorInfo {
    pub const fn new(kind: ErrorKind, code: &'static str, message: &'static str) -> Self {
        Self {
            kind,
            code,
            message,
        }
    }

    pub const fn kind(&self) -> ErrorKind {
        self.kind
    }

    pub const fn code(&self) -> &'static str {
        self.code
    }

    pub const fn message(&self) -> &'static str {
        self.message
    }
}
