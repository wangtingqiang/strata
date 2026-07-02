use std::time::Duration;

use axum::extract::MatchedPath;
use http::{HeaderMap, HeaderName};
use opentelemetry::{global, propagation::Extractor};
use tower_http::{
    classify::{ServerErrorsAsFailures, SharedClassifier},
    trace::{
        DefaultOnBodyChunk, DefaultOnEos, MakeSpan, OnFailure, OnRequest, OnResponse, TraceLayer,
    },
};
use tracing_opentelemetry::OpenTelemetrySpanExt;

#[derive(Clone, Copy, Debug, Default)]
pub struct HttpMakeSpan;

#[derive(Clone, Copy, Debug, Default)]
pub struct HttpOnRequest;

#[derive(Clone, Copy, Debug, Default)]
pub struct HttpOnResponse;

#[derive(Clone, Copy, Debug, Default)]
pub struct HttpOnFailure;

pub fn server_trace_layer() -> TraceLayer<
    SharedClassifier<ServerErrorsAsFailures>,
    HttpMakeSpan,
    HttpOnRequest,
    HttpOnResponse,
    DefaultOnBodyChunk,
    DefaultOnEos,
    HttpOnFailure,
> {
    TraceLayer::new_for_http()
        .make_span_with(HttpMakeSpan)
        .on_request(HttpOnRequest)
        .on_response(HttpOnResponse)
        .on_failure(HttpOnFailure)
}

impl<B> MakeSpan<B> for HttpMakeSpan {
    fn make_span(&mut self, request: &http::Request<B>) -> tracing::Span {
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

        let _ = span.set_parent(global::get_text_map_propagator(|propagator| {
            propagator.extract(&HeaderExtractor(request.headers()))
        }));

        span
    }
}

impl<B> OnRequest<B> for HttpOnRequest {
    fn on_request(&mut self, _: &http::Request<B>, _: &tracing::Span) {}
}

impl<B> OnResponse<B> for HttpOnResponse {
    fn on_response(self, response: &http::Response<B>, latency: Duration, span: &tracing::Span) {
        let status_code = response.status().as_u16();
        let latency_ms = latency.as_millis();

        span.record("http.response.status_code", status_code);

        if response.status().is_server_error() {
            tracing::error!(%latency_ms, "request completed");
        } else if response.status().is_client_error() {
            tracing::warn!(%latency_ms, "request completed");
        } else {
            tracing::info!(%latency_ms, "request completed");
        }
    }
}

impl<FailureClass> OnFailure<FailureClass> for HttpOnFailure {
    fn on_failure(&mut self, _: FailureClass, _: Duration, _: &tracing::Span) {}
}

struct HeaderExtractor<'a>(&'a HeaderMap);

impl Extractor for HeaderExtractor<'_> {
    fn get(&self, key: &str) -> Option<&str> {
        self.0.get(key).and_then(|value| value.to_str().ok())
    }

    fn keys(&self) -> Vec<&str> {
        self.0.keys().map(HeaderName::as_str).collect()
    }
}
