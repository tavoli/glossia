mod text_loader;
mod position_tracker;
mod history_manager;
mod navigation_trait;

pub use text_loader::TextLoader;
pub use position_tracker::PositionTracker;
pub use history_manager::HistoryManager;
pub use navigation_trait::{
    NavigationStrategy, LinearNavigationStrategy, 
    ParagraphNavigationStrategy, SpeedReadingStrategy
};

use glossia_shared::AppError;

/// Navigation service that handles text loading, position tracking, and history
pub struct NavigationService {
    text_loader: TextLoader,
    position_tracker: PositionTracker,
    history_manager: HistoryManager,
}

impl NavigationService {
    pub fn new() -> Self {
        Self {
            text_loader: TextLoader::new(),
            position_tracker: PositionTracker::new(),
            history_manager: HistoryManager::new(),
        }
    }

    /// Load text and reset position
    pub fn load_text(&mut self, text: &str) -> Result<(), AppError> {
        let sentences = self.text_loader.load_text(text)?;
        self.position_tracker.reset(sentences.len());
        self.history_manager.clear(); // Clear history when loading new text
        Ok(())
    }

    /// Get current sentence
    pub fn current_sentence(&self) -> Option<String> {
        if let Some(sentences) = self.text_loader.get_sentences() {
            let position = self.position_tracker.current_position();
            sentences.get(position).cloned()
        } else {
            None
        }
    }

    /// Move to next sentence
    pub fn advance(&mut self) -> bool {
        let old_position = self.position_tracker.current_position();
        let moved = self.position_tracker.advance();
        if moved {
            self.history_manager.add_position(old_position);
        }
        moved
    }

    /// Move to previous sentence
    pub fn previous(&mut self) -> bool {
        let old_position = self.position_tracker.current_position();
        let moved = self.position_tracker.previous();
        if moved {
            self.history_manager.add_position(old_position);
        }
        moved
    }

    /// Jump to specific position
    pub fn goto_position(&mut self, position: usize) -> bool {
        let old_position = self.position_tracker.current_position();
        let moved = self.position_tracker.goto(position);
        if moved {
            self.history_manager.add_position(old_position);
        }
        moved
    }

    /// Go back in history
    pub fn go_back(&mut self) -> bool {
        if let Some(position) = self.history_manager.go_back() {
            self.position_tracker.goto(position);
            true
        } else {
            false
        }
    }

    /// Go forward in history
    pub fn go_forward(&mut self) -> bool {
        if let Some(position) = self.history_manager.go_forward() {
            self.position_tracker.goto(position);
            true
        } else {
            false
        }
    }

    /// Get all sentences
    pub fn get_sentences(&self) -> Option<&Vec<String>> {
        self.text_loader.get_sentences()
    }

    /// Get current position
    pub fn current_position(&self) -> usize {
        self.position_tracker.current_position()
    }

    /// Get total sentences
    pub fn total_sentences(&self) -> usize {
        self.position_tracker.total_sentences()
    }

    /// Check if at beginning
    pub fn is_at_beginning(&self) -> bool {
        self.position_tracker.is_at_beginning()
    }

    /// Check if at end
    pub fn is_at_end(&self) -> bool {
        self.position_tracker.is_at_end()
    }

    /// Get reading progress (0.0 to 1.0)
    pub fn progress(&self) -> f64 {
        self.position_tracker.progress()
    }

    /// Check if can go back in history
    pub fn can_go_back(&self) -> bool {
        self.history_manager.can_go_back()
    }

    /// Check if can go forward in history
    pub fn can_go_forward(&self) -> bool {
        self.history_manager.can_go_forward()
    }
}

impl Default for NavigationService {
    fn default() -> Self {
        Self::new()
    }
}
