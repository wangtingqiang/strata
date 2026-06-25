use std::time::Duration;

use opentelemetry::{KeyValue, global, trace::TracerProvider as _};
use opentelemetry_otlp::{MetricExporter, Protocol, SpanExporter, WithExportConfig};
#[cfg(feature = "telemetry-http")]
use opentelemetry_sdk::propagation::TraceContextPropagator;
use opentelemetry_sdk::{Resource, metrics::SdkMeterProvider, trace::SdkTracerProvider};
use tracing_subscriber::{
    EnvFilter, Layer, fmt::time::LocalTime, layer::SubscriberExt as _, util::SubscriberInitExt as _,
};

use crate::telemetry::{TelemetryInitError, TelemetryLocalConfig, TelemetryRemoteConfig};

#[derive(Debug)]
pub struct TelemetryGuard {
    tracer_provider: Option<SdkTracerProvider>,
    meter_provider: Option<SdkMeterProvider>,
}

impl Drop for TelemetryGuard {
    fn drop(&mut self) {
        if let Some(ref provider) = self.tracer_provider {
            let _ = provider.shutdown();
        }
        if let Some(ref provider) = self.meter_provider {
            let _ = provider.shutdown();
        }
    }
}

pub fn init(
    local: &TelemetryLocalConfig,
    remote: &TelemetryRemoteConfig,
) -> Result<TelemetryGuard, TelemetryInitError> {
    #[cfg(feature = "telemetry-http")]
    global::set_text_map_propagator(TraceContextPropagator::new());

    let local_layer = match local {
        TelemetryLocalConfig::Enabled { filter } => {
            let env_filter =
                EnvFilter::try_new(filter).map_err(TelemetryInitError::InvalidFilter)?;

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

    let (tracer_provider, meter_provider, remote_layer) = match remote {
        TelemetryRemoteConfig::Enabled {
            filter,
            service_name,
            service_version,
            otlp_http_endpoint,
            otlp_http_timeout_ms,
        } => {
            let tracer_provider = build_tracer_provider(
                service_name,
                service_version,
                otlp_http_endpoint,
                *otlp_http_timeout_ms,
            )?;

            let meter_provider = build_meter_provider(
                service_name,
                service_version,
                otlp_http_endpoint,
                *otlp_http_timeout_ms,
            )?;

            global::set_tracer_provider(tracer_provider.clone());
            global::set_meter_provider(meter_provider.clone());

            let tracer = tracer_provider.tracer(service_name.clone());

            let env_filter =
                EnvFilter::try_new(filter).map_err(TelemetryInitError::InvalidFilter)?;

            let layer = Some(
                tracing_opentelemetry::layer()
                    .with_tracer(tracer)
                    .with_filter(env_filter),
            );

            (Some(tracer_provider), Some(meter_provider), layer)
        }
        TelemetryRemoteConfig::Disabled => (None, None, None),
    };

    tracing_subscriber::registry()
        .with(local_layer)
        .with(remote_layer)
        .try_init()
        .map_err(TelemetryInitError::InitSubscriber)?;

    Ok(TelemetryGuard {
        tracer_provider,
        meter_provider,
    })
}

fn build_tracer_provider(
    service_name: &str,
    service_version: &str,
    otlp_http_endpoint: &str,
    otlp_http_timeout_ms: u64,
) -> Result<SdkTracerProvider, TelemetryInitError> {
    let endpoint = otlp_http_endpoint.trim();

    if endpoint.is_empty() {
        return Err(TelemetryInitError::InvalidRemoteEndpoint);
    }

    if otlp_http_timeout_ms == 0 {
        return Err(TelemetryInitError::InvalidRemoteTimeout);
    }

    let resource = Resource::builder_empty()
        .with_attributes([
            KeyValue::new("service.name", service_name.to_owned()),
            KeyValue::new("service.version", service_version.to_owned()),
        ])
        .build();

    let exporter = SpanExporter::builder()
        .with_http()
        .with_protocol(Protocol::HttpBinary)
        .with_endpoint(endpoint.to_owned())
        .with_timeout(Duration::from_millis(otlp_http_timeout_ms))
        .build()
        .map_err(TelemetryInitError::BuildTraceExporter)?;

    let provider = SdkTracerProvider::builder()
        .with_resource(resource)
        .with_batch_exporter(exporter)
        .build();

    Ok(provider)
}

fn build_meter_provider(
    service_name: &str,
    service_version: &str,
    otlp_http_endpoint: &str,
    otlp_http_timeout_ms: u64,
) -> Result<SdkMeterProvider, TelemetryInitError> {
    let endpoint = otlp_http_endpoint.trim();

    if endpoint.is_empty() {
        return Err(TelemetryInitError::InvalidRemoteEndpoint);
    }

    if otlp_http_timeout_ms == 0 {
        return Err(TelemetryInitError::InvalidRemoteTimeout);
    }

    let resource = Resource::builder_empty()
        .with_attributes([
            KeyValue::new("service.name", service_name.to_owned()),
            KeyValue::new("service.version", service_version.to_owned()),
        ])
        .build();

    let exporter = MetricExporter::builder()
        .with_http()
        .with_protocol(Protocol::HttpBinary)
        .with_endpoint(endpoint.to_owned())
        .with_timeout(Duration::from_millis(otlp_http_timeout_ms))
        .build()
        .map_err(TelemetryInitError::BuildMetricExporter)?;

    let provider = SdkMeterProvider::builder()
        .with_resource(resource)
        .with_periodic_exporter(exporter)
        .build();

    Ok(provider)
}
