use crate::error::PortError;

pub trait FromPortError: Sized + From<PortError> {
    fn invalid_argument(
        message: impl Into<String>,
        source: impl std::error::Error + Send + Sync + 'static,
    ) -> Self {
        PortError::invalid_argument(message, source).into()
    }

    fn invalid_argument_from_message(message: impl Into<String>) -> Self {
        PortError::invalid_argument_from_message(message).into()
    }

    fn data_corrupted(
        message: impl Into<String>,
        source: impl std::error::Error + Send + Sync + 'static,
    ) -> Self {
        PortError::data_corrupted(message, source).into()
    }

    fn data_corrupted_from_message(message: impl Into<String>) -> Self {
        PortError::data_corrupted_from_message(message).into()
    }

    fn unexpected(
        message: impl Into<String>,
        source: impl std::error::Error + Send + Sync + 'static,
    ) -> Self {
        PortError::unexpected(message, source).into()
    }

    fn unexpected_from_message(message: impl Into<String>) -> Self {
        PortError::unexpected_from_message(message).into()
    }
}

impl<T> FromPortError for T where T: Sized + From<PortError> {}
