use dioxus::prelude::*;
use glossia_shared::WordMeaning;
use crate::components::WordMeaningItem;
use std::collections::HashSet;

#[component] 
pub fn WordMeanings(
    words: Vec<WordMeaning>, 
    on_expand_word: EventHandler<String>, 
    reading_state: Signal<glossia_book_reader::ReadingState>,
    current_sentence: String,
    theme: crate::theme::Theme,
) -> Element {
    if words.is_empty() {
        return None;
    }

    // Track which words are expanded
    let expanded_words = use_signal(|| HashSet::<String>::new());

    rsx! {
        div {
            class: "word-meanings-container",
            
            div {
                class: "meanings-list",
                
                for (index, word_meaning) in words.iter().enumerate() {
                    WordMeaningItem {
                        word_meaning: word_meaning.clone(),
                        is_last: index == words.len() - 1,
                        expanded_words,
                        on_expand_word: on_expand_word.clone(),
                        reading_state: reading_state.clone(),
                        current_sentence: current_sentence.clone(),
                    }
                }
            }
        }
    }
}
