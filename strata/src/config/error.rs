/// 环境字符串解析失败时返回的错误。
#[derive(Debug, thiserror::Error)]
pub enum EnvironmentError {
    /// 环境值不在支持列表中。
    #[error(
        "unsupported environment value `{value}`, expected one of local/development/test/staging/production"
    )]
    Unsupported {
        /// 不支持的环境值。
        value: String,
    },
}

/// 配置加载错误。
#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    /// 读取 APP_ENVIRONMENT 失败。
    #[error("failed to read APP_ENVIRONMENT: {0}")]
    ReadAppEnvironment(#[source] std::env::VarError),

    /// 环境值不合法。
    #[error(transparent)]
    InvalidEnvironment(#[from] EnvironmentError),

    /// 读取 APP_CONFIG_DIR 失败。
    #[error("failed to read APP_CONFIG_DIR: {0}")]
    ReadAppConfigDir(#[source] std::env::VarError),

    /// APP_CONFIG_DIR 已设置但为空。
    #[error("APP_CONFIG_DIR is set but empty")]
    EmptyAppConfigDir,

    /// 未指定配置目录。
    #[error("config directory is not specified")]
    ConfigDirUnspecified,

    /// 配置目录不存在。
    #[error("config directory `{path}` does not exist")]
    ConfigDirNotFound {
        /// 配置目录路径。
        path: String,
    },

    /// 配置目录不是目录。
    #[error("config directory `{path}` is not a directory")]
    ConfigDirInvalid {
        /// 配置目录路径。
        path: String,
    },

    /// 配置文件不存在。
    #[error("config file `{path}` does not exist")]
    ConfigFileNotFound {
        /// 配置文件路径。
        path: String,
    },

    /// 配置构建失败。
    #[error("failed to build config from `{path}`: {source}")]
    BuildConfig {
        /// 配置文件路径。
        path: String,
        /// 根源错误。
        #[source]
        source: config::ConfigError,
    },

    /// 配置反序列化失败。
    #[error("failed to deserialize config from `{path}`: {source}")]
    DeserializeConfig {
        /// 配置文件路径。
        path: String,
        /// 根源错误。
        #[source]
        source: config::ConfigError,
    },
}
