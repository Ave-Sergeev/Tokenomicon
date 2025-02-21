use serde::Deserialize;

#[derive(Deserialize)]
pub struct TokenizeRequest {
    pub text: String,
}
