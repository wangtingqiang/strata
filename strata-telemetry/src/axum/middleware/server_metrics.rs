use std::{sync::LazyLock, time::Instant};

use axum::{body::Body, extract::MatchedPath, middleware::Next, response::Response};
use http::Request;
use opentelemetry::{KeyValue, global, metrics::Histogram};

static HTTP_SERVER_DURATION: LazyLock<Histogram<f64>> = LazyLock::new(|| {
    global::meter("strata-telemetry")
        .f64_histogram("http.server.duration")
        .with_unit("s")
        .with_description("HTTP server request duration")
        .build()
});

/// Axum 中间件：记录 HTTP 请求耗时直方图（指标 `http.server.duration`，含路由与状态码标签）。
pub async fn server_metrics(req: Request<Body>, next: Next) -> Response {
    let route = req
        .extensions()
        .get::<MatchedPath>()
        .map(MatchedPath::as_str)
        .unwrap_or_else(|| req.uri().path())
        .to_owned();

    let start = Instant::now();

    let response = next.run(req).await;

    HTTP_SERVER_DURATION.record(
        start.elapsed().as_secs_f64(),
        &[
            KeyValue::new("http.route", route),
            KeyValue::new("http.status_code", response.status().as_u16() as i64),
        ],
    );

    response
}
