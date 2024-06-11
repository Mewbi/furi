use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize, Default)]
pub struct AppConfig {
    pub name: String,
    pub server: ServerConfig,
    pub clickhouse: ClickhouseConfig
}

#[derive(Debug, Deserialize, Default)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub cors_addr: String
}

#[derive(Debug, Deserialize, Default)]
pub struct ClickhouseConfig {
    pub host: String,
    pub port: u16
}


pub fn read_config_file(path: &str) -> Result<AppConfig, Box<dyn std::error::Error>> {
    let config_content = fs::read_to_string(path)?;
    let config: AppConfig = toml::from_str(&config_content)?;
    Ok(config)
}
