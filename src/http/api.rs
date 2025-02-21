use crate::models::request::TokenizeRequest;
use crate::models::response::TokenizeResponse;
use crate::service::shared_tokenizer::SharedTokenizer;
use crate::service::tokenizer::Tokenizer;
use axum::response::Response;
use axum::routing::get;
use axum::{
    extract::{Extension, Json, Path},
    http::StatusCode,
    response::IntoResponse,
    routing::{post, Router},
};

pub fn system_routes() -> Router {
    Router::new().route("/health", get(health_check))
}

pub fn tokenize_routes(shared_tokenizer: SharedTokenizer) -> Router {
    Router::new()
        .route("/tokenize/{method}", post(tokenize))
        .layer(Extension(shared_tokenizer))
}

async fn health_check() -> Response {
    StatusCode::OK.into_response()
}

async fn tokenize(
    Extension(state): Extension<SharedTokenizer>,
    Path(method): Path<String>,
    Json(payload): Json<TokenizeRequest>,
) -> Result<Json<TokenizeResponse>, (StatusCode, String)> {
    let tokenizer = state.tokenizer.lock().unwrap();

    let tokens = match method.as_str() {
        "bpe" => tokenizer.tokenize_bpe(&payload.text),
        "words" => Tokenizer::tokenize_words(&payload.text),
        "chars" => Tokenizer::tokenize_chars(&payload.text),
        _ => return Err((StatusCode::BAD_REQUEST, "Invalid tokenization method".to_string())),
    };

    Ok(Json(TokenizeResponse { tokens }))
}
