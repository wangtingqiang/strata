//! 可观测性：基于 tracing 与 OpenTelemetry OTLP 的日志、分布式追踪与指标初始化。

#![warn(missing_docs)]

mod config;
mod error;
mod guard;
mod init;

pub use config::{TelemetryConfig, TelemetryLocalConfig, TelemetryRemoteConfig};
pub use error::TelemetryInitError;
pub use guard::TelemetryGuard;

#[cfg(feature = "http")]
/// Axum 中间件：请求追踪、服务指标与 trace 上下文注入。
pub mod http;
