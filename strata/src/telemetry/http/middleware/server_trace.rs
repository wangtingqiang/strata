use std::time::Instant;

use axum::{
    extract::{MatchedPath, Request},
    middleware::Next,
    response::Response,
};
use http::{HeaderMap, HeaderName};
use tracing::Instrument;
use tracing_opentelemetry::OpenTelemetrySpanExt;

pub async fn server_trace(request: Request, next: Next) -> Response {
    let route = request
        .extensions()
        .get::<MatchedPath>()
        .map(MatchedPath::as_str)
        .unwrap_or("unmatched");

    let span = tracing::info_span!(
        "http.server.request",
        otel.kind = %"server",
        http.request.method = %request.method(),
        http.route = %route,
        http.response.status_code = tracing::field::Empty,
    );

    let _ = span.set_parent(opentelemetry::global::get_text_map_propagator(
        |propagator| propagator.extract(&HeaderExtractor(request.headers())),
    ));

    let start = Instant::now();
    let response = next.run(request).instrument(span.clone()).await;
    let latency_ms = start.elapsed().as_millis();

    let _enter = span.enter();

    span.record("http.response.status_code", response.status().as_u16());

    if response.status().is_server_error() {
        tracing::error!(%latency_ms, "request completed");
    } else if response.status().is_client_error() {
        tracing::warn!(%latency_ms, "request completed");
    } else {
        tracing::info!(%latency_ms, "request completed");
    }

    response
}

struct HeaderExtractor<'a>(&'a HeaderMap);

impl opentelemetry::propagation::Extractor for HeaderExtractor<'_> {
    fn get(&self, key: &str) -> Option<&str> {
        self.0.get(key).and_then(|value| value.to_str().ok())
    }

    fn keys(&self) -> Vec<&str> {
        self.0.keys().map(HeaderName::as_str).collect()
    }
}
