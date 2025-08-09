use dioxus::prelude::*;
use glossia_reading_engine::ReadingEngine;
use glossia_shared::{AppError, SimplificationResponse};


/// Custom hook for managing sentence simplification with caching and proactive fetching
pub fn use_simplification(
    mut reading_state: Signal<ReadingEngine>,
    sentence_to_fetch: Signal<String>,
) -> Resource<Option<Result<SimplificationResponse, AppError>>> {
    use_resource(move || {
        let sentence = sentence_to_fetch.read().clone();
        async move {
            if sentence.is_empty() {
                return None;
            }
            
            // Check cache first (read-only operation)
            let cached_result = reading_state.read().get_cached_simplified(&sentence);
            
            if let Some(cached) = cached_result {
                // Proactively fetch next sentence while we're here
                let (should_fetch_next, current_pos) = {
                    let state = reading_state.read();
                    (state.position() + 1 < state.total_sentences(), state.position())
                };
                
                if should_fetch_next {
                    // Get next sentence without modifying position
                    let next_sentence = {
                        let state = reading_state.read();
                        state.get_sentence_at_position(current_pos + 1).unwrap_or_default()
                    };
                    
                    // Only fetch if not cached
                    if reading_state.read().get_cached_simplified(&next_sentence).is_none() {
                        let mut reading_state_for_proactive = reading_state.clone();
                        let next_sentence_clone = next_sentence.clone();
                        spawn(async move {
                            // Double-check cache (since some time may have passed)
                            if reading_state_for_proactive.read().get_cached_simplified(&next_sentence_clone).is_some() {
                                return; // Already cached by another operation
                            }
                            
                            // Use static method to avoid holding any borrow across await
                            let response = glossia_reading_engine::ReadingEngine::simplify_sentence_static(&next_sentence_clone).await;
                            // Cache the result afterwards (borrow is dropped from above block)
                            if let Ok(response) = response {
                                reading_state_for_proactive.write().cache_simplification_result(next_sentence_clone, response);
                            }
                        });
                    }
                }
                
                return Some(Ok(cached));
            }

            // Fetch from API without holding any borrow
            let result: Result<SimplificationResponse, AppError> = 
                glossia_reading_engine::ReadingEngine::simplify_sentence_static(&sentence).await;
            
            // Cache the result if successful (separate mutable operation, borrow is dropped from above block)
            if let Ok(ref response) = result {
                reading_state.write().cache_simplification_result(sentence.clone(), response.clone());
            }
            
            Some(result)
        }
    })
}

/// Helper function to trigger sentence fetching
#[allow(dead_code)]
pub fn trigger_sentence_fetch(
    reading_state: Signal<ReadingEngine>,
    mut sentence_to_fetch: Signal<String>,
) {
    if let Some(sentence) = reading_state.read().current_sentence() {
        sentence_to_fetch.set(sentence);
    }
}
