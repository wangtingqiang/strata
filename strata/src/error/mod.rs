mod data_corrupted;
mod invalid_argument;
mod port;
mod presentation;
mod unexpected;

pub use data_corrupted::DataCorruptedError;
pub use invalid_argument::InvalidArgumentError;
pub use port::{FromPortError, PortError};
pub use presentation::{ErrorInfo, ErrorKind};
pub use unexpected::UnexpectedError;

#[cfg(feature = "macros")]
pub use strata_macros::ErrorInfo;

pub type BoxedError = Box<dyn std::error::Error + Send + Sync + 'static>;

pub fn error_source_chain_fmt(
    e: &impl std::error::Error,
    f: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result {
    let mut source = e.source();

    while let Some(cause) = source {
        write!(f, ": {cause}")?;
        source = cause.source();
    }

    Ok(())
}
