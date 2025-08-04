use dioxus::prelude::*;
use glossia_shared::WordMeaning;
use crate::utils::generate_word_color;
use crate::components::ImageGallery;
use crate::hooks::{use_image_cache, use_image_fetcher};
use std::collections::HashSet;
use glossia_book_reader::ReadingState;

#[component]
pub fn WordMeaningItem(
    word_meaning: WordMeaning,
    is_last: bool,
    expanded_words: Signal<HashSet<String>>,
    on_expand_word: EventHandler<String>,
    reading_state: Signal<ReadingState>,
    current_sentence: String,
) -> Element {
    let is_expanded = expanded_words.read().contains(&word_meaning.word);
    
    // Image cache for this component
    let image_cache = use_image_cache();
    let mut fetch_images = use_image_fetcher(reading_state, image_cache);
    
    let toggle_expansion = {
        let word = word_meaning.word.clone();
        let word_meaning_text = word_meaning.meaning.clone();
        let sentence_context = current_sentence.clone();
        let mut expanded_words = expanded_words.clone();
        let on_expand_word = on_expand_word.clone();
        
        move |_| {
            let is_currently_expanded = expanded_words.read().contains(&word);
            
            if is_currently_expanded {
                // Collapse this word
                expanded_words.write().remove(&word);
            } else {
                // Expand this word
                expanded_words.write().insert(word.clone());
                on_expand_word.call(word.clone());
                
                // Trigger image fetch for this specific word
                fetch_images(word.clone(), word_meaning_text.clone(), sentence_context.clone());
            }
        }
    };

    rsx! {
        div {
            class: format!("meaning-item {}", if !is_last { "border-bottom" } else { "" }),
            
            div {
                class: "word-header",
                onclick: toggle_expansion,
                
                div {
                    class: "word-label",
                    style: format!("background: {};", generate_word_color(&word_meaning.word)),
                    "{word_meaning.word}"
                }
                
                div {
                    class: "meaning-definition",
                    "{word_meaning.meaning}"
                }
                
                div {
                    class: format!("expand-icon {}", if is_expanded { "expanded" } else { "" }),
                    if is_expanded { "▼" } else { "▶" }
                }
            }
            
            if is_expanded {
                ImageGallery {
                    word: word_meaning.word.clone(),
                    image_cache,
                }
            }
        }
    }
}
