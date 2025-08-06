use dioxus::prelude::*;
use crate::vocabulary::VocabularyManager;
use glossia_shared::types::{KnownWords, WordMeaning};

#[derive(Clone, Debug)]
pub struct VocabularyState {
    pub known_words: KnownWords,
    pub known_words_count: usize,
    pub manager: VocabularyManager,
}

impl VocabularyState {
    pub fn new() -> Result<Self, anyhow::Error> {
        let manager = VocabularyManager::new()?;
        let known_words = manager.load_known_words()?;
        let known_words_count = known_words.count();
        
        Ok(Self {
            known_words,
            known_words_count,
            manager,
        })
    }

    pub fn refresh(&mut self) -> Result<(), anyhow::Error> {
        self.known_words = self.manager.load_known_words()?;
        self.known_words_count = self.known_words.count();
        Ok(())
    }

    pub fn add_known_word(&mut self, word: &str) -> Result<bool, anyhow::Error> {
        let was_new = self.manager.add_known_word(word)?;
        if was_new {
            self.refresh()?;
        }
        Ok(was_new)
    }

    pub fn remove_known_word(&mut self, word: &str) -> Result<bool, anyhow::Error> {
        let was_removed = self.manager.remove_known_word(word)?;
        if was_removed {
            self.refresh()?;
        }
        Ok(was_removed)
    }

    pub fn add_word_encounter(&mut self, word: &str) -> Result<(u32, bool), anyhow::Error> {
        let (count, promoted) = self.manager.add_word_encounter(word)?;
        if promoted {
            self.refresh()?;
        }
        Ok((count, promoted))
    }

    pub fn filter_known_words(&self, words: &[WordMeaning]) -> Vec<WordMeaning> {
        words.iter()
            .filter(|word_meaning| !self.known_words.contains(&word_meaning.word))
            .cloned()
            .collect()
    }

    pub fn get_word_progress(&self, word: &str) -> Result<(u32, bool), anyhow::Error> {
        self.manager.get_word_progress(word)
    }
}

pub fn use_vocabulary() -> Signal<VocabularyState> {
    use_signal(|| {
        VocabularyState::new().unwrap_or_else(|e| {
            eprintln!("Failed to initialize vocabulary: {}", e);
            // Return a default state with empty manager - this might fail but we'll handle it gracefully
            VocabularyState {
                known_words: KnownWords::new(),
                known_words_count: 0,
                manager: VocabularyManager::new().unwrap_or_else(|_| {
                    panic!("Cannot create vocabulary manager")
                }),
            }
        })
    })
}
