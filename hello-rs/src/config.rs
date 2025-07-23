use std::error::Error;
use std::path::{Path, PathBuf};

use figment::Figment;
use figment::providers::{Env, Format, Serialized, Toml};
use serde::{Deserialize, Serialize};
use tracing::level_filters::LevelFilter;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    log_level: LogLevel,
    http_addr: String,
}

impl Config {
    pub fn path() -> PathBuf {
        let config_dir = std::env::var("CONFIG_HOME");
        let cargo_dir = std::env::var("CARGO_MANIFEST_DIR");
        let root_dir = config_dir.or(cargo_dir).unwrap_or("".to_string());

        std::path::Path::new(&root_dir).join("config.toml")
    }

    pub fn load(path: &Path) -> Result<Self, Box<dyn Error>> {
        let figment = Figment::new()
            .merge(Serialized::defaults(Config::default()))
            .merge(Toml::file(path))
            .merge(Env::prefixed("WORKSPACE_"));
        let config: Self = figment.extract()?;

        Ok(config)
    }

    pub fn log_level(&self) -> LogLevel {
        self.log_level.clone()
    }

    pub fn http_addr(&self) -> String {
        self.http_addr.clone()
    }
}

impl From<LogLevel> for LevelFilter {
    fn from(value: LogLevel) -> Self {
        match value {
            LogLevel::Trace => LevelFilter::TRACE,
            LogLevel::Debug => LevelFilter::DEBUG,
            LogLevel::Info => LevelFilter::INFO,
            LogLevel::Warn => LevelFilter::WARN,
            LogLevel::Error => LevelFilter::ERROR,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            log_level: LogLevel::Info,
            http_addr: "0.0.0.0:8080".to_string(),
        }
    }
}
