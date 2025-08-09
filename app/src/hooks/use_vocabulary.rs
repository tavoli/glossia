use dioxus::prelude::*;
use glossia_vocabulary_manager::VocabularyManager;
use glossia_shared::WordMeaning;
use anyhow::Result;

pub struct VocabularyState {
    pub known_words_count: usize,
    pub manager: VocabularyManager,
}

impl VocabularyState {
    pub fn new() -> Result<Self, anyhow::Error> {
        let manager = VocabularyManager::new().map_err(|e| anyhow::anyhow!("{:?}", e))?;
        let known_words_count = manager.get_known_words_count();
        
        Ok(Self {
            known_words_count,
            manager,
        })
    }

    pub fn refresh(&mut self) -> Result<(), anyhow::Error> {
        self.known_words_count = self.manager.get_known_words_count();
        Ok(())
    }

    pub fn add_known_word(&mut self, word: &str) -> Result<bool, anyhow::Error> {
        self.manager.add_known_word(word).map_err(|e| anyhow::anyhow!("{:?}", e))?;
        self.refresh()?;
        Ok(true) // The new vocabulary manager doesn't return whether it was new
    }

    pub fn remove_known_word(&mut self, word: &str) -> Result<bool, anyhow::Error> {
        self.manager.remove_known_word(word).map_err(|e| anyhow::anyhow!("{:?}", e))?;
        self.refresh()?;
        Ok(true) // Simplified - assume it was removed
    }

    pub fn add_word_encounter(&mut self, word: &str) -> Result<(u32, bool), anyhow::Error> {
        let (count, promoted) = self.manager.add_word_encounter(word).map_err(|e| anyhow::anyhow!("{:?}", e))?;
        if promoted {
            self.refresh()?;
        }
        Ok((count as u32, promoted))
    }

    pub fn filter_known_words(&self, words: &[WordMeaning]) -> Vec<WordMeaning> {
        self.manager.filter_known_words(words)
    }

    pub fn get_word_progress(&self, _word: &str) -> Result<(u32, bool), anyhow::Error> {
        // The new vocabulary manager doesn't have get_word_progress
        // We'll return a placeholder for now
        Ok((0, false))
    }
}

pub fn use_vocabulary() -> Signal<VocabularyState> {
    use_signal(|| {
        VocabularyState::new().unwrap_or_else(|e| {
            tracing::error!(
                event = "vocabulary_initialization_failed",
                component = "vocabulary_hook",
                error = %e,
                "Failed to initialize vocabulary manager"
            );
            // Return a default state with empty manager - this might fail but we'll handle it gracefully
            VocabularyState {
                known_words_count: 0,
                manager: VocabularyManager::new().unwrap_or_else(|_| {
                    panic!("Cannot create vocabulary manager")
                }),
            }
        })
    })
}
