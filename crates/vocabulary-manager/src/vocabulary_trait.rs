use async_trait::async_trait;
use glossia_shared::{AppError, WordMeaning};
use std::collections::HashSet;

/// Trait for vocabulary storage backends
/// Enables different storage implementations (memory, file, database, cloud)
#[async_trait]
pub trait VocabularyStore: Send + Sync {
    /// Add a word encounter and return (new_count, became_known)
    async fn add_word_encounter(&mut self, word: &str) -> Result<(usize, bool), AppError>;
    
    /// Add a word to known words manually
    async fn add_known_word(&mut self, word: &str) -> Result<(), AppError>;
    
    /// Remove a word from known words
    async fn remove_known_word(&mut self, word: &str) -> Result<(), AppError>;
    
    /// Get all known words
    async fn get_all_known_words(&self) -> Result<Vec<String>, AppError>;
    
    /// Get known words count
    async fn get_known_words_count(&self) -> usize;
    
    /// Filter out known words from a word list
    fn filter_known_words(&self, words: &[WordMeaning]) -> Vec<WordMeaning>;
    
    /// Add a manual word selection (temporary, session-based)
    fn add_manual_word(&mut self, word: String);
    
    /// Remove a manual word selection
    fn remove_manual_word(&mut self, word: &str);
    
    /// Check if a word is manually selected
    fn is_manual_word(&self, word: &str) -> bool;
    
    /// Get all manual words
    fn get_manual_words(&self) -> &HashSet<String>;
    
    /// Clear all manual words
    fn clear_manual_words(&mut self);
    
    /// Get combined words (from API + manual) for display
    fn get_combined_words(&self, api_words: &[WordMeaning]) -> Vec<WordMeaning>;
    
    /// Save vocabulary state (for persistent backends)
    async fn save(&self) -> Result<(), AppError>;
    
    /// Load vocabulary state (for persistent backends)
    async fn load(&mut self) -> Result<(), AppError>;
    
    /// Get storage backend name for debugging
    fn backend_name(&self) -> &str;
}

/// Memory-based vocabulary store (current implementation)
pub struct MemoryVocabularyStore {
    known_words: HashSet<String>,
    word_counts: std::collections::HashMap<String, usize>,
    manual_words: HashSet<String>,
    threshold: usize,
}

impl MemoryVocabularyStore {
    pub fn new() -> Self {
        Self {
            known_words: HashSet::new(),
            word_counts: std::collections::HashMap::new(),
            manual_words: HashSet::new(),
            threshold: 3, // Configurable threshold for automatic known words
        }
    }
    
    pub fn with_threshold(mut self, threshold: usize) -> Self {
        self.threshold = threshold;
        self
    }
}

impl Default for MemoryVocabularyStore {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl VocabularyStore for MemoryVocabularyStore {
    async fn add_word_encounter(&mut self, word: &str) -> Result<(usize, bool), AppError> {
        let normalized = word.to_lowercase();
        let count = self.word_counts.entry(normalized.clone()).and_modify(|c| *c += 1).or_insert(1);
        let new_count = *count;
        
        let became_known = if new_count >= self.threshold && !self.known_words.contains(&normalized) {
            self.known_words.insert(normalized);
            true
        } else {
            false
        };
        
        Ok((new_count, became_known))
    }
    
    async fn add_known_word(&mut self, word: &str) -> Result<(), AppError> {
        self.known_words.insert(word.to_lowercase());
        Ok(())
    }
    
    async fn remove_known_word(&mut self, word: &str) -> Result<(), AppError> {
        self.known_words.remove(&word.to_lowercase());
        Ok(())
    }
    
    async fn get_all_known_words(&self) -> Result<Vec<String>, AppError> {
        Ok(self.known_words.iter().cloned().collect())
    }
    
    async fn get_known_words_count(&self) -> usize {
        self.known_words.len()
    }
    
    fn filter_known_words(&self, words: &[WordMeaning]) -> Vec<WordMeaning> {
        words
            .iter()
            .filter(|word| !self.known_words.contains(&word.word.to_lowercase()))
            .cloned()
            .collect()
    }
    
    fn add_manual_word(&mut self, word: String) {
        self.manual_words.insert(word.to_lowercase());
    }
    
    fn remove_manual_word(&mut self, word: &str) {
        self.manual_words.remove(&word.to_lowercase());
    }
    
    fn is_manual_word(&self, word: &str) -> bool {
        self.manual_words.contains(&word.to_lowercase())
    }
    
    fn get_manual_words(&self) -> &HashSet<String> {
        &self.manual_words
    }
    
    fn clear_manual_words(&mut self) {
        self.manual_words.clear();
    }
    
    fn get_combined_words(&self, api_words: &[WordMeaning]) -> Vec<WordMeaning> {
        let mut combined = api_words.to_vec();
        
        // Add manual words that aren't already in the API response
        for manual_word in &self.manual_words {
            if !api_words.iter().any(|w| w.word.to_lowercase() == *manual_word) {
                combined.push(WordMeaning {
                    word: manual_word.clone(),
                    meaning: "Loading...".to_string(),
                    is_phrase: false,
                    timestamp: None,
                });
            }
        }
        
        combined
    }
    
    async fn save(&self) -> Result<(), AppError> {
        // Memory store doesn't persist - this is a no-op
        Ok(())
    }
    
    async fn load(&mut self) -> Result<(), AppError> {
        // Memory store doesn't persist - this is a no-op
        Ok(())
    }
    
