use thiserror::Error;

/// 环境字符串解析失败时返回的错误。
#[derive(Debug, Error)]
pub enum EnvironmentError {
    #[error(
        "unsupported environment value [{value}], expected one of local/development/test/staging/production"
    )]
    Unsupported { value: String },
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("failed to read APP_ENVIRONMENT: {0}")]
    ReadAppEnvironment(#[source] std::env::VarError),

    #[error(transparent)]
    InvalidEnvironment(#[from] EnvironmentError),

    #[error("failed to read APP_CONFIG_DIR: {0}")]
    ReadAppConfigDir(#[source] std::env::VarError),

    #[error("APP_CONFIG_DIR is set but empty")]
    EmptyAppConfigDir,

    #[error("config directory is not specified")]
    ConfigDirUnspecified,

    #[error("config directory [{path}] does not exist")]
    ConfigDirNotFound { path: String },

    #[error("config directory [{path}] is not a directory")]
    ConfigDirInvalid { path: String },

    #[error("config file [{path}] does not exist")]
    ConfigFileNotFound { path: String },

    #[error("failed to build config from [{path}]: {source}")]
    BuildConfig {
        path: String,
        #[source]
        source: config::ConfigError,
    },

    #[error("failed to deserialize config from [{path}]: {source}")]
    DeserializeConfig {
        path: String,
        #[source]
        source: config::ConfigError,
    },
}
