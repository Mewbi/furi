use axum::{
    response::{Response, IntoResponse, Redirect},
    http::StatusCode,
    Json,
    extract::{State, Path}
};

use serde::{Deserialize, Serialize};
use std::sync::Arc;

use url::Url;
use rand::{distributions::Alphanumeric, Rng};

use crate::AppState;

#[derive(Serialize)]
pub struct Message {
    message: String
}

#[derive(Serialize)]
pub struct ShortenerMessage {
    pub original_url: String,
    pub short_url: String
}

pub enum ApiResponse {
    OK(Message),
    Created(ShortenerMessage),
    Redirect(String)
}

impl IntoResponse for ApiResponse {
    fn into_response(self) -> Response {
        match self {
            Self::OK(data) => (StatusCode::OK, Json(data)).into_response(),
            Self::Created(data) => (StatusCode::CREATED, Json(data)).into_response(),
            Self::Redirect(url) => (Redirect::permanent(&url)).into_response()
        }
    }
}

pub enum ApiError {
    BadRequest,
    InternalServerError
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            Self::BadRequest => (StatusCode::BAD_REQUEST).into_response(),
            Self::InternalServerError => (StatusCode::INTERNAL_SERVER_ERROR).into_response()
        }
    }
}

#[derive(Deserialize)]
pub struct UserUrl {
    url: String
}

pub async fn status() -> Result<ApiResponse, ApiError>  {
    Ok(ApiResponse::OK(Message { message: "Ok".to_string() }))
}

pub async fn get_url(
    Path(id): Path<String>,
    State(state): State<Arc<AppState>>
) -> Result<ApiResponse, ApiError>  {

    if id.len() != 7 {
        return Err(ApiError::BadRequest);
    }

    match state.get_url_from_db(&id).await {
        Ok(url) => {
            return Ok(ApiResponse::Redirect(url));
        },

        Err(err) => {
            println!("{}", err);
            return Err(ApiError::InternalServerError);
        }
    };
}

pub async fn create_url(
    State(state): State<Arc<AppState>>,
    Json(data): Json<UserUrl>
) -> Result<ApiResponse, ApiError> {

    let url = Url::parse(&data.url).unwrap();
    let uri = random_uri(7);

    match state.create_url_in_db(url.to_string(), uri).await {
        Ok(result) => {
            return Ok(ApiResponse::Created(result));
        },
        Err(err) => {
            println!("{}", err);
            return Err(ApiError::InternalServerError)
        }
    }
    
}

fn random_uri(size: usize) -> String {
    let uri: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(size)
        .map(char::from)
        .collect();
    uri
}

// TODO: Entender essa parada aq
fn internal_error<E>(err: E) -> ApiError
where
    E: std::error::Error,
{
    ApiError::InternalServerError
}
