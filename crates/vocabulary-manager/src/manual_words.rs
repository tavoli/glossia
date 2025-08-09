use std::collections::{HashSet, HashMap};
use std::time::{SystemTime, UNIX_EPOCH};

/// Manages manually selected words with timestamps
pub struct ManualWordsManager {
    manual_words: HashSet<String>,
    word_timestamps: HashMap<String, u64>,
}

impl ManualWordsManager {
    pub fn new() -> Self {
        Self {
            manual_words: HashSet::new(),
            word_timestamps: HashMap::new(),
        }
    }

    /// Add a word to the manual words set with current timestamp
    pub fn add_word(&mut self, word: String) {
        let word_lower = word.to_lowercase();
        self.manual_words.insert(word_lower.clone());
        
        // Record timestamp in milliseconds
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;
        self.word_timestamps.insert(word_lower, timestamp);
    }

    /// Remove a word from the manual words set
    pub fn remove_word(&mut self, word: &str) {
        let word_lower = word.to_lowercase();
        self.manual_words.remove(&word_lower);
        self.word_timestamps.remove(&word_lower);
    }

    /// Check if a word is manually selected
    pub fn is_manual_word(&self, word: &str) -> bool {
        self.manual_words.contains(&word.to_lowercase())
    }

    /// Get all manual words
    pub fn get_all_words(&self) -> &HashSet<String> {
        &self.manual_words
    }

    /// Get all manual words sorted by timestamp (newest first)
    pub fn get_words_sorted_by_time(&self) -> Vec<(String, u64)> {
        let mut words_with_time: Vec<(String, u64)> = self.word_timestamps
            .iter()
            .map(|(word, &timestamp)| (word.clone(), timestamp))
            .collect();
        
        // Sort by timestamp descending (newest first)
        words_with_time.sort_by(|a, b| b.1.cmp(&a.1));
        words_with_time
    }

    /// Clear all manual words
    pub fn clear(&mut self) {
        self.manual_words.clear();
        self.word_timestamps.clear();
    }

    /// Get the count of manual words
    pub fn count(&self) -> usize {
        self.manual_words.len()
    }
}

impl Default for ManualWordsManager {
    fn default() -> Self {
        Self::new()
    }
}
