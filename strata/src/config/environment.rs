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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn as_str_returns_expected_identifiers() {
        assert_eq!(Environment::Local.as_str(), "local");
        assert_eq!(Environment::Development.as_str(), "development");
        assert_eq!(Environment::Test.as_str(), "test");
        assert_eq!(Environment::Staging.as_str(), "staging");
        assert_eq!(Environment::Production.as_str(), "production");
    }

    #[test]
    fn parses_all_supported_values() {
        assert_eq!(Environment::try_from("local").unwrap(), Environment::Local);
        assert_eq!(
            Environment::try_from("development").unwrap(),
            Environment::Development
        );
        assert_eq!(Environment::try_from("test").unwrap(), Environment::Test);
        assert_eq!(
            Environment::try_from("staging").unwrap(),
            Environment::Staging
        );
        assert_eq!(
            Environment::try_from("production").unwrap(),
            Environment::Production
        );
    }

    #[test]
    fn parsing_is_case_insensitive() {
        assert_eq!(Environment::try_from("LOCAL").unwrap(), Environment::Local);
        assert_eq!(
            Environment::try_from("Production").unwrap(),
            Environment::Production
        );
    }

    #[test]
    fn parsing_trims_whitespace() {
        assert_eq!(
            Environment::try_from(" local ").unwrap(),
            Environment::Local
        );
        assert_eq!(
            Environment::try_from("  test\n").unwrap(),
            Environment::Test
        );
    }

    #[test]
    fn unsupported_value_rejects_with_original() {
        let error = Environment::try_from("unknown").unwrap_err();

        assert!(matches!(
            error,
            EnvironmentError::Unsupported { value } if value == "unknown"
        ));
    }

    #[test]
    fn parses_owned_string() {
        assert_eq!(
            Environment::try_from("local".to_owned()).unwrap(),
            Environment::Local
        );
    }

    #[test]
    fn default_is_local() {
        assert_eq!(Environment::default(), Environment::Local);
    }
}
