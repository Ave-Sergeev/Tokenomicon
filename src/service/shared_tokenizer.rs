use crate::service::tokenizer::Tokenizer;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct SharedTokenizer {
    pub(crate) tokenizer: Arc<Mutex<Tokenizer>>,
}

impl SharedTokenizer {
    pub fn new() -> Self {
        let mut tokenizer = Tokenizer::new();
        tokenizer
            .load_vocab_from_file("./vocab/multi.wiki.bpe.vs1000000.vocab")
            .unwrap();

        SharedTokenizer {
            tokenizer: Arc::new(Mutex::new(tokenizer)),
        }
    }
}
