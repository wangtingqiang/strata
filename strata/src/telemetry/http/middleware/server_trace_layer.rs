use std::time::Duration;

use axum::extract::MatchedPath;
use http::{HeaderMap, HeaderName, Request};
use opentelemetry::{global, propagation::Extractor, trace::TraceContextExt};
use tower_http::{
    classify::{ServerErrorsAsFailures, SharedClassifier},
    trace::{MakeSpan, OnRequest, OnResponse, TraceLayer},
};
use tracing_opentelemetry::OpenTelemetrySpanExt;

#[derive(Clone, Copy, Debug, Default)]
pub struct HttpMakeSpan;

#[derive(Clone, Copy, Debug, Default)]
pub struct HttpOnRequest;

#[derive(Clone, Copy, Debug, Default)]
pub struct HttpOnResponse;

pub fn server_trace_layer()
-> TraceLayer<SharedClassifier<ServerErrorsAsFailures>, HttpMakeSpan, HttpOnRequest, HttpOnResponse>
{
    TraceLayer::new_for_http()
        .make_span_with(HttpMakeSpan)
        .on_request(HttpOnRequest)
        .on_response(HttpOnResponse)
}

impl<B> MakeSpan<B> for HttpMakeSpan {
    fn make_span(&mut self, request: &Request<B>) -> tracing::Span {
        let route = request
            .extensions()
            .get::<MatchedPath>()
            .map(MatchedPath::as_str)
            .unwrap_or_else(|| request.uri().path())
            .to_owned();

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
    fn on_request(&mut self, _request: &Request<B>, span: &tracing::Span) {
        let context = span.context();
        let otel_span = context.span();
        let span_context = otel_span.span_context();

        if !span_context.is_valid() {
            return;
        }

        tracing::info!(trace_id = %span_context.trace_id(), "request started");
    }
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

struct HeaderExtractor<'a>(&'a HeaderMap);

impl Extractor for HeaderExtractor<'_> {
    fn get(&self, key: &str) -> Option<&str> {
        self.0.get(key).and_then(|value| value.to_str().ok())
    }

    fn keys(&self) -> Vec<&str> {
        self.0.keys().map(HeaderName::as_str).collect()
    }
}
