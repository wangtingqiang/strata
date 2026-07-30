use std::borrow::Cow;

use crate::error_kind::ErrorKind;

pub trait ErrorInfo {
    fn kind(&self) -> ErrorKind;
    fn code(&self) -> &'static str;
    fn message(&self) -> Cow<'static, str>;

    fn is_severe(&self) -> bool {
        self.kind().is_severe()
    }
}
