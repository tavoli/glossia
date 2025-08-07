use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// Generate a consistent color for a given word
pub fn generate_word_color(word: &str) -> String {
    // Create a hash of the word for consistency
    let mut hasher = DefaultHasher::new();
    word.to_lowercase().hash(&mut hasher);
    let hash = hasher.finish();

    // Define a palette of readable colors with good contrast
    let colors = [
        "#e53e3e", // Red
        "#dd6b20", // Orange
        "#d69e2e", // Yellow
        "#38a169", // Green
        "#319795", // Teal
        "#3182ce", // Blue
        "#805ad5", // Purple
        "#d53f8c", // Pink
        "#2d3748", // Gray
        "#744210", // Brown
    ];

    // Select color based on hash
    let index = (hash as usize) % colors.len();
    colors[index].to_string()
}

/// Tokenize text into word elements for click handling
pub fn tokenize_text_for_clicks(text: &str) -> Vec<String> {
    // Split text into words and non-word characters (spaces, punctuation, etc.)
    let mut tokens = Vec::new();
    let mut current_token = String::new();
    let mut is_word = false;

    for ch in text.chars() {
        let char_is_word = ch.is_alphabetic();

        if char_is_word != is_word {
            // Character type changed, push current token if not empty
            if !current_token.is_empty() {
                tokens.push(current_token.clone());
                current_token.clear();
            }
            is_word = char_is_word;
        }

        current_token.push(ch);
    }

    // Push the last token if not empty
    if !current_token.is_empty() {
        tokens.push(current_token);
    }

    tokens
}

/// Check if a token is a word (contains only alphabetic characters)
pub fn is_word_token(token: &str) -> bool {
    !token.is_empty() && token.chars().all(|c| c.is_alphabetic())
}

/// Represents a span of tokens that should be highlighted together
#[derive(Debug, Clone)]
pub struct HighlightSpan {
    pub start_index: usize,
    pub end_index: usize,
    pub text: String,
    pub is_phrase: bool,
}

/// Find phrase matches in tokenized text
pub fn find_phrase_matches(tokens: &[String], word_meanings: &[glossia_shared::types::WordMeaning]) -> Vec<HighlightSpan> {
    if tokens.is_empty() || word_meanings.is_empty() {
        return Vec::new();
    }

    let mut spans = Vec::new();

    // First, find phrase matches (longer spans have priority)
    for word_meaning in word_meanings.iter().filter(|wm| wm.is_phrase) {
        let phrase_words: Vec<&str> = word_meaning.word.split_whitespace().collect();
        if phrase_words.is_empty() {
            continue;
        }

        let mut i = 0;
        while i < tokens.len() {
            if let Some(end_idx) = try_match_phrase_at(&tokens, i, &phrase_words) {
                if end_idx < tokens.len() {
                    let phrase_text = tokens[i..=end_idx]
                        .iter()
                        .cloned()
                        .collect::<Vec<_>>()
                        .join("");

                    spans.push(HighlightSpan {
                        start_index: i,
                        end_index: end_idx,
                        text: phrase_text,
                        is_phrase: true,
                    });
                }

                i = end_idx + 1; // Skip past this phrase
            } else {
                i += 1;
            }
        }
    }

    // Then, find individual word matches that don't overlap with phrases
    for (token_idx, token) in tokens.iter().enumerate() {
        if !is_word_token(token) {
            continue;
        }

        // Check if this token is already covered by a phrase
        let is_covered = spans.iter().any(|span|
            token_idx >= span.start_index && token_idx <= span.end_index
        );

        if !is_covered {
            // Check if this token matches any single word
            let word_match = word_meanings.iter()
                .filter(|wm| !wm.is_phrase)
                .any(|wm| wm.word.to_lowercase() == token.to_lowercase());

            if word_match {
                spans.push(HighlightSpan {
                    start_index: token_idx,
                    end_index: token_idx,
                    text: token.clone(),
                    is_phrase: false,
                });
            }
        }
    }

    // Sort spans by start index for easier processing
    spans.sort_by_key(|span| span.start_index);
    spans
}

/// Try to match a phrase starting at the given token index
fn try_match_phrase_at(tokens: &[String], start_idx: usize, phrase_words: &[&str]) -> Option<usize> {
    let mut token_idx = start_idx;
    let mut phrase_word_idx = 0;

    while phrase_word_idx < phrase_words.len() && token_idx < tokens.len() {
        let token = &tokens[token_idx];

        if is_word_token(token) {
            if token.to_lowercase() == phrase_words[phrase_word_idx].to_lowercase() {
                phrase_word_idx += 1;
            } else {
                return None; // Phrase doesn't match
            }
        }
        // Skip non-word tokens (spaces, punctuation) but don't advance phrase_word_idx

        token_idx += 1;
    }

    if phrase_word_idx == phrase_words.len() && token_idx > 0 {
        Some(token_idx - 1) // Return the last token index of the phrase
    } else {
        None
    }
}
