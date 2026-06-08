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
    pub fn data_corrupted(message: impl Into<String>) -> Self {
        DataCorruptedError::new(message).into()
    }

    pub fn data_corrupted_with_source(
        message: impl Into<String>,
        source: impl std::error::Error + Send + Sync + 'static,
    ) -> Self {
        DataCorruptedError::with_source(message, source).into()
    }

    pub fn unexpected(message: impl Into<String>) -> Self {
        UnexpectedError::new(message).into()
    }

    pub fn unexpected_with_source(
        message: impl Into<String>,
        source: impl std::error::Error + Send + Sync + 'static,
    ) -> Self {
        UnexpectedError::with_source(message, source).into()
    }
}
