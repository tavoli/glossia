use glossia_shared::AppError;

/// Trait for different navigation strategies
/// Enables different reading modes (linear, adaptive, speed reading, etc.)
pub trait NavigationStrategy: Send + Sync {
    /// Load text and initialize navigation state
    fn load_text(&mut self, text: &str) -> Result<(), AppError>;
    
    /// Get current content to display
    fn current_content(&self) -> Option<String>;
    
    /// Move to next content unit
    fn next(&mut self) -> bool;
    
    /// Move to previous content unit
    fn previous(&mut self) -> bool;
    
    /// Jump to specific position (0.0 to 1.0)
    fn goto_progress(&mut self, progress: f64) -> bool;
    
    /// Get current progress (0.0 to 1.0)
    fn progress(&self) -> f64;
    
    /// Check if at beginning
    fn is_at_beginning(&self) -> bool;
    
    /// Check if at end
    fn is_at_end(&self) -> bool;
    
    /// Get strategy name for debugging
    fn strategy_name(&self) -> &str;
    
    /// Get recommended reading speed (words per minute)
    fn recommended_wpm(&self) -> Option<u32> {
        None
    }
    
    /// Get recommended pause duration between units (milliseconds)
    fn recommended_pause_ms(&self) -> Option<u32> {
        None
    }
    
    /// Get units processed in current session
    fn units_processed(&self) -> usize;
    
    /// Reset navigation state
    fn reset(&mut self);
}

/// Linear sentence-by-sentence navigation (current default)
pub struct LinearNavigationStrategy {
    sentences: Vec<String>,
    current_position: usize,
    total_units_processed: usize,
}

impl LinearNavigationStrategy {
    pub fn new() -> Self {
        Self {
            sentences: Vec::new(),
            current_position: 0,
            total_units_processed: 0,
        }
    }
}

impl Default for LinearNavigationStrategy {
    fn default() -> Self {
        Self::new()
    }
}

impl NavigationStrategy for LinearNavigationStrategy {
    fn load_text(&mut self, text: &str) -> Result<(), AppError> {
        use glossia_text_parser::split_into_sentences;
        
        self.sentences = split_into_sentences(text);
        self.current_position = 0;
        self.total_units_processed = 0;
        Ok(())
    }
    
    fn current_content(&self) -> Option<String> {
        self.sentences.get(self.current_position).cloned()
    }
    
    fn next(&mut self) -> bool {
        if self.current_position < self.sentences.len().saturating_sub(1) {
            self.current_position += 1;
            self.total_units_processed += 1;
            true
        } else {
            false
        }
    }
    
    fn previous(&mut self) -> bool {
        if self.current_position > 0 {
            self.current_position -= 1;
            true
        } else {
            false
        }
    }
    
    fn goto_progress(&mut self, progress: f64) -> bool {
        if self.sentences.is_empty() {
            return false;
        }
        
        let progress = progress.clamp(0.0, 1.0);
        let target_position = (progress * (self.sentences.len() - 1) as f64).round() as usize;
        
        if target_position < self.sentences.len() {
            self.current_position = target_position;
            true
        } else {
            false
        }
    }
    
    fn progress(&self) -> f64 {
        if self.sentences.len() <= 1 {
            if self.sentences.is_empty() { 0.0 } else { 1.0 }
        } else {
            self.current_position as f64 / (self.sentences.len() - 1) as f64
        }
    }
    
    fn is_at_beginning(&self) -> bool {
        self.current_position == 0
    }
    
    fn is_at_end(&self) -> bool {
        self.sentences.is_empty() || self.current_position >= self.sentences.len() - 1
    }
    
    fn strategy_name(&self) -> &str {
        "Linear"
    }
    
    fn units_processed(&self) -> usize {
        self.total_units_processed
    }
    
    fn reset(&mut self) {
        self.current_position = 0;
        self.total_units_processed = 0;
    }
}

/// Paragraph-based navigation for faster reading
pub struct ParagraphNavigationStrategy {
    paragraphs: Vec<String>,
    current_position: usize,
    total_units_processed: usize,
}

