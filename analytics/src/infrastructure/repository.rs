use clickhouse::{Client, Row};
use serde::{Deserialize, Serialize};
use std::future::Future;
use std::{fs, io};
use tokio::io::AsyncReadExt;

use crate::api::handlers::AnalyticsRequest;
use crate::config::ClickhouseConfig;

pub trait Repository {
    fn get_uri_access_count(
        &self,
        id: String,
        info: AnalyticsRequest,
    ) -> impl Future<Output = Result<Vec<UriRequestTimeSeries>, Box<dyn std::error::Error>>> + Send;
    fn get_uri_device_count(
        &self,
        id: String,
        info: AnalyticsRequest,
    ) -> impl Future<Output = Result<Vec<UriDeviceCount>, Box<dyn std::error::Error>>> + Send;
    fn get_uri_country_count(
        &self,
        id: String,
        info: AnalyticsRequest,
    ) -> impl Future<Output = Result<Vec<UriCountryCount>, Box<dyn std::error::Error>>> + Send;
}

pub struct Databases {
    pub clickhouse: Client,
}

#[derive(Debug, Serialize, Row, Deserialize)]
pub struct UriMetrics {
    pub uri: String,
    pub req_total: u64,
    pub req_time_series: Vec<UriRequestTimeSeries>,
    pub device_count: Vec<UriDeviceCount>,
    pub country_access: Vec<UriCountryCount>,
}

#[derive(Debug, Serialize, Row, Deserialize)]
pub struct UriRequestTimeSeries {
    pub date: u32,
    pub count: u64,
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

fn get_table(aggregation: String) -> Result<String, String> {
    match aggregation.as_str() {
        "1min" => Ok("furi.analytics_1min".to_string()),
        "1hour" => Ok("furi.analytics_1hour".to_string()),
        "1day" => Ok("furi.analytics_1day".to_string()),
        _ => Err("Invalid aggregation table".to_string()),
    }
}

impl Repository for Databases {
    async fn get_uri_access_count(
        &self,
        uri: String,
        info: AnalyticsRequest,
    ) -> Result<Vec<UriRequestTimeSeries>, Box<dyn std::error::Error>> {
        let table = match get_table(info.aggregation) {
            Ok(t) => t,
            Err(err) => return Err(Box::from(err)),
        };

        let query = format!(
            "SELECT date, sum(count) AS count
                            FROM {}
                            WHERE uri = ?
                            AND date >= ?
                            AND date <= ?
                            GROUP BY date
                            ORDER BY date",
            table
        );

        let records = self
            .clickhouse
            .query(query.as_str())
            .bind(&uri)
            .bind(info.from)
            .bind(info.to)
            .fetch_all::<UriRequestTimeSeries>()
            .await?;

        Ok(records)
    }

    async fn get_uri_device_count(
        &self,
        uri: String,
        info: AnalyticsRequest,
    ) -> Result<Vec<UriDeviceCount>, Box<dyn std::error::Error>> {
        let table = match get_table(info.aggregation) {
            Ok(t) => t,
            Err(err) => return Err(Box::from(err)),
        };

        let query = format!(
            "SELECT device_type, sum(count) AS count
                            FROM {}
                            WHERE uri = ?
                            AND date >= ?
                            AND date <= ?
                            GROUP BY device_type
                            ORDER BY device_type",
            table
        );

        let records = self
            .clickhouse
            .query(query.as_str())
            .bind(&uri)
            .bind(info.from)
            .bind(info.to)
            .fetch_all::<UriDeviceCount>()
            .await?;

        Ok(records)
    }

    async fn get_uri_country_count(
        &self,
        uri: String,
        info: AnalyticsRequest,
    ) -> Result<Vec<UriCountryCount>, Box<dyn std::error::Error>> {
        let table = match get_table(info.aggregation) {
            Ok(t) => t,
            Err(err) => return Err(Box::from(err)),
        };

        let query = format!(
            "SELECT country, sum(count) AS count
                            FROM {}
                            WHERE uri = ?
                            AND date >= ?
                            AND date <= ?
                            GROUP BY country
                            ORDER BY country",
            table
        );

        let records = self
            .clickhouse
            .query(query.as_str())
            .bind(&uri)
            .bind(info.from)
            .bind(info.to)
            .fetch_all::<UriCountryCount>()
            .await?;

        Ok(records)
    }
}

pub async fn connect_clickhouse(
    config: &ClickhouseConfig,
) -> Result<clickhouse::Client, Box<dyn std::error::Error>> {
    let address = format!("http://{}:{}", config.host, config.port);
    let client = Client::default().with_url(address);
    migrate_database(&client).await.unwrap();
    Ok(client)
}

pub async fn migrate_database(
    client: &clickhouse::Client,
) -> Result<(), Box<dyn std::error::Error>> {
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
