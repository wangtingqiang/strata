//! 通用工具集：错误模型、配置加载、密码与令牌、文本与时间工具等。

#![warn(missing_docs)]

/// 配置加载。
#[cfg(feature = "config")]
pub mod config;
/// 错误模型与端口错误。
#[cfg(feature = "error")]
pub mod error;
/// 哈希工具。
#[cfg(feature = "hash")]
pub mod hash;
/// 不透明令牌。
#[cfg(feature = "opaque-token")]
pub mod opaque_token;
/// 分页参数与响应。
#[cfg(feature = "pagination")]
pub mod pagination;
/// 密码哈希。
#[cfg(feature = "password")]
pub mod password;
/// 反序列化辅助。
#[cfg(feature = "serde")]
pub mod serde;
/// 异步任务工具。
#[cfg(feature = "task")]
pub mod task;
/// 文本处理。
#[cfg(feature = "text")]
pub mod text;
/// 时间解析与格式化。
#[cfg(feature = "time")]
pub mod time;
/// 输入校验。
#[cfg(feature = "validation")]
pub mod validation;
