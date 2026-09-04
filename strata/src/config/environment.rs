use crate::config::EnvironmentError;

/// 应用运行环境，用于决定加载 `<environment>.toml`。
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Environment {
    /// 本地环境（默认）。
    #[default]
    Local,
    /// 开发环境。
    Development,
    /// 测试环境。
    Test,
    /// 预发布环境。
    Staging,
    /// 生产环境。
    Production,
}

impl Environment {
    /// 环境标识字符串。
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Local => "local",
            Self::Development => "development",
            Self::Test => "test",
            Self::Staging => "staging",
            Self::Production => "production",
        }
    }
}

impl TryFrom<&str> for Environment {
    type Error = EnvironmentError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.trim().to_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "development" => Ok(Self::Development),
            "test" => Ok(Self::Test),
            "staging" => Ok(Self::Staging),
            "production" => Ok(Self::Production),
            _ => Err(EnvironmentError::Unsupported {
                value: value.to_string(),
            }),
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = EnvironmentError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Environment::try_from(value.as_str())
    }
}
