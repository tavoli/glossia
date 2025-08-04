use dioxus::prelude::*;
use glossia_book_reader::ReadingState;
use glossia_shared::{AppError, SimplificationResponse};

/// Custom hook for managing sentence simplification with caching and proactive fetching
pub fn use_simplification(
    mut reading_state: Signal<ReadingState>,
    sentence_to_fetch: Signal<String>,
) -> Resource<Option<Result<SimplificationResponse, AppError>>> {
    use_resource(move || {
        let sentence = sentence_to_fetch.read().clone();
        async move {
            if sentence.is_empty() {
                return None;
            }
            
            // Check cache first
            let cached_result = {
                let state_read = reading_state.read();
                state_read.cache.get_simplified(&sentence)
            };
            
            if let Some(cached) = cached_result {
                // Proactively fetch next sentence while we're here
                let should_fetch_next = {
                    let state = reading_state.read();
                    if state.position() + 1 < state.sentences().len() {
                        let next_sentence = &state.sentences()[state.position() + 1];
                        state.cache.get_simplified(next_sentence).is_none()
                    } else {
                        false
                    }
                };
                
                if should_fetch_next {
                    let (next_sentence, api_client) = {
                        let state = reading_state.read();
                        (state.sentences()[state.position() + 1].clone(), state.api_client.clone())
                    };
                    
                    let mut reading_state_for_proactive = reading_state.clone();
                    spawn(async move {
                        let request = glossia_shared::SimplificationRequest { sentence: next_sentence.clone() };
                        if let Ok(result) = api_client.simplify(request).await {
                            let mut state = reading_state_for_proactive.write();
                            state.cache.cache_simplified(next_sentence, result);
                        }
                    });
                }
                
                return Some(Ok(cached));
            }

            // Fetch from API
            let result = {
                let api_client = reading_state.read().api_client.clone();
                api_client.simplify(glossia_shared::SimplificationRequest { sentence: sentence.clone() }).await
            };
            
            // Cache the result if successful
            if let Ok(ref res) = result {
                reading_state.write().cache.cache_simplified(sentence.clone(), res.clone());
            }
            
            Some(result)
        }
    })
}

/// Helper function to trigger sentence fetching
pub fn trigger_sentence_fetch(
    reading_state: Signal<ReadingState>,
    mut sentence_to_fetch: Signal<String>,
) {
    if let Some(sentence) = reading_state.read().current_sentence() {
        sentence_to_fetch.set(sentence);
    }
}
