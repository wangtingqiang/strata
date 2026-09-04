//! 错误信息模型：ErrorInfo trait 与 #[derive(ErrorInfo)] 派生宏。

#![warn(missing_docs)]

mod error_info;
mod error_kind;

pub use error_info::ErrorInfo;
pub use error_kind::ErrorKind;

/// 为错误枚举派生 [`ErrorInfo`] 实现。
///
/// 每个变体通过 `#[info(...)]` 声明 `kind`、`code`、`message` 三个属性：
///
/// ```
/// use strata_error::{ErrorInfo, ErrorKind};
///
/// #[derive(Debug, ErrorInfo)]
/// enum ApiError {
///     #[info(kind = "Validation", code = "E001", message = "invalid value: {field}")]
///     Invalid { field: String },
///
///     #[info(kind = "NotFound", code = "E002", message = "resource {0} not found")]
///     NotFound(String),
///
///     #[info(kind = "Internal", code = "E003", message = "internal error")]
///     Internal,
/// }
///
/// let err = ApiError::NotFound("user-1".into());
/// assert_eq!(err.kind(), ErrorKind::NotFound);
/// assert_eq!(err.code(), "E002");
/// assert_eq!(err.message(), "resource user-1 not found");
/// ```
///
/// `message` 支持位置占位符（`{0}`、`{1}`）与具名字段占位符（`{field}`），不支持格式说明符；
/// `{{` 与 `}}` 用于转义字面花括号。变体可声明为 `#[info(transparent)]`，将
/// `kind`/`code`/`message` 委托给其唯一字段。
pub use strata_error_macros::ErrorInfo;
