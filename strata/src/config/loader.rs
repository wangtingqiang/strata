use std::path::{Path, PathBuf};

use serde::de::DeserializeOwned;

use crate::config::{ConfigError, Environment};

const APP_ENVIRONMENT: &str = "APP_ENVIRONMENT";
const APP_CONFIG_DIR: &str = "APP_CONFIG_DIR";

/// 通用配置加载器。
///
/// 加载优先级：
/// 1. 显式参数 `with_environment` / `with_config_dir`
/// 2. 环境变量 `APP_ENVIRONMENT` / `APP_CONFIG_DIR`
/// 3. 默认值 `local`
#[derive(Debug, Default)]
pub struct ConfigLoader {
    environment: Option<Environment>,
    config_dir: Option<PathBuf>,
}

impl ConfigLoader {
    /// 新建加载器。
    pub fn new() -> Self {
        Self::default()
    }

    /// 显式指定环境。
    pub fn with_environment(mut self, environment: Environment) -> Self {
        self.environment = Some(environment);
        self
    }

    /// 显式指定配置目录。
    pub fn with_config_dir(mut self, dir: impl Into<PathBuf>) -> Self {
        self.config_dir = Some(dir.into());
        self
    }

    /// 按解析结果加载 `<environment>.toml` 并反序列化为目标类型。
    pub fn load<T: DeserializeOwned>(self) -> Result<T, ConfigError> {
        let environment = self.resolve_environment()?;
        let config_dir = self.resolve_config_dir()?;
        validate_config_dir(&config_dir)?;

        let config_path = config_dir.join(format!("{}.toml", environment.as_str()));
        if !config_path.exists() {
            return Err(ConfigError::ConfigFileNotFound {
                path: path_string(&config_path),
            });
        }

        let settings = config::Config::builder()
            .add_source(config::File::from(config_path.clone()))
            .build()
            .map_err(|source| ConfigError::BuildConfig {
                path: path_string(&config_path),
                source,
            })?;

        settings
            .try_deserialize::<T>()
            .map_err(|source| ConfigError::DeserializeConfig {
                path: path_string(&config_path),
                source,
            })
    }

    fn resolve_environment(&self) -> Result<Environment, ConfigError> {
        match self.environment {
            Some(environment) => Ok(environment),
            None => resolve_environment_from_env(std::env::var(APP_ENVIRONMENT)),
        }
    }

    fn resolve_config_dir(&self) -> Result<PathBuf, ConfigError> {
        if let Some(path) = resolve_config_dir_from_env(std::env::var(APP_CONFIG_DIR))? {
            return Ok(path);
        }

        if let Some(path) = &self.config_dir {
            return Ok(path.clone());
        }

        Err(ConfigError::ConfigDirUnspecified)
    }
}

fn resolve_environment_from_env(
    value: Result<String, std::env::VarError>,
) -> Result<Environment, ConfigError> {
    match value {
        Ok(raw) => Environment::try_from(raw.as_str()).map_err(ConfigError::from),
        Err(std::env::VarError::NotPresent) => Ok(Environment::default()),
        Err(source) => Err(ConfigError::ReadAppEnvironment(source)),
    }
}

fn resolve_config_dir_from_env(
    value: Result<String, std::env::VarError>,
) -> Result<Option<PathBuf>, ConfigError> {
    match value {
        Ok(path) => {
            if path.trim().is_empty() {
                Err(ConfigError::EmptyAppConfigDir)
            } else {
                Ok(Some(PathBuf::from(path)))
            }
        }
        Err(std::env::VarError::NotPresent) => Ok(None),
        Err(source) => Err(ConfigError::ReadAppConfigDir(source)),
    }
}

fn validate_config_dir(dir: &Path) -> Result<(), ConfigError> {
    if !dir.exists() {
        return Err(ConfigError::ConfigDirNotFound {
            path: path_string(dir),
        });
    }

    if !dir.is_dir() {
        return Err(ConfigError::ConfigDirInvalid {
            path: path_string(dir),
        });
    }

    Ok(())
}

fn path_string(path: &Path) -> String {
    path.display().to_string()
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;

    use serde::Deserialize;

    use super::*;

    #[derive(Debug, Deserialize)]
    struct TestConfig {
        host: String,
        port: u16,
    }

    fn temp_config_dir() -> PathBuf {
        let dir = std::env::temp_dir().join(format!("strata-config-test-{}", std::process::id()));
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn loads_config_by_environment() {
        let dir = temp_config_dir();
        fs::write(dir.join("test.toml"), "host = \"127.0.0.1\"\nport = 8080\n").unwrap();

        let config: TestConfig = ConfigLoader::new()
            .with_environment(Environment::Test)
            .with_config_dir(&dir)
            .load()
            .unwrap();

        assert_eq!(config.host, "127.0.0.1");
        assert_eq!(config.port, 8080);
    }

    #[test]
    fn errors_when_config_dir_missing() {
        let missing = std::env::temp_dir().join("strata-config-test-does-not-exist");

        let result: Result<TestConfig, ConfigError> =
            ConfigLoader::new().with_config_dir(missing).load();

        assert!(matches!(result, Err(ConfigError::ConfigDirNotFound { .. })));
    }

    #[test]
    fn errors_when_config_dir_is_file() {
        let dir = temp_config_dir();
        let file = dir.join("not-a-dir.toml");
        fs::write(&file, "x = 1\n").unwrap();

        let result: Result<TestConfig, ConfigError> =
            ConfigLoader::new().with_config_dir(&file).load();

        assert!(matches!(result, Err(ConfigError::ConfigDirInvalid { .. })));
    }

    #[test]
    fn errors_when_config_file_missing() {
        let dir = temp_config_dir();

        let result: Result<TestConfig, ConfigError> = ConfigLoader::new()
            .with_environment(Environment::Production)
            .with_config_dir(&dir)
            .load();

        assert!(matches!(
            result,
            Err(ConfigError::ConfigFileNotFound { .. })
        ));
    }
}
