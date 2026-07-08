mod propagation;

pub mod middleware;

pub use propagation::HeaderMapExt;

use opentelemetry_sdk::propagation::TraceContextPropagator;
use std::sync::Once;

static INIT_PROPAGATOR: Once = Once::new();

pub(in crate::telemetry::http) fn ensure_propagator() {
    INIT_PROPAGATOR.call_once(|| {
        opentelemetry::global::set_text_map_propagator(TraceContextPropagator::new());
    });
}
