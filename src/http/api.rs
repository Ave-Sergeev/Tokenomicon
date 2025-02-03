use axum::http::{StatusCode, Uri};
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::Router;

// TODO: Возвращать Ok(200)
async fn health_check() -> &'static str {
    "Alive and well, eagle!"
}

async fn test_function() -> Response {
    StatusCode::OK.into_response()
}

pub fn system_routes() -> Router {
    Router::new().route("/health", get(health_check))
}

pub fn test_routes() -> Router {
    Router::new().route("/test", get(test_function))
}

async fn fallback(uri: Uri) -> (StatusCode, String) {
    (StatusCode::NOT_FOUND, format!("Handler for route '{uri}' not found!"))
}
