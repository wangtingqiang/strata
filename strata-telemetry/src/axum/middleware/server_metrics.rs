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

#[cfg(test)]
mod tests {
    use axum::{
        Router,
        body::Body,
        http::{Request, StatusCode},
        middleware,
        routing::get,
    };
    use opentelemetry::Value;
    use opentelemetry_sdk::metrics::{
        InMemoryMetricExporter, SdkMeterProvider,
        data::{AggregatedMetrics, HistogramDataPoint, MetricData},
    };
    use tower::ServiceExt;

    use super::*;

    #[test]
    fn records_duration_histogram_with_route_and_status_labels() {
        let exporter = InMemoryMetricExporter::default();
        let provider = SdkMeterProvider::builder()
            .with_periodic_exporter(exporter.clone())
            .build();
        opentelemetry::global::set_meter_provider(provider.clone());

        let app = Router::new()
            .route("/items/{id}", get(|| async { StatusCode::OK }))
            .route("/fail", get(|| async { StatusCode::INTERNAL_SERVER_ERROR }))
            .route_layer(middleware::from_fn(server_metrics));

        // `HTTP_SERVER_DURATION` 在首次访问时绑定全局 meter，因此断言集中在单个测试内串行完成。
        let runtime = tokio::runtime::Builder::new_current_thread()
            .build()
            .unwrap();
        runtime.block_on(async {
            let ok = app
                .clone()
                .oneshot(
                    Request::builder()
                        .uri("/items/7")
                        .body(Body::empty())
                        .unwrap(),
                )
                .await
                .unwrap();
            assert_eq!(ok.status(), StatusCode::OK);

            let error = app
                .oneshot(Request::builder().uri("/fail").body(Body::empty()).unwrap())
                .await
                .unwrap();
            assert_eq!(error.status(), StatusCode::INTERNAL_SERVER_ERROR);
        });

        provider.force_flush().unwrap();

        let resource_metrics = exporter.get_finished_metrics().unwrap();
        let metrics: Vec<_> = resource_metrics
            .iter()
            .flat_map(|resource| resource.scope_metrics())
            .flat_map(|scope| scope.metrics())
            .collect();

        let duration = metrics
            .iter()
            .find(|metric| metric.name() == "http.server.duration")
            .expect("duration metric should be recorded");

        let AggregatedMetrics::F64(MetricData::Histogram(histogram)) = duration.data() else {
            panic!("expected f64 histogram data");
        };

        let points: Vec<&HistogramDataPoint<f64>> = histogram.data_points().collect();
        assert_eq!(points.len(), 2);
        assert!(
            points
                .iter()
                .any(|point| has_labels(point, "/items/{id}", 200))
        );
        assert!(points.iter().any(|point| has_labels(point, "/fail", 500)));
    }

    fn has_labels(point: &HistogramDataPoint<f64>, route: &str, status: i64) -> bool {
        point.attributes().any(|attribute| {
            attribute.key.as_str() == "http.route" && attribute.value.as_str() == route
        }) && point.attributes().any(|attribute| {
            attribute.key.as_str() == "http.status_code"
                && matches!(&attribute.value, Value::I64(value) if *value == status)
        })
    }
}
