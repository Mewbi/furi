use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPoolOptions;
use sqlx::{FromRow, PgPool};
use std::future::Future;
use std::{fs, io};
use tokio::io::AsyncReadExt;

use crate::api::handlers::AnalyticsRequest;
use crate::config::TimescaleConfig;

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
    pub timescale: PgPool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UriMetrics {
    pub uri: String,
    pub req_total: i64,
    pub req_time_series: Vec<UriRequestTimeSeries>,
    pub device_count: Vec<UriDeviceCount>,
    pub country_access: Vec<UriCountryCount>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct UriRequestTimeSeries {
    pub date: i64,
    pub count: i64,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct UriDeviceCount {
    pub device_type: String,
    pub count: i64,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct UriCountryCount {
    pub country: String,
    pub count: i64,
}

fn get_table(aggregation: &str) -> Result<&'static str, String> {
    match aggregation {
        "1min" => Ok("analytics_1min"),
        "1hour" => Ok("analytics_1hour"),
        "1day" => Ok("analytics_1day"),
        _ => Err("Invalid aggregation table".to_string()),
    }
}

impl Repository for Databases {
    async fn get_uri_access_count(
        &self,
        uri: String,
        info: AnalyticsRequest,
    ) -> Result<Vec<UriRequestTimeSeries>, Box<dyn std::error::Error>> {
        let table = match get_table(&info.aggregation) {
            Ok(t) => t,
            Err(err) => return Err(Box::from(err)),
        };

        let query = format!(
            "SELECT EXTRACT(EPOCH FROM bucket)::BIGINT AS date, SUM(count)::BIGINT AS count
             FROM {}
             WHERE uri = $1
               AND bucket >= to_timestamp($2)
               AND bucket <= to_timestamp($3)
             GROUP BY bucket
             ORDER BY bucket",
            table
        );

        let records = sqlx::query_as::<_, UriRequestTimeSeries>(&query)
            .bind(&uri)
            .bind(info.from as f64)
            .bind(info.to as f64)
            .fetch_all(&self.timescale)
            .await?;

        Ok(records)
    }

    async fn get_uri_device_count(
        &self,
        uri: String,
        info: AnalyticsRequest,
    ) -> Result<Vec<UriDeviceCount>, Box<dyn std::error::Error>> {
        let table = match get_table(&info.aggregation) {
            Ok(t) => t,
            Err(err) => return Err(Box::from(err)),
        };

        let query = format!(
            "SELECT device_type, SUM(count)::BIGINT AS count
             FROM {}
             WHERE uri = $1
               AND bucket >= to_timestamp($2)
               AND bucket <= to_timestamp($3)
             GROUP BY device_type
             ORDER BY device_type",
            table
        );

        let records = sqlx::query_as::<_, UriDeviceCount>(&query)
            .bind(&uri)
            .bind(info.from as f64)
            .bind(info.to as f64)
            .fetch_all(&self.timescale)
            .await?;

        Ok(records)
    }

    async fn get_uri_country_count(
        &self,
        uri: String,
        info: AnalyticsRequest,
    ) -> Result<Vec<UriCountryCount>, Box<dyn std::error::Error>> {
        let table = match get_table(&info.aggregation) {
            Ok(t) => t,
            Err(err) => return Err(Box::from(err)),
        };

        let query = format!(
            "SELECT country, SUM(count)::BIGINT AS count
             FROM {}
             WHERE uri = $1
               AND bucket >= to_timestamp($2)
               AND bucket <= to_timestamp($3)
             GROUP BY country
             ORDER BY country",
            table
        );

        let records = sqlx::query_as::<_, UriCountryCount>(&query)
            .bind(&uri)
            .bind(info.from as f64)
            .bind(info.to as f64)
            .fetch_all(&self.timescale)
            .await?;

        Ok(records)
    }
}

pub async fn connect_timescale(
    config: &TimescaleConfig,
) -> Result<PgPool, Box<dyn std::error::Error>> {
    let connection_string = format!(
        "postgres://{}:{}@{}:{}/{}",
        config.user, config.password, config.host, config.port, config.db
    );

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&connection_string)
        .await?;

    migrate_database(&pool).await?;
    Ok(pool)
}

pub async fn migrate_database(pool: &PgPool) -> Result<(), Box<dyn std::error::Error>> {
    let mut entries = fs::read_dir("./src/infrastructure/migrations")?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;
    entries.sort();

    for path in entries {
        if path.is_file() {
            let mut file = tokio::fs::File::open(&path).await?;
            let mut sql = String::new();
            file.read_to_string(&mut sql).await?;
            sqlx::raw_sql(&sql).execute(pool).await?;
            println!("Ran migration: {:?}", path);
        }
    }
    Ok(())
}
