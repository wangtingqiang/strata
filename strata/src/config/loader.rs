use std::path::{Path, PathBuf};

use serde::de::DeserializeOwned;

use crate::config::{ConfigError, Environment};

const APP_ENVIRONMENT: &str = "APP_ENVIRONMENT";
const APP_CONFIG_DIR: &str = "APP_CONFIG_DIR";

/// 通用配置加载器。
///
/// 加载优先级：
/// 1. 环境变量 `APP_ENVIRONMENT` / `APP_CONFIG_DIR`
/// 2. 显式参数 `with_environment` / `with_config_dir`
/// 3. 默认值：环境回落到 `local`；配置目录无默认值，缺失时返回
///    [`ConfigError::ConfigDirUnspecified`]
///
/// `APP_CONFIG_DIR` 已设置但为空时按配置错误处理，返回
/// [`ConfigError::EmptyAppConfigDir`]，不会回落到显式参数。
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

    /// 显式指定环境；`APP_ENVIRONMENT` 已设置时以环境变量为准。
    pub fn with_environment(mut self, environment: Environment) -> Self {
        self.environment = Some(environment);
        self
    }

    /// 显式指定配置目录；`APP_CONFIG_DIR` 已设置时以环境变量为准。
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
        match env_var(APP_ENVIRONMENT).map_err(ConfigError::ReadAppEnvironment)? {
            Some(raw) => Environment::try_from(raw.as_str()).map_err(ConfigError::from),
            None => Ok(self.environment.unwrap_or_default()),
        }
    }

    fn resolve_config_dir(&self) -> Result<PathBuf, ConfigError> {
        match env_var(APP_CONFIG_DIR).map_err(ConfigError::ReadAppConfigDir)? {
            Some(raw) => {
                let dir = raw.trim();
                if dir.is_empty() {
                    return Err(ConfigError::EmptyAppConfigDir);
                }

                Ok(PathBuf::from(dir))
            }
            None => self
                .config_dir
                .clone()
                .ok_or(ConfigError::ConfigDirUnspecified),
        }
    }
}

/// 读取环境变量，未设置时返回 `None`。
fn env_var(key: &str) -> Result<Option<String>, std::env::VarError> {
    match std::env::var(key) {
        Ok(value) => Ok(Some(value)),
        Err(std::env::VarError::NotPresent) => Ok(None),
        Err(source) => Err(source),
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
