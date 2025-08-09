/// Manages transient state for the reading engine
pub struct StateManager {
    is_processing: bool,
    last_error: Option<String>,
    session_start: std::time::Instant,
    sentences_read: usize,
    words_learned: usize,
}

impl StateManager {
    pub fn new() -> Self {
        Self {
            is_processing: false,
            last_error: None,
            session_start: std::time::Instant::now(),
            sentences_read: 0,
            words_learned: 0,
        }
    }

    /// Reset all state (useful when loading new text)
    pub fn reset(&mut self) {
        self.is_processing = false;
        self.last_error = None;
        self.session_start = std::time::Instant::now();
        self.sentences_read = 0;
        self.words_learned = 0;
    }

    /// Processing state
    pub fn is_processing(&self) -> bool {
        self.is_processing
    }

    pub fn set_processing(&mut self, processing: bool) {
        self.is_processing = processing;
    }

    /// Error state
    pub fn get_last_error(&self) -> Option<&str> {
        self.last_error.as_deref()
    }

    pub fn set_error(&mut self, error: String) {
        self.last_error = Some(error);
    }

    pub fn clear_error(&mut self) {
        self.last_error = None;
    }

    /// Session statistics
    pub fn session_duration(&self) -> std::time::Duration {
        self.session_start.elapsed()
    }

    pub fn sentences_read(&self) -> usize {
        self.sentences_read
    }

    pub fn increment_sentences_read(&mut self) {
        self.sentences_read += 1;
    }

    pub fn words_learned(&self) -> usize {
        self.words_learned
    }

    pub fn increment_words_learned(&mut self) {
        self.words_learned += 1;
    }

    /// Reading rate calculations
    pub fn sentences_per_minute(&self) -> f64 {
        let duration_minutes = self.session_duration().as_secs_f64() / 60.0;
        if duration_minutes > 0.0 {
            self.sentences_read as f64 / duration_minutes
        } else {
            0.0
        }
    }

    pub fn words_per_minute(&self) -> f64 {
        let duration_minutes = self.session_duration().as_secs_f64() / 60.0;
        if duration_minutes > 0.0 {
            self.words_learned as f64 / duration_minutes
        } else {
            0.0
        }
    }

    /// Reset session stats without clearing other state
    pub fn reset_session_stats(&mut self) {
        self.session_start = std::time::Instant::now();
        self.sentences_read = 0;
        self.words_learned = 0;
    }
}

impl Default for StateManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Session statistics for reporting
pub struct SessionStats {
    pub duration: std::time::Duration,
    pub sentences_read: usize,
    pub words_learned: usize,
    pub sentences_per_minute: f64,
    pub words_per_minute: f64,
}

impl StateManager {
    pub fn get_session_stats(&self) -> SessionStats {
        SessionStats {
            duration: self.session_duration(),
            sentences_read: self.sentences_read(),
            words_learned: self.words_learned(),
            sentences_per_minute: self.sentences_per_minute(),
            words_per_minute: self.words_per_minute(),
        }
    }
}
