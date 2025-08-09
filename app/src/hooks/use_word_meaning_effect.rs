use dioxus::prelude::*;
use crate::hooks::AppState;

/// Manages word meaning fetching effects
pub fn use_word_meaning_effect(app_state: &mut AppState) {
    let word_meaning_result = crate::hooks::use_word_meanings(
        app_state.reading_state, 
        app_state.word_to_fetch
    );
    
    let mut word_to_fetch = app_state.word_to_fetch.clone();
    
    use_effect(move || {
        match word_meaning_result.read().as_ref() {
            Some(Some(Ok(meaning))) => {
                let current_word = word_to_fetch.read().clone();
                if !current_word.is_empty() && !meaning.is_empty() {
                    tracing::info!(
                        "Word meaning fetched successfully for '{}', clearing word_to_fetch", 
                        current_word
                    );
                    word_to_fetch.set(String::new());
                }
            }
            Some(Some(Err(e))) => {
                let current_word = word_to_fetch.read().clone();
                tracing::error!(
                    "Failed to fetch meaning for word '{}': {}", 
                    current_word, 
                    e
                );
                // Clear even on error to prevent infinite retries
                word_to_fetch.set(String::new());
            }
            _ => {}
        }
    });
}