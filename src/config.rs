use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize, Default)]
pub struct AppConfig {
    pub name: String,
    pub server: ServerConfig,
    pub redis: RedisConfig,
    pub postgres: PostgresConfig,
}

#[derive(Debug, Deserialize, Default)]
pub struct ServerConfig {
    pub port: u16,
    pub host: String,
}

#[derive(Debug, Deserialize, Default)]
pub struct RedisConfig {
    pub host: String,
}

#[derive(Debug, Deserialize, Default)]
pub struct PostgresConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub db: String,
}

pub fn read_config_file(path: &str) -> Result<AppConfig, Box<dyn std::error::Error>> {
    let config_content = fs::read_to_string(path)?;
    let config: AppConfig = toml::from_str(&config_content)?;
    Ok(config)
}
