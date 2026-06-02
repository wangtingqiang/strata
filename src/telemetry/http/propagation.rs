use http::{HeaderMap, HeaderName, HeaderValue};
use opentelemetry::{global, propagation::Injector, trace::TraceContextExt};
use tracing::Span;
use tracing_opentelemetry::OpenTelemetrySpanExt;

pub trait HeaderMapExt {
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
