#![allow(non_snake_case)]
mod components;
mod theme;
mod utils;

use dioxus::prelude::*;
use components::{FloatingButton, ProgressBar, ReadingContainer, TextInputModal, ThemeToggle, WordMeanings, WordMeaningsStyles};
use theme::{use_theme, Theme};
use glossia_book_reader::ReadingState;
use glossia_shared::AppError;

fn main() {
    dioxus_desktop::launch::launch(App, vec![], Default::default());
}

fn user_friendly_error(error: &AppError) -> String {
    match error {
        AppError::ApiError(msg) if msg.contains("404") => {
            "The AI service is temporarily unavailable. Please try again later.".to_string()
        },
        AppError::ApiError(msg) if msg.contains("401") || msg.contains("403") => {
            "Authentication error with the AI service. Please check your connection.".to_string()
        },
        AppError::ApiError(msg) if msg.contains("timeout") || msg.contains("network") => {
            "Network connection issue. Please check your internet connection.".to_string()
        },
        AppError::ParseError(_) => {
            "The AI response couldn't be processed. Please try again.".to_string()
        },
        AppError::InvalidResponseContent => {
            "The AI service returned an unexpected response. Please try again.".to_string()
        },
        AppError::EmptyBook => {
            "No text to process. Please add some text first.".to_string()
        },
        _ => {
            "Something went wrong. Please try again.".to_string()
        }
    }
}

