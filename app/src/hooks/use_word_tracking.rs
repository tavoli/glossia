use dioxus::prelude::*;
use std::collections::HashSet;
use crate::hooks::VocabularyState;
use crate::utils::word_utils::{track_word_encounters, format_promotion_message};
use glossia_shared::types::WordMeaning;

/// Hook for tracking word encounters and managing promotion notifications
pub fn use_word_tracking(
    current_sentence: String,
    words: &Vec<WordMeaning>,
    encounter_tracked_sentences: Signal<HashSet<String>>,
    vocabulary_state: Signal<VocabularyState>,
    promotion_notification: Signal<Option<String>>,
) {
    let mut encounter_tracked_sentences_mut = encounter_tracked_sentences.clone();
    let mut vocabulary_state_mut = vocabulary_state.clone();
    let mut promotion_notification_mut = promotion_notification.clone();
    
    // Track encounters for words
    let promoted_words = track_word_encounters(
        &current_sentence,
        words,
        &mut encounter_tracked_sentences_mut,
        &mut vocabulary_state_mut,
    );
    
    // Show notification for promoted words
    if let Some(notification_text) = format_promotion_message(&promoted_words) {
        promotion_notification_mut.set(Some(notification_text));
        
        // Clear notification after 3 seconds
        spawn(async move {
            tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
            promotion_notification_mut.set(None);
        });
    }
}