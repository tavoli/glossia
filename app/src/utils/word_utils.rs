use glossia_shared::types::WordMeaning;
use crate::hooks::{VocabularyState, trigger_word_meaning_fetch};
use dioxus::prelude::*;
use std::collections::HashSet;

/// Check if a word is already highlighted/difficult based on current sentence words
pub fn is_word_already_difficult(
    word: &str,
    combined_words: &[WordMeaning],
) -> bool {
    combined_words.iter().any(|w| w.word.to_lowercase() == word.to_lowercase())
}

/// Get combined and filtered words for display, sorted by timestamp (newest first)
pub fn get_display_words(
    api_words: &[WordMeaning],
    reading_state: &glossia_reading_engine::ReadingEngine,
    vocabulary_state: &VocabularyState,
) -> Vec<WordMeaning> {
    let combined_words = reading_state.get_combined_words_with_cache(api_words);
    let mut filtered_words = vocabulary_state.filter_known_words(&combined_words);
    
    // Sort by timestamp: words with timestamps (manual) come first, sorted newest to oldest
    // Words without timestamps (from API) come after
    filtered_words.sort_by(|a, b| {
        match (a.timestamp, b.timestamp) {
            (Some(t1), Some(t2)) => t2.cmp(&t1), // Both have timestamps: newest first
            (Some(_), None) => std::cmp::Ordering::Less, // a has timestamp, b doesn't: a comes first
            (None, Some(_)) => std::cmp::Ordering::Greater, // b has timestamp, a doesn't: b comes first
            (None, None) => std::cmp::Ordering::Equal, // Neither has timestamp: keep original order
        }
    });
    
    filtered_words
}

/// Format promotion notification message
pub fn format_promotion_message(promoted_words: &[String]) -> Option<String> {
    if promoted_words.is_empty() {
        return None;
    }

    let message = if promoted_words.len() == 1 {
        format!("'{}' added to known words!", promoted_words[0])
    } else {
        format!("{} words added to known words!", promoted_words.len())
    };
    
    Some(message)
}

/// Helper function to track word encounters for a sentence
/// Returns a list of promoted words
pub fn track_word_encounters(
    sentence_key: &str,
    words: &[WordMeaning],
    encounter_tracked_sentences: &mut Signal<HashSet<String>>,
    vocabulary_state: &mut Signal<VocabularyState>,
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
pub fn handle_word_click(
    word: &str,
    reading_state: &mut Signal<glossia_reading_engine::ReadingEngine>,
    vocabulary_state: &mut Signal<VocabularyState>,
    word_to_fetch: Signal<String>,
) {
    // Get current sentence and check if word is already highlighted (difficult)
    let current_sentence_str = reading_state.read().current_sentence().unwrap_or_default();
    let cached_result = reading_state.read().get_cached_simplified(&current_sentence_str);
    
    let empty_vec = Vec::new();
    let api_words = cached_result.as_ref().map(|r| &r.words).unwrap_or(&empty_vec);
    let combined_words = reading_state.read().get_combined_words(api_words);
    let filtered_words = vocabulary_state.read().filter_known_words(&combined_words);
    let is_already_difficult = is_word_already_difficult(word, &filtered_words);
    
    if is_already_difficult {
        // Word is already highlighted/difficult - add to known words
        if let Err(e) = vocabulary_state.write().add_known_word(word) {
            tracing::error!(
                event = "add_known_word_failed",
                component = "word_utils",
                word = word,
                error = %e,
                "Failed to add known word"
            );
        }
    } else {
        // Word is normal text - add to both vocabulary encounters and manual list, then fetch meaning
        if let Err(e) = vocabulary_state.write().add_word_encounter(word) {
            tracing::error!(
                event = "add_word_encounter_failed",
                component = "word_utils",
                word = word,
                error = %e,
                "Failed to add word encounter"
            );
        }
        reading_state.write().add_manual_word(word.to_string());
        trigger_word_meaning_fetch(word.to_string(), word_to_fetch);
    }
}
