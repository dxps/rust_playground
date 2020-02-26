use config::ConfigError;
use serde::Deserialize;

/// Server specific configuration.
#[derive(Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: i32,
}

/// Application specific configuration.
#[derive(Deserialize)]
pub struct Config {
    pub server: ServerConfig,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        let mut cfg = config::Config::new();
        cfg.merge(config::Environment::new())?;
        cfg.try_into()
    }
}
