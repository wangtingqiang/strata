use thiserror::Error;

#[derive(Debug, Error)]
pub enum TelemetryInitError {
    #[error("otlp_http_timeout_ms must be greater than 0")]
    InvalidRemoteTimeout,

    #[error("otlp_http_endpoint must not be empty")]
    InvalidRemoteEndpoint,

    #[error("invalid telemetry filter: {0}")]
    InvalidFilter(#[source] tracing_subscriber::filter::ParseError),

    #[error("build otlp http trace exporter failed: {0}")]
    BuildTraceExporter(#[source] opentelemetry_otlp::ExporterBuildError),

    #[error("initialize tracing subscriber failed: {0}")]
    InitSubscriber(#[source] tracing_subscriber::util::TryInitError),
}
