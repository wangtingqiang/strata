use std::borrow::Cow;

use crate::error_kind::ErrorKind;

/// 错误信息描述。
pub trait ErrorInfo {
    /// 错误类别。
    fn kind(&self) -> ErrorKind;

    /// 错误码。
    fn code(&self) -> &'static str;

    /// 错误消息。
    fn message(&self) -> Cow<'static, str>;

    /// 是否为严重错误。
    fn is_severe(&self) -> bool {
        self.kind().is_severe()
    }
}
