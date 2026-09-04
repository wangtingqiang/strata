use crate::error::PortError;

/// 端口错误便捷构造 trait：实现 `From<PortError>` 的类型可直接构造各类端口错误。
pub trait FromPortError: Sized + From<PortError> {
    /// 构造参数不合法端口错误。
    fn invalid_argument(
        message: impl Into<String>,
        source: impl std::error::Error + Send + Sync + 'static,
    ) -> Self {
        PortError::invalid_argument(message, source).into()
    }

    /// 构造仅有消息的参数不合法端口错误。
    fn invalid_argument_from_message(message: impl Into<String>) -> Self {
        PortError::invalid_argument_from_message(message).into()
    }

    /// 构造数据损坏端口错误。
    fn data_corrupted(
        message: impl Into<String>,
        source: impl std::error::Error + Send + Sync + 'static,
    ) -> Self {
        PortError::data_corrupted(message, source).into()
    }

    /// 构造仅有消息的数据损坏端口错误。
    fn data_corrupted_from_message(message: impl Into<String>) -> Self {
        PortError::data_corrupted_from_message(message).into()
    }

    /// 构造意外端口错误。
    fn unexpected(
        message: impl Into<String>,
        source: impl std::error::Error + Send + Sync + 'static,
    ) -> Self {
        PortError::unexpected(message, source).into()
    }

    /// 构造仅有消息的意外端口错误。
    fn unexpected_from_message(message: impl Into<String>) -> Self {
        PortError::unexpected_from_message(message).into()
    }
}

impl<T> FromPortError for T where T: Sized + From<PortError> {}
