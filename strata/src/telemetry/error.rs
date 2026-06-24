use thiserror::Error;

#[derive(Debug, Error)]
pub enum TelemetryInitError {
    #[error("invalid telemetry filter: {message}")]
    InvalidFilter { message: String },

    #[error("otlp_http_timeout_ms must be greater than 0")]
    InvalidRemoteTimeout,

    #[error("build otlp http trace exporter failed: {0}")]
    BuildTraceExporter(#[source] opentelemetry_otlp::ExporterBuildError),

    #[error("initialize tracing subscriber failed: {message}")]
    InitSubscriber { message: String },
}
