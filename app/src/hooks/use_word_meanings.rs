use dioxus::prelude::*;
use glossia_reading_engine::ReadingEngine;
use glossia_shared::AppError;
use tracing::{info, warn, error, debug};

/// Custom hook for fetching word meanings for manually selected words
pub fn use_word_meanings(
    mut reading_state: Signal<ReadingEngine>,
    word_to_fetch: Signal<String>,
) -> Resource<Option<Result<String, AppError>>> {
    use_resource(move || {
        let word = word_to_fetch.read().clone();
        async move {
            if word.is_empty() {
                debug!("use_word_meanings: No word to fetch (empty string)");
                return None;
            }
            
            info!("use_word_meanings: Starting fetch for word: '{}'", word);
            
            // Check cache first (read-only operation)
            let cached_result = reading_state.read().get_cached_word_meaning(&word);
            
            if let Some(cached) = cached_result {
                if !cached.trim().is_empty() {
                    info!("use_word_meanings: Cache HIT for word '{}', returning cached meaning", word);
                    return Some(Ok(cached));
                } else {
                    warn!("use_word_meanings: Cache hit but empty meaning for word '{}'", word);
                }
            } else {
                info!("use_word_meanings: Cache MISS for word '{}', will fetch from API", word);
            }

            // Get current sentence for context (read-only operation)
            let context = reading_state.read().current_sentence().unwrap_or_default();
            debug!("use_word_meanings: Using context for '{}': {}", word, context);

            // Fetch from API without holding any borrow
            info!("use_word_meanings: Making API call for word '{}'", word);
            let result: Result<String, AppError> = 
                glossia_reading_engine::ReadingEngine::get_word_meaning_static(&word, &context).await;
            
            // Cache the result if successful (separate mutable operation)
            match &result {
                Ok(meaning) => {
                    info!("use_word_meanings: API SUCCESS for word '{}', meaning length: {} chars", word, meaning.len());
                    debug!("use_word_meanings: Caching meaning for '{}'", word);
                    reading_state.write().cache_word_meaning_result(word.clone(), meaning.clone());
                    info!("use_word_meanings: Successfully cached meaning for '{}'", word);
                }
                Err(e) => {
                    error!("use_word_meanings: API FAILED for word '{}': {}", word, e);
                }
            }
            
            Some(result)
        }
    })
}

/// Helper function to trigger word meaning fetch
pub fn trigger_word_meaning_fetch(
    word: String,
    mut word_to_fetch: Signal<String>,
) {
    if !word.is_empty() {
        word_to_fetch.set(word);
    }
}