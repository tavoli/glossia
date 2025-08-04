#![allow(non_snake_case)]
mod components;
mod theme;
mod utils;
mod hooks;
mod services;

use dioxus::prelude::*;
use components::{FloatingButton, ProgressBar, ReadingContainer, TextInputModal, ThemeToggle, WordMeanings, WordMeaningsStyles, ErrorDisplay};
use theme::{use_theme, Theme};
use hooks::{use_reading_state, use_navigation, use_simplification, trigger_sentence_fetch};


fn main() {
    dioxus_desktop::launch::launch(App, vec![], Default::default());
}



#[component]
fn App() -> Element {
    // Use custom hooks for cleaner state management
    let mut reading_state = use_reading_state();
    let mut show_input_modal = use_signal(|| true);
    let mut sentence_to_fetch = use_signal(String::new);
    
    // Theme state
    let mut theme_mode = use_theme();
    let theme = Theme::from_mode(*theme_mode.read());
    
    // Navigation hooks
    let (mut on_next, mut on_prev) = use_navigation(reading_state);
    
    // Use the custom simplification hook
    let future_simplification = use_simplification(reading_state, sentence_to_fetch);


    // Enhanced navigation with sentence fetching
    let enhanced_on_next = {
        let sentence_to_fetch = sentence_to_fetch.clone();
        let reading_state = reading_state.clone();
        move |_| {
            on_next();
            trigger_sentence_fetch(reading_state, sentence_to_fetch);
        }
    };
    
    let enhanced_on_prev = {
        let sentence_to_fetch = sentence_to_fetch.clone();
        let reading_state = reading_state.clone();
        move |_| {
            on_prev();
            trigger_sentence_fetch(reading_state, sentence_to_fetch);
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
                current: if reading_state().total_sentences() > 0 { reading_state().position() + 1 } else { 0 },
                total: reading_state().total_sentences(),
                theme: theme.clone()
            }
            
            div {
                class: "centered-container",
                style: "flex-grow: 1; display: flex; align-items: center; justify-content: center; flex-direction: column; position: relative; z-index: 10;",
                
                {
                    let current_sentence = reading_state().current_sentence();
                    let current_sentence_str = current_sentence.clone().unwrap_or_default();
                    
                    // Check if we have a cached result for current sentence
                    let cached_result = reading_state.read().cache.get_simplified(&current_sentence_str);
                    
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
                            words: cached_result.as_ref().map(|r| r.words.clone()),
                            theme: theme.clone(),
                            on_next: enhanced_on_next,
                            on_prev: enhanced_on_prev,
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
            if reading_state().total_sentences() > 0 || !show_input_modal() {
                FloatingButton {
                    count: if reading_state().total_sentences() > 0 {
                        reading_state().total_sentences().saturating_sub(reading_state().position() + 1)
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
