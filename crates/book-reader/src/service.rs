use glossia_api_client::{OpenAIClient, BraveImageClient, LLMClient};
use glossia_shared::{AppError, SimplificationRequest, WordMeaning, ImageQueryOptimizationRequest};
use crate::cache_manager::CacheManager;
use std::collections::HashSet;

/// Service layer that handles business logic for text processing
/// Separated from UI state for better testability and maintainability
pub struct GlossiaService {
    pub llm_client: Box<dyn LLMClient>,
    pub image_client: BraveImageClient,
    pub cache: CacheManager,
}

impl GlossiaService {
    pub fn new() -> Result<Self, AppError> {
        Ok(Self {
            llm_client: Box::new(OpenAIClient::new()?),
            image_client: BraveImageClient::new(),
            cache: CacheManager::new(),
        })
    }

    /// Create service with custom LLM client (useful for testing)
    pub fn with_llm_client(llm_client: Box<dyn LLMClient>) -> Self {
        Self {
            llm_client,
            image_client: BraveImageClient::new(),
            cache: CacheManager::new(),
        }
    }

    /// Simplify a sentence and cache the result
    pub async fn simplify_sentence(&mut self, sentence: &str) -> Result<glossia_shared::SimplificationResponse, AppError> {
        // Check cache first
        if let Some(cached) = self.cache.get_simplified(sentence) {
            return Ok(cached);
        }

        // Make API request
        let request = SimplificationRequest {
            sentence: sentence.to_string(),
        };
        let response = self.llm_client.simplify(request).await?;

        // Cache the result
        self.cache.cache_simplified(sentence.to_string(), response.clone());

        Ok(response)
    }

    /// Get word meaning and cache it
    pub async fn get_word_meaning(&mut self, word: &str, context: &str) -> Result<String, AppError> {
        // Check cache first
        if let Some(cached) = self.cache.get_word_meaning(word) {
            return Ok(cached);
        }

        // Make API request
        let meaning = self.llm_client.get_word_meaning(word, context).await?;

        // Cache the result
        self.cache.cache_word_meaning(word.to_string(), meaning.clone());

        Ok(meaning)
    }

    /// Optimize image query
    pub async fn optimize_image_query(&self, word: &str, context: &str, meaning: &str) -> Result<String, AppError> {
        let request = ImageQueryOptimizationRequest {
            word: word.to_string(),
            sentence_context: context.to_string(),
            word_meaning: meaning.to_string(),
        };

        let response = self.llm_client.optimize_image_query(request).await?;
        Ok(response.optimized_query)
    }

    /// Clear text-related caches when loading new content
    pub fn clear_text_caches(&mut self) {
        self.cache.clear_text_caches();
    }

    /// Get cached word meaning if available
    pub fn get_cached_word_meaning(&self, word: &str) -> Option<String> {
        self.cache.get_word_meaning(word)
    }

    /// Get cached simplified result if available
    pub fn get_cached_simplified(&self, sentence: &str) -> Option<glossia_shared::SimplificationResponse> {
        self.cache.get_simplified(sentence)
    }
}

impl Default for GlossiaService {
    fn default() -> Self {
        Self::new().expect("Failed to create GlossiaService")
    }
}

/// Pure state management for text navigation
/// Contains no async operations or business logic
#[derive(Debug, Clone)]
pub struct PureReadingState {
    pub manual_words: HashSet<String>,
}

impl Default for PureReadingState {
    fn default() -> Self {
        Self::new()
    }
}

impl PureReadingState {
    pub fn new() -> Self {
        Self {
            manual_words: HashSet::new(),
        }
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

    /// Clear manual words when loading new text
    pub fn clear(&mut self) {
        self.manual_words.clear();
    }

    /// Get combined words (from API + manual) for current sentence
    pub fn get_combined_words(&self, api_words: &[WordMeaning], service: &GlossiaService) -> Vec<WordMeaning> {
        let mut combined_words = api_words.to_vec();
        
        // Add manual words that aren't already in API words
        let api_word_set: HashSet<String> = api_words.iter()
            .map(|w| w.word.to_lowercase())
            .collect();
        
        for manual_word in &self.manual_words {
            if !api_word_set.contains(manual_word) {
                let meaning = service.get_cached_word_meaning(manual_word)
                    .unwrap_or_else(|| "Loading meaning...".to_string());
                
                combined_words.push(WordMeaning::new_word(
                    manual_word.clone(),
                    meaning,
                ));
            }
        }
        
        combined_words
    }
}
