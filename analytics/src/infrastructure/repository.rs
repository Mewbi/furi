use clickhouse::Client;
use chrono::DateTime;
use chrono::prelude::*;
use tokio::io::AsyncReadExt;
use std::future::Future;

use crate::config::ClickhouseConfig;

pub trait Repository {
    fn get_uri_metrics(&self, id: String) -> impl Future<Output = Result<Vec<UriMetrics>, Box<dyn std::error::Error>>> + Send;
}

pub struct Databases {
    pub clickhouse: clickhouse::Client,
}

#[derive(Debug)]
pub struct UriMetrics  {
    pub date: DateTime<Utc>,
    pub ip: String,
    pub user_agent: String,
    pub country: String,
}

impl Repository for Databases {

    async fn get_uri_metrics(&self, uri: String) -> Result<Vec<UriMetrics>, Box<dyn std::error::Error>> {
        let query = format!("SELECT date, ip, user_agent, country FROM furi.analytics_raw WHERE uri = {}", uri);
        let rows = self
            .clickhouse
            .query(&query)
            .fetch_all::<(u64, String, String, String)>()
            .await?;
    

        let records = rows
            .into_iter()
            .map(|(timestamp, ip, user_agent, country)| {
            // Convert timestamp (u64) to DateTime<Utc>
            let naive_datetime = DateTime::from_timestamp(timestamp as i64, 0).unwrap().naive_utc();
            let datetime: DateTime<Utc> = DateTime::from_naive_utc_and_offset(naive_datetime, Utc);

            UriMetrics {
                date: datetime,
                ip,
                user_agent,
                country,
            }
        }).collect();

        Ok(records)
    }

}

pub async fn connect_clickhouse(config: &ClickhouseConfig) -> Result<clickhouse::Client,Box<dyn std::error::Error>> {
    let address = format!("http://{}:{}", config.host, config.port);
    let clickhouse_client = Client::default()
        .with_url(address);
    Ok(clickhouse_client)
}

pub async fn migrate_database(client: &clickhouse::Client) -> Result<(), Box<dyn std::error::Error>> {

    let mut entries = tokio::fs::read_dir("./migrations/").await?;
    while let Some(entry) = entries.next_entry().await? {
        let path = entry.path();
        if path.is_file() {
            let mut file = tokio::fs::File::open(&path).await?;
            let mut sql = String::new();
            file.read_to_string(&mut sql).await?;
            client.query(&sql).execute().await?;
            println!("Ran migration: {:?}", path);
        }
    }
    Ok(())
}
