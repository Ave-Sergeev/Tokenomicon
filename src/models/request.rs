use serde::Deserialize;

#[derive(Deserialize)]
pub struct SimpleRequest {
    pub text: String,
    pub method: Method,
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Method {
    Words,
    Chars,
}

#[derive(Deserialize)]
pub struct TextRequest {
    pub text: String,
}

#[derive(Deserialize)]
pub struct DecodeRequest {
    pub tokens: Vec<u32>,
}

#[derive(Deserialize)]
pub struct TrainRequest {
    pub size: usize,
    pub text: String,
}
