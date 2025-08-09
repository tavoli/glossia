use glossia_text_parser::split_into_sentences;
use glossia_shared::AppError;

/// Handles text loading and sentence splitting
pub struct TextLoader {
    sentences: Option<Vec<String>>,
}

impl TextLoader {
    pub fn new() -> Self {
        Self {
            sentences: None,
        }
    }

    /// Load text and split into sentences
    pub fn load_text(&mut self, text: &str) -> Result<Vec<String>, AppError> {
        if text.trim().is_empty() {
            return Err(AppError::config_error("Text cannot be empty"));
        }

        let sentences = split_into_sentences(text);
        
        if sentences.is_empty() {
            return Err(AppError::config_error("No sentences found in text"));
        }

        self.sentences = Some(sentences.clone());
        Ok(sentences)
    }

    /// Get loaded sentences
    pub fn get_sentences(&self) -> Option<&Vec<String>> {
        self.sentences.as_ref()
    }

    /// Clear loaded sentences
    pub fn clear(&mut self) {
        self.sentences = None;
    }

    /// Check if text is loaded
    pub fn has_text(&self) -> bool {
        self.sentences.is_some()
    }

    /// Get sentence count
    pub fn sentence_count(&self) -> usize {
        self.sentences.as_ref().map(|s| s.len()).unwrap_or(0)
    }
}

impl Default for TextLoader {
    fn default() -> Self {
        Self::new()
    }
}