#[component]
fn App() -> Element {
    // Single state for everything
    let mut reading_state = use_signal(ReadingState::new);
    let mut show_input_modal = use_signal(|| true);
    let mut sentence_to_fetch = use_signal(String::new);
    
    // Theme state
    let mut theme_mode = use_theme();
    let theme = Theme::from_mode(*theme_mode.read());
    
    let future_simplification = use_resource(move || {
        let sentence = sentence_to_fetch.read().clone();
        async move {
            if sentence.is_empty() {
                return None;
            }
            
            
            // Check cache first
            let cached_result = {
                let state_read = reading_state.read();
                let cached = state_read.simplified_cache.get(&sentence).cloned();
                cached
            };
            
            if let Some(cached) = cached_result {
                
                // While we're here, check if we should proactively fetch next sentence
                let should_fetch_next = {
                    let state = reading_state.read();
                    if state.position + 1 < state.sentences.len() {
                        let next_sentence = &state.sentences[state.position + 1];
                        state.simplified_cache.get(next_sentence).is_none()
                    } else {
                        false
                    }
                };
                
                if should_fetch_next {
                    let (next_sentence, api_client) = {
                        let state = reading_state.read();
                        (state.sentences[state.position + 1].clone(), state.api_client.clone())
                    };
                    
                    let mut reading_state_for_proactive = reading_state.clone();
                    spawn(async move {
                        let request = glossia_shared::SimplificationRequest { sentence: next_sentence.clone() };
                        if let Ok(result) = api_client.simplify(request).await {
                            let mut state = reading_state_for_proactive.write();
                            state.simplified_cache.insert(next_sentence, result);
                            drop(state); // Explicitly drop the write lock
                        }
                    });
                }
                
                return Some(Ok(cached));
            }

            // Fetch from API using cloned client
            let result = {
                let api_client = reading_state.read().api_client.clone();
                api_client.simplify(glossia_shared::SimplificationRequest { sentence: sentence.clone() }).await
            };
            
            
            // Cache the result if successful
            if let Ok(ref res) = result {
                reading_state.write().simplified_cache.insert(sentence.clone(), res.clone());
            }
            
            Some(result)
        }
    });


    let on_next = move |_| {
        // Perform write operation and explicitly drop the lock
        {
            reading_state.write().next();
        } // Write lock is dropped here
        
        // Now safely perform read operation
        if let Some(sentence) = reading_state.read().current_sentence() {
            sentence_to_fetch.set(sentence);
        }
    };
    
    let on_prev = move |_| {
        // Perform write operation and explicitly drop the lock
        {
            reading_state.write().previous();
        } // Write lock is dropped here
        
        // Now safely perform read operation
        if let Some(sentence) = reading_state.read().current_sentence() {
            sentence_to_fetch.set(sentence);
        }
    };

    rsx! {
        style { "body {{ margin: 0; padding: 0; background: {theme.background}; color: {theme.text_primary}; }}" }
        WordMeaningsStyles { theme: theme.clone() }
        ThemeToggle { 
            theme_mode, 
            on_toggle: move |_| {
                let new_mode = match *theme_mode.read() {
                    crate::theme::ThemeMode::Light => crate::theme::ThemeMode::Dark,
                    crate::theme::ThemeMode::Dark => crate::theme::ThemeMode::Light,
                };
                theme_mode.set(new_mode);
            }
        }
        div {
            class: "app-container",
            style: "min-height: 100vh; width: 100%; background: {theme.background}; font-family: Georgia, serif; display: flex; flex-direction: column; position: relative;",
            
            // Theme-aware grid background
            div {
                style: "position: absolute; inset: 0; z-index: 1; background: {theme.background}; background-image: linear-gradient(to right, {theme.border} 1px, transparent 1px), linear-gradient(to bottom, {theme.border} 1px, transparent 1px), radial-gradient(circle at 50% 60%, rgba(236,72,153,0.15) 0%, rgba(168,85,247,0.05) 40%, transparent 70%); background-size: 40px 40px, 40px 40px, 100% 100%;"
            }
            
            ProgressBar { 
                current: if reading_state().total_sentences > 0 { reading_state().position + 1 } else { 0 },
                total: reading_state().total_sentences,
                theme: theme.clone()
            }
            
            div {
                class: "centered-container",
                style: "flex-grow: 1; display: flex; align-items: center; justify-content: center; flex-direction: column; position: relative; z-index: 10;",
                
                {
                    let current_sentence = reading_state().current_sentence();
                    let current_sentence_str = current_sentence.clone().unwrap_or_default();
                    
                    // Check if we have a cached result for current sentence
                    let cached_result = reading_state.read().simplified_cache.get(&current_sentence_str).cloned();
                    
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
                                div {
                                    class: "error-message",
                                    style: "background: {theme.error_bg}; color: {theme.error}; padding: 15px; border-radius: 8px; margin-bottom: 20px; border: 1px solid {theme.border}; text-align: center;",
                                    "⚠️ {user_friendly_error(e)}"
                                }
                            }
                        }
                        
                        ReadingContainer {
                            original: current_sentence,
                            simplified: cached_result.as_ref().map(|r| r.simplified.clone()),
                            is_loading,
                            words: cached_result.as_ref().map(|r| r.words.clone()),
                            theme: theme.clone(),
                            on_next,
                            on_prev,
                        }
                        
                        if let Some(ref cached) = cached_result {
                            WordMeanings { 
                                words: cached.words.clone(),
                                reading_state: reading_state,
                                current_sentence: current_sentence_str.clone(),
                                theme: theme.clone(),
                                on_expand_word: move |_word: String| {}
                            }
                        }
                    }
                }
            }
            
            // Show floating button if there are sentences, or if modal is closed but no text loaded
            if reading_state().total_sentences > 0 || !show_input_modal() {
                FloatingButton {
                    count: if reading_state().total_sentences > 0 {
                        reading_state().total_sentences.saturating_sub(reading_state().position + 1)
                    } else {
                        0 // Show 0 when no text is loaded
                    },
                    onclick: move |_| show_input_modal.set(true)
                }
            }
            
            if show_input_modal() {
                TextInputModal {
                    onsubmit: move |text: String| {
                        if !text.is_empty() {
                            // Update single state - perform write operation and explicitly drop the lock
                            {
                                reading_state.write().load_text(&text);
                            } // Write lock is dropped here
                            
                            // Now safely perform read operation
                            if let Some(sentence) = reading_state.read().current_sentence() {
                                sentence_to_fetch.set(sentence);
                            }
                            show_input_modal.set(false);
                        }
                    },
                    onclose: move |_| show_input_modal.set(false)
                }
            }
        }
    }
}
