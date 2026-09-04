/// 可观测性初始化错误。
#[derive(Debug, thiserror::Error)]
pub enum TelemetryInitError {
    /// 上报超时未配置或为 0。
    #[error("otlp_http_timeout_ms must be greater than 0")]
    InvalidRemoteTimeout,

    /// 上报端点为空白字符串。
    #[error("otlp_http_endpoint must not be empty")]
    InvalidRemoteEndpoint,

    /// 过滤器表达式非法。
    #[error("invalid telemetry filter: {0}")]
    InvalidFilter(#[source] tracing_subscriber::filter::ParseError),

    /// OTLP trace exporter 构建失败。
    #[error("build otlp http trace exporter failed: {0}")]
    BuildTraceExporter(#[source] opentelemetry_otlp::ExporterBuildError),

    /// OTLP metric exporter 构建失败。
    #[error("build otlp http metric exporter failed: {0}")]
    BuildMetricExporter(#[source] opentelemetry_otlp::ExporterBuildError),

    /// tracing subscriber 初始化失败。
    #[error("initialize tracing subscriber failed: {0}")]
    InitSubscriber(#[source] tracing_subscriber::util::TryInitError),
}
