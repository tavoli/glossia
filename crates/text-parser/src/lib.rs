use regex::Regex;
use once_cell::sync::Lazy;

// Compile regex patterns once at startup for better performance
static SENTENCE_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"([.?!|;])\s+").expect("Invalid sentence splitting regex")
});

static WORD_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\b[a-zA-Z']+\b").expect("Invalid word extraction regex")
});

/// Splits a block of text into a list of sentences.
/// Sentences are split by '.', '?', '!', and '|'.
pub fn split_into_sentences(text: &str) -> Vec<String> {
    if text.is_empty() {
        return vec![];
    }
    let mut sentences = Vec::new();
    let mut last_end = 0;
    
    for mat in SENTENCE_REGEX.find_iter(text) {
        let sentence = text[last_end..mat.end()].trim();
        if !sentence.is_empty() {
            sentences.push(sentence.to_string());
        }
        last_end = mat.end();
    }
    
    // Add the remaining text if any
    if last_end < text.len() {
        let remaining = text[last_end..].trim();
        if !remaining.is_empty() {
            sentences.push(remaining.to_string());
        }
    }
    
    sentences
}

/// Extracts words from a text sentence, removing punctuation
pub fn extract_words(text: &str) -> Vec<String> {
    WORD_REGEX
        .find_iter(text)
        .map(|mat| mat.as_str().to_lowercase())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_into_sentences() {
        let text = "Hello world. This is a test! How are you? Final sentence";
        let sentences = split_into_sentences(text);
        
        assert_eq!(sentences.len(), 4);
        assert_eq!(sentences[0], "Hello world.");
        assert_eq!(sentences[1], "This is a test!");
        assert_eq!(sentences[2], "How are you?");
        assert_eq!(sentences[3], "Final sentence");
    }

    #[test]
    fn test_empty_text() {
        let sentences = split_into_sentences("");
        assert!(sentences.is_empty());
    }

    #[test]
    fn test_single_sentence() {
        let text = "Single sentence without punctuation";
        let sentences = split_into_sentences(text);
        
        assert_eq!(sentences.len(), 1);
        assert_eq!(sentences[0], "Single sentence without punctuation");
    }

    #[test]
    fn test_sentence_with_pipe() {
        let text = "First sentence. Second sentence| Third sentence.";
        let sentences = split_into_sentences(text);
        
        assert_eq!(sentences.len(), 3);
        assert_eq!(sentences[0], "First sentence.");
        assert_eq!(sentences[1], "Second sentence|");
        assert_eq!(sentences[2], "Third sentence.");
    }

    #[test]
    fn test_extract_words() {
        let text = "Hello, world! This is a test.";
        let words = extract_words(text);
        
        assert_eq!(words.len(), 6);
        assert_eq!(words[0], "hello");
        assert_eq!(words[1], "world");
        assert_eq!(words[2], "this");
        assert_eq!(words[3], "is");
        assert_eq!(words[4], "a");
        assert_eq!(words[5], "test");
    }

    #[test]
    fn test_extract_words_with_apostrophe() {
        let text = "Don't you think it's great?";
        let words = extract_words(text);
        
        assert_eq!(words.len(), 5);
        assert_eq!(words[0], "don't");
        assert_eq!(words[1], "you");
        assert_eq!(words[2], "think");
        assert_eq!(words[3], "it's");
        assert_eq!(words[4], "great");
    }
}
