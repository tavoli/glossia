use glossia_shared::types::WordMeaning;
use crate::hooks::VocabularyState;

/// Check if a word is already highlighted/difficult based on current sentence words
pub fn is_word_already_difficult(
    word: &str,
    combined_words: &[WordMeaning],
) -> bool {
    combined_words.iter().any(|w| w.word.to_lowercase() == word.to_lowercase())
}

/// Get combined and filtered words for display
pub fn get_display_words(
    api_words: &[WordMeaning],
    reading_state: &glossia_book_reader::ReadingState,
    vocabulary_state: &VocabularyState,
) -> Vec<WordMeaning> {
    let combined_words = reading_state.get_combined_words(api_words);
    vocabulary_state.filter_known_words(&combined_words)
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
