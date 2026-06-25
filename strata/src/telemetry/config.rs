use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct TelemetryConfig {
    pub local: TelemetryLocalConfig,
    pub remote: TelemetryRemoteConfig,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(try_from = "TelemetryLocalConfigHelper")]
pub enum TelemetryLocalConfig {
    Disabled,
    Enabled { filter: String },
}

#[derive(Debug, Clone, Deserialize)]
#[serde(try_from = "TelemetryRemoteConfigHelper")]
pub enum TelemetryRemoteConfig {
    Disabled,
    Enabled {
        service_name: String,
        service_version: String,
        filter: String,
        otlp_http_endpoint: String,
        otlp_http_timeout_ms: u64,
    },
}

#[derive(Debug, Clone, Deserialize)]
struct TelemetryLocalConfigHelper {
    enabled: bool,
    filter: Option<String>,
}

impl TryFrom<TelemetryLocalConfigHelper> for TelemetryLocalConfig {
    type Error = &'static str;

    fn try_from(h: TelemetryLocalConfigHelper) -> Result<Self, Self::Error> {
        if !h.enabled {
            return Ok(Self::Disabled);
        }
        Ok(Self::Enabled {
            filter: h.filter.ok_or("filter is required when enabled")?,
        })
    }
}

#[derive(Debug, Clone, Deserialize)]
struct TelemetryRemoteConfigHelper {
    enabled: bool,
    service_name: Option<String>,
    service_version: Option<String>,
    filter: Option<String>,
    otlp_http_endpoint: Option<String>,
    otlp_http_timeout_ms: Option<u64>,
}

impl TryFrom<TelemetryRemoteConfigHelper> for TelemetryRemoteConfig {
    type Error = &'static str;

    fn try_from(h: TelemetryRemoteConfigHelper) -> Result<Self, Self::Error> {
        if !h.enabled {
            return Ok(Self::Disabled);
        }
        Ok(Self::Enabled {
            service_name: h
                .service_name
                .ok_or("service_name is required when enabled")?,
            service_version: h
                .service_version
                .ok_or("service_version is required when enabled")?,
            filter: h.filter.ok_or("filter is required when enabled")?,
            otlp_http_endpoint: h
                .otlp_http_endpoint
                .ok_or("otlp_http_endpoint is required when enabled")?,
            otlp_http_timeout_ms: h
                .otlp_http_timeout_ms
                .ok_or("otlp_http_timeout_ms is required when enabled")?,
        })
    }
}
