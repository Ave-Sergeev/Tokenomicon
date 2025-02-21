use crate::http::api::{system_routes, tokenize_routes};
use crate::service::shared_tokenizer::SharedTokenizer;
use crate::setting::settings::Settings;
use axum::http::{StatusCode, Uri};
use axum::Router;
use tokio::net::TcpListener;

mod http;
mod models;
mod service;
mod setting;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let settings = Settings::new("config.yaml")?;
    println!("Settings:\n{}", settings.json_pretty());

    let address = &format!("{}:{}", settings.server.host, settings.server.port);
    println!("Server listening on {}", address);

    let listener = TcpListener::bind(address).await?;

    axum::serve(listener, routes()).await?;

    Ok(())
}

fn routes() -> Router {
    let shared_tokenizer = SharedTokenizer::new();

    let all_routes = Router::new()
        .nest("/v1", system_routes())
        .nest("/v1", tokenize_routes(shared_tokenizer));

    Router::new().nest("/api", all_routes).fallback(fallback)
}

async fn fallback(uri: Uri) -> (StatusCode, String) {
    (StatusCode::NOT_FOUND, format!("Handler for route '{uri}' not found!"))
}
