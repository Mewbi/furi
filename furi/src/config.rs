use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize, Default, Clone)]
pub struct AppConfig {
    pub name: String,
    pub uri_size: usize,
    pub domain: String,
    pub server: ServerConfig,
    pub redis: RedisConfig,
    pub postgres: PostgresConfig,
    pub redpanda: RedpandaConfig,
    pub geoip: GeoipConfig,
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct ServerConfig {
    pub port: u16,
    pub host: String,
    pub cors_addr: String,
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct RedisConfig {
    pub host: String,
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct PostgresConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub db: String,
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct RedpandaConfig {
    pub host: String,
    pub port: u16,
    pub send_interval: u64
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct GeoipConfig {
    pub country_file: String,
    pub city_file: String,
}

pub fn read_config_file(path: &str) -> Result<AppConfig, Box<dyn std::error::Error>> {
    let config_content = fs::read_to_string(path)?;
    let config: AppConfig = toml::from_str(&config_content)?;
    Ok(config)
}
