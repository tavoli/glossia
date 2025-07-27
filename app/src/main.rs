#![allow(non_snake_case)]
mod components;
mod utils;

use dioxus::prelude::*;
use components::{FloatingButton, ProgressBar, ReadingContainer, TextInputModal, WordMeanings};
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
    
    let future_simplification = use_resource(move || {
        let sentence = sentence_to_fetch.read().clone();
        println!("Resource triggered with sentence: '{}'", sentence);
        async move {
            if sentence.is_empty() {
                println!("Sentence is empty, returning None");
                return None;
            }
            
            println!("Processing sentence: '{}'", sentence);
            
            // Check cache first
            let cached_result = {
                let state_read = reading_state.read();
                let cached = state_read.simplified_cache.get(&sentence).cloned();
                println!("Cache check for '{}': {}", sentence, if cached.is_some() { "found" } else { "not found" });
                cached
            };
            
            if let Some(cached) = cached_result {
                println!("Returning cached result for: '{}'", sentence);
                
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
                    
                    println!("Proactively fetching next sentence: '{}'", next_sentence);
                    let mut reading_state_clone = reading_state.clone();
                    spawn(async move {
                        let request = glossia_shared::SimplificationRequest { sentence: next_sentence.clone() };
                        if let Ok(result) = api_client.simplify(request).await {
                            reading_state_clone.write().simplified_cache.insert(next_sentence, result);
                            println!("Proactively cached next sentence");
                        }
                    });
                }
                
                return Some(Ok(cached));
            }

            println!("Making API call for: '{}'", sentence);
            // Fetch from API using cloned client
            let result = {
                let api_client = reading_state.read().api_client.clone();
                api_client.simplify(glossia_shared::SimplificationRequest { sentence: sentence.clone() }).await
            };
            
            println!("API call result: {:?}", result.is_ok());
            
            // Cache the result if successful
            if let Ok(ref res) = result {
                reading_state.write().simplified_cache.insert(sentence.clone(), res.clone());
                println!("Cached result for: '{}'", sentence);
            }
            
            Some(result)
        }
    });


    let on_next = move |_| {
        reading_state.write().next();
        if let Some(sentence) = reading_state.read().current_sentence() {
            println!("Setting sentence_to_fetch (next): '{}'", sentence);
            sentence_to_fetch.set(sentence);
        }
    };
    
    let on_prev = move |_| {
        reading_state.write().previous();
        if let Some(sentence) = reading_state.read().current_sentence() {
            println!("Setting sentence_to_fetch (prev): '{}'", sentence);
            sentence_to_fetch.set(sentence);
        }
    };

    rsx! {
        style { "body {{ margin: 0; padding: 0; }}" }
        div {
            class: "app-container",
            style: "height: 100vh; background: linear-gradient(to bottom, #f9f7f3, #f5f3ef); font-family: Georgia, serif; display: flex; flex-direction: column;",
            
            ProgressBar { 
                current: if reading_state().total_sentences > 0 { reading_state().position + 1 } else { 0 },
                total: reading_state().total_sentences 
            }
            
            div {
                class: "centered-container",
                style: "flex-grow: 1; display: flex; align-items: center; justify-content: center; flex-direction: column;",
                
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
                                    style: "background: #ffebee; color: #c62828; padding: 15px; border-radius: 8px; margin-bottom: 20px; border: 1px solid #ffcdd2; text-align: center;",
                                    "⚠️ {user_friendly_error(e)}"
                                }
                            }
                        }
                        
                        ReadingContainer {
                            original: current_sentence,
                            simplified: cached_result.as_ref().map(|r| r.simplified.clone()),
                            is_loading,
                            words: cached_result.as_ref().map(|r| r.words.clone()),
                            on_next,
                            on_prev,
                        }
                        
                        if let Some(ref cached) = cached_result {
                            WordMeanings { words: cached.words.clone() }
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
                            println!("Loading text: '{}'", text);
                            // Update single state
                            reading_state.write().load_text(&text);
                            
                            if let Some(sentence) = reading_state.read().current_sentence() {
                                println!("Setting initial sentence_to_fetch: '{}'", sentence);
                                sentence_to_fetch.set(sentence);
                            }
                            show_input_modal.set(false);
                        }
                    },
                    onclose: move |_| {
                        show_input_modal.set(false);
                    }
                }
            }
        }
    }
}
