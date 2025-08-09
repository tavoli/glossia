use dioxus::prelude::*;
use std::collections::HashMap;
use crate::services::{ImageService, ImageFetchState};
use glossia_reading_engine::ReadingEngine;

/// Custom hook for managing image cache and fetching
pub fn use_image_cache() -> Signal<HashMap<String, ImageFetchState>> {
    use_signal(HashMap::new)
}

/// Custom hook for fetching images for a specific word
pub fn use_image_fetcher(
    reading_state: Signal<ReadingEngine>,
    mut image_cache: Signal<HashMap<String, ImageFetchState>>,
) -> impl FnMut(String, String, String) + 'static {
    move |word: String, word_meaning: String, sentence_context: String| {
        // Check if we already have images for this word
        if image_cache.read().contains_key(&word) {
            return;
        }
        
        // Mark as loading
        image_cache.write().insert(word.clone(), ImageFetchState::Loading);
        
        // Spawn async task
        let word_clone = word.clone();
        let word_meaning_clone = word_meaning;
        let sentence_clone = sentence_context;
        let mut image_cache_clone = image_cache.clone();
        let mut reading_state_clone = reading_state.clone();
        
        spawn(async move {
            match ImageService::fetch_images_for_word(
                &word_clone,
                &word_meaning_clone,
                &sentence_clone,
                &mut reading_state_clone,
            ).await {
                Ok(images) => {
                    image_cache_clone.write().insert(word_clone, ImageFetchState::Loaded(images));
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
}
