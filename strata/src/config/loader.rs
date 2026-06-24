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
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_environment(mut self, environment: Environment) -> Self {
        self.environment = Some(environment);
        self
    }

    pub fn with_config_dir(mut self, dir: impl Into<PathBuf>) -> Self {
        self.config_dir = Some(dir.into());
        self
    }

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
