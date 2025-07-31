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
    http_host: String,
    http_port: u16,
}

impl Config {
    pub fn home() -> PathBuf {
        let home_dir = std::env::var("WORKSPACE_HOME")
            .or_else(|_| std::env::var("CARGO_MANIFEST_DIR"))
            .unwrap_or_else(|_| "".to_string());

        std::path::Path::new(&home_dir).to_path_buf()
    }

    pub fn path() -> PathBuf {
        Self::home().join("config.toml")
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

    pub fn http_host(&self) -> &str {
        &self.http_host
    }

    pub fn http_port(&self) -> u16 {
        self.http_port
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
            http_host: "0.0.0.0".to_string(),
            http_port: 8080,
        }
    }
}