impl ParagraphNavigationStrategy {
    pub fn new() -> Self {
        Self {
            paragraphs: Vec::new(),
            current_position: 0,
            total_units_processed: 0,
        }
    }
}

impl Default for ParagraphNavigationStrategy {
    fn default() -> Self {
        Self::new()
    }
}

impl NavigationStrategy for ParagraphNavigationStrategy {
    fn load_text(&mut self, text: &str) -> Result<(), AppError> {
        // Split by double newlines to create paragraphs
        self.paragraphs = text
            .split("\n\n")
            .map(|p| p.trim())
            .filter(|p| !p.is_empty())
            .map(|p| p.to_string())
            .collect();
        
        if self.paragraphs.is_empty() {
            // Fallback: treat entire text as one paragraph
            self.paragraphs.push(text.to_string());
        }
        
        self.current_position = 0;
        self.total_units_processed = 0;
        Ok(())
    }
    
    fn current_content(&self) -> Option<String> {
        self.paragraphs.get(self.current_position).cloned()
    }
    
    fn next(&mut self) -> bool {
        if self.current_position < self.paragraphs.len().saturating_sub(1) {
            self.current_position += 1;
            self.total_units_processed += 1;
            true
        } else {
            false
        }
    }
    
    fn previous(&mut self) -> bool {
        if self.current_position > 0 {
            self.current_position -= 1;
            true
        } else {
            false
        }
    }
    
    fn goto_progress(&mut self, progress: f64) -> bool {
        if self.paragraphs.is_empty() {
            return false;
        }
        
        let progress = progress.clamp(0.0, 1.0);
        let target_position = (progress * (self.paragraphs.len() - 1) as f64).round() as usize;
        
        if target_position < self.paragraphs.len() {
            self.current_position = target_position;
            true
        } else {
            false
        }
    }
    
    fn progress(&self) -> f64 {
        if self.paragraphs.len() <= 1 {
            if self.paragraphs.is_empty() { 0.0 } else { 1.0 }
        } else {
            self.current_position as f64 / (self.paragraphs.len() - 1) as f64
        }
    }
    
    fn is_at_beginning(&self) -> bool {
        self.current_position == 0
    }
    
    fn is_at_end(&self) -> bool {
        self.paragraphs.is_empty() || self.current_position >= self.paragraphs.len() - 1
    }
    
    fn strategy_name(&self) -> &str {
        "Paragraph"
    }
    
    fn recommended_wpm(&self) -> Option<u32> {
        Some(200) // Faster reading for paragraphs
    }
    
    fn units_processed(&self) -> usize {
        self.total_units_processed
    }
    
    fn reset(&mut self) {
        self.current_position = 0;
        self.total_units_processed = 0;
    }
}

/// Speed reading navigation with configurable chunk sizes
pub struct SpeedReadingStrategy {
    chunks: Vec<String>,
    current_position: usize,
    total_units_processed: usize,
    chunk_size: usize, // words per chunk
    wpm: u32,
}

impl SpeedReadingStrategy {
    pub fn new() -> Self {
        Self {
            chunks: Vec::new(),
            current_position: 0,
            total_units_processed: 0,
            chunk_size: 5, // Default 5 words per chunk
            wpm: 300,      // Default 300 WPM
        }
    }
    
    pub fn with_chunk_size(mut self, chunk_size: usize) -> Self {
        self.chunk_size = chunk_size.max(1);
        self
    }
    
    pub fn with_wpm(mut self, wpm: u32) -> Self {
        self.wpm = wpm.max(100);
        self
    }
}

impl Default for SpeedReadingStrategy {
    fn default() -> Self {
        Self::new()
    }
}

impl NavigationStrategy for SpeedReadingStrategy {
    fn load_text(&mut self, text: &str) -> Result<(), AppError> {
        use glossia_text_parser::extract_words;
        
        let words = extract_words(text);
        self.chunks = words
            .chunks(self.chunk_size)
            .map(|chunk| chunk.join(" "))
            .collect();
        
        self.current_position = 0;
        self.total_units_processed = 0;
        Ok(())
    }
    
    fn current_content(&self) -> Option<String> {
        self.chunks.get(self.current_position).cloned()
    }
    
