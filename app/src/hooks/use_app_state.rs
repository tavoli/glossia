use dioxus::prelude::*;
use crate::hooks::{use_reading_state, use_vocabulary, VocabularyState};
use crate::theme::{use_theme, ThemeMode, Theme};
use std::collections::HashSet;
use tracing::{instrument, info, debug};

/// Centralized application state management
#[derive(Clone)]
pub struct AppState {
    pub reading_state: Signal<glossia_reading_engine::ReadingEngine>,
    pub vocabulary_state: Signal<VocabularyState>,
    pub theme_mode: Signal<ThemeMode>,
    pub theme: Theme,
    pub show_input_modal: Signal<bool>,
    pub show_known_words_modal: Signal<bool>,
    pub sentence_to_fetch: Signal<String>,
    pub word_to_fetch: Signal<String>,
    pub promotion_notification: Signal<Option<String>>,
    pub encounter_tracked_sentences: Signal<HashSet<String>>,
    pub last_clipboard_text: Signal<Option<String>>,
    pub current_clipboard_text: Signal<Option<String>>,
    pub show_clipboard_toast: Signal<bool>,
}

impl AppState {
    /// Toggle between light and dark themes
    pub fn toggle_theme(&mut self) {
        let new_mode = match *self.theme_mode.read() {
            ThemeMode::Light => ThemeMode::Dark,
            ThemeMode::Dark => ThemeMode::Light,
        };
        self.theme_mode.set(new_mode);
    }

    /// Show the input modal for new text
    pub fn show_input_modal(&mut self) {
        self.show_input_modal.set(true);
    }

    /// Hide the input modal
    pub fn hide_input_modal(&mut self) {
        self.show_input_modal.set(false);
    }

    /// Show the known words modal
    pub fn show_known_words_modal(&mut self) {
        self.show_known_words_modal.set(true);
    }

    /// Hide the known words modal
    pub fn hide_known_words_modal(&mut self) {
        self.show_known_words_modal.set(false);
    }

    /// Load new text into the reading state
    #[instrument(skip(self, text), fields(text_length = text.len(), text_empty = text.is_empty()))]
    pub fn load_text(&mut self, text: String) {
        info!(text_length = text.len(), "Loading new text into reading state");
        if !text.is_empty() {
            // Update reading state
            {
                if let Err(e) = self.reading_state.write().load_text(&text) {
                    tracing::error!(
                        event = "text_load_failed",
                        component = "app_state",
                        error = ?e,
                        text_length = text.len(),
                        "Failed to load text into reading engine"
                    );
                    return;
                }
            }
            
            // Clear encounter tracking for new text
            self.encounter_tracked_sentences.write().clear();
            
            // Set sentence to fetch if available
            if let Some(sentence) = self.reading_state.read().current_sentence() {
                debug!(sentence_length = sentence.len(), "Setting initial sentence to fetch");
                self.sentence_to_fetch.set(sentence);
            }
            
            // Update last clipboard text to prevent toast from showing for this text
            self.last_clipboard_text.set(Some(text.clone()));
            
            info!(total_sentences = self.reading_state.read().total_sentences(), "Text loaded successfully");
            self.hide_input_modal();
        }
    }
    
    /// Load text from clipboard without showing modal
    pub fn load_text_from_clipboard(&mut self) {
        let text = self.current_clipboard_text.read().clone();
        if let Some(text) = text {
            info!("Loading text from clipboard: {} chars", text.len());
            
            // Load the text
            self.load_text(text.clone());
            
            // Update last clipboard text
            self.last_clipboard_text.set(Some(text));
            
            // Hide the toast
            self.show_clipboard_toast.set(false);
            
            // Make sure modal is not shown
            self.hide_input_modal();
        }
    }
    
    /// Dismiss clipboard toast
    pub fn dismiss_clipboard_toast(&mut self) {
        self.show_clipboard_toast.set(false);
        // Update last clipboard to prevent re-showing
        let text = self.current_clipboard_text.read().clone();
        if let Some(text) = text {
            self.last_clipboard_text.set(Some(text));
        }
    }

    /// Get the current known words count
    pub fn known_words_count(&self) -> usize {
        self.vocabulary_state.read().known_words_count
    }

    /// Get the floating button count
    pub fn floating_button_count(&self) -> usize {
        if self.reading_state.read().total_sentences() > 0 {
            self.reading_state.read().total_sentences().saturating_sub(self.reading_state.read().position() + 1)
        } else {
            0
        }
    }

    /// Get progress bar values
    pub fn progress_values(&self) -> (usize, usize) {
        let total = self.reading_state.read().total_sentences();
        let current = if total > 0 { 
            self.reading_state.read().position() + 1 
        } else { 
            0 
        };
        (current, total)
    }

    /// Set promotion notification
    #[allow(dead_code)]
    pub fn set_promotion_notification(&mut self, message: Option<String>) {
        self.promotion_notification.set(message);
    }

    /// Clear promotion notification after delay
    #[allow(dead_code)]
    pub fn clear_promotion_notification_after_delay(&mut self) {
        let mut notification_clone = self.promotion_notification.clone();
        spawn(async move {
            tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
            notification_clone.set(None);
        });
    }
}

/// Hook to manage centralized application state
pub fn use_app_state() -> AppState {
    let reading_state = use_reading_state();
    let vocabulary_state = use_vocabulary();
    let theme_mode = use_theme();
    let theme = Theme::from_mode(*theme_mode.read());
    
    // Don't show input modal by default - we'll check clipboard first
    let show_input_modal = use_signal(|| false);
    let show_known_words_modal = use_signal(|| false);
    let sentence_to_fetch = use_signal(String::new);
    let word_to_fetch = use_signal(String::new);
    let promotion_notification = use_signal(|| None::<String>);
    let encounter_tracked_sentences = use_signal(|| HashSet::<String>::new());
    let last_clipboard_text = use_signal(|| None::<String>);
    let current_clipboard_text = use_signal(|| None::<String>);
    let show_clipboard_toast = use_signal(|| false);

    AppState {
        reading_state,
        vocabulary_state,
        theme_mode,
        theme,
        show_input_modal,
        show_known_words_modal,
        sentence_to_fetch,
        word_to_fetch,
        promotion_notification,
        encounter_tracked_sentences,
        last_clipboard_text,
        current_clipboard_text,
        show_clipboard_toast,
    }
}
