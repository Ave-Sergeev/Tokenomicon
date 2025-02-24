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
    pub fn new() -> Self {
        let mut vocab = HashMap::new();
        vocab.insert(UNKNOWN_TOKEN.to_vec(), 0);

        let reverse_vocab = vec![(0, UNKNOWN_TOKEN.to_vec())].into_iter().collect();

        Self {
            vocab,
            merges: vec![],
            reverse_vocab,
            unk_id: 0,
        }
    }

    pub fn train(&mut self, text: &str, vocab_size: usize) {
        let bytes = text.as_bytes();
        let mut tokens: Vec<Vec<u8>> = bytes.iter().map(|&b| vec![b]).collect();

        let mut vocab = self.vocab.clone();
        let mut reverse_vocab = self.reverse_vocab.clone();
        let mut current_id = *reverse_vocab.keys().max().unwrap_or(&0) + 1;

        let unique_bytes: HashSet<Vec<u8>> = bytes.iter().map(|&b| vec![b]).collect();
        for byte in unique_bytes {
            if !vocab.contains_key(&byte) {
                vocab.insert(byte.clone(), current_id);
                reverse_vocab.insert(current_id, byte);
                current_id += 1;
            }
        }

        while vocab.len() < vocab_size {
            let mut pair_counts = HashMap::new();

            for pair in tokens.windows(2) {
                let key = (pair[0].clone(), pair[1].clone());
                *pair_counts.entry(key).or_insert(0) += 1;
            }

            if let Some(((p1, p2), _)) = pair_counts.into_iter().max_by_key(|&(_, count)| count) {
                let new_token = [&p1[..], &p2[..]].concat();

                if vocab.contains_key(&new_token) {
                    continue;
                }

                self.merges.push((p1.clone(), p2.clone()));

                vocab.insert(new_token.clone(), current_id);
                reverse_vocab.insert(current_id, new_token);
                current_id += 1;

                let mut new_tokens = Vec::with_capacity(tokens.len());
                let mut i = 0;

                while i < tokens.len() {
                    if i < tokens.len() - 1 && tokens[i] == p1 && tokens[i + 1] == p2 {
                        new_tokens.push([&p1[..], &p2[..]].concat());
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

    pub fn encode(&self, text: &str) -> Vec<u32> {
        let mut tokens: Vec<Vec<u8>> = text.as_bytes().iter().map(|&b| vec![b]).collect();

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
            .map(|t| *self.vocab.get(t).unwrap_or(&self.unk_id))
            .collect()
    }

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
