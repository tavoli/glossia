mod cache_manager;
mod navigation_state;

pub use cache_manager::CacheManager;
pub use navigation_state::NavigationState;

use glossia_api_client::{OpenAIClient, BraveImageClient};
use glossia_shared::WordMeaning;
use std::collections::HashSet;

#[derive(Clone)]
pub struct ReadingState {
    pub navigation: NavigationState,
    pub cache: CacheManager,
    pub api_client: OpenAIClient,
    pub image_client: BraveImageClient,
    pub manual_words: HashSet<String>,
}

impl Default for ReadingState {
    fn default() -> Self {
        Self::new()
    }
}

impl ReadingState {
    pub fn new() -> Self {
        Self {
            navigation: NavigationState::new(),
            cache: CacheManager::new(),
            api_client: OpenAIClient::new(),
            image_client: BraveImageClient::new(),
            manual_words: HashSet::new(),
        }
    }

    pub fn load_text(&mut self, text: &str) {
        self.navigation.load_text(text);
        self.cache.clear_text_caches(); // Keep image cache for reuse
        self.manual_words.clear(); // Clear manual words when loading new text
    }

    pub fn current_sentence(&self) -> Option<String> {
        self.navigation.current_sentence()
    }

    pub fn next(&mut self) {
        self.navigation.next();
    }

    pub fn previous(&mut self) {
        self.navigation.previous();
    }

    // Convenience getters for backward compatibility
    pub fn sentences(&self) -> &[String] {
        &self.navigation.sentences
    }

    pub fn position(&self) -> usize {
        self.navigation.position
    }

    pub fn total_sentences(&self) -> usize {
        self.navigation.total_sentences
    }

    /// Add a word to the manual words set
    pub fn add_manual_word(&mut self, word: String) {
        self.manual_words.insert(word.to_lowercase());
    }

    /// Remove a word from the manual words set
    pub fn remove_manual_word(&mut self, word: &str) {
        self.manual_words.remove(&word.to_lowercase());
    }

    /// Check if a word is manually selected
    pub fn is_manual_word(&self, word: &str) -> bool {
        self.manual_words.contains(&word.to_lowercase())
    }

    /// Get all manual words
    pub fn get_manual_words(&self) -> &HashSet<String> {
        &self.manual_words
    }

    /// Get combined words (from API + manual) for current sentence
    pub fn get_combined_words(&self, api_words: &[WordMeaning]) -> Vec<WordMeaning> {
        let mut combined_words = api_words.to_vec();
        
        // Add manual words that aren't already in API words
        let api_word_set: HashSet<String> = api_words.iter()
            .map(|w| w.word.to_lowercase())
            .collect();
        
        for manual_word in &self.manual_words {
            if !api_word_set.contains(manual_word) {
                let meaning = self.cache.get_word_meaning(manual_word)
                    .unwrap_or_else(|| "Loading meaning...".to_string());
                
                combined_words.push(WordMeaning {
                    word: manual_word.clone(),
                    meaning,
                });
            }
        }
        
        combined_words
    }

    /// Fetch word meaning for a manually selected word
    pub async fn fetch_word_meaning(&mut self, word: String, context: String) -> Result<(), glossia_shared::AppError> {
        // Check if we already have the meaning cached
        if self.cache.get_word_meaning(&word).is_some() {
            return Ok(());
        }

        // Fetch meaning from API
        let meaning = self.api_client.get_word_meaning(&word, &context).await?;
        
        // Cache the result
        self.cache.cache_word_meaning(word, meaning);
        
        Ok(())
    }
}