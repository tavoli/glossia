use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use crate::theme::{Theme, ThemeMode};

/// Generate a consistent color for a given word (legacy function for backwards compatibility)
pub fn generate_word_color(word: &str) -> String {
    // Use light theme colors by default for backwards compatibility
    generate_word_color_themed(word, &Theme::light())
}

/// Generate a consistent color for a given word based on the current theme
pub fn generate_word_color_themed(word: &str, theme: &Theme) -> String {
    // Create a hash of the word for consistency
    let mut hasher = DefaultHasher::new();
    word.to_lowercase().hash(&mut hasher);
    let hash = hasher.finish();

    let colors = match theme.mode {
        ThemeMode::Light => light_theme_colors(),
        ThemeMode::Dark => dark_theme_colors(),
    };

    // Select color based on hash
    let index = (hash as usize) % colors.len();
    colors[index].to_string()
}

/// Color palette optimized for light theme
/// Colors are vibrant but not too bright, ensuring good readability on white/light backgrounds
fn light_theme_colors() -> [&'static str; 12] {
    [
        "#d63384", // Bright Pink - good contrast on light
        "#fd7e14", // Vibrant Orange
        "#ffc107", // Golden Yellow
        "#20c997", // Teal Green
        "#0dcaf0", // Cyan Blue
        "#6f42c1", // Purple
        "#dc3545", // Red
        "#198754", // Forest Green
        "#0d6efd", // Primary Blue
        "#6610f2", // Indigo
        "#d63384", // Magenta
        "#495057", // Dark Gray
    ]
}

/// Color palette optimized for dark theme
/// Colors are bright and saturated to ensure good visibility on dark backgrounds
fn dark_theme_colors() -> [&'static str; 12] {
    [
        "#ff6b9d", // Bright Pink - excellent visibility on dark
        "#ffa726", // Light Orange
        "#ffeb3b", // Bright Yellow
        "#4caf50", // Light Green
        "#29b6f6", // Light Blue
        "#ab47bc", // Light Purple
        "#ef5350", // Light Red
        "#66bb6a", // Mint Green
        "#42a5f5", // Sky Blue
        "#7e57c2", // Light Indigo
        "#ec407a", // Light Magenta
        "#bdbdbd", // Light Gray
    ]
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::theme::{Theme, ThemeMode};

    #[test]
    fn test_theme_aware_colors() {
        let light_theme = Theme::light();
        let dark_theme = Theme::dark();
        
        let test_word = "example";
        
        // Get colors for both themes
        let light_color = generate_word_color_themed(test_word, &light_theme);
        let dark_color = generate_word_color_themed(test_word, &dark_theme);
        
        // Colors should be different for different themes
        assert_ne!(light_color, dark_color);
        
        // Colors should be consistent for the same word and theme
        let light_color_2 = generate_word_color_themed(test_word, &light_theme);
        let dark_color_2 = generate_word_color_themed(test_word, &dark_theme);
        
        assert_eq!(light_color, light_color_2);
        assert_eq!(dark_color, dark_color_2);
        
        // Colors should be valid hex codes
        assert!(light_color.starts_with('#'));
        assert!(dark_color.starts_with('#'));
        assert_eq!(light_color.len(), 7); // #RRGGBB format
        assert_eq!(dark_color.len(), 7); // #RRGGBB format
    }

    #[test]
    fn test_backwards_compatibility() {
        let test_word = "compatibility";
        
        // Legacy function should still work
        let legacy_color = generate_word_color(test_word);
        
        // Should be same as light theme
        let light_theme_color = generate_word_color_themed(test_word, &Theme::light());
        
        assert_eq!(legacy_color, light_theme_color);
    }
}
