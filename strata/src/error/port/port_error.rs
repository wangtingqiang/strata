use crate::error::{DataCorruptedError, InvalidArgumentError, UnexpectedError};

/// 端口错误：参数不合法 / 数据损坏 / 意外。
#[derive(Debug, thiserror::Error)]
pub enum PortError {
    /// 参数不合法。
    #[error(transparent)]
    InvalidArgument(#[from] InvalidArgumentError),

    /// 数据损坏。
    #[error(transparent)]
    DataCorrupted(#[from] DataCorruptedError),

    /// 意外错误。
    #[error(transparent)]
    Unexpected(#[from] UnexpectedError),
}

impl PortError {
    /// 构造参数不合法端口错误。
    pub fn invalid_argument(
        message: impl Into<String>,
        source: impl std::error::Error + Send + Sync + 'static,
    ) -> Self {
        InvalidArgumentError::new(message, source).into()
    }

    /// 构造仅有消息的参数不合法端口错误。
    pub fn invalid_argument_from_message(message: impl Into<String>) -> Self {
        InvalidArgumentError::from_message(message).into()
    }

    /// 构造数据损坏端口错误。
    pub fn data_corrupted(
        message: impl Into<String>,
        source: impl std::error::Error + Send + Sync + 'static,
    ) -> Self {
        DataCorruptedError::new(message, source).into()
    }

    /// 构造仅有消息的数据损坏端口错误。
    pub fn data_corrupted_from_message(message: impl Into<String>) -> Self {
        DataCorruptedError::from_message(message).into()
    }

    /// 构造意外端口错误。
    pub fn unexpected(
        message: impl Into<String>,
        source: impl std::error::Error + Send + Sync + 'static,
    ) -> Self {
        UnexpectedError::new(message, source).into()
    }

    /// 构造仅有消息的意外端口错误。
    pub fn unexpected_from_message(message: impl Into<String>) -> Self {
        UnexpectedError::from_message(message).into()
    }
}
