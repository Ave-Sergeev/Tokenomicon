use serde::Serialize;

#[derive(Serialize)]
pub struct TokenizeResponse {
    pub tokens: Vec<String>,
}
