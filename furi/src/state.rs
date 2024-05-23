use std::sync::Arc;

use crate::config::{read_config_file, AppConfig};
use crate::infrastructure::repository::{connect_redis, connect_postgres, Repository, Databases};

#[derive(Debug)]
pub struct AppState<T:Repository> {
    pub config: AppConfig,
    pub repository: T
}


pub async fn init_state() -> Arc<AppState<Databases>> {

    let conf = match read_config_file("config.toml") {
        Ok(c) => c,
        Err(err) => panic!("Error loading config file: {}", err),
    };

    let conn_red = match connect_redis(&conf.redis).await {
        Ok(r) => r,
        Err(err) => panic!("Error connecting to redis: {}", err)
    };

    let conn_postgres = match connect_postgres(&conf.postgres).await {
        Ok(p) => p,
        Err(err) => panic!("Error connecting to postgres: {}", err),
    };

    Arc::new(AppState { config: conf, repository: Databases { redis: conn_red, postgres: conn_postgres } })
}
