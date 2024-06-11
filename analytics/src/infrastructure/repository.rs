use clickhouse::{Client, Row};
use tokio::io::AsyncReadExt;
use std::future::Future;
use std::{io,fs};
use serde::{Serialize, Deserialize};

use crate::config::ClickhouseConfig;

pub trait Repository {
    fn get_uri_metrics(&self, id: String) -> impl Future<Output = Result<UriMetrics, Box<dyn std::error::Error>>> + Send;
}

pub struct Databases {
    pub clickhouse: Client,
}

#[derive(Debug, Serialize, Row, Deserialize)]
pub struct UriMetrics  {
    pub uri: String,
    pub req_total: u64,
    pub req_time_series: Vec<UriRequestTimeSeries>,
    pub device_count: Vec<UriDeviceCount>,
    pub country_access: Vec<UriCountryCount>
}

#[derive(Debug, Serialize, Row, Deserialize)]
pub struct UriRequestTimeSeries {
    pub date: u32,
    pub count: u64
}

#[derive(Debug, Serialize, Row, Deserialize)]
pub struct UriDeviceCount {
    pub device_type: String,
    pub count: u64,
}

#[derive(Debug, Serialize, Row, Deserialize)]
pub struct UriCountryCount {
    pub country: String,
    pub count: u64,
}

impl Repository for Databases {

    async fn get_uri_metrics(&self, uri: String) -> Result<UriMetrics, Box<dyn std::error::Error>> {
        let query = format!("SELECT date, sum(count) AS count
                            FROM furi.analytics_1min
                            WHERE uri = ?
                            GROUP BY date
                            ORDER BY date");

        let records = self.clickhouse
            .query(&query)
            .bind(&uri)
            .fetch_all::<UriRequestTimeSeries>().await?;

        let mut total: u64 = 0;
        for r in records.iter() {
            total += r.count;
        }
        
        let query2 = format!("SELECT device_type, sum(count) AS count
                            FROM furi.analytics_1min
                            WHERE uri = ?
                            GROUP BY device_type
                            ORDER BY device_type");

        let records2 = self.clickhouse
            .query(&query2)
            .bind(&uri)
            .fetch_all::<UriDeviceCount>().await?;

        let query3 = format!("SELECT country, sum(count) AS count
                            FROM furi.analytics_1min
                            WHERE uri = ?
                            GROUP BY country
                            ORDER BY country");

        let records3 = self.clickhouse
            .query(&query3)
            .bind(&uri)
            .fetch_all::<UriCountryCount>().await?;
        
        let uri_metrics = UriMetrics { 
            uri: uri.clone(),
            req_total: total,
            req_time_series: records,
            device_count: records2,
            country_access: records3
        };
        Ok(uri_metrics)
    }

}

pub async fn connect_clickhouse(config: &ClickhouseConfig) -> Result<clickhouse::Client, Box<dyn std::error::Error>> {
    let address = format!("http://{}:{}", config.host, config.port);
    let client = Client::default().with_url(address);
    migrate_database(&client).await.unwrap();
    Ok(client)
}

pub async fn migrate_database(client: &clickhouse::Client) -> Result<(), Box<dyn std::error::Error>> {

    let mut entries = fs::read_dir("./src/infrastructure/migrations")?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;
    entries.sort();

    for path in entries {
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
