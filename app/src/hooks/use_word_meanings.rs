use dioxus::prelude::*;
use glossia_book_reader::ReadingState;
use glossia_shared::AppError;

/// Custom hook for fetching word meanings for manually selected words
pub fn use_word_meanings(
    mut reading_state: Signal<ReadingState>,
    word_to_fetch: Signal<String>,
) -> Resource<Option<Result<String, AppError>>> {
    use_resource(move || {
        let word = word_to_fetch.read().clone();
        async move {
            if word.is_empty() {
                return None;
            }
            
            // Check cache first
            let cached_result = {
                let state_read = reading_state.read();
                state_read.cache.get_word_meaning(&word)
            };
            
            if let Some(cached) = cached_result {
                return Some(Ok(cached));  
            }

            // Get current sentence for context
            let context = {
                let state = reading_state.read();
                state.current_sentence().unwrap_or_default()
            };

            // Fetch from API
            let result = {
                let api_client = reading_state.read().api_client.clone();
                api_client.get_word_meaning(&word, &context).await
            };
            
            // Cache the result if successful
            if let Ok(ref meaning) = result {
                reading_state.write().cache.cache_word_meaning(word.clone(), meaning.clone());
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