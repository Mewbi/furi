use tokio::sync::mpsc::Sender;
use std::sync::Arc;

use crate::config::{read_config_file, AppConfig};
use crate::infrastructure::{
    repository::{connect_redis, connect_postgres, read_geoip, Repository, Databases},
    redpanda::UserData
};

#[derive(Debug)]
pub struct AppState<T:Repository> {
    pub config: AppConfig,
    pub repository: T,
    pub analytics: Sender<UserData>
}

pub async fn init_state(tx: Sender<UserData>) -> Arc<AppState<Databases>> {

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
    
    let geoip_db = match read_geoip(&conf.geoip).await {
        Ok(p) => p,
        Err(err) => panic!("Error reading geoip db: {}", err),
    };

    Arc::new(AppState { 
        config: conf, 
        repository: Databases { 
            redis: conn_red, 
            postgres: conn_postgres, 
            geoip: geoip_db 
        },
        analytics: tx
    })
}
