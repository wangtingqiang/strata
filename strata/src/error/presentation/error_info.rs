/// 对外错误信息，约束为稳定的 `code + message`。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ErrorInfo {
    code: &'static str,
    message: &'static str,
}

impl ErrorInfo {
    pub const fn new(code: &'static str, message: &'static str) -> Self {
        Self { code, message }
    }

    pub const fn code(&self) -> &'static str {
        self.code
    }

    pub const fn message(&self) -> &'static str {
        self.message
    }
}
