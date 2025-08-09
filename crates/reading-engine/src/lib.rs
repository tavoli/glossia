mod cache_engine;
mod reading_orchestrator;
mod state_manager;

pub use cache_engine::CacheEngine;
pub use reading_orchestrator::ReadingOrchestrator;
pub use state_manager::StateManager;

use glossia_shared::{AppError, WordMeaning, SimplificationResponse};
use glossia_navigation_service::NavigationService;
use glossia_vocabulary_manager::VocabularyManager;
use std::collections::HashSet;

/// High-level reading engine that orchestrates all reading functionality
/// This replaces the complex ReadingState from book-reader
pub struct ReadingEngine {
    navigation: NavigationService,
    vocabulary: VocabularyManager,
    cache: CacheEngine,
    orchestrator: ReadingOrchestrator,
    state: StateManager,
}

impl ReadingEngine {
    pub fn new() -> Result<Self, AppError> {
        Ok(Self {
            navigation: NavigationService::new(),
            vocabulary: VocabularyManager::new()?,
            cache: CacheEngine::new(),
            orchestrator: ReadingOrchestrator::new()?,
            state: StateManager::new(),
        })
    }

    /// Load text and reset all state
    pub fn load_text(&mut self, text: &str) -> Result<(), AppError> {
        self.navigation.load_text(text)?;
        self.vocabulary.clear_manual_words();
        self.cache.clear_text_caches();
        self.state.reset();
        Ok(())
    }

    /// Get current sentence
    pub fn current_sentence(&self) -> Option<String> {
        self.navigation.current_sentence()
    }

    /// Move to next sentence
    pub fn next(&mut self) -> bool {
        self.navigation.advance()
    }

    /// Move to previous sentence
    pub fn previous(&mut self) -> bool {
        self.navigation.previous()
    }

    /// Get navigation position info
    pub fn position(&self) -> usize {
        self.navigation.current_position()
    }

    pub fn total_sentences(&self) -> usize {
        self.navigation.total_sentences()
    }

    /// Add manual word selection
    pub fn add_manual_word(&mut self, word: String) {
        self.vocabulary.add_manual_word(word);
    }

    /// Remove manual word selection
    pub fn remove_manual_word(&mut self, word: &str) {
        self.vocabulary.remove_manual_word(word);
    }

    /// Check if word is manually selected
    pub fn is_manual_word(&self, word: &str) -> bool {
        self.vocabulary.is_manual_word(word)
    }

    /// Get all manual words
    pub fn get_manual_words(&self) -> &HashSet<String> {
        self.vocabulary.get_manual_words()
    }

    /// Get combined words for current sentence
    pub fn get_combined_words(&self, api_words: &[WordMeaning]) -> Vec<WordMeaning> {
        // Get the current sentence to filter manual words
        let current_sentence = self.current_sentence().unwrap_or_default();
        self.vocabulary.get_combined_words(api_words, &current_sentence)
    }

    /// Get combined words with cached meanings lookup
    pub fn get_combined_words_with_cache(&self, api_words: &[WordMeaning]) -> Vec<WordMeaning> {
        // Get the current sentence to filter manual words
        let current_sentence = self.current_sentence().unwrap_or_default();
        self.vocabulary.get_combined_words_with_cache(api_words, &current_sentence, |word| {
            self.cache.get_word_meaning(word)
        })
    }

    /// Vocabulary management
    pub fn add_word_encounter(&mut self, word: &str) -> Result<(usize, bool), AppError> {
        self.vocabulary.add_word_encounter(word)
    }

    pub fn add_known_word(&mut self, word: &str) -> Result<(), AppError> {
        self.vocabulary.add_known_word(word)
    }

    pub fn remove_known_word(&mut self, word: &str) -> Result<(), AppError> {
        self.vocabulary.remove_known_word(word)
    }

    pub fn get_all_known_words(&self) -> Result<Vec<String>, AppError> {
        self.vocabulary.get_all_known_words()
    }

    pub fn filter_known_words(&self, words: &[WordMeaning]) -> Vec<WordMeaning> {
        self.vocabulary.filter_known_words(words)
    }

    pub fn known_words_count(&self) -> usize {
        self.vocabulary.get_known_words_count()
    }

    /// Cache management
    pub fn get_cached_simplification(&self, sentence: &str) -> Option<SimplificationResponse> {
        self.cache.get_simplified(sentence)
    }

    pub fn cache_simplification(&mut self, sentence: String, response: SimplificationResponse) {
        self.cache.cache_simplified(sentence, response);
    }

