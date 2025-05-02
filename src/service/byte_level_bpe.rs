use std::collections::{HashMap, HashSet};

const UNKNOWN_TOKEN: &[u8] = b"<unk>";

#[derive(Debug)]
pub struct ByteLevelBPE {
    vocab: HashMap<Vec<u8>, u32>,
    merges: Vec<(Vec<u8>, Vec<u8>)>,
    reverse_vocab: HashMap<u32, Vec<u8>>,
    unk_id: u32,
}

impl ByteLevelBPE {
    /// Initializes a new instance `ByteLevelBPE` with vocabulary with an unknown token and reverse vocabulary
    pub fn new() -> Self {
        let unk_id = 0;
        let merges = vec![];
        let mut vocab = HashMap::new();
        let reverse_vocab = HashMap::from([(0, UNKNOWN_TOKEN.to_vec())]);

        vocab.insert(UNKNOWN_TOKEN.to_vec(), 0);

        Self {
            vocab,
            merges,
            reverse_vocab,
            unk_id,
        }
    }

    /// Getter for vocabulary
    pub fn vocab(&self) -> &HashMap<Vec<u8>, u32> {
        &self.vocab
    }

    /// Training the token dictionary and merge list
    pub fn train(&mut self, text: &str, vocab_size: usize) {
        let bytes = text.as_bytes();
        let mut tokens = bytes.iter().map(|&byte| vec![byte]).collect::<Vec<Vec<_>>>();

        let mut vocab = self.vocab.clone();
        let mut reverse_vocab = self.reverse_vocab.clone();
        let mut current_id = *reverse_vocab.keys().max().unwrap_or(&0) + 1;

        let unique_bytes = bytes.iter().map(|&byte| vec![byte]).collect::<HashSet<Vec<_>>>();

        for byte in unique_bytes {
            if !vocab.contains_key(&byte) {
                vocab.insert(byte.clone(), current_id);
                reverse_vocab.insert(current_id, byte);
                current_id += 1;
            }
        }

        while vocab.len() <= vocab_size {
            let mut pair_counts = HashMap::new();

            for pair in tokens.windows(2) {
                let key = (pair[0].clone(), pair[1].clone());
                *pair_counts.entry(key).or_insert(0) += 1;
            }

            if let Some(((pair1, pair2), _)) = pair_counts.into_iter().max_by_key(|&(_, count)| count) {
                let new_token = [&pair1[..], &pair2[..]].concat();

                if vocab.contains_key(&new_token) {
                    continue;
                }

                self.merges.push((pair1.clone(), pair2.clone()));

                vocab.insert(new_token.clone(), current_id);
                reverse_vocab.insert(current_id, new_token);
                current_id += 1;

                let mut new_tokens = Vec::with_capacity(tokens.len());
                let mut i = 0;

                while i < tokens.len() {
                    if i < tokens.len() - 1 && tokens[i] == pair1 && tokens[i + 1] == pair2 {
                        new_tokens.push([&pair1[..], &pair2[..]].concat());
                        i += 2;
                    } else {
                        new_tokens.push(tokens[i].clone());
                        i += 1;
                    }
                }

                tokens = new_tokens;
            } else {
                break;
            }
        }

        self.vocab = vocab;
        self.reverse_vocab = reverse_vocab;
    }

    /// Converting input text into a vector of token identifiers
    pub fn encode(&self, text: &str) -> Vec<u32> {
        let bytes = text.as_bytes();
        let mut tokens = bytes.iter().map(|&byte| vec![byte]).collect::<Vec<Vec<_>>>();

        for (p1, p2) in &self.merges {
            let merged = [&p1[..], &p2[..]].concat();
            let mut i = 0;

            while i < tokens.len().saturating_sub(1) {
                if tokens[i] == *p1 && tokens[i + 1] == *p2 {
                    tokens.splice(i..=i + 1, vec![merged.clone()]);
                    i = i.saturating_sub(1);
                } else {
                    i += 1;
                }
            }
        }

        tokens
            .iter()
            .map(|token| *self.vocab.get(token).unwrap_or(&self.unk_id))
            .collect::<Vec<_>>()
    }

    /// Convert a vector of token IDs back to text
    pub fn decode(&self, ids: &[u32]) -> String {
        let mut bytes = Vec::new();

        for &id in ids {
            if let Some(token) = self.reverse_vocab.get(&id) {
                bytes.extend(token);
            } else {
                bytes.extend(UNKNOWN_TOKEN);
            }
        }

        String::from_utf8_lossy(&bytes).into_owned()
    }
}
