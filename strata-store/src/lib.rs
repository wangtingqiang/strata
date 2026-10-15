//! 外部数据系统客户端适配：配置与连接封装。

#![warn(missing_docs)]

/// ClickHouse 客户端适配。
#[cfg(feature = "clickhouse")]
pub mod clickhouse;
/// Redis 客户端适配。
#[cfg(feature = "redis")]
pub mod redis;
/// SQLx 连接池适配（MySQL / PostgreSQL）。
#[cfg(any(feature = "sqlx-mysql", feature = "sqlx-postgres"))]
pub mod sqlx;
