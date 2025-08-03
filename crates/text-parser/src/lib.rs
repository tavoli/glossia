use regex::Regex;

/// Splits a block of text into a list of sentences.
/// Sentences are split by '.', '?', '!', and '|'.
pub fn split_into_sentences(text: &str) -> Vec<String> {
    if text.is_empty() {
        return vec![];
    }

    // Split by sentence-ending punctuation, keeping the delimiter.
    let re = match Regex::new(r"([.?!|;])\s+") {
        Ok(regex) => regex,
        Err(_) => {
            // If regex compilation fails, fallback to simple splitting
            return text.split(&['.', '?', '!', '|', ';'])
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();
        }
    };
    let mut sentences = Vec::new();
    let mut last_end = 0;
    
    for mat in re.find_iter(text) {
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
