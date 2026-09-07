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

#[cfg(test)]
mod tests {
    use std::error::Error;

    use super::*;

    #[test]
    fn constructors_map_to_expected_variants() {
        assert!(matches!(
            PortError::invalid_argument("a", std::io::Error::other("boom")),
            PortError::InvalidArgument(_)
        ));
        assert!(matches!(
            PortError::invalid_argument_from_message("a"),
            PortError::InvalidArgument(_)
        ));
        assert!(matches!(
            PortError::data_corrupted("d", std::io::Error::other("boom")),
            PortError::DataCorrupted(_)
        ));
        assert!(matches!(
            PortError::data_corrupted_from_message("d"),
            PortError::DataCorrupted(_)
        ));
        assert!(matches!(
            PortError::unexpected("u", std::io::Error::other("boom")),
            PortError::Unexpected(_)
        ));
        assert!(matches!(
            PortError::unexpected_from_message("u"),
            PortError::Unexpected(_)
        ));
    }

    #[test]
    fn with_source_constructors_carry_source() {
        let error = PortError::invalid_argument("a", std::io::Error::other("boom"));

        match error {
            PortError::InvalidArgument(inner) => {
                assert_eq!(inner.message(), "a");
                assert!(inner.source().is_some());
            }
            _ => panic!("expected InvalidArgument variant"),
        }
    }

    #[test]
    fn from_message_constructors_carry_no_source() {
        let error = PortError::data_corrupted_from_message("d");

        match error {
            PortError::DataCorrupted(inner) => {
                assert_eq!(inner.message(), "d");
                assert!(inner.source().is_none());
            }
            _ => panic!("expected DataCorrupted variant"),
        }
    }

    #[test]
    fn from_conversions_preserve_variant() {
        assert!(matches!(
            PortError::from(InvalidArgumentError::from_message("a")),
            PortError::InvalidArgument(_)
        ));
        assert!(matches!(
            PortError::from(DataCorruptedError::from_message("d")),
            PortError::DataCorrupted(_)
        ));
        assert!(matches!(
            PortError::from(UnexpectedError::from_message("u")),
            PortError::Unexpected(_)
        ));
    }
}
