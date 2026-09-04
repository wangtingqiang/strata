use crate::error::{BoxedError, error_source_chain_fmt};

#[derive(Debug, thiserror::Error)]
pub struct UnexpectedError {
    message: String,
    #[source]
    source: Option<BoxedError>,
}

impl UnexpectedError {
    pub fn new(
        message: impl Into<String>,
        source: impl std::error::Error + Send + Sync + 'static,
    ) -> Self {
        Self {
            message: message.into(),
            source: Some(Box::new(source)),
        }
    }

    pub fn from_message(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            source: None,
        }
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn source_ref(&self) -> Option<&BoxedError> {
        self.source.as_ref()
    }
}

impl std::fmt::Display for UnexpectedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "unexpected({})", self.message)?;
        error_source_chain_fmt(&self, f)
    }
}
