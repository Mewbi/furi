use std::sync::Arc;

use crate::config::{read_config_file, AppConfig};
use crate::infrastructure::repository::{connect_timescale, Databases, Repository};

#[derive(Debug)]
pub struct AppState<T: Repository> {
    pub config: AppConfig,
    pub repository: T,
}

pub async fn init_state() -> Arc<AppState<Databases>> {
    let conf = match read_config_file("config.toml") {
        Ok(c) => c,
        Err(err) => panic!("Error loading config file: {}", err),
    };

    let timescale = match connect_timescale(&conf.timescale).await {
        Ok(c) => c,
        Err(err) => panic!("Error connecting to TimescaleDB: {}", err),
    };

    Arc::new(AppState {
        config: conf,
        repository: Databases { timescale },
    })
}
