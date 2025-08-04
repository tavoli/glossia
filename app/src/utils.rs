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