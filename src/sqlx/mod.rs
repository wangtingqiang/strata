#[cfg(feature = "sqlx-mysql")]
mod mysql;
#[cfg(feature = "sqlx-mysql")]
pub use mysql::{MySqlPoolConfig, MySqlPoolInitError};

#[cfg(feature = "sqlx-postgres")]
mod postgres;
#[cfg(feature = "sqlx-postgres")]
pub use postgres::{PgPoolConfig, PgPoolInitError};
