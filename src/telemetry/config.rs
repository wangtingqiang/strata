use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct TelemetryConfig {
    pub filter: String,
    pub local: TelemetryLocalConfig,
    pub remote: TelemetryRemoteConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TelemetryLocalConfig {
    pub enabled: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TelemetryRemoteConfig {
    pub enabled: bool,
    pub otlp_http_endpoint: Option<String>,
    pub timeout_ms: u64,
}
