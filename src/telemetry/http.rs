use std::time::Duration;

use axum::extract::MatchedPath;
use http::{HeaderMap, HeaderName, HeaderValue, Request};
use opentelemetry::{
    Context, global,
    propagation::{Extractor, Injector},
    trace::TraceContextExt as _,
};
use tower_http::{
    classify::{ServerErrorsAsFailures, SharedClassifier},
    trace::{MakeSpan, OnRequest, OnResponse, TraceLayer},
};
use tracing::{Span, error, info, warn};
use tracing_opentelemetry::OpenTelemetrySpanExt;

pub fn server_trace_layer()
-> TraceLayer<SharedClassifier<ServerErrorsAsFailures>, HttpMakeSpan, HttpOnRequest, HttpOnResponse>
{
    TraceLayer::new_for_http()
        .make_span_with(HttpMakeSpan)
        .on_request(HttpOnRequest)
        .on_response(HttpOnResponse)
}

pub fn inject_trace_context(headers: &mut HeaderMap) {
    let span = Span::current();
    let context = span.context();
    if !context.span().span_context().is_valid() {
        return;
    }

    global::get_text_map_propagator(|propagator| {
        propagator.inject_context(&context, &mut HeaderInjector(headers));
    });
}

fn extract_trace_context(headers: &HeaderMap) -> Context {
    global::get_text_map_propagator(|propagator| propagator.extract(&HeaderExtractor(headers)))
}

fn record_trace_context(span: &Span) {
    let context = span.context();
    let otel_span = context.span();
    let span_context = otel_span.span_context();
    if !span_context.is_valid() {
        return;
    }

    span.record("trace_id", span_context.trace_id().to_string());
    span.record("span_id", span_context.span_id().to_string());
}

#[derive(Clone, Copy, Debug, Default)]
pub struct HttpMakeSpan;

impl<B> MakeSpan<B> for HttpMakeSpan {
    fn make_span(&mut self, request: &Request<B>) -> Span {
        make_server_span(request)
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct HttpOnRequest;

impl<B> OnRequest<B> for HttpOnRequest {
    fn on_request(&mut self, request: &Request<B>, span: &Span) {
        record_server_trace_context(request, span);
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct HttpOnResponse;

impl<B> OnResponse<B> for HttpOnResponse {
    fn on_response(self, response: &http::Response<B>, latency: Duration, span: &Span) {
        log_server_response(response, latency, span);
    }
}

fn make_server_span<B>(request: &Request<B>) -> Span {
    let route = request
        .extensions()
        .get::<MatchedPath>()
        .map(MatchedPath::as_str)
        .unwrap_or_else(|| request.uri().path())
        .to_owned();
    let path = request.uri().path().to_owned();
    let span = tracing::info_span!(
        "http.server.request",
        otel.kind = "server",
        trace_id = tracing::field::Empty,
        span_id = tracing::field::Empty,
        http.request.method = %request.method(),
        http.route = %route,
        url.path = %path,
        http.response.status_code = tracing::field::Empty,
    );
    let _ = span.set_parent(extract_trace_context(request.headers()));
    span
}

fn record_server_trace_context<B>(_request: &Request<B>, span: &Span) {
    record_trace_context(span);
}

fn log_server_response<B>(response: &http::Response<B>, latency: Duration, span: &Span) {
    let status_code = response.status().as_u16();
    span.record("http.response.status_code", status_code);

    let _guard = span.enter();
    if response.status().is_server_error() {
        error!(
            latency_ms = latency.as_millis(),
            status_code, "request completed"
        );
    } else if response.status().is_client_error() {
        warn!(
            latency_ms = latency.as_millis(),
            status_code, "request completed"
        );
    } else {
        info!(
            latency_ms = latency.as_millis(),
            status_code, "request completed"
        );
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

struct HeaderInjector<'a>(&'a mut HeaderMap);

impl Injector for HeaderInjector<'_> {
    fn set(&mut self, key: &str, value: String) {
        let Ok(header_name) = HeaderName::from_bytes(key.as_bytes()) else {
            return;
        };
        let Ok(header_value) = HeaderValue::from_str(value.as_str()) else {
            return;
        };
        self.0.insert(header_name, header_value);
    }
}
