use glossia_shared::{SimplificationResponse, ImageResult};
use std::collections::HashMap;

/// Centralized cache management for reading engine
pub struct CacheEngine {
    simplified_cache: HashMap<String, SimplificationResponse>,
    image_cache: HashMap<String, Vec<ImageResult>>,
    word_meaning_cache: HashMap<String, String>,
    optimized_query_cache: HashMap<String, String>,
}

impl CacheEngine {
    pub fn new() -> Self {
        Self {
            simplified_cache: HashMap::new(),
            image_cache: HashMap::new(),
            word_meaning_cache: HashMap::new(),
            optimized_query_cache: HashMap::new(),
        }
    }

    /// Simplification cache methods
    pub fn get_simplified(&self, sentence: &str) -> Option<SimplificationResponse> {
        self.simplified_cache.get(sentence).cloned()
    }

    pub fn cache_simplified(&mut self, sentence: String, response: SimplificationResponse) {
        self.simplified_cache.insert(sentence, response);
    }

    pub fn has_simplified(&self, sentence: &str) -> bool {
        self.simplified_cache.contains_key(sentence)
    }

    /// Image cache methods
    pub fn get_images(&self, word: &str) -> Option<Vec<ImageResult>> {
        self.image_cache.get(word).cloned()
    }

    pub fn cache_images(&mut self, word: String, images: Vec<ImageResult>) {
        self.image_cache.insert(word, images);
    }

    pub fn has_images(&self, word: &str) -> bool {
        self.image_cache.contains_key(word)
    }

    /// Word meaning cache methods
    pub fn get_word_meaning(&self, word: &str) -> Option<String> {
        self.word_meaning_cache.get(word).cloned()
    }

    pub fn cache_word_meaning(&mut self, word: String, meaning: String) {
        self.word_meaning_cache.insert(word, meaning);
    }

    pub fn has_word_meaning(&self, word: &str) -> bool {
        self.word_meaning_cache.contains_key(word)
    }

    /// Optimized query cache methods
    pub fn get_optimized_query(&self, context_key: &str) -> Option<String> {
        self.optimized_query_cache.get(context_key).cloned()
    }

    pub fn cache_optimized_query(&mut self, context_key: String, query: String) {
        self.optimized_query_cache.insert(context_key, query);
    }

    pub fn has_optimized_query(&self, context_key: &str) -> bool {
        self.optimized_query_cache.contains_key(context_key)
    }

    /// Cache management
    pub fn clear_all_caches(&mut self) {
        self.simplified_cache.clear();
        self.image_cache.clear();
        self.word_meaning_cache.clear();
        self.optimized_query_cache.clear();
    }

    pub fn clear_text_caches(&mut self) {
        self.simplified_cache.clear();
        self.word_meaning_cache.clear();
        // Keep image cache for reuse across texts
    }

    pub fn clear_simplified_cache(&mut self) {
        self.simplified_cache.clear();
    }

    /// Cache statistics
    pub fn simplified_cache_size(&self) -> usize {
        self.simplified_cache.len()
    }

    pub fn image_cache_size(&self) -> usize {
        self.image_cache.len()
    }

    pub fn word_meaning_cache_size(&self) -> usize {
        self.word_meaning_cache.len()
    }

    /// Memory management
    pub fn cleanup_old_entries(&mut self, max_entries: usize) {
        if self.simplified_cache.len() > max_entries {
            // Keep only the most recent entries (simplified approach)
            let excess = self.simplified_cache.len() - max_entries;
            let keys_to_remove: Vec<String> = self.simplified_cache.keys().take(excess).cloned().collect();
            for key in keys_to_remove {
                self.simplified_cache.remove(&key);
            }
        }

        if self.image_cache.len() > max_entries {
            let excess = self.image_cache.len() - max_entries;
            let keys_to_remove: Vec<String> = self.image_cache.keys().take(excess).cloned().collect();
            for key in keys_to_remove {
                self.image_cache.remove(&key);
            }
        }

        if self.word_meaning_cache.len() > max_entries {
            let excess = self.word_meaning_cache.len() - max_entries;
            let keys_to_remove: Vec<String> = self.word_meaning_cache.keys().take(excess).cloned().collect();
            for key in keys_to_remove {
                self.word_meaning_cache.remove(&key);
            }
        }
    }
}

impl Default for CacheEngine {
    fn default() -> Self {
        Self::new()
    }
}
