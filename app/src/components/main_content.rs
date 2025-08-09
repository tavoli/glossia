use dioxus::prelude::*;
use crate::components::{ReadingContainer, WordMeanings, ErrorDisplay};
use crate::hooks::{use_simplification, VocabularyState};
use crate::utils::word_utils::{get_display_words, track_word_encounters, handle_word_click, format_promotion_message};
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
    // Use the custom simplification hook
    let future_simplification = use_simplification(reading_state, sentence_to_fetch);
    
    // Clone signals for closures  
    let mut reading_state_next = reading_state.clone();
    let mut reading_state_prev = reading_state.clone();
    let mut reading_state_word_click = reading_state.clone();
    let mut vocabulary_state_word_click = vocabulary_state.clone();
    let mut sentence_to_fetch_next = sentence_to_fetch.clone();
    let mut sentence_to_fetch_prev = sentence_to_fetch.clone();
    let mut encounter_tracked_sentences_mut = encounter_tracked_sentences.clone();
    let mut vocabulary_state_encounter = vocabulary_state.clone();
    let mut promotion_notification_set = promotion_notification.clone();
    
    rsx! {
        div {
            class: "centered-container",
            style: "flex-grow: 1; display: flex; align-items: center; justify-content: center; flex-direction: column; position: relative; z-index: 10;",
            
            {
                let current_sentence = reading_state.read().current_sentence();
                let current_sentence_str = current_sentence.clone().unwrap_or_default();
                
                // Check if we have a cached result for current sentence
                let cached_result = reading_state.read().get_cached_simplified(&current_sentence_str);
                
                // Track encounters for words when we have a cached result (words are being displayed)
                if let Some(ref result) = cached_result {
                    let promoted_words = track_word_encounters(
                        &current_sentence_str,
                        &result.words,
                        &mut encounter_tracked_sentences_mut,
                        &mut vocabulary_state_encounter,
                    );
                    
                    // Show notification for promoted words
                    if let Some(notification_text) = format_promotion_message(&promoted_words) {
                        promotion_notification_set.set(Some(notification_text));
                        
                        // Clear notification after 3 seconds
                        let mut notification_clone = promotion_notification_set.clone();
                        spawn(async move {
                            tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
                            notification_clone.set(None);
                        });
                    }
                }
                
                // Check if there's an error for this sentence
                let sentence_being_fetched = sentence_to_fetch.read().clone();
                let has_error = if let Some(Some(Err(_))) = future_simplification.read().as_ref() {
                    sentence_being_fetched == current_sentence_str
                } else {
                    false
                };
                
                // Check if we're currently loading this sentence (but not if there's an error or cached result)
                let is_loading = !sentence_being_fetched.is_empty() && 
                               sentence_being_fetched == current_sentence_str &&
                               cached_result.is_none() &&
                               !has_error;
                
                rsx! {
                    if has_error {
                        if let Some(Some(Err(e))) = future_simplification.read().as_ref() {
                            ErrorDisplay { 
                                error: e.clone(),
                                theme: theme.clone()
                            }
                        }
                    }
                    
                    ReadingContainer {
                        original: current_sentence,
                        simplified: cached_result.as_ref().map(|r| r.simplified.clone()),
                        is_loading,
                        words: {
                            let empty_vec = Vec::new();
                            let api_words = cached_result.as_ref().map(|r| &r.words).unwrap_or(&empty_vec);
                            let filtered_words = get_display_words(api_words, &reading_state.read(), &vocabulary_state.read());
                            Some(filtered_words)
                        },
                        theme: theme.clone(),
                        on_next: move |_| {
                            reading_state_next.write().next();
                            if let Some(sentence) = reading_state_next.read().current_sentence() {
                                sentence_to_fetch_next.set(sentence);
                            }
                        },
                        on_prev: move |_| {
                            reading_state_prev.write().previous();
                            if let Some(sentence) = reading_state_prev.read().current_sentence() {
                                sentence_to_fetch_prev.set(sentence);
                            }
                        },
                        on_word_click: move |word: String| {
                            handle_word_click(
                                &word,
                                &mut reading_state_word_click,
                                &mut vocabulary_state_word_click,
                                word_to_fetch
                            );
                        }
                    }
                    
                    // Show word meanings if we have API words or manual words
                    {
                        let empty_vec = Vec::new();
                        let api_words = cached_result.as_ref().map(|r| &r.words).unwrap_or(&empty_vec);
                        let filtered_words = get_display_words(api_words, &reading_state.read(), &vocabulary_state.read());
                        
                        if !filtered_words.is_empty() {
                            rsx! {
                                WordMeanings { 
                                    words: filtered_words,
                                    reading_state: reading_state,
                                    current_sentence: current_sentence_str.clone(),
                                    theme: theme.clone(),
                                    on_expand_word: move |_word: String| {}
                                }
                            }
                        } else {
                            None
                        }
                    }
                }
            }
        }
    }
}
