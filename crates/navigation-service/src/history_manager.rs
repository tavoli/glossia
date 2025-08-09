/// Manages navigation history for back/forward functionality
pub struct HistoryManager {
    history: Vec<usize>,
    current_index: Option<usize>,
    max_history: usize,
}

impl HistoryManager {
    pub fn new() -> Self {
        Self {
            history: Vec::new(),
            current_index: None,
            max_history: 50, // Limit history to prevent memory issues
        }
    }

    /// Add a position to history
    pub fn add_position(&mut self, position: usize) {
        // If we're in the middle of history, truncate everything after current position
        if let Some(current) = self.current_index {
            self.history.truncate(current + 1);
        }

        self.history.push(position);
        
        // Limit history size
        if self.history.len() > self.max_history {
            self.history.remove(0);
        }
        
        self.current_index = Some(self.history.len() - 1);
    }

    /// Go back in history
    pub fn go_back(&mut self) -> Option<usize> {
        if let Some(current) = self.current_index {
            if current > 0 {
                self.current_index = Some(current - 1);
                return Some(self.history[current - 1]);
            }
        }
        None
    }

    /// Go forward in history
    pub fn go_forward(&mut self) -> Option<usize> {
        if let Some(current) = self.current_index {
            if current + 1 < self.history.len() {
                self.current_index = Some(current + 1);
                return Some(self.history[current + 1]);
            }
        }
        None
    }

    /// Check if can go back
    pub fn can_go_back(&self) -> bool {
        self.current_index.is_some_and(|i| i > 0)
    }

    /// Check if can go forward
    pub fn can_go_forward(&self) -> bool {
        self.current_index.is_some_and(|i| i + 1 < self.history.len())
    }

    /// Clear history
    pub fn clear(&mut self) {
        self.history.clear();
        self.current_index = None;
    }

    /// Get history length
    pub fn len(&self) -> usize {
        self.history.len()
    }

    /// Check if history is empty
    pub fn is_empty(&self) -> bool {
        self.history.is_empty()
    }

    /// Set maximum history size
    pub fn set_max_history(&mut self, max: usize) {
        self.max_history = max;
        if self.history.len() > max {
            let excess = self.history.len() - max;
            self.history.drain(0..excess);
            if let Some(ref mut current) = self.current_index {
                *current = current.saturating_sub(excess);
            }
        }
    }
}

impl Default for HistoryManager {
    fn default() -> Self {
        Self::new()
    }
}
