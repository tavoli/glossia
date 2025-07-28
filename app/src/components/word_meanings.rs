use dioxus::prelude::*;
use glossia_shared::{WordMeaning, ImageResult};
use crate::utils::generate_word_color;
use std::collections::{HashSet, HashMap};

#[component] 
pub fn WordMeanings(
    words: Vec<WordMeaning>, 
    on_expand_word: EventHandler<String>, 
    reading_state: Signal<glossia_book_reader::ReadingState>,
    current_sentence: String,
) -> Element {
    if words.is_empty() {
        return None;
    }

    // Track which words are expanded
    let expanded_words = use_signal(|| HashSet::<String>::new());
    
    // Store image results for each word independently
    let image_cache = use_signal(|| HashMap::<String, ImageFetchState>::new());

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
                        image_cache,
                        on_expand_word: on_expand_word.clone(),
                        reading_state: reading_state.clone(),
                        current_sentence: current_sentence.clone(),
                    }
                }
            }
        }
    }
}

#[derive(Clone, Debug)]
enum ImageFetchState {
    Loading,
    Loaded(Vec<ImageResult>),
    Error(String),
}

#[component]
fn WordMeaningItem(
    word_meaning: WordMeaning,
    is_last: bool,
    expanded_words: Signal<HashSet<String>>,
    image_cache: Signal<HashMap<String, ImageFetchState>>,
    on_expand_word: EventHandler<String>,
    reading_state: Signal<glossia_book_reader::ReadingState>,
    current_sentence: String,
) -> Element {
    let is_expanded = expanded_words.read().contains(&word_meaning.word);
    
    // Simple fetch function for images with context
    let fetch_images_for_word = {
        let word = word_meaning.word.clone();
        let word_meaning_text = word_meaning.meaning.clone();
        let sentence_context = current_sentence.clone();
        let mut image_cache = image_cache.clone();
        let reading_state = reading_state.clone();
        
        move || {
            // Check if we already have images for this word
            if image_cache.read().contains_key(&word) {
                return;
            }
            
            // Mark as loading
            image_cache.write().insert(word.clone(), ImageFetchState::Loading);
            
            // Spawn async task with context
            let word_clone = word.clone();
            let word_meaning_clone = word_meaning_text.clone();
            let sentence_clone = sentence_context.clone();
            let mut image_cache_clone = image_cache.clone();
            let mut reading_state_clone = reading_state.clone();
            
            spawn(async move {
                // Use the new context-aware optimized image fetching method
                match reading_state_clone.write().optimize_and_fetch_images(
                    word_clone.clone(),
                    sentence_clone,
                    word_meaning_clone
                ).await {
                    Ok(images) => {
                        image_cache_clone.write().insert(
                            word_clone, 
                            ImageFetchState::Loaded(images)
                        );
                    }
                    Err(e) => {
                        image_cache_clone.write().insert(
                            word_clone,
                            ImageFetchState::Error(format!("Failed to load images: {}", e))
                        );
                    }
                }
            });
        }
    };
    
    let toggle_expansion = {
        let word = word_meaning.word.clone();
        let mut expanded_words = expanded_words.clone();
        let on_expand_word = on_expand_word.clone();
        let mut fetch_images = fetch_images_for_word.clone();
        
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
                fetch_images();
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

#[component]
fn ImageGallery(
    word: String,
    image_cache: Signal<HashMap<String, ImageFetchState>>,
) -> Element {
    let image_state = image_cache.read().get(&word).cloned();
    
    match image_state {
        Some(ImageFetchState::Loading) | None => rsx! {
            div {
                class: "image-gallery",
                div {
                    class: "gallery-message loading",
                    span { class: "spinner", }
                    "Loading images..."
                }
            }
        },
        Some(ImageFetchState::Loaded(images)) => {
            if images.is_empty() {
                rsx! {
                    div {
                        class: "image-gallery",
                        div {
                            class: "gallery-message",
                            "No images found"
                        }
                    }
                }
            } else {
                rsx! {
                    div {
                        class: "image-gallery",
                        
                        div {
                            class: "gallery-header",
                            "Images for \"{word}\""
                        }
                        
                        div {
                            class: "images-grid",
                            for (index, image) in images.iter().enumerate() {
                                ImageThumbnail {
                                    key: "{word}-{index}",
                                    image: image.clone(),
                                }
                            }
                        }
                    }
                }
            }
        },
        Some(ImageFetchState::Error(error)) => rsx! {
            div {
                class: "image-gallery",
                div {
                    class: "gallery-message error",
                    "⚠ {error}"
                }
            }
        },
    }
}

#[component]
fn ImageThumbnail(image: ImageResult) -> Element {
    rsx! {
        div {
            class: "image-item",
            
            img {
                src: "{image.thumbnail_url}",
                alt: "{image.title}",
                loading: "lazy",
            }
            
            div {
                class: "image-title",
                title: "{image.title}",
                "{image.title}"
            }
        }
    }
}
