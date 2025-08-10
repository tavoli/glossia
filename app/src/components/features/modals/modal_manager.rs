use dioxus::prelude::*;
use crate::components::features::vocabulary::KnownWordsModal;
use crate::components::features::modals::TextInputModal;
use crate::hooks::AppState;

/// Manages all modal rendering based on app state
pub fn modal_manager(app_state: AppState) -> Element {
    let mut input_submit_state = app_state.clone();
    let mut input_close_state = app_state.clone();
    let mut known_modal_close_state = app_state.clone();
    let mut known_modal_remove_state = app_state.clone();
    
    rsx! {
        if *app_state.show_input_modal.read() {
            TextInputModal {
                theme: app_state.theme.clone(),
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
                                component = "modal_manager",
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
                            component = "modal_manager",
                            word = %word,
                            error = %e,
                            "Failed to remove known word from modal"
                        );
                    }
                }
            }
        }
    }
}