use config::Config as ConfigConfig;
use config::ConfigError;
use once_cell::sync::Lazy;
use serde::Deserialize;

use crate::server::Server;

// Global Config
pub static CONFIG: Lazy<Config> = Lazy::new(|| Config::new().unwrap());

#[derive(Debug, Deserialize)]
pub struct Config {
    pub token: String,
    pub server: Server,
}

impl Config {
    pub fn new() -> Result<Self, ConfigError> {
        ConfigConfig::builder()
            .add_source(config::File::with_name("config"))
            .build()?
            .try_deserialize()
    }
}
