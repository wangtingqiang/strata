use crate::error::ErrorInfo;

/// 将语义错误映射为稳定错误信息。
pub trait ToErrorInfo {
    fn to_error_info(&self) -> ErrorInfo;
}

impl ToErrorInfo for ErrorInfo {
    fn to_error_info(&self) -> ErrorInfo {
        *self
    }
}

impl<T> ToErrorInfo for &T
where
    T: ToErrorInfo + ?Sized,
{
    fn to_error_info(&self) -> ErrorInfo {
        (*self).to_error_info()
    }
}
