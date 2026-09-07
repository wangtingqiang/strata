# Strata

Rust 后端服务基础设施工具集，提供错误模型、统一 API 响应、配置加载、存储客户端适配、JWT、可观测性等通用能力。

Workspace 组成：

- `strata`：核心工具集（配置、错误、分页、密码、文本/时间/校验等）
- `strata-error` + `strata-error-macros`：错误信息模型与派生宏
- `strata-axum`：统一响应体与请求提取器
- `strata-jwt`：Ed25519 JWT 签名、验证与 JWK 导出
- `strata-store`：ClickHouse / Redis / S3 / SQLx（MySQL、PostgreSQL）适配
- `strata-telemetry`：tracing + OpenTelemetry 可观测性
