use serde::Serialize;

use axum::{
    Json,
    response::{Response, IntoResponse, Redirect},
    http::StatusCode
};

#[derive(Serialize)]
pub struct Message {
    pub message: String
}

#[derive(Serialize)]
pub struct ShortenerMessage {
    pub original_url: String,
    pub short_url: String
}

pub enum ApiResponse {
    OK(Option<String>),
    Created(ShortenerMessage),
    Redirect(String)
}

impl IntoResponse for ApiResponse {
    fn into_response(self) -> Response {
        match self {
            Self::OK(data) => {
                match data {
                    Some(m) => (StatusCode::OK, Json(Message{ message: m})).into_response(),
                    None => (StatusCode::OK).into_response()
                }
            },
            Self::Created(data) => (StatusCode::CREATED, Json(data)).into_response(),
            Self::Redirect(url) => (Redirect::permanent(&url)).into_response()
        }
    }
}

pub enum ApiError {
    BadRequest(Option<String>),
    InternalServerError,
    Redirect(String)
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
            Self::InternalServerError => (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
            Self::Redirect(url) => (Redirect::permanent(&url)).into_response()
        }
    }
}
