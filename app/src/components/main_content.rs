use dioxus::prelude::*;
use crate::components::{ErrorDisplay};
use crate::components::features::reading::{ContentDisplay, SentenceProcessor};
use crate::hooks::{use_simplification, VocabularyState};
use crate::theme::Theme;
use std::collections::HashSet;

#[component]
pub fn MainContent(
    reading_state: Signal<glossia_reading_engine::ReadingEngine>,
    vocabulary_state: Signal<VocabularyState>,
    sentence_to_fetch: Signal<String>,
    word_to_fetch: Signal<String>,
    encounter_tracked_sentences: Signal<HashSet<String>>,
    promotion_notification: Signal<Option<String>>,
    theme: Theme,
) -> Element {
    // Use the simplification hook
    let future_simplification = use_simplification(reading_state, sentence_to_fetch);
    
    // Navigation handlers
    let mut reading_state_next = reading_state.clone();
    let mut reading_state_prev = reading_state.clone();
    let mut sentence_to_fetch_next = sentence_to_fetch.clone();
    let mut sentence_to_fetch_prev = sentence_to_fetch.clone();
    
    let on_next = move |_| {
        reading_state_next.write().next();
        if let Some(sentence) = reading_state_next.read().current_sentence() {
            sentence_to_fetch_next.set(sentence);
        }
    };
    
    let on_prev = move |_| {
        reading_state_prev.write().previous();
        if let Some(sentence) = reading_state_prev.read().current_sentence() {
            sentence_to_fetch_prev.set(sentence);
        }
    };
    
    rsx! {
        div {
            class: "centered-container",
            style: "flex-grow: 1; display: flex; align-items: center; justify-content: center; flex-direction: column; position: relative; z-index: 10;",
            
            {
                let current_sentence = reading_state.read().current_sentence();
                let current_sentence_str = current_sentence.clone().unwrap_or_default();
                let cached_result = reading_state.read().get_cached_simplified(&current_sentence_str);
                
                // Determine current state
                let sentence_being_fetched = sentence_to_fetch.read().clone();
                let has_error = if let Some(Some(Err(_))) = future_simplification.read().as_ref() {
                    sentence_being_fetched == current_sentence_str
                } else {
                    false
                };
                
                let is_loading = !sentence_being_fetched.is_empty() && 
                               sentence_being_fetched == current_sentence_str &&
                               cached_result.is_none() &&
                               !has_error;
                
                rsx! {
                    // Process sentence for word tracking
                    SentenceProcessor {
                        current_sentence: current_sentence_str.clone(),
                        cached_result: cached_result.clone(),
                        encounter_tracked_sentences: encounter_tracked_sentences,
                        vocabulary_state: vocabulary_state.clone(),
                        promotion_notification: promotion_notification,
                    }
                    // Error state
                    if has_error {
                        if let Some(Some(Err(e))) = future_simplification.read().as_ref() {
                            ErrorDisplay { 
                                error: e.clone(),
                                theme: theme.clone()
                            }
                        }
                    }
                    
                    // Content display directly (loading is handled within ReadingLayout now)
                    ContentDisplay {
                        original: current_sentence.clone(),
                        simplified: cached_result.as_ref().map(|r| r.simplified.clone()),
                        words: cached_result.as_ref().map(|r| r.words.clone()).unwrap_or_default(),
                        is_loading,
                        theme: theme.clone(),
                        reading_state: reading_state,
                        vocabulary_state: vocabulary_state,
                        word_to_fetch: word_to_fetch,
                        on_next: on_next,
                        on_prev: on_prev,
                    }
                }
            }
        }
    }
}