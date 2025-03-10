use crate::http::api::{system_routes, tokenize_routes};
use crate::service::shared_state::Shared;
use crate::setting::settings::Settings;
use axum::Router;
use axum::http::{StatusCode, Uri};
use std::error::Error;
use tokio::net::TcpListener;

mod http;
mod models;
mod service;
mod setting;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let settings = Settings::new("config.yaml").map_err(|err| format!("Failed to load settings: {err}"))?;
    println!("Settings:\n{}\n", settings.json_pretty()?);

    let address = format!("{}:{}", settings.server.host, settings.server.port);
    println!("Server listening on {address}");

    let listener = TcpListener::bind(address).await?;

    axum::serve(listener, routes())
        .await
        .map_err(|err| format!("Server returned error: {err}"))?;

    Ok(())
}

fn routes() -> Router {
    let shared_state = Shared::new();

    let all_routes = Router::new()
        .nest("/v1", system_routes())
        .nest("/v1", tokenize_routes(shared_state));

    Router::new().nest("/api", all_routes).fallback(fallback)
}

async fn fallback(uri: Uri) -> (StatusCode, String) {
    (StatusCode::NOT_FOUND, format!("Handler for route '{uri}' not found!"))
}
