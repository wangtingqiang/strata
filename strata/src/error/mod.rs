mod data_corrupted;
mod invalid_argument;
mod port;
mod unexpected;

pub use data_corrupted::DataCorruptedError;
pub use invalid_argument::InvalidArgumentError;
pub use port::{FromPortError, PortError};
pub use unexpected::UnexpectedError;

/// 盒装错误（`Send + Sync`）。
pub type BoxedError = Box<dyn std::error::Error + Send + Sync + 'static>;

/// 将错误链逐级写入格式化器。
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
