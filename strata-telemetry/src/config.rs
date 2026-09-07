use serde::Deserialize;

/// 可观测性配置：本地日志输出与远端 OTLP 上报。
#[derive(Debug, Clone, Deserialize)]
pub struct TelemetryConfig {
    /// 本地日志输出配置。
    pub local: TelemetryLocalConfig,
    /// 远端 OTLP 上报配置。
    pub remote: TelemetryRemoteConfig,
}

/// 本地日志输出配置。
#[derive(Debug, Clone, Deserialize)]
#[serde(try_from = "TelemetryLocalConfigHelper")]
pub enum TelemetryLocalConfig {
    /// 关闭本地日志输出。
    Disabled,
    /// 开启本地日志输出。
    Enabled {
        /// tracing 过滤器表达式（EnvFilter）。
        filter: String,
    },
}

/// 远端 OTLP 上报配置。
#[derive(Debug, Clone, Deserialize)]
#[serde(try_from = "TelemetryRemoteConfigHelper")]
pub enum TelemetryRemoteConfig {
    /// 关闭远端上报。
    Disabled,
    /// 开启远端上报。
    Enabled {
        /// tracing 过滤器表达式（EnvFilter）。
        filter: String,
        /// OTLP HTTP 上报端点。
        otlp_http_endpoint: String,
        /// 上报超时（毫秒）。
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

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    fn config_with(local: serde_json::Value, remote: serde_json::Value) -> TelemetryConfig {
        serde_json::from_value(json!({ "local": local, "remote": remote })).unwrap()
    }

    #[test]
    fn deserializes_both_disabled() {
        let config: TelemetryConfig = serde_json::from_value(json!({
            "local": { "enabled": false },
            "remote": { "enabled": false },
        }))
        .unwrap();

        assert!(matches!(config.local, TelemetryLocalConfig::Disabled));
        assert!(matches!(config.remote, TelemetryRemoteConfig::Disabled));
    }

    #[test]
    fn deserializes_local_enabled_with_filter() {
        let config = config_with(
            json!({ "enabled": true, "filter": "info" }),
            json!({ "enabled": false }),
        );

        assert!(matches!(
            config.local,
            TelemetryLocalConfig::Enabled { ref filter } if filter == "info"
        ));
    }

    #[test]
    fn local_enabled_requires_filter() {
        let result: Result<TelemetryConfig, _> = serde_json::from_value(
            json!({ "local": { "enabled": true }, "remote": { "enabled": false } }),
        );

        assert!(result.is_err());
    }

    #[test]
    fn deserializes_remote_enabled_with_all_fields() {
        let config = config_with(
            json!({ "enabled": false }),
            json!({ "enabled": true, "filter": "info", "otlp_http_endpoint": "http://collector:4318", "otlp_http_timeout_ms": 5000 }),
        );

        match config.remote {
            TelemetryRemoteConfig::Enabled {
                filter,
                otlp_http_endpoint,
                otlp_http_timeout_ms,
            } => {
                assert_eq!(filter, "info");
                assert_eq!(otlp_http_endpoint, "http://collector:4318");
                assert_eq!(otlp_http_timeout_ms, 5000);
            }
            _ => panic!("expected remote enabled"),
        }
    }

    #[test]
    fn remote_enabled_requires_all_fields() {
        for remote in [
            json!({ "enabled": true, "otlp_http_endpoint": "http://x", "otlp_http_timeout_ms": 1 }),
            json!({ "enabled": true, "filter": "info", "otlp_http_timeout_ms": 1 }),
            json!({ "enabled": true, "filter": "info", "otlp_http_endpoint": "http://x" }),
        ] {
            let result: Result<TelemetryConfig, _> = serde_json::from_value(json!({
                "local": { "enabled": false },
                "remote": remote,
            }));

            assert!(result.is_err());
        }
    }
}
