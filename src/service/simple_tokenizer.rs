pub struct SimpleTokenizer {}

impl SimpleTokenizer {
    /// Tokenizes text into words using whitespace separation
    pub fn tokenize_words(text: &str) -> Vec<String> {
        text.split_whitespace().map(|s| s.to_string()).collect()
    }

    /// Tokenizes text into individual characters
    pub fn tokenize_chars(text: &str) -> Vec<String> {
        text.chars().map(|c| c.to_string()).collect()
    }
}
