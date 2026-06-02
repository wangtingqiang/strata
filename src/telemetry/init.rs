use std::time::Duration;

use opentelemetry::{KeyValue, global, trace::TracerProvider as _};
use opentelemetry_otlp::{Protocol, SpanExporter, WithExportConfig};
use opentelemetry_sdk::{Resource, propagation::TraceContextPropagator, trace::SdkTracerProvider};
use tracing_subscriber::{
    EnvFilter, Layer, fmt::time::LocalTime, layer::SubscriberExt as _, util::SubscriberInitExt as _,
};

use crate::telemetry::{TelemetryInitError, TelemetryLocalConfig, TelemetryRemoteConfig};

#[derive(Debug)]
pub struct TelemetryGuard {
    tracer_provider: SdkTracerProvider,
}

pub fn init(
    local: &TelemetryLocalConfig,
    remote: &TelemetryRemoteConfig,
) -> Result<TelemetryGuard, TelemetryInitError> {
    global::set_text_map_propagator(TraceContextPropagator::new());

    let tracer_provider = build_tracer_provider(remote)?;

    let local_layer = match local {
        TelemetryLocalConfig::Enabled { filter } => {
            let env_filter =
                EnvFilter::try_new(filter).map_err(|error| TelemetryInitError::InvalidFilter {
                    message: error.to_string(),
                })?;
            Some(
                tracing_subscriber::fmt::layer()
                    .with_target(false)
                    .with_timer(LocalTime::rfc_3339())
                    .compact()
                    .with_filter(env_filter),
            )
        }
        TelemetryLocalConfig::Disabled => None,
    };

    let remote_layer = match remote {
        TelemetryRemoteConfig::Enabled {
            filter,
            service_name,
            ..
        } => {
            let env_filter =
                EnvFilter::try_new(filter).map_err(|error| TelemetryInitError::InvalidFilter {
                    message: error.to_string(),
                })?;
            let tracer = tracer_provider.tracer(service_name.clone());
            global::set_tracer_provider(tracer_provider.clone());
            Some(
                tracing_opentelemetry::layer()
                    .with_tracer(tracer)
                    .with_filter(env_filter),
            )
        }
        TelemetryRemoteConfig::Disabled => None,
    };

    tracing_subscriber::registry()
        .with(local_layer)
        .with(remote_layer)
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
    remote: &TelemetryRemoteConfig,
) -> Result<SdkTracerProvider, TelemetryInitError> {
    let resource = match remote {
        TelemetryRemoteConfig::Disabled => Resource::builder_empty().build(),
        TelemetryRemoteConfig::Enabled {
            service_name,
            service_version,
            ..
        } => Resource::builder_empty()
            .with_attributes([
                KeyValue::new("service.name", service_name.clone()),
                KeyValue::new("service.version", service_version.clone()),
            ])
            .build(),
    };

    let builder = SdkTracerProvider::builder().with_resource(resource);

    let TelemetryRemoteConfig::Enabled {
        otlp_http_endpoint,
        otlp_http_timeout_ms,
        ..
    } = remote
    else {
        return Ok(builder.build());
    };

    if *otlp_http_timeout_ms == 0 {
        return Err(TelemetryInitError::InvalidRemoteTimeout);
    }

    let endpoint = otlp_http_endpoint.trim();
    let exporter = SpanExporter::builder()
        .with_http()
        .with_protocol(Protocol::HttpBinary)
        .with_endpoint(endpoint.to_owned())
        .with_timeout(Duration::from_millis(*otlp_http_timeout_ms))
        .build()
        .map_err(|source| TelemetryInitError::BuildTraceExporter { source })?;

    Ok(builder.with_batch_exporter(exporter).build())
}