    fn backend_name(&self) -> &str {
        "Memory"
    }
}

/// File-based vocabulary store for persistence
pub struct FileVocabularyStore {
    memory_store: MemoryVocabularyStore,
    file_path: std::path::PathBuf,
}

impl FileVocabularyStore {
    pub fn new(file_path: impl Into<std::path::PathBuf>) -> Self {
        Self {
            memory_store: MemoryVocabularyStore::new(),
            file_path: file_path.into(),
        }
    }
}

#[async_trait]
impl VocabularyStore for FileVocabularyStore {
    async fn add_word_encounter(&mut self, word: &str) -> Result<(usize, bool), AppError> {
        self.memory_store.add_word_encounter(word).await
    }
    
    async fn add_known_word(&mut self, word: &str) -> Result<(), AppError> {
        self.memory_store.add_known_word(word).await
    }
    
    async fn remove_known_word(&mut self, word: &str) -> Result<(), AppError> {
        self.memory_store.remove_known_word(word).await
    }
    
    async fn get_all_known_words(&self) -> Result<Vec<String>, AppError> {
        self.memory_store.get_all_known_words().await
    }
    
    async fn get_known_words_count(&self) -> usize {
        self.memory_store.get_known_words_count().await
    }
    
    fn filter_known_words(&self, words: &[WordMeaning]) -> Vec<WordMeaning> {
        self.memory_store.filter_known_words(words)
    }
    
    fn add_manual_word(&mut self, word: String) {
        self.memory_store.add_manual_word(word);
    }
    
    fn remove_manual_word(&mut self, word: &str) {
        self.memory_store.remove_manual_word(word);
    }
    
    fn is_manual_word(&self, word: &str) -> bool {
        self.memory_store.is_manual_word(word)
    }
    
    fn get_manual_words(&self) -> &HashSet<String> {
        self.memory_store.get_manual_words()
    }
    
    fn clear_manual_words(&mut self) {
        self.memory_store.clear_manual_words();
    }
    
    fn get_combined_words(&self, api_words: &[WordMeaning]) -> Vec<WordMeaning> {
        self.memory_store.get_combined_words(api_words)
    }
    
    async fn save(&self) -> Result<(), AppError> {
        use std::collections::HashMap;
        
        #[derive(serde::Serialize)]
        struct VocabularyData {
            known_words: HashSet<String>,
            word_counts: HashMap<String, usize>,
        }
        
        let data = VocabularyData {
            known_words: self.memory_store.known_words.clone(),
            word_counts: self.memory_store.word_counts.clone(),
        };
        
        let json = serde_json::to_string_pretty(&data)
            .map_err(|e| AppError::config_error(format!("Failed to serialize vocabulary: {e}")))?;
        
        if let Some(parent) = self.file_path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| AppError::config_error(format!("Failed to create directory: {e}")))?;
        }
        
        std::fs::write(&self.file_path, json)
            .map_err(|e| AppError::config_error(format!("Failed to write vocabulary file: {e}")))?;
        
        Ok(())
    }
    
    async fn load(&mut self) -> Result<(), AppError> {
        use std::collections::HashMap;
        
        #[derive(serde::Deserialize)]
        struct VocabularyData {
            known_words: HashSet<String>,
            word_counts: HashMap<String, usize>,
        }
        
        if !self.file_path.exists() {
            return Ok(()); // No file to load, start fresh
        }
        
        let json = std::fs::read_to_string(&self.file_path)
            .map_err(|e| AppError::config_error(format!("Failed to read vocabulary file: {e}")))?;
        
        let data: VocabularyData = serde_json::from_str(&json)
            .map_err(|e| AppError::config_error(format!("Failed to deserialize vocabulary: {e}")))?;
        
        self.memory_store.known_words = data.known_words;
        self.memory_store.word_counts = data.word_counts;
        
        Ok(())
    }
    
    fn backend_name(&self) -> &str {
        "File"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    
    #[tokio::test]
    async fn test_memory_store_word_encounter() {
        let mut store = MemoryVocabularyStore::new().with_threshold(2);
        
        let (count, became_known) = store.add_word_encounter("test").await.unwrap();
        assert_eq!(count, 1);
        assert!(!became_known);
        
        let (count, became_known) = store.add_word_encounter("test").await.unwrap();
        assert_eq!(count, 2);
        assert!(became_known);
        
        assert_eq!(store.get_known_words_count().await, 1);
    }
    
    #[tokio::test]
    async fn test_memory_store_manual_words() {
        let mut store = MemoryVocabularyStore::new();
        
        store.add_manual_word("manual".to_string());
        assert!(store.is_manual_word("manual"));
        assert_eq!(store.get_manual_words().len(), 1);
        
        store.remove_manual_word("manual");
        assert!(!store.is_manual_word("manual"));
        assert_eq!(store.get_manual_words().len(), 0);
    }
    
    #[tokio::test]
    async fn test_file_store_persistence() {
        let temp_file = NamedTempFile::new().unwrap();
        let mut store = FileVocabularyStore::new(temp_file.path());
        
        store.add_known_word("test").await.unwrap();
        store.save().await.unwrap();
        
        let mut new_store = FileVocabularyStore::new(temp_file.path());
        new_store.load().await.unwrap();
        
        let known_words = new_store.get_all_known_words().await.unwrap();
        assert!(known_words.contains(&"test".to_string()));
    }
}
