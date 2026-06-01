use thiserror::Error;

#[derive(Debug, Error)]
pub enum TelemetryInitError {
    #[error("invalid telemetry filter: {message}")]
    InvalidFilter { message: String },
    #[error("remote otlp http endpoint must not be empty")]
    EmptyRemoteEndpoint,
    #[error("remote timeout ms must be greater than 0")]
    InvalidRemoteTimeout,
    #[error("build otlp http trace exporter failed")]
    BuildTraceExporter {
        #[source]
        source: opentelemetry_otlp::ExporterBuildError,
    },
    #[error("initialize tracing subscriber failed: {message}")]
    InitSubscriber { message: String },
}
