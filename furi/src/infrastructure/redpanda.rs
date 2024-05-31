use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::ClientConfig;
use chrono::DateTime;
use serde::Serialize;
use tokio::sync::mpsc::Receiver;
use tokio::time::{self, Duration, timeout};
use std::collections::VecDeque;
use serde_json;
use chrono::prelude::*;
use chrono::serde::ts_seconds;

#[derive(Debug, Serialize)]
pub struct UserData {
    #[serde(with = "ts_seconds")]
    pub date: DateTime<Utc>,
    pub uri: String,
    pub ip: String,
    pub user_agent: String,
    pub country: String,
}

pub async fn handle_user_data(mut rx: Receiver<UserData>) {
    let mut buffer = VecDeque::new();
    let mut interval = time::interval(Duration::from_secs(10));

    loop {
        tokio::select! {
            Some(data) = rx.recv() => {
                buffer.push_back(data);

                if buffer.len() >= 1000 {
                    send_data(buffer.drain(..).collect()).await;
                }
            }
            _ = interval.tick() => {
                if !buffer.is_empty() {
                    send_data(buffer.drain(..).collect()).await;
                }
            }
        }
    }
}

async fn send_data(data: Vec<UserData>) {
    let producer: FutureProducer = match ClientConfig::new()
        .set("bootstrap.servers", "localhost:9092")
        .create() {
            Ok(p) => p,
            Err(err) => {
                println!("producer creation error: {}", err);
                return;
            }
        };

    for user_data in data {
        let json_data = match serde_json::to_string(&user_data) {
            Ok(json) => json,
            Err(err) => {
                println!("error encoding json: {}", err);
                continue;
            }
        };

        let record = FutureRecord::to("analytics")
            .payload(&json_data)
            .key(&user_data.ip);

        match timeout(Duration::from_secs(5), producer.send(record, Duration::from_secs(0))).await {
            Ok(Ok(delivery)) => {
                println!("sended: {:?}", delivery);
            }
            Ok(Err((e, _))) => {
                eprintln!("error delivering message: {:?}", e);
            }
            Err(_) => {
                eprintln!("delivery timeout");
            }
        }
    }

}
