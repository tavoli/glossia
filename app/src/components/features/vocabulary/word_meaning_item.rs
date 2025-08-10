use dioxus::prelude::*;
use glossia_shared::WordMeaning;
use crate::utils::generate_word_color_themed;
use crate::theme::Theme;
use crate::components::ImageGallery;
use crate::hooks::{use_image_cache, use_image_fetcher, use_vocabulary};
use std::collections::HashSet;
use glossia_reading_engine::ReadingEngine;

#[component]
pub fn WordMeaningItem(
    word_meaning: WordMeaning,
    is_last: bool,
    expanded_words: Signal<HashSet<String>>,
    on_expand_word: EventHandler<String>,
    reading_state: Signal<ReadingEngine>,
    current_sentence: String,
    theme: Theme,
) -> Element {
    let is_expanded = expanded_words.read().contains(&word_meaning.word);
    
    // Get vocabulary state for progress tracking
    let vocabulary_state = use_vocabulary();
    
    // Get word progress (encounter count)
    let (encounter_count, is_known) = vocabulary_state.read()
        .get_word_progress(&word_meaning.word)
        .unwrap_or((0, false));
    
    // Image cache for this component
    let image_cache = use_image_cache();
    let mut fetch_images = use_image_fetcher(reading_state, image_cache);
    
    let toggle_expansion = {
        let word = word_meaning.word.clone();
        let word_meaning_text = word_meaning.meaning.clone();
        let sentence_context = current_sentence.clone();
        let mut expanded_words = expanded_words.clone();
        let on_expand_word = on_expand_word.clone();
        let image_cache_check = image_cache.clone();
        
        move |_| {
            let is_currently_expanded = expanded_words.read().contains(&word);
            
            if is_currently_expanded {
                // Collapse this word
                expanded_words.write().remove(&word);
            } else {
                // Expand this word
                expanded_words.write().insert(word.clone());
                on_expand_word.call(word.clone());
                
                // Check cache state at the time of expansion
                let has_cached_images = image_cache_check.read().contains_key(&word);
                
                // Only fetch images if not already cached
                if !has_cached_images {
                    fetch_images(word.clone(), word_meaning_text.clone(), sentence_context.clone());
                }
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
                    style: format!("color: {};", generate_word_color_themed(&word_meaning.word, &theme)),
                    "{word_meaning.word}"
                }
                
                div {
                    class: "meaning-definition",
                    "{word_meaning.meaning}"
                }
                
                // Progress indicator showing encounter count
                if encounter_count > 0 && !is_known {
                    div {
                        class: "progress-container",
                        style: "display: flex; flex-direction: column; align-items: center; margin-left: auto; gap: 2px;",
                        
                        div {
                            class: "progress-text",
                            style: "font-size: 0.7em; color: #666; white-space: nowrap;",
                            title: format!("Seen {} times - {} more to auto-add", encounter_count, 12_u32.saturating_sub(encounter_count)),
                            "{encounter_count}/12"
                        }
                        
                        div {
                            class: "progress-bar-bg",
                            style: "width: 40px; height: 3px; background: rgba(0,0,0,0.1); border-radius: 2px; overflow: hidden;",
                            
                            div {
                                class: "progress-bar-fill",
                                style: format!(
                                    "height: 100%; background: {}; width: {}%; transition: width 0.3s ease;",
                                    if encounter_count >= 10 { "#ff6b6b" } else if encounter_count >= 6 { "#ffa726" } else { "#4fc3f7" },
                                    (encounter_count as f32 / 12.0 * 100.0) as u32
                                ),
                            }
                        }
                    }
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
