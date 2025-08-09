use dioxus::prelude::*;
use glossia_shared::types::WordMeaning;
use crate::components::features::reading::ReadingLayout;
use crate::theme::Theme;
use crate::utils::word_utils::{get_display_words, handle_word_click};

/// Displays the main reading content with words
#[component]
pub fn ContentDisplay(
    original: Option<String>,
    simplified: Option<String>,
    words: Vec<WordMeaning>,
    is_loading: bool,
    theme: Theme,
    reading_state: Signal<glossia_reading_engine::ReadingEngine>,
    vocabulary_state: Signal<crate::hooks::VocabularyState>,
    word_to_fetch: Signal<String>,
    on_next: EventHandler<()>,
    on_prev: EventHandler<()>,
) -> Element {
    let mut reading_state_word_click = reading_state.clone();
    let mut vocabulary_state_word_click = vocabulary_state.clone();
    let word_to_fetch_click = word_to_fetch.clone();
    
    let filtered_words = get_display_words(&words, &reading_state.read(), &vocabulary_state.read());
    
    rsx! {
        ReadingLayout {
            original: original.clone(),
            simplified: simplified,
            is_loading,
            words: filtered_words,
            theme: theme.clone(),
            reading_state: reading_state,
            on_next: move |_| on_next.call(()),
            on_prev: move |_| on_prev.call(()),
            on_word_click: move |word: String| {
                handle_word_click(
                    &word,
                    &mut reading_state_word_click,
                    &mut vocabulary_state_word_click,
                    word_to_fetch_click
                );
            },
            on_expand_word: move |_word: String| {}
        }
    }
}