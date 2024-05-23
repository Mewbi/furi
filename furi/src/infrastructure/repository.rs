use bb8::Pool;
use bb8_redis::RedisConnectionManager;
use bb8_redis::redis::AsyncCommands;
use bb8_redis::bb8;
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::NoTls;

use crate::config::{RedisConfig, PostgresConfig};

pub trait Repository {
    fn get_url(&self, id: String) -> impl std::future::Future<Output = Result<String, Box<dyn std::error::Error>>> + Send;
    fn create_url(&self, url: String, uri: String) -> impl std::future::Future<Output = Result<(), Box<dyn std::error::Error>>> + Send;
}

#[derive(Debug)]
pub struct Databases {
    pub redis: Pool<RedisConnectionManager>,
    pub postgres: Pool<PostgresConnectionManager<NoTls>>
}

impl Repository for Databases {

    async fn get_url(&self, id: String) -> Result<String, Box<dyn std::error::Error>> {
        let mut redis_conn = self.redis.get().await?;
        let result: String = redis_conn.get(&id).await.unwrap_or_else(|_| String::new());
        
        if result.len() != 0 {
            return Ok(result);
        }
        
        let pg_conn = self.postgres.get().await?;
        let row = pg_conn.query_one("SELECT url FROM uris WHERE uri = $1", &[&id]).await?;
        let url: String = row.try_get(0)?;
        
        let mut redis_conn = self.redis.get().await?;
        redis_conn.set_ex::<&str, &str, ()>(id.as_str(), &url, 3600).await?;
        
        Ok(url)
    }

    async fn create_url(&self, url: String, uri: String) -> Result<(), Box<dyn std::error::Error>> {

        let pg_conn = self.postgres.get().await?;
        let sql = "INSERT INTO uris (uri, url, created_at) VALUES ($1, $2, now())";
        let stmt = pg_conn.prepare(sql).await?;

        pg_conn.execute(&stmt, &[&uri, &url.to_string()]).await?;
        Ok(())
    }
}

pub async fn connect_redis(config: &RedisConfig) -> Result<Pool<RedisConnectionManager>,Box<dyn std::error::Error>> {
    let manager_redis = RedisConnectionManager::new(format!("redis://{}", config.host))?;
    let pool_redis = bb8::Pool::builder().build(manager_redis).await?;
    Ok(pool_redis)
}

pub async fn connect_postgres(config: &PostgresConfig) -> Result<Pool<PostgresConnectionManager<NoTls>>,Box<dyn std::error::Error>> {
    let address: String = format!("host={} port={} user={} password={} dbname={}", config.host, config.port, config.user, config.password, config.db);
    let manager_postgres = PostgresConnectionManager::new_from_stringlike(address, NoTls)?;
    let pool_postgres = Pool::builder().build(manager_postgres).await?;
    migrate_database(&pool_postgres).await?;
    Ok(pool_postgres)
}

pub async fn migrate_database(pool: &Pool<PostgresConnectionManager<NoTls>>) -> Result<(), Box<dyn std::error::Error>> {
    let client = pool.get().await?;
    let migration_query = include_str!("./migrations/schema.sql");
    client.batch_execute(migration_query).await?;
    Ok(())
}
