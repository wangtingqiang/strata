use crate::error::{BoxedError, error_source_chain_fmt};

/// 数据损坏错误。
#[derive(Debug, thiserror::Error)]
pub struct DataCorruptedError {
    message: String,
    #[source]
    source: Option<BoxedError>,
}

impl DataCorruptedError {
    /// 构建带消息与根源错误的数据损坏错误。
    pub fn new(
        message: impl Into<String>,
        source: impl std::error::Error + Send + Sync + 'static,
    ) -> Self {
        Self {
            message: message.into(),
            source: Some(Box::new(source)),
        }
    }

    /// 构建仅有消息的数据损坏错误。
    pub fn from_message(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            source: None,
        }
    }

    /// 错误消息。
    pub fn message(&self) -> &str {
        &self.message
    }

    /// 根源错误。
    pub fn source_ref(&self) -> Option<&BoxedError> {
        self.source.as_ref()
    }
}

impl std::fmt::Display for DataCorruptedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "data_corrupted({})", self.message)?;
        error_source_chain_fmt(&self, f)
    }
}
