/// Tracks current position in text navigation
pub struct PositionTracker {
    position: usize,
    total_sentences: usize,
}

impl PositionTracker {
    pub fn new() -> Self {
        Self {
            position: 0,
            total_sentences: 0,
        }
    }

    /// Reset position and set total sentences
    pub fn reset(&mut self, total_sentences: usize) {
        self.position = 0;
        self.total_sentences = total_sentences;
    }

    /// Move to next position
    pub fn advance(&mut self) -> bool {
        if self.position + 1 < self.total_sentences {
            self.position += 1;
            true
        } else {
            false
        }
    }

    /// Move to previous position
    pub fn previous(&mut self) -> bool {
        if self.position > 0 {
            self.position -= 1;
            true
        } else {
            false
        }
    }

    /// Go to specific position
    pub fn goto(&mut self, position: usize) -> bool {
        if position < self.total_sentences {
            self.position = position;
            true
        } else {
            false
        }
    }

    /// Get current position
    pub fn current_position(&self) -> usize {
        self.position
    }

    /// Get total sentences
    pub fn total_sentences(&self) -> usize {
        self.total_sentences
    }

    /// Check if at beginning
    pub fn is_at_beginning(&self) -> bool {
        self.position == 0
    }

    /// Check if at end
    pub fn is_at_end(&self) -> bool {
        self.position + 1 >= self.total_sentences
    }

    /// Get progress as percentage (0.0 to 1.0)
    pub fn progress(&self) -> f64 {
        if self.total_sentences == 0 {
            0.0
        } else {
            self.position as f64 / self.total_sentences.max(1) as f64
        }
    }

    /// Get remaining sentences
    pub fn remaining_sentences(&self) -> usize {
        self.total_sentences.saturating_sub(self.position + 1)
    }
}

impl Default for PositionTracker {
    fn default() -> Self {
        Self::new()
    }
}
