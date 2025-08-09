use dioxus::prelude::*;
use crate::components::{
    FloatingButton, ProgressBar, TextInputModal, ThemeToggle, 
    KnownWordsCounter, KnownWordsModal, WordMeaningsStyles,
    MainContent
};
use crate::hooks::{use_app_state, use_word_meanings};

#[component]
pub fn App() -> Element {
    let mut app_state = use_app_state();
    
    // Use the word meanings hook
    let word_meaning_result = use_word_meanings(app_state.reading_state, app_state.word_to_fetch);
    
    // Handle word meaning result when it becomes available
    use_effect(move || {
        match word_meaning_result.read().as_ref() {
            Some(Some(Ok(meaning))) => {
                let current_word = app_state.word_to_fetch.read().clone();
                if !current_word.is_empty() && !meaning.is_empty() {
                    tracing::info!("app.rs: Word meaning fetched successfully for '{}', clearing word_to_fetch", current_word);
                    // Clear the word_to_fetch to indicate completion
                    app_state.word_to_fetch.set(String::new());
                }
            }
            Some(Some(Err(e))) => {
                let current_word = app_state.word_to_fetch.read().clone();
                tracing::error!("app.rs: Failed to fetch meaning for word '{}': {}", current_word, e);
                // Clear the word_to_fetch even on error to prevent infinite retries
                app_state.word_to_fetch.set(String::new());
            }
            _ => {}
        }
    });
    
    // Clone what we need for closures
    let mut theme_state = app_state.clone();
    let mut known_words_state = app_state.clone();
    let mut input_modal_state = app_state.clone();
    let mut input_submit_state = app_state.clone();
    let mut input_close_state = app_state.clone();
    let mut known_modal_close_state = app_state.clone();
    let mut known_modal_remove_state = app_state.clone();
    
    rsx! {
        style { "body {{ margin: 0; padding: 0; background: {app_state.theme.background}; color: {app_state.theme.text_primary}; }}" }
        style { {include_str!("../../assets/fonts/spectral.css")} }
        WordMeaningsStyles { theme: app_state.theme.clone() }
        
        ThemeToggle { 
            theme_mode: app_state.theme_mode, 
            on_toggle: move |_| {
                theme_state.toggle_theme();
            }
        }
        
        KnownWordsCounter {
            count: app_state.known_words_count(),
            theme: app_state.theme.clone(),
            on_click: move |_| known_words_state.show_known_words_modal()
        }
        
        div {
            class: "app-container",
            style: "min-height: 100vh; width: 100%; background: {app_state.theme.background}; font-family: 'Alegreya', Palatino, 'Book Antiqua', serif; display: flex; flex-direction: column; position: relative;",
            tabindex: 0,
            onkeydown: move |e| {
                if !*app_state.show_input_modal.read() && app_state.reading_state.read().total_sentences() > 0 {
                    match e.key() {
                        Key::ArrowRight => {
                            app_state.reading_state.write().next();
                            if let Some(sentence) = app_state.reading_state.read().current_sentence() {
                                app_state.sentence_to_fetch.set(sentence);
                            }
                        },
                        Key::ArrowLeft => {
                            app_state.reading_state.write().previous();
                            if let Some(sentence) = app_state.reading_state.read().current_sentence() {
                                app_state.sentence_to_fetch.set(sentence);
                            }
                        },
                        _ => {}
                    }
                }
            },
            
            // Theme-aware grid background
            div {
                style: "position: absolute; inset: 0; z-index: 1; background: {app_state.theme.background}; background-image: linear-gradient(to right, {app_state.theme.border} 1px, transparent 1px), linear-gradient(to bottom, {app_state.theme.border} 1px, transparent 1px), radial-gradient(circle at 50% 60%, rgba(236,72,153,0.15) 0%, rgba(168,85,247,0.05) 40%, transparent 70%); background-size: 40px 40px, 40px 40px, 100% 100%;"
            }
            
            ProgressBar { 
                current: {
                    let (current, _) = app_state.progress_values();
                    current
                },
                total: {
                    let (_, total) = app_state.progress_values();
                    total
                },
                theme: app_state.theme.clone()
            }
            
            MainContent {
                reading_state: app_state.reading_state,
                vocabulary_state: app_state.vocabulary_state,
                sentence_to_fetch: app_state.sentence_to_fetch,
                word_to_fetch: app_state.word_to_fetch,
                encounter_tracked_sentences: app_state.encounter_tracked_sentences,
                promotion_notification: app_state.promotion_notification,
                theme: app_state.theme.clone(),
            }
            
            // Show floating button if there are sentences, or if modal is closed but no text loaded
            if app_state.should_show_floating_button() {
                FloatingButton {
                    count: app_state.floating_button_count(),
                    onclick: move |_| input_modal_state.show_input_modal()
                }
            }
            
            if *app_state.show_input_modal.read() {
                TextInputModal {
                    onsubmit: move |text: String| {
                        input_submit_state.load_text(text);
                    },
                    onclose: move |_| input_close_state.hide_input_modal()
                }
            }
            
            if *app_state.show_known_words_modal.read() {
                KnownWordsModal {
                    words: {
                        match app_state.vocabulary_state.read().manager.get_all_known_words() {
                            Ok(words) => words,
                            Err(e) => {
                                tracing::error!(
                                    event = "get_known_words_failed",
                                    component = "app",
                                    error = %e,
                                    "Failed to get known words for modal display"
                                );
                                Vec::new()
                            }
                        }
                    },
                    theme: app_state.theme.clone(),
                    on_close: move |_| known_modal_close_state.hide_known_words_modal(),
                    on_remove_word: move |word: String| {
                        if let Err(e) = known_modal_remove_state.vocabulary_state.write().remove_known_word(&word) {
                            tracing::error!(
                                event = "remove_known_word_failed",
                                component = "app",
                                word = %word,
                                error = %e,
                                "Failed to remove known word from modal"
                            );
                        }
                    }
                }
            }
        }
        
        // TODO: Add notification system back
        // NotificationSystem {
        //     notification: (*app_state.promotion_notification.read()).clone(),
        //     theme: app_state.theme.clone()
        // }
    }
}
