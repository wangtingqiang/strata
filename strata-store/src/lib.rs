//! 外部数据系统客户端适配：ClickHouse、Redis、S3、SQLx（MySQL/PostgreSQL）的配置、连接与签名封装。

#![warn(missing_docs)]

/// ClickHouse 客户端适配。
#[cfg(feature = "clickhouse")]
pub mod clickhouse;
/// Redis 客户端适配。
#[cfg(feature = "redis")]
pub mod redis;
/// S3 客户端适配（配置与 AWS SigV4 签名）。
#[cfg(feature = "s3")]
pub mod s3;
/// SQLx 连接池适配（MySQL / PostgreSQL）。
#[cfg(any(feature = "sqlx-mysql", feature = "sqlx-postgres"))]
pub mod sqlx;