    /// High-level orchestration
    pub async fn process_sentence(&mut self, sentence: &str) -> Result<SimplificationResponse, AppError> {
        self.orchestrator.process_sentence(sentence, &mut self.cache).await
    }

    /// Get reading progress (0.0 to 1.0)
    pub fn progress(&self) -> f64 {
        self.navigation.progress()
    }

    /// Navigation history
    pub fn can_go_back(&self) -> bool {
        self.navigation.can_go_back()
    }

    pub fn can_go_forward(&self) -> bool {
        self.navigation.can_go_forward()
    }

    pub fn go_back(&mut self) -> bool {
        self.navigation.go_back()
    }

    pub fn go_forward(&mut self) -> bool {
        self.navigation.go_forward()
    }

    /// State management
    pub fn is_processing(&self) -> bool {
        self.state.is_processing()
    }

    pub fn set_processing(&mut self, processing: bool) {
        self.state.set_processing(processing);
    }

    // Compatibility methods for the app to match the old ReadingState API
    
    /// Get cached simplified response (alias for get_cached_simplification)
    pub fn get_cached_simplified(&self, sentence: &str) -> Option<SimplificationResponse> {
        self.get_cached_simplification(sentence)
    }

    /// Simplify a sentence using the orchestrator
    pub async fn simplify_sentence(&mut self, sentence: &str) -> Result<SimplificationResponse, AppError> {
        self.process_sentence(sentence).await
    }

    /// Static method to simplify a sentence without any state access
    /// Always calls the LLM - cache checking should be done separately
    pub async fn simplify_sentence_static(sentence: &str) -> Result<SimplificationResponse, AppError> {
        // Create LLM client and make the call without any state access
        use glossia_llm_client::{LLMClientFactory, SimplificationRequest};
        let factory = LLMClientFactory::new();
        let client = factory.create_client()?;
        
        let request = SimplificationRequest {
            sentence: sentence.to_string(),
        };

        client.simplify(request).await
    }

    /// Cache a simplification result (separate from the async operation)
    pub fn cache_simplification_result(&mut self, sentence: String, response: SimplificationResponse) {
        self.cache.cache_simplified(sentence, response);
    }

    /// Get word meaning (delegated to LLM client through orchestrator)
    pub async fn get_word_meaning(&mut self, word: &str, context: &str) -> Result<String, AppError> {
        // For now, we'll need to add this functionality to the orchestrator
        // This is a temporary compatibility method
        use glossia_llm_client::LLMClientFactory;
        let factory = LLMClientFactory::new();
        let client = factory.create_client()?;
        client.get_word_meaning(word, context).await
    }

    /// Static method to get word meaning without any state access
    pub async fn get_word_meaning_static(word: &str, context: &str) -> Result<String, AppError> {
        use glossia_llm_client::LLMClientFactory;
        let factory = LLMClientFactory::new();
        let client = factory.create_client()?;
        client.get_word_meaning(word, context).await
    }

    /// Cache methods for compatibility
    pub fn cache_optimized_query(&mut self, context_key: String, query: String) {
        self.cache.cache_optimized_query(context_key, query);
    }

    pub fn get_optimized_query(&self, context_key: &str) -> Option<String> {
        self.cache.get_optimized_query(context_key)
    }

    pub fn cache_images(&mut self, word: String, images: Vec<glossia_shared::ImageResult>) {
        self.cache.cache_images(word, images);
    }

    pub fn get_images(&self, word: &str) -> Option<Vec<glossia_shared::ImageResult>> {
        self.cache.get_images(word)
    }

    pub fn get_cached_word_meaning(&self, word: &str) -> Option<String> {
        self.cache.get_word_meaning(word)
    }

    pub fn cache_word_meaning(&mut self, word: String, meaning: String) {
        self.cache.cache_word_meaning(word, meaning);
    }

    /// Cache a word meaning result (separate from the async operation)
    pub fn cache_word_meaning_result(&mut self, word: String, meaning: String) {
        self.cache.cache_word_meaning(word, meaning);
    }

    /// Get sentence at specific position without changing current position
    pub fn get_sentence_at_position(&self, position: usize) -> Option<String> {
        if let Some(sentences) = self.navigation.get_sentences() {
            sentences.get(position).cloned()
        } else {
            None
        }
    }
}

impl Default for ReadingEngine {
    fn default() -> Self {
        Self::new().expect("Failed to create ReadingEngine")
    }
}
