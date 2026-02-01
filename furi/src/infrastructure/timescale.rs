use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::collections::VecDeque;
use tokio::sync::mpsc::Receiver;
use tokio::time::{self, Duration};

use crate::config::TimescaleConfig;

#[derive(Debug, Serialize)]
pub struct UserData {
    pub date: DateTime<Utc>,
    pub uri: String,
    pub ip: String,
    pub user_agent: String,
    pub country: String,
}

pub async fn create_pool(config: &TimescaleConfig) -> Result<PgPool, sqlx::Error> {
    let connection_string = format!(
        "postgres://{}:{}@{}:{}/{}",
        config.user, config.password, config.host, config.port, config.db
    );

    PgPoolOptions::new()
        .max_connections(5)
        .connect(&connection_string)
        .await
}

pub async fn handle_user_data(
    mut rx: Receiver<UserData>,
    pool: PgPool,
    buffer_size: usize,
    flush_interval: u64,
) {
    let mut buffer: VecDeque<UserData> = VecDeque::new();
    let mut interval = time::interval(Duration::from_secs(flush_interval));

    loop {
        tokio::select! {
            Some(data) = rx.recv() => {
                buffer.push_back(data);

                if buffer.len() >= buffer_size {
                    flush_buffer(&pool, &mut buffer).await;
                }
            }
            _ = interval.tick() => {
                if !buffer.is_empty() {
                    flush_buffer(&pool, &mut buffer).await;
                }
            }
        }
    }
}

async fn flush_buffer(pool: &PgPool, buffer: &mut VecDeque<UserData>) {
    let data: Vec<UserData> = buffer.drain(..).collect();

    if data.is_empty() {
        return;
    }

    // Build batch insert query
    let mut query = String::from(
        "INSERT INTO analytics_raw (time, uri, ip, user_agent, country) VALUES ",
    );

    let mut values: Vec<String> = Vec::with_capacity(data.len());
    for (i, _) in data.iter().enumerate() {
        let offset = i * 5;
        values.push(format!(
            "(${}, ${}, ${}, ${}, ${})",
            offset + 1,
            offset + 2,
            offset + 3,
            offset + 4,
            offset + 5
        ));
    }
    query.push_str(&values.join(", "));

    let mut query_builder = sqlx::query(&query);
    for user_data in &data {
        query_builder = query_builder
            .bind(&user_data.date)
            .bind(&user_data.uri)
            .bind(&user_data.ip)
            .bind(&user_data.user_agent)
            .bind(&user_data.country);
    }

    match query_builder.execute(pool).await {
        Ok(result) => {
            println!("Inserted {} analytics records", result.rows_affected());
        }
        Err(err) => {
            eprintln!("Error inserting analytics data: {}", err);
        }
    }
}
