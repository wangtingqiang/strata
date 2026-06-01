use crate::error::PortError;

pub trait FromPortError: Sized + From<PortError> {
    fn data_corrupted(message: impl Into<String>) -> Self {
        PortError::data_corrupted(message).into()
    }

    fn data_corrupted_with_source(
        message: impl Into<String>,
        source: impl std::error::Error + Send + Sync + 'static,
    ) -> Self {
        PortError::data_corrupted_with_source(message, source).into()
    }

    fn unexpected(message: impl Into<String>) -> Self {
        PortError::unexpected(message).into()
    }

    fn unexpected_with_source(
        message: impl Into<String>,
        source: impl std::error::Error + Send + Sync + 'static,
    ) -> Self {
        PortError::unexpected_with_source(message, source).into()
    }
}

impl<T> FromPortError for T where T: Sized + From<PortError> {}
