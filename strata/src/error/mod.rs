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
