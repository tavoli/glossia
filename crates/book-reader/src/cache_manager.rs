use std::collections::{HashMap, hash_map::DefaultHasher};
use std::hash::{Hash, Hasher};
use glossia_shared::{SimplificationResponse, ImageResult};

/// Service for managing different types of caches
#[derive(Clone, Default)]
pub struct CacheManager {
    simplified_cache: HashMap<String, SimplificationResponse>,
    image_cache: HashMap<String, Vec<ImageResult>>,
    optimized_query_cache: HashMap<String, String>, // context_key -> optimized query
    word_meaning_cache: HashMap<String, String>, // word -> meaning
}

impl CacheManager {
    pub fn new() -> Self {
        Self::default()
    }

    // Simplification cache methods
    pub fn get_simplified(&self, sentence: &str) -> Option<SimplificationResponse> {
        self.simplified_cache.get(sentence).cloned()
    }

    pub fn cache_simplified(&mut self, sentence: String, response: SimplificationResponse) {
        self.simplified_cache.insert(sentence, response);
    }

    // Image cache methods
    pub fn get_images(&self, word: &str) -> Option<Vec<ImageResult>> {
        self.image_cache.get(word).cloned()
    }

    pub fn cache_images(&mut self, word: String, images: Vec<ImageResult>) {
        self.image_cache.insert(word, images);
    }

    // Optimized query cache methods
    pub fn get_optimized_query(&self, context_key: &str) -> Option<String> {
        self.optimized_query_cache.get(context_key).cloned()
    }

    pub fn cache_optimized_query(&mut self, context_key: String, query: String) {
        self.optimized_query_cache.insert(context_key, query);
    }

    // Word meaning cache methods
    pub fn get_word_meaning(&self, word: &str) -> Option<String> {
        self.word_meaning_cache.get(word).cloned()
    }

    pub fn cache_word_meaning(&mut self, word: String, meaning: String) {
        self.word_meaning_cache.insert(word.to_lowercase(), meaning);
    }

    // Helper method to generate context-aware cache key
    pub fn generate_context_key(&self, word: &str, sentence_context: &str) -> String {
        let mut hasher = DefaultHasher::new();
        word.hash(&mut hasher);
        sentence_context.hash(&mut hasher);
        format!("{}_{:x}", word, hasher.finish())
    }

    /// Clear all caches (useful when loading new text)
    pub fn clear_all(&mut self) {
        self.simplified_cache.clear();
        self.image_cache.clear();
        self.optimized_query_cache.clear();
        self.word_meaning_cache.clear();
    }

    /// Clear only text-related caches (keep image cache for reuse)
    pub fn clear_text_caches(&mut self) {
        self.simplified_cache.clear();
        self.optimized_query_cache.clear();
        // Keep word meanings cache as words can be reused across texts
    }
}
