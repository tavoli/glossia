use glossia_shared::{AppError, SimplificationResponse, SimplificationRequest};
use glossia_llm_client::{LLMClient, LLMClientFactory};
use crate::cache_engine::CacheEngine;

/// Orchestrates the high-level reading workflow
pub struct ReadingOrchestrator {
    llm_client: Box<dyn LLMClient>,
}

impl ReadingOrchestrator {
    pub fn new() -> Result<Self, AppError> {
        let factory = LLMClientFactory::new();
        Ok(Self {
            llm_client: factory.create_client()?,
        })
    }

    /// Create orchestrator with custom LLM client (useful for testing)
    pub fn with_llm_client(llm_client: Box<dyn LLMClient>) -> Self {
        Self { llm_client }
    }

    /// Process a sentence through the complete reading pipeline
    pub async fn process_sentence(
        &self,
        sentence: &str,
        cache: &mut CacheEngine,
    ) -> Result<SimplificationResponse, AppError> {
        // Check cache first
        if let Some(cached_response) = cache.get_simplified(sentence) {
            return Ok(cached_response);
        }

        // Process with LLM
        let request = SimplificationRequest {
            sentence: sentence.to_string(),
        };

        let response = self.llm_client.simplify(request).await?;

        // Cache the response
        cache.cache_simplified(sentence.to_string(), response.clone());

        Ok(response)
    }

    /// Process multiple sentences in batch
    pub async fn process_sentences_batch(
        &self,
        sentences: &[String],
        cache: &mut CacheEngine,
    ) -> Result<Vec<SimplificationResponse>, AppError> {
        let mut results = Vec::new();
        
        for sentence in sentences {
            let result = self.process_sentence(sentence, cache).await?;
            results.push(result);
        }

        Ok(results)
    }

    /// Preprocess upcoming sentences for better UX
    pub async fn preprocess_next_sentences(
        &self,
        current_position: usize,
        sentences: &[String],
        cache: &mut CacheEngine,
        lookahead: usize,
    ) -> Result<(), AppError> {
        let start = current_position + 1;
        let end = (start + lookahead).min(sentences.len());

        for i in start..end {
            if let Some(sentence) = sentences.get(i) {
                // Only process if not already cached
                if !cache.has_simplified(sentence) {
                    let _ = self.process_sentence(sentence, cache).await;
                }
            }
        }

        Ok(())
    }

    /// Check if a sentence needs processing
    pub fn needs_processing(&self, sentence: &str, cache: &CacheEngine) -> bool {
        !cache.has_simplified(sentence)
    }

    /// Get processing statistics
    pub fn get_cache_stats(&self, cache: &CacheEngine) -> CacheStats {
        CacheStats {
            simplified_entries: cache.simplified_cache_size(),
            image_entries: cache.image_cache_size(),
            word_meaning_entries: cache.word_meaning_cache_size(),
        }
    }
}

/// Statistics about cache usage
pub struct CacheStats {
    pub simplified_entries: usize,
    pub image_entries: usize,
    pub word_meaning_entries: usize,
}

impl CacheStats {
    pub fn total_entries(&self) -> usize {
        self.simplified_entries + self.image_entries + self.word_meaning_entries
    }
}
