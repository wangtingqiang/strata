use opentelemetry_sdk::{metrics::SdkMeterProvider, trace::SdkTracerProvider};

/// 可观测性资源守卫：析构时关闭 tracer 与 meter provider，确保数据上报完成。
#[derive(Debug)]
pub struct TelemetryGuard {
    tracer_provider: Option<SdkTracerProvider>,
    meter_provider: Option<SdkMeterProvider>,
}

impl TelemetryGuard {
    /// 使用可选的 tracer / meter provider 构建守卫。
    pub fn new(
        tracer_provider: Option<SdkTracerProvider>,
        meter_provider: Option<SdkMeterProvider>,
    ) -> Self {
        Self {
            tracer_provider,
            meter_provider,
        }
    }
}

impl Drop for TelemetryGuard {
    fn drop(&mut self) {
        if let Some(ref provider) = self.tracer_provider {
            let _ = provider.shutdown();
        }
        if let Some(ref provider) = self.meter_provider {
            let _ = provider.shutdown();
        }
    }
}
