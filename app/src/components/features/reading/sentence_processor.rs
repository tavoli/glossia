use dioxus::prelude::*;
use std::collections::HashSet;
use crate::hooks::{VocabularyState, use_word_tracking};
use glossia_shared::SimplificationResponse;

/// Handles sentence processing and word tracking
#[component]
pub fn SentenceProcessor(
    current_sentence: String,
    cached_result: Option<SimplificationResponse>,
    encounter_tracked_sentences: Signal<HashSet<String>>,
    vocabulary_state: Signal<VocabularyState>,
    promotion_notification: Signal<Option<String>>,
) -> Element {
    // Track word encounters when we have a cached result
    if let Some(ref result) = cached_result {
        use_word_tracking(
            current_sentence.clone(),
            &result.words,
            encounter_tracked_sentences,
            vocabulary_state,
            promotion_notification,
        );
    }
    
    // This component is purely for side effects
    None
}