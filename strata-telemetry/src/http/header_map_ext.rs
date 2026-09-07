use http::{HeaderMap, HeaderName, HeaderValue};
use opentelemetry::{global, propagation::Injector, trace::TraceContextExt};
use tracing::Span;
use tracing_opentelemetry::OpenTelemetrySpanExt;

/// 向 [`http::HeaderMap`] 注入当前 span 的 trace 上下文。
pub trait HeaderMapExt {
    /// 将当前 span 的 trace 上下文注入请求头（如 `traceparent`）。
    fn inject_trace_context(&mut self);
}

impl HeaderMapExt for HeaderMap {
    fn inject_trace_context(&mut self) {
        let span = Span::current();
        let context = span.context();
        if !context.span().span_context().is_valid() {
            return;
        }

        global::get_text_map_propagator(|propagator| {
            propagator.inject_context(&context, &mut HeaderInjector(self));
        });
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

#[cfg(test)]
mod tests {
    use axum::http::HeaderMap;
    use opentelemetry::trace::TracerProvider;
    use opentelemetry_sdk::propagation::TraceContextPropagator;
    use tracing_subscriber::layer::SubscriberExt;

    use super::*;

    #[test]
    fn no_op_without_valid_trace_context() {
        let mut headers = HeaderMap::new();

        headers.inject_trace_context();

        assert!(headers.get("traceparent").is_none());
    }

    #[test]
    fn injects_traceparent_with_valid_context() {
        let provider = opentelemetry_sdk::trace::SdkTracerProvider::builder()
            .with_sampler(opentelemetry_sdk::trace::Sampler::AlwaysOn)
            .build();
        opentelemetry::global::set_tracer_provider(provider.clone());
        opentelemetry::global::set_text_map_propagator(TraceContextPropagator::new());

        let subscriber = tracing_subscriber::registry()
            .with(tracing_opentelemetry::layer().with_tracer(provider.tracer("test")));

        tracing::subscriber::with_default(subscriber, || {
            let span = tracing::info_span!("inject-test");
            let _enter = span.enter();

            let mut headers = HeaderMap::new();
            headers.inject_trace_context();

            let traceparent = headers
                .get("traceparent")
                .expect("traceparent should be injected")
                .to_str()
                .unwrap();

            assert!(traceparent.starts_with("00-"));
        });
    }
}
