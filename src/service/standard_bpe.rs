use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::iter;
use std::path::Path;
use unicode_segmentation::UnicodeSegmentation;

const SENTENCE_START_TOKEN: &str = "<s>";
const SENTENCE_END_TOKEN: &str = "</s>";
const WORD_BREAK_CHAR: char = '▁';
const UNKNOWN_TOKEN: &str = "<unk>";

#[derive(Debug)]
pub struct StandardBPE {
    vocab: HashMap<String, isize>,
}

impl StandardBPE {
    /// Initializes a new instance `StandardBPE` with empty vocabulary
    pub fn new() -> Self {
        Self { vocab: HashMap::new() }
    }

    /// Tokenizes text using the BPE algorithm
    pub fn tokenize(&self, text: &str) -> Vec<String> {
        self.split_into_sentences(text).flatten().collect::<Vec<_>>()
    }

    fn split_into_sentences<'a>(
        &'a self,
        text: &'a str,
    ) -> impl Iterator<Item = impl Iterator<Item = String> + 'a> + 'a {
        UnicodeSegmentation::unicode_sentences(text).map(move |sentence| self.tokenize_sentence_with_markers(sentence))
    }

    fn tokenize_sentence_with_markers<'a>(&'a self, sentence: &'a str) -> impl Iterator<Item = String> + 'a {
        iter::once(SENTENCE_START_TOKEN.to_string())
            .chain(
                sentence
                    .unicode_words()
                    .flat_map(move |word| self.tokenize_word(&format!("{}{}", WORD_BREAK_CHAR, word.to_lowercase()))),
            )
            .chain(iter::once(SENTENCE_END_TOKEN.to_string()))
    }

    fn tokenize_word(&self, text: &str) -> Vec<String> {
        if text.is_empty() {
            return vec![];
        }

        let word = text.chars().collect::<Vec<_>>();

        for len in (1..=word.len()).rev() {
            let matches = (0..=(word.len() - len))
                .filter_map(|start| {
                    let end = start + len;
                    let candidate = word[start..end].iter().collect::<String>();
                    self.vocab.get(&candidate).map(|_| (candidate, start, end))
                })
                .collect::<Vec<_>>();

            if let Some((candidate, start, end)) = matches
                .into_iter()
                .max_by_key(|(candidate, _, _)| self.vocab[candidate])
            {
                let left = word[..start].iter().collect::<String>();
                let right = word[end..].iter().collect::<String>();

                return [self.tokenize_word(&left), vec![candidate], self.tokenize_word(&right)].concat();
            }
        }

        vec![UNKNOWN_TOKEN.to_string()]
    }

    /// Loading a dictionary from a file
    pub fn load_vocab_from_file<P: AsRef<Path>>(&mut self, path: P) -> io::Result<()> {
        let file = File::open(path)?;
        let reader = io::BufReader::new(file);

        for line in reader.lines() {
            let line = line?;
            let parts = line.split_whitespace().collect::<Vec<_>>();

            if parts.len() >= 2 {
                let token = parts[0].to_string();
                let value = parts[1].parse::<isize>().map_err(|err| {
                    io::Error::new(io::ErrorKind::InvalidData, format!("Failed to parse value: {err}"))
                })?;

                self.vocab.insert(token, value);
            }
        }

        Ok(())
    }
}
