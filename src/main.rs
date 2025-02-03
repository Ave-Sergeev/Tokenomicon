use crate::http::api::*;
use crate::setting::settings::Settings;
use axum::http::{StatusCode, Uri};
use axum::Router;
use tokio::net::TcpListener;

mod http;
mod repository;
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
    let all_routes = Router::new().nest("/v1", system_routes()).nest("/v1", test_routes());

    Router::new().nest("/api", all_routes).fallback(fallback)
}

async fn fallback(uri: Uri) -> (StatusCode, String) {
    (StatusCode::NOT_FOUND, format!("Handler for route '{uri}' not found!"))
}
