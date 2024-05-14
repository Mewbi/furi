use axum::{
    response::{Response, IntoResponse, Redirect},
    http::StatusCode,
    Json,
    extract::{State, Path}
};

use serde::{Deserialize, Serialize};
use std::sync::Arc;
use redis::AsyncCommands;

use url::Url;
use rand::{distributions::Alphanumeric, Rng};

use crate::AppState;

#[derive(Serialize)]
pub struct Message {
    message: String
}

#[derive(Serialize)]
pub struct ShortenerMessage {
    original_url: String,
    short_url: String
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

    println!("{:?}", state);
    println!("{}", id);

    let mut conn = state.redis.get().await.map_err(internal_error)?;
    let result: String = conn.get(&id).await.unwrap_or_else(|_| {
        String::new()
    });
    if result.len() != 0 {
        return Ok(ApiResponse::Redirect(result));
    }

    let conn = state.postgres.get().await.map_err(internal_error)?;
    let row = conn
        .query_one("SELECT url FROM uris WHERE uri = $1", &[&id])
        .await
        .map_err(internal_error)?;
    let url: String = row.try_get(0).map_err(internal_error)?;

    let mut conn_red = state.redis.get().await.unwrap();
    conn_red.set_ex::<&str, &str, ()>(&id, &url, 3600).await.unwrap();

    Ok(ApiResponse::Redirect(url))
}

pub async fn create_url(
    State(state): State<Arc<AppState>>,
    Json(data): Json<UserUrl>
) -> Result<ApiResponse, ApiError> {
    println!("{:?}", state);
    println!("{}", data.url);

    let url = Url::parse(&data.url).unwrap();

    let uri = random_uri(7);

    let conn = state.postgres.get().await.map_err(internal_error)?;
    let sql = "INSERT INTO uris (uri, url, created_at) VALUES ($1, $2, now())";
    let stmt = conn.prepare(sql).await.map_err(internal_error)?;

    conn.execute(&stmt, &[&uri, &url.to_string()]).await.map_err(internal_error)?;

    let short_url = format!("http://{}:{}/{}", state.server.host, state.server.port, uri);
    let msg = ShortenerMessage { original_url: url.to_string(), short_url: short_url };
    Ok(ApiResponse::Created(msg))
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
