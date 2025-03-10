use crate::service::byte_level_bpe::ByteLevelBPE;
use crate::service::standard_bpe::StandardBPE;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct Shared {
    pub standard_bpe: Arc<Mutex<StandardBPE>>,
    pub byte_level_bpe: Arc<Mutex<ByteLevelBPE>>,
}

impl Shared {
    pub fn new() -> Self {
        let mut standard_bpe = StandardBPE::new();
        let byte_level_bpe = ByteLevelBPE::new();

        standard_bpe
            .load_vocab_from_file("./vocab/multi.wiki.bpe.vs1000000.vocab")
            .expect("Failed load vocab from file");

        Shared {
            standard_bpe: Arc::new(Mutex::new(standard_bpe)),
            byte_level_bpe: Arc::new(Mutex::new(byte_level_bpe)),
        }
    }
}
