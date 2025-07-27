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

/// Highlight words in text with their corresponding colors
pub fn highlight_words_in_text(text: &str, words: &[glossia_shared::WordMeaning]) -> String {
    let mut highlighted_text = text.to_string();
    
    for word_meaning in words {
        let word = &word_meaning.word;
        let color = generate_word_color(word);
        
        // Create a case-insensitive regex pattern for the word
        // We need to be careful about word boundaries to avoid partial matches
        let pattern = format!(r"\b{}\b", regex::escape(&word.to_lowercase()));
        
        if let Ok(re) = regex::RegexBuilder::new(&pattern)
            .case_insensitive(true)
            .build() {
            
            highlighted_text = re.replace_all(&highlighted_text, |caps: &regex::Captures| {
                let matched_word = &caps[0];
                format!(
                    r#"<span style="color: white; font-weight: 600; background: {}; padding: 3px 8px; border-radius: 16px; font-size: 0.95em; box-shadow: 0 1px 3px rgba(0,0,0,0.15);">{}</span>"#,
                    color,
                    matched_word
                )
            }).to_string();
        }
    }
    
    highlighted_text
}