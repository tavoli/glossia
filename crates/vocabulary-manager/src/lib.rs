mod word_tracker;
mod manual_words;
mod known_words_filter;
mod vocabulary_trait;

pub use word_tracker::WordTracker;
pub use manual_words::ManualWordsManager;
pub use known_words_filter::KnownWordsFilter;
pub use vocabulary_trait::{VocabularyStore, MemoryVocabularyStore, FileVocabularyStore};

use glossia_shared::{WordMeaning, AppError};
use std::collections::HashSet;
use tracing::{instrument, info, debug};

/// Centralized vocabulary management system
/// Combines word tracking, known words filtering, and manual word selection
pub struct VocabularyManager {
    word_tracker: WordTracker,
    manual_words: ManualWordsManager,
    known_words_filter: KnownWordsFilter,
}

impl VocabularyManager {
    pub fn new() -> Result<Self, AppError> {
        Ok(Self {
            word_tracker: WordTracker::new()?,
            manual_words: ManualWordsManager::new(),
            known_words_filter: KnownWordsFilter::new()?,
        })
    }

    /// Add a word encounter (increments count, may promote to known)
    #[instrument(skip(self), fields(word = %word))]
    pub fn add_word_encounter(&mut self, word: &str) -> Result<(usize, bool), AppError> {
        debug!("Adding word encounter for: '{}'", word);
        let result = self.word_tracker.add_encounter(word)?;
        if result.1 {
            info!("Word '{}' promoted to known after {} encounters", word, result.0);
        } else {
            debug!("Word '{}' encounter count: {}", word, result.0);
        }
        Ok(result)
    }

    /// Add a word to known words manually
    #[instrument(skip(self), fields(word = %word))]
    pub fn add_known_word(&mut self, word: &str) -> Result<(), AppError> {
        info!("Manually adding word to known words: '{}'", word);
        self.known_words_filter.add_known_word(word)?;
        debug!("Known words count now: {}", self.get_known_words_count());
        Ok(())
    }

    /// Remove a word from known words
    #[instrument(skip(self), fields(word = %word))]
    pub fn remove_known_word(&mut self, word: &str) -> Result<(), AppError> {
        info!("Removing word from known words: '{}'", word);
        self.known_words_filter.remove_known_word(word)?;
        debug!("Known words count now: {}", self.get_known_words_count());
        Ok(())
    }

    /// Get all known words
    pub fn get_all_known_words(&self) -> Result<Vec<String>, AppError> {
        self.known_words_filter.get_all_known_words()
    }

    /// Get known words count
    pub fn get_known_words_count(&self) -> usize {
        self.known_words_filter.get_count()
    }

    /// Filter out known words from a word list
    #[instrument(skip(self, words), fields(input_count = words.len()))]
    pub fn filter_known_words(&self, words: &[WordMeaning]) -> Vec<WordMeaning> {
        let filtered = self.known_words_filter.filter_words(words);
        debug!("Filtered {} words from {} (removed {} known words)", filtered.len(), words.len(), words.len() - filtered.len());
        filtered
    }

    /// Add a manual word selection
    pub fn add_manual_word(&mut self, word: String) {
        self.manual_words.add_word(word);
    }

    /// Remove a manual word selection
    pub fn remove_manual_word(&mut self, word: &str) {
        self.manual_words.remove_word(word);
    }

    /// Check if a word is manually selected
    pub fn is_manual_word(&self, word: &str) -> bool {
        self.manual_words.is_manual_word(word)
    }

    /// Get all manual words
    pub fn get_manual_words(&self) -> &HashSet<String> {
        self.manual_words.get_all_words()
    }

    /// Clear all manual words (useful when loading new text)
    pub fn clear_manual_words(&mut self) {
        self.manual_words.clear();
    }

    /// Get combined words (from API + manual) for display
    /// Now takes a cache lookup function to get meanings for manual words
    /// Only includes manual words that are present in the current sentence
    pub fn get_combined_words_with_cache<F>(&self, api_words: &[WordMeaning], current_sentence: &str, cache_lookup: F) -> Vec<WordMeaning> 
    where
        F: Fn(&str) -> Option<String>,
    {
        let mut combined = api_words.to_vec();
        
        // Get manual words with their timestamps
        let manual_words_with_time = self.manual_words.get_words_sorted_by_time();
        debug!("VocabularyManager: Processing {} manual words for sentence", manual_words_with_time.len());
        
        // Convert sentence to lowercase for case-insensitive matching
        let sentence_lower = current_sentence.to_lowercase();
        
        // Add manual words that:
        // 1. Aren't already in the API response
        // 2. Are actually present in the current sentence
        for (manual_word, timestamp) in manual_words_with_time {
            // Check if the word is present in the current sentence (case-insensitive)
            if !sentence_lower.contains(&manual_word.to_lowercase()) {
                debug!("VocabularyManager: Skipping manual word '{}' - not in current sentence", manual_word);
                continue; // Skip words not in the current sentence
            }
            
            if !api_words.iter().any(|w| w.word.to_lowercase() == manual_word.to_lowercase()) {
                let cached_meaning = cache_lookup(&manual_word);
                let meaning = cached_meaning.clone().unwrap_or_else(|| "Loading...".to_string());
                
                if cached_meaning.is_some() {
                    debug!("VocabularyManager: Manual word '{}' has cached meaning (len: {})", manual_word, meaning.len());
                } else {
                    debug!("VocabularyManager: Manual word '{}' has NO cached meaning, showing 'Loading...'", manual_word);
                }
                
                combined.push(WordMeaning {
                    word: manual_word,
                    meaning,
                    is_phrase: false,
                    timestamp: Some(timestamp),
                });
            } else {
                debug!("VocabularyManager: Manual word '{}' already in API words, skipping", manual_word);
            }
        }
        
        debug!("VocabularyManager: Returning {} combined words (API + manual)", combined.len());
        combined
    }

    /// Get combined words (from API + manual) for display - legacy version
    pub fn get_combined_words(&self, api_words: &[WordMeaning], current_sentence: &str) -> Vec<WordMeaning> {
        self.get_combined_words_with_cache(api_words, current_sentence, |_| None)
    }
}

impl Default for VocabularyManager {
    fn default() -> Self {
        Self::new().expect("Failed to create VocabularyManager")
    }
}
