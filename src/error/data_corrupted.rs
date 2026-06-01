use thiserror::Error;

use crate::error::{BoxedError, error_source_chain_fmt};

#[derive(Debug, Error)]
pub struct DataCorruptedError {
    message: String,
    #[source]
    source: Option<BoxedError>,
}

impl DataCorruptedError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            source: None,
        }
    }

    pub fn with_source(
        message: impl Into<String>,
        source: impl std::error::Error + Send + Sync + 'static,
    ) -> Self {
        Self {
            message: message.into(),
            source: Some(Box::new(source)),
        }
    }

    pub fn message(&self) -> &str {
        &self.message
    }

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
