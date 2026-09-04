use crate::error::{DataCorruptedError, InvalidArgumentError, UnexpectedError};

#[derive(Debug, thiserror::Error)]
pub enum PortError {
    #[error(transparent)]
    InvalidArgument(#[from] InvalidArgumentError),

    #[error(transparent)]
    DataCorrupted(#[from] DataCorruptedError),

    #[error(transparent)]
    Unexpected(#[from] UnexpectedError),
}

impl PortError {
    pub fn invalid_argument(
        message: impl Into<String>,
        source: impl std::error::Error + Send + Sync + 'static,
    ) -> Self {
        InvalidArgumentError::new(message, source).into()
    }

    pub fn invalid_argument_from_message(message: impl Into<String>) -> Self {
        InvalidArgumentError::from_message(message).into()
    }

    pub fn data_corrupted(
        message: impl Into<String>,
        source: impl std::error::Error + Send + Sync + 'static,
    ) -> Self {
        DataCorruptedError::new(message, source).into()
    }

    pub fn data_corrupted_from_message(message: impl Into<String>) -> Self {
        DataCorruptedError::from_message(message).into()
    }

    pub fn unexpected(
        message: impl Into<String>,
        source: impl std::error::Error + Send + Sync + 'static,
    ) -> Self {
        UnexpectedError::new(message, source).into()
    }

    pub fn unexpected_from_message(message: impl Into<String>) -> Self {
        UnexpectedError::from_message(message).into()
    }
}
