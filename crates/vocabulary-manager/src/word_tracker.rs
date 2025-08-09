use glossia_shared::AppError;
use std::collections::HashMap;

/// Tracks word encounters and handles promotion to known words
pub struct WordTracker {
    word_counts: HashMap<String, usize>,
    promotion_threshold: usize,
}

impl WordTracker {
    pub fn new() -> Result<Self, AppError> {
        Ok(Self {
            word_counts: HashMap::new(),
            promotion_threshold: 3, // Promote after 3 encounters
        })
    }

    /// Add an encounter for a word, returns (count, was_promoted)
    pub fn add_encounter(&mut self, word: &str) -> Result<(usize, bool), AppError> {
        let normalized_word = word.to_lowercase();
        let count = self.word_counts.entry(normalized_word).and_modify(|c| *c += 1).or_insert(1);
        
        let was_promoted = *count == self.promotion_threshold;
        
        Ok((*count, was_promoted))
    }

    /// Get encounter count for a word
    pub fn get_count(&self, word: &str) -> usize {
        self.word_counts.get(&word.to_lowercase()).copied().unwrap_or(0)
    }

    /// Set the promotion threshold
    pub fn set_promotion_threshold(&mut self, threshold: usize) {
        self.promotion_threshold = threshold;
    }

    /// Get the current promotion threshold
    pub fn get_promotion_threshold(&self) -> usize {
        self.promotion_threshold
    }

    /// Clear all word counts
    pub fn clear(&mut self) {
        self.word_counts.clear();
    }

    /// Get all tracked words and their counts
    pub fn get_all_counts(&self) -> &HashMap<String, usize> {
        &self.word_counts
    }
}
