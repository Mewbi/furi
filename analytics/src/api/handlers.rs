use super::response::{ApiError, ApiResponse, OkTypes};
use axum::extract::{Json, Path, State};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::infrastructure::repository::{Repository, UriMetrics};
use crate::state::AppState;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AnalyticsRequest {
    pub aggregation: String,
    pub from: u64,
    pub to: u64,
}

const LIMIT_MIN: u64 = 86400;
const LIMIT_HOUR: u64 = 86400 * 7;
const LIMIT_DAY: u64 = 86400 * 7 * 31;

impl AnalyticsRequest {
    fn validate(&self) -> Result<(), String> {
        if !["1min", "1hour", "1day"].contains(&self.aggregation.as_str()) {
            return Err("Invalid aggregation value".to_string());
        }

        if self.from > self.to {
            return Err("'from' timestamp must be less than 'to' timestamp".to_string());
        }

        let diff = self.to - self.from;
        match self.aggregation.as_str() {
            "1min" if diff > LIMIT_MIN => {
                return Err("Period too big, select a shorter one".to_string())
            }
            "1hour" if diff > LIMIT_HOUR => {
                return Err("Period too big, select a shorter one".to_string())
            }
            "1day" if diff > LIMIT_DAY => {
                return Err("Period too big, select a shorter one".to_string())
            }
            _ => {}
        }

        Ok(())
    }
}

pub async fn status() -> Result<ApiResponse, ApiError> {
    Ok(ApiResponse::OK(OkTypes::Empty))
}

pub async fn get_uri_metrics<T: Repository + Send + Sync>(
    State(state): State<Arc<AppState<T>>>,
    Path(id): Path<String>,
    Json(payload): Json<AnalyticsRequest>,
) -> Result<ApiResponse, ApiError> {
    match payload.validate() {
        Ok(_) => {}
        Err(reason) => return Err(ApiError::BadRequest(Some(reason))),
    }

    let uri_access_count = match state
        .repository
        .get_uri_access_count(id.clone(), payload.clone())
        .await
    {
        Ok(metrics) => metrics,
        Err(err) => {
            println!("error getting uri access count metrics: {}", err);
            return Err(ApiError::InternalServerError);
        }
    };

    let uri_device_count = match state
        .repository
        .get_uri_device_count(id.clone(), payload.clone())
        .await
    {
        Ok(metrics) => metrics,
        Err(err) => {
            println!("error getting uri device count metrics: {}", err);
            return Err(ApiError::InternalServerError);
        }
    };

    let uri_country_count = match state
        .repository
        .get_uri_country_count(id.clone(), payload.clone())
        .await
    {
        Ok(metrics) => metrics,
        Err(err) => {
            println!("error getting uri country count metrics: {}", err);
            return Err(ApiError::InternalServerError);
        }
    };

    let mut total: u64 = 0;
    for r in uri_access_count.iter() {
        total += r.count;
    }

    let metrics = UriMetrics {
        uri: id,
        req_total: total,
        req_time_series: uri_access_count,
        device_count: uri_device_count,
        country_access: uri_country_count,
    };
    Ok(ApiResponse::OK(OkTypes::Metrics(metrics)))
}
