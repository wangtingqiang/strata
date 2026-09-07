use std::time::Instant;

use axum::{
    extract::{MatchedPath, Request},
    middleware::Next,
    response::Response,
};
use http::{HeaderMap, HeaderName};
use tracing::Instrument;
use tracing_opentelemetry::OpenTelemetrySpanExt;

/// Axum 中间件：为每个请求创建 span 并提取远端 trace 上下文，结束时记录状态码与耗时。
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

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use axum::{
        Router,
        body::Body,
        http::{Request, StatusCode},
        middleware,
        routing::get,
    };
    use tower::ServiceExt;

    use super::*;

    /// 捕获 new_span 的名称与字段，用于断言中间件创建的 span 内容。
    #[derive(Default)]
    struct RecordingSubscriber {
        spans: Arc<Mutex<Vec<(String, Vec<(String, String)>)>>>,
    }

    impl tracing::Subscriber for RecordingSubscriber {
        fn enabled(&self, _metadata: &tracing::Metadata<'_>) -> bool {
            true
        }

        fn new_span(&self, span: &tracing::span::Attributes<'_>) -> tracing::span::Id {
            let mut fields = Vec::new();
            span.record(&mut FieldVisitor(&mut fields));
            let mut spans = self.spans.lock().unwrap();
            spans.push((span.metadata().name().to_owned(), fields));
            tracing::span::Id::from_u64(spans.len() as u64)
        }

        fn record(&self, _span: &tracing::span::Id, _values: &tracing::span::Record<'_>) {}

        fn record_follows_from(&self, _span: &tracing::span::Id, _follows: &tracing::span::Id) {}

        fn event(&self, _event: &tracing::Event<'_>) {}

        fn enter(&self, _span: &tracing::span::Id) {}

        fn exit(&self, _span: &tracing::span::Id) {}
    }

    struct FieldVisitor<'a>(&'a mut Vec<(String, String)>);

    impl tracing::field::Visit for FieldVisitor<'_> {
        fn record_str(&mut self, field: &tracing::field::Field, value: &str) {
            self.0.push((field.name().to_owned(), value.to_owned()));
        }

        fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
            self.0.push((field.name().to_owned(), format!("{value:?}")));
        }

        fn record_u64(&mut self, field: &tracing::field::Field, value: u64) {
            self.0.push((field.name().to_owned(), value.to_string()));
        }

        fn record_i64(&mut self, field: &tracing::field::Field, value: i64) {
            self.0.push((field.name().to_owned(), value.to_string()));
        }

        fn record_bool(&mut self, field: &tracing::field::Field, value: bool) {
            self.0.push((field.name().to_owned(), value.to_string()));
        }
    }

    fn send_request(app: &Router, uri: &str) -> StatusCode {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .build()
            .unwrap();

        runtime
            .block_on(
                app.clone()
                    .oneshot(Request::builder().uri(uri).body(Body::empty()).unwrap()),
            )
            .unwrap()
            .status()
    }

    fn recorded_trace_span(
        spans: &Mutex<Vec<(String, Vec<(String, String)>)>>,
    ) -> (String, Vec<(String, String)>) {
        spans
            .lock()
            .unwrap()
            .iter()
            .find(|(name, _)| name == "http.server.request")
            .cloned()
            .expect("server trace span should be recorded")
    }

    #[test]
    fn records_span_with_matched_route() {
        let app = Router::new()
            .route("/users/{id}", get(|| async { StatusCode::OK }))
            .route_layer(middleware::from_fn(server_trace));

        let subscriber = RecordingSubscriber::default();
        let spans = subscriber.spans.clone();

        tracing::subscriber::with_default(subscriber, || {
            assert_eq!(send_request(&app, "/users/42"), StatusCode::OK);
        });

        let (name, fields) = recorded_trace_span(&spans);
        assert_eq!(name, "http.server.request");
        assert!(fields.contains(&("http.route".to_owned(), "/users/{id}".to_owned())));
        assert!(fields.contains(&("http.request.method".to_owned(), "GET".to_owned())));
    }

    #[test]
    fn records_span_with_unmatched_route_fallback() {
        let app = Router::new()
            .route("/users/{id}", get(|| async { StatusCode::OK }))
            .layer(middleware::from_fn(server_trace));

        let subscriber = RecordingSubscriber::default();
        let spans = subscriber.spans.clone();

        tracing::subscriber::with_default(subscriber, || {
            assert_eq!(send_request(&app, "/nope"), StatusCode::NOT_FOUND);
        });

        let (_, fields) = recorded_trace_span(&spans);
        assert!(fields.contains(&("http.route".to_owned(), "unmatched".to_owned())));
    }
}
