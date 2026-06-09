use thiserror::Error;

use crate::error::{DataCorruptedError, UnexpectedError};

#[derive(Debug, Error)]
pub enum PortError {
    #[error(transparent)]
    DataCorrupted(#[from] DataCorruptedError),

    #[error(transparent)]
    Unexpected(#[from] UnexpectedError),
}

impl PortError {
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
