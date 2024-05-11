use axum::{
    response::{Response, IntoResponse},
    routing::{get, post},
    http::{StatusCode},
    Json,
    Router,
    extract::{State, Path}
};
use serde::{Deserialize, Serialize};
use tower_http::trace::TraceLayer;
use std::sync::Arc;

#[derive(Serialize)]
struct Message {
    message: String
}

enum ApiResponse {
    OK,
    Created,
    JsonData(Vec<Message>),
}

impl IntoResponse for ApiResponse {
    fn into_response(self) -> Response {
        match self {
            Self::OK => (StatusCode::OK).into_response(),
            Self::Created => (StatusCode::CREATED).into_response(),
            Self::JsonData(data) => (StatusCode::OK, Json(data)).into_response()
        }
    }
}

enum ApiError {
    BadRequest,
    Forbidden,
    Unauthorised,
    InternalServerError
}

#[derive(Debug)]
struct AppState {
    // will have db connection
    name: String
}

#[derive(Deserialize)]
struct Url {
    url: String
}

fn init_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/:id", get(get_url))
        .route("/url", post(create_url))
        .with_state(state)
        .layer(TraceLayer::new_for_http())
}

fn init_state() -> Arc<AppState> {
    Arc::new(AppState { name: "Databases poggers".to_string() } )
}

async fn get_url(
    Path(id): Path<String>,
    State(state): State<Arc<AppState>>
) -> // Result<ApiResponse, ApiError> 
    &'static str {
    println!("{:?}", state);
    println!("{}", id);
    // if state.name == "err" {
    //     return Err(ApiError::InternalServerError);
    // }
    // Ok(ApiResponse::OK)
    "Tome"
}

async fn create_url(
    State(state): State<Arc<AppState>>,
    Json(data): Json<Url>
) -> // Result<ApiResponse, ApiError>
    &'static str {
    println!("{:?}", state);
    println!("{}", data.url);
    //Ok(ApiResponse::Created)
    "Hello"
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_max_level(tracing::Level::DEBUG).init();

    let state = init_state();
    let app = init_router(state);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

