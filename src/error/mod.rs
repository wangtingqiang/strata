mod data_corrupted;
mod port;
mod presentation;
mod unexpected;

pub use data_corrupted::DataCorruptedError;
pub use port::{FromPortError, PortError};
pub use presentation::{ErrorInfo, ToErrorInfo};
pub use unexpected::UnexpectedError;

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
