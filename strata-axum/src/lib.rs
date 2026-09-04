//! Axum 框架集成。

#![warn(missing_docs)]

mod header_map_ext;

/// 请求提取器。
pub mod extract;
/// API 响应层。
pub mod response;

pub use header_map_ext::HeaderMapExt;
