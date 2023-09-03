use config::Config as ConfigConfig;
use config::ConfigError;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub container: String,
    pub host: String,
    pub query_port: u16,
    pub rcon_port: u16,
    pub rcon_password: String,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub token: String,
    pub server: ServerConfig,
}

impl Config {
    pub fn new() -> Result<Self, ConfigError> {
        ConfigConfig::builder()
            .add_source(config::File::with_name("config"))
            .build()?
            .try_deserialize()
    }
}
