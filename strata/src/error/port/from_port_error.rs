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

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    enum TestError {
        InvalidArgument(String),
        DataCorrupted(String),
        Unexpected(String),
    }

    impl From<PortError> for TestError {
        fn from(value: PortError) -> Self {
            match value {
                PortError::InvalidArgument(inner) => {
                    Self::InvalidArgument(inner.message().to_owned())
                }
                PortError::DataCorrupted(inner) => Self::DataCorrupted(inner.message().to_owned()),
                PortError::Unexpected(inner) => Self::Unexpected(inner.message().to_owned()),
            }
        }
    }

    #[test]
    fn trait_methods_delegate_to_correct_variants() {
        let source = || std::io::Error::other("boom");

        assert!(matches!(
            TestError::invalid_argument("a", source()),
            TestError::InvalidArgument(message) if message == "a"
        ));
        assert!(matches!(
            TestError::invalid_argument_from_message("a"),
            TestError::InvalidArgument(message) if message == "a"
        ));
        assert!(matches!(
            TestError::data_corrupted("d", source()),
            TestError::DataCorrupted(message) if message == "d"
        ));
        assert!(matches!(
            TestError::data_corrupted_from_message("d"),
            TestError::DataCorrupted(message) if message == "d"
        ));
        assert!(matches!(
            TestError::unexpected("u", source()),
            TestError::Unexpected(message) if message == "u"
        ));
        assert!(matches!(
            TestError::unexpected_from_message("u"),
            TestError::Unexpected(message) if message == "u"
        ));
    }
}
