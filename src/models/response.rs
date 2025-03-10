use std::collections::HashMap;
use serde::Serialize;

#[derive(Serialize)]
pub struct TokenizeResponse {
    pub tokens: Vec<String>,
}

#[derive(Serialize)]
pub struct EncodeResponse {
    pub tokens: Vec<u32>,
}

#[derive(Serialize)]
pub struct DecodeResponse {
    pub text: String,
}

#[derive(Serialize)]
pub struct VocabResponse {
    pub vocab_size: usize,
    pub vocab: HashMap<String, u32>
}
