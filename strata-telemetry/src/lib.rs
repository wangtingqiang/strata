//! 可观测性：基于 tracing 与 OpenTelemetry OTLP 的日志、分布式追踪与指标初始化。

#![warn(missing_docs)]

mod config;
mod error;
mod guard;
mod init;

pub use config::{TelemetryConfig, TelemetryLocalConfig, TelemetryRemoteConfig};
pub use error::TelemetryInitError;
pub use guard::TelemetryGuard;

/// HTTP 传输相关工具。
#[cfg(feature = "http")]
pub mod http;

/// Axum 框架集成。
#[cfg(feature = "axum")]
pub mod axum;