    fn next(&mut self) -> bool {
        if self.current_position < self.chunks.len().saturating_sub(1) {
            self.current_position += 1;
            self.total_units_processed += 1;
            true
        } else {
            false
        }
    }
    
    fn previous(&mut self) -> bool {
        if self.current_position > 0 {
            self.current_position -= 1;
            true
        } else {
            false
        }
    }
    
    fn goto_progress(&mut self, progress: f64) -> bool {
        if self.chunks.is_empty() {
            return false;
        }
        
        let progress = progress.clamp(0.0, 1.0);
        let target_position = (progress * (self.chunks.len() - 1) as f64).round() as usize;
        
        if target_position < self.chunks.len() {
            self.current_position = target_position;
            true
        } else {
            false
        }
    }
    
    fn progress(&self) -> f64 {
        if self.chunks.len() <= 1 {
            if self.chunks.is_empty() { 0.0 } else { 1.0 }
        } else {
            self.current_position as f64 / (self.chunks.len() - 1) as f64
        }
    }
    
    fn is_at_beginning(&self) -> bool {
        self.current_position == 0
    }
    
    fn is_at_end(&self) -> bool {
        self.chunks.is_empty() || self.current_position >= self.chunks.len() - 1
    }
    
    fn strategy_name(&self) -> &str {
        "SpeedReading"
    }
    
    fn recommended_wpm(&self) -> Option<u32> {
        Some(self.wpm)
    }
    
    fn recommended_pause_ms(&self) -> Option<u32> {
        // Calculate pause based on WPM and chunk size
        let words_per_chunk = self.chunk_size as u32;
        let ms_per_word = 60_000 / self.wpm; // 60,000 ms per minute
        Some(ms_per_word * words_per_chunk)
    }
    
    fn units_processed(&self) -> usize {
        self.total_units_processed
    }
    
    fn reset(&mut self) {
        self.current_position = 0;
        self.total_units_processed = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear_navigation() {
        let mut strategy = LinearNavigationStrategy::new();
        let text = "First sentence. Second sentence. Third sentence.";
        
        strategy.load_text(text).unwrap();
        assert_eq!(strategy.current_content(), Some("First sentence.".to_string()));
        assert!(strategy.is_at_beginning());
        assert!(!strategy.is_at_end());
        
        assert!(strategy.next());
        assert_eq!(strategy.current_content(), Some("Second sentence.".to_string()));
        
        assert!(strategy.next());
        assert_eq!(strategy.current_content(), Some("Third sentence.".to_string()));
        assert!(strategy.is_at_end());
        
        assert!(!strategy.next()); // Can't go beyond end
    }

    #[test]
    fn test_paragraph_navigation() {
        let mut strategy = ParagraphNavigationStrategy::new();
        let text = "First paragraph.\n\nSecond paragraph with more text.\n\nThird paragraph.";
        
        strategy.load_text(text).unwrap();
        assert_eq!(strategy.current_content(), Some("First paragraph.".to_string()));
        
        assert!(strategy.next());
        assert_eq!(strategy.current_content(), Some("Second paragraph with more text.".to_string()));
        
        assert!(strategy.next());
        assert_eq!(strategy.current_content(), Some("Third paragraph.".to_string()));
    }

    #[test]
    fn test_speed_reading_strategy() {
        let mut strategy = SpeedReadingStrategy::new().with_chunk_size(2);
        let text = "one two three four five six";
        
        strategy.load_text(text).unwrap();
        assert_eq!(strategy.current_content(), Some("one two".to_string()));
        
        assert!(strategy.next());
        assert_eq!(strategy.current_content(), Some("three four".to_string()));
        
        assert!(strategy.next());
        assert_eq!(strategy.current_content(), Some("five six".to_string()));
    }

    #[test]
    fn test_progress_calculation() {
        let mut strategy = LinearNavigationStrategy::new();
        strategy.load_text("One. Two. Three.").unwrap();
        
        assert_eq!(strategy.progress(), 0.0); // At beginning
        
        strategy.next();
        assert_eq!(strategy.progress(), 0.5); // Middle
        
        strategy.next();
        assert_eq!(strategy.progress(), 1.0); // At end
    }
}
