use crate::models::request::{DecodeRequest, Method, SimpleRequest, TextRequest, TrainRequest};
use crate::models::response::{DecodeResponse, EncodeResponse, TokenizeResponse, VocabResponse};
use crate::service::shared_state::Shared;
use crate::service::simple_tokenizer::SimpleTokenizer;
use axum::response::Response;
use axum::routing::get;
use axum::{
    extract::{Extension, Json},
    http::StatusCode,
    response::IntoResponse,
    routing::{Router, post},
};
use std::collections::HashMap;
use std::sync::Mutex;

pub fn system_routes() -> Router {
    Router::new().route("/health", get(health_check))
}

pub fn tokenize_routes(shared_state: Shared) -> Router {
    Router::new()
        .nest(
            "/tokenize",
            Router::new()
                .route("/simple", post(simple_tokenize))
                .route("/standard-bpe", post(bpe_tokenize))
                .route("/byte-level-bpe/train", post(bl_bpe_train))
                .route("/byte-level-bpe/encode", post(bl_bpe_encode))
                .route("/byte-level-bpe/decode", post(bl_bpe_decode)),
        )
        .layer(Extension(shared_state))
}

async fn health_check() -> Response {
    StatusCode::OK.into_response()
}

async fn simple_tokenize(Json(payload): Json<SimpleRequest>) -> Result<Json<TokenizeResponse>, (StatusCode, String)> {
    let tokens = match payload.method {
        Method::Words => SimpleTokenizer::tokenize_words(&payload.text),
        Method::Chars => SimpleTokenizer::tokenize_chars(&payload.text),
    };

    Ok(Json(TokenizeResponse { tokens }))
}

async fn bpe_tokenize(
    Extension(state): Extension<Shared>,
    Json(payload): Json<TextRequest>,
) -> Result<Json<TokenizeResponse>, (StatusCode, String)> {
    let tokens = with_locked_mutex(&state.standard_bpe, |tokenizer| tokenizer.tokenize(&payload.text))?;

    Ok(Json(TokenizeResponse { tokens }))
}

async fn bl_bpe_train(
    Extension(state): Extension<Shared>,
    Json(payload): Json<TrainRequest>,
) -> Result<Json<VocabResponse>, (StatusCode, String)> {
    with_locked_mutex(&state.byte_level_bpe, |tokenizer| tokenizer.train(&payload.text, payload.size))?;

    let vocab = with_locked_mutex(&state.byte_level_bpe, |tokenizer| {
        tokenizer
            .get_vocab()
            .iter()
            .map(|(k, v)| (String::from_utf8_lossy(k).to_string(), *v))
            .collect::<HashMap<_, _>>()
    })?;

    let vocab_size = vocab.len();

    Ok(Json(VocabResponse { vocab_size, vocab }))
}

async fn bl_bpe_encode(
    Extension(state): Extension<Shared>,
    Json(payload): Json<TextRequest>,
) -> Result<Json<EncodeResponse>, (StatusCode, String)> {
    let tokens = with_locked_mutex(&state.byte_level_bpe, |tokenizer| tokenizer.encode(&payload.text))?;

    Ok(Json(EncodeResponse { tokens }))
}

async fn bl_bpe_decode(
    Extension(state): Extension<Shared>,
    Json(payload): Json<DecodeRequest>,
) -> Result<Json<DecodeResponse>, (StatusCode, String)> {
    let text = with_locked_mutex(&state.byte_level_bpe, |tokenizer| tokenizer.decode(&payload.tokens))?;

    Ok(Json(DecodeResponse { text }))
}

fn with_locked_mutex<T, F, R>(mutex: &Mutex<T>, func: F) -> Result<R, (StatusCode, String)>
where
    F: FnOnce(&mut T) -> R,
{
    let mut guard = mutex
        .lock()
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to lock mutex: {err}")))?;
    Ok(func(&mut guard))
}
