use serde::Serialize;

use axum::{
    Json,
    response::{Response, IntoResponse},
    http::StatusCode
};

use crate::infrastructure::repository::UriMetrics;

#[derive(Serialize)]
pub struct ShortenerMessage {
    pub original_url: String,
    pub short_url: String
}

pub enum ApiResponse {
    OK(OkTypes),
}

pub enum OkTypes {
    Empty,
    Metrics(UriMetrics)
}

#[derive(Serialize)]
pub struct Message {
    pub message: String
}

impl IntoResponse for ApiResponse {
    fn into_response(self) -> Response {
        match self {
            ApiResponse::OK(msg) => {
                match msg {
                    OkTypes::Empty => (StatusCode::OK).into_response(),
                    OkTypes::Metrics(metrics) => (StatusCode::OK, Json(metrics)).into_response()
                }
            }
        }
    }
}

pub enum ApiError {
    BadRequest(Option<String>),
    InternalServerError,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            Self::BadRequest(data) => {
                match data {
                    Some(m) => (StatusCode::BAD_REQUEST, Json(Message{ message: m})).into_response(),
                    None => (StatusCode::BAD_REQUEST).into_response(),
                }
            },
            Self::InternalServerError => (StatusCode::INTERNAL_SERVER_ERROR).into_response()
        }
    }
}
