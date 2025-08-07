#![allow(non_snake_case)]
mod components;
mod theme;
mod utils;
mod hooks;
mod services;
mod vocabulary;

use dioxus::prelude::*;
use components::{FloatingButton, ProgressBar, ReadingContainer, TextInputModal, ThemeToggle, WordMeanings, WordMeaningsStyles, ErrorDisplay, KnownWordsCounter, KnownWordsModal};
use theme::{use_theme, Theme};
use hooks::{use_reading_state, use_navigation, use_simplification, trigger_sentence_fetch, use_word_meanings, trigger_word_meaning_fetch, use_vocabulary};
use utils::word_utils::{is_word_already_difficult, get_display_words, format_promotion_message};


fn main() {
    dioxus_desktop::launch::launch(App, vec![], Default::default());
}



#[component]
fn App() -> Element {
    // Use custom hooks for cleaner state management
    let mut reading_state = use_reading_state();
    let mut show_input_modal = use_signal(|| true);
    let mut sentence_to_fetch = use_signal(String::new);
    let word_to_fetch = use_signal(String::new);
    
    // Theme state
    let mut theme_mode = use_theme();
    let theme = Theme::from_mode(*theme_mode.read());
    
    // Vocabulary state
    let mut vocabulary_state = use_vocabulary();
    let mut show_known_words_modal = use_signal(|| false);
    
    // Navigation hooks
    let (mut on_next, mut on_prev) = use_navigation(reading_state);
    
    // Use the custom simplification hook
    let future_simplification = use_simplification(reading_state, sentence_to_fetch);
    
    // Use the word meanings hook
    let _word_meaning_result = use_word_meanings(reading_state, word_to_fetch);
    
    // Track if we need to show a promotion notification
    let mut promotion_notification = use_signal(|| None::<String>);
    
    // Track which sentences have already had their word encounters recorded
    let mut encounter_tracked_sentences = use_signal(|| std::collections::HashSet::<String>::new());


    // Enhanced navigation with sentence fetching for buttons
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

    // Separate closures for keyboard navigation
    let mut keyboard_on_next = {
        let sentence_to_fetch = sentence_to_fetch.clone();
        let reading_state = reading_state.clone();
        let (mut on_next, _) = use_navigation(reading_state);
        move || {
            on_next();
            trigger_sentence_fetch(reading_state, sentence_to_fetch);
        }
    };
    
    let mut keyboard_on_prev = {
        let sentence_to_fetch = sentence_to_fetch.clone();
        let reading_state = reading_state.clone();
        let (_, mut on_prev) = use_navigation(reading_state);
        move || {
            on_prev();
            trigger_sentence_fetch(reading_state, sentence_to_fetch);
        }
    };

    rsx! {
        style { "body {{ margin: 0; padding: 0; background: {theme.background}; color: {theme.text_primary}; }}" }
        style { {include_str!("../assets/fonts/spectral.css")} }
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
        KnownWordsCounter {
            count: {
                let vocab_state = vocabulary_state.read();
                vocab_state.known_words_count
            },
            theme: theme.clone(),
            on_click: move |_| show_known_words_modal.set(true)
        }
        div {
            class: "app-container",
            style: "min-height: 100vh; width: 100%; background: {theme.background}; font-family: 'Alegreya', Palatino, 'Book Antiqua', serif; display: flex; flex-direction: column; position: relative;",
            tabindex: 0,
            onkeydown: move |e| {
                if !show_input_modal() && reading_state().total_sentences() > 0 {
                    match e.key() {
                        Key::ArrowRight => keyboard_on_next(),
                        Key::ArrowLeft => keyboard_on_prev(),
                        _ => {}
                    }
                }
            },
            
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
                    
                    // Track encounters for words when we have a cached result (words are being displayed)
                    if let Some(ref result) = cached_result {
                        let promoted_words = track_word_encounters(
                            &current_sentence_str,
                            &result.words,
                            &mut encounter_tracked_sentences,
                            &mut vocabulary_state,
                        );
                        
                        // Show notification for promoted words
                        if let Some(notification_text) = format_promotion_message(&promoted_words) {
                            promotion_notification.set(Some(notification_text));
                            
                            // Clear notification after 3 seconds
                            let mut notification_clone = promotion_notification.clone();
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
                            on_next: enhanced_on_next,
                            on_prev: enhanced_on_prev,
                            on_word_click: move |word: String| {
                                handle_word_click(
                                    &word,
                                    &mut reading_state,
                                    &mut vocabulary_state,
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
                            
                            // Clear encounter tracking for new text
                            encounter_tracked_sentences.write().clear();
                            
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
            
            if show_known_words_modal() {
                KnownWordsModal {
                    words: {
                        match vocabulary_state.read().manager.get_all_known_words() {
                            Ok(words) => words,
                            Err(e) => {
                                eprintln!("Failed to get known words: {}", e);
                                Vec::new()
                            }
                        }
                    },
                    theme: theme.clone(),
                    on_close: move |_| show_known_words_modal.set(false),
                    on_remove_word: move |word: String| {
                        if let Err(e) = vocabulary_state.write().remove_known_word(&word) {
                            eprintln!("Failed to remove known word: {}", e);
                        }
                    }
                }
            }

        }
    }
}

/// Helper function to track word encounters for a sentence
/// Returns a list of promoted words
fn track_word_encounters(
    sentence_key: &str,
    words: &[glossia_shared::types::WordMeaning],
    encounter_tracked_sentences: &mut Signal<std::collections::HashSet<String>>,
    vocabulary_state: &mut Signal<crate::hooks::VocabularyState>,
) -> Vec<String> {
    let already_tracked = encounter_tracked_sentences.read().contains(sentence_key);
    
    if already_tracked {
        return Vec::new();
    }
    
    // Mark this sentence as tracked
    encounter_tracked_sentences.write().insert(sentence_key.to_string());
    
    let mut vocab_state = vocabulary_state.write();
    let mut promoted_words = Vec::new();
    
    for word_meaning in words {
        if let Ok((_count, promoted)) = vocab_state.add_word_encounter(&word_meaning.word) {
            if promoted {
                promoted_words.push(word_meaning.word.clone());
            }
        }
    }
    
    promoted_words
}

/// Helper function to handle word clicks
fn handle_word_click(
    word: &str,
    reading_state: &mut Signal<glossia_book_reader::ReadingState>,
    vocabulary_state: &mut Signal<crate::hooks::VocabularyState>,
    word_to_fetch: Signal<String>,
) {
    // Get current sentence and check if word is already highlighted (difficult)
    let current_sentence_str = reading_state.read().current_sentence().unwrap_or_default();
    let cached_result = reading_state.read().cache.get_simplified(&current_sentence_str);
    
    let empty_vec = Vec::new();
    let api_words = cached_result.as_ref().map(|r| &r.words).unwrap_or(&empty_vec);
    let combined_words = reading_state.read().get_combined_words(api_words);
    let filtered_words = vocabulary_state.read().filter_known_words(&combined_words);
    let is_already_difficult = is_word_already_difficult(word, &filtered_words);
    
    if is_already_difficult {
        // Word is already highlighted/difficult - add to known words
        if let Err(e) = vocabulary_state.write().add_known_word(word) {
            eprintln!("Failed to add known word: {}", e);
        }
    } else {
        // Word is normal text - add to both vocabulary encounters and manual list, then fetch meaning
        if let Err(e) = vocabulary_state.write().add_word_encounter(word) {
            eprintln!("Failed to add word encounter: {}", e);
        }
        reading_state.write().add_manual_word(word.to_string());
        trigger_word_meaning_fetch(word.to_string(), word_to_fetch);
    }
}
