use glossia_shared::{WordMeaning, AppError};
use std::collections::HashSet;

/// Manages known words and filters them from word lists
pub struct KnownWordsFilter {
    known_words: HashSet<String>,
}

impl KnownWordsFilter {
    pub fn new() -> Result<Self, AppError> {
        Ok(Self {
            known_words: HashSet::new(),
        })
    }

    /// Add a word to known words
    pub fn add_known_word(&mut self, word: &str) -> Result<(), AppError> {
        self.known_words.insert(word.to_lowercase());
        Ok(())
    }

    /// Remove a word from known words
    pub fn remove_known_word(&mut self, word: &str) -> Result<(), AppError> {
        self.known_words.remove(&word.to_lowercase());
        Ok(())
    }

    /// Check if a word is known
    pub fn is_known_word(&self, word: &str) -> bool {
        self.known_words.contains(&word.to_lowercase())
    }

    /// Get all known words
    pub fn get_all_known_words(&self) -> Result<Vec<String>, AppError> {
        Ok(self.known_words.iter().cloned().collect())
    }

    /// Get count of known words
    pub fn get_count(&self) -> usize {
        self.known_words.len()
    }

    /// Filter out known words from a word list
    pub fn filter_words(&self, words: &[WordMeaning]) -> Vec<WordMeaning> {
        words.iter()
            .filter(|word_meaning| !self.is_known_word(&word_meaning.word))
            .cloned()
            .collect()
    }

    /// Clear all known words
    pub fn clear(&mut self) {
        self.known_words.clear();
    }

    /// Load known words from a collection
    pub fn load_known_words(&mut self, words: impl IntoIterator<Item = String>) {
        self.known_words.extend(words.into_iter().map(|w| w.to_lowercase()));
    }
}
