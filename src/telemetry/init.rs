use std::time::Duration;

use opentelemetry::{KeyValue, global, trace::TracerProvider as _};
use opentelemetry_otlp::{Protocol, SpanExporter, WithExportConfig};
use opentelemetry_sdk::{Resource, propagation::TraceContextPropagator, trace::SdkTracerProvider};
use tracing_subscriber::{
    EnvFilter, fmt::time::LocalTime, layer::SubscriberExt as _, util::SubscriberInitExt as _,
};

use crate::telemetry::{TelemetryConfig, TelemetryInitError};

#[derive(Debug, Clone, Copy)]
pub struct ServiceInfo {
    pub name: &'static str,
    pub version: &'static str,
}

#[derive(Debug)]
pub struct TelemetryGuard {
    tracer_provider: SdkTracerProvider,
}

pub fn init(
    service: ServiceInfo,
    config: &TelemetryConfig,
) -> Result<TelemetryGuard, TelemetryInitError> {
    global::set_text_map_propagator(TraceContextPropagator::new());

    let tracer_provider = build_tracer_provider(service, config)?;
    let tracer = tracer_provider.tracer(service.name.to_owned());
    global::set_tracer_provider(tracer_provider.clone());

    let env_filter = EnvFilter::try_new(config.filter.as_str()).map_err(|error| {
        TelemetryInitError::InvalidFilter {
            message: error.to_string(),
        }
    })?;
    let local_layer = config.local.enabled.then(|| {
        tracing_subscriber::fmt::layer()
            .with_target(false)
            .with_timer(LocalTime::rfc_3339())
            .compact()
    });
    let otel_layer = tracing_opentelemetry::layer().with_tracer(tracer);

    tracing_subscriber::registry()
        .with(env_filter)
        .with(otel_layer)
        .with(local_layer)
        .try_init()
        .map_err(|error| TelemetryInitError::InitSubscriber {
            message: error.to_string(),
        })?;

    Ok(TelemetryGuard { tracer_provider })
}

impl Drop for TelemetryGuard {
    fn drop(&mut self) {
        let _ = self.tracer_provider.shutdown();
    }
}

fn build_tracer_provider(
    service: ServiceInfo,
    config: &TelemetryConfig,
) -> Result<SdkTracerProvider, TelemetryInitError> {
    let resource = Resource::builder_empty()
        .with_attributes([
            KeyValue::new("service.name", service.name.to_owned()),
            KeyValue::new("service.version", service.version.to_owned()),
        ])
        .build();
    let builder = SdkTracerProvider::builder().with_resource(resource);

    if !config.remote.enabled {
        return Ok(builder.build());
    }

    if config.remote.timeout_ms == 0 {
        return Err(TelemetryInitError::InvalidRemoteTimeout);
    }

    let endpoint = config
        .remote
        .otlp_http_endpoint
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .ok_or(TelemetryInitError::EmptyRemoteEndpoint)?;
    let exporter = SpanExporter::builder()
        .with_http()
        .with_protocol(Protocol::HttpBinary)
        .with_endpoint(endpoint.to_owned())
        .with_timeout(Duration::from_millis(config.remote.timeout_ms))
        .build()
        .map_err(|source| TelemetryInitError::BuildTraceExporter { source })?;

    Ok(builder.with_batch_exporter(exporter).build())
}
