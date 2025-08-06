use std::fs;
use std::path::PathBuf;
use anyhow::{Result, Context};
use glossia_shared::types::{KnownWords, WordEncounters};

const PROMOTION_THRESHOLD: u32 = 12;

#[derive(Clone, Debug)]
pub struct VocabularyManager {
    glossia_dir: PathBuf,
}

impl VocabularyManager {
    pub fn new() -> Result<Self> {
        let home_dir = dirs::home_dir()
            .context("Could not find home directory")?;
        
        let glossia_dir = home_dir.join(".glossia");
        
        // Create .glossia directory if it doesn't exist
        if !glossia_dir.exists() {
            fs::create_dir_all(&glossia_dir)
                .context("Failed to create .glossia directory")?;
        }
        
        Ok(Self { glossia_dir })
    }

    fn known_words_path(&self) -> PathBuf {
        self.glossia_dir.join("known_words.json")
    }

    fn word_encounters_path(&self) -> PathBuf {
        self.glossia_dir.join("word_encounters.json")
    }

    pub fn load_known_words(&self) -> Result<KnownWords> {
        let path = self.known_words_path();
        
        if !path.exists() {
            return Ok(KnownWords::new());
        }

        let content = fs::read_to_string(&path)
            .context("Failed to read known_words.json")?;
        
        let known_words: KnownWords = serde_json::from_str(&content)
            .context("Failed to parse known_words.json")?;
        
        Ok(known_words)
    }

    pub fn save_known_words(&self, known_words: &KnownWords) -> Result<()> {
        let path = self.known_words_path();
        let content = serde_json::to_string_pretty(known_words)
            .context("Failed to serialize known words")?;
        
        fs::write(&path, content)
            .context("Failed to write known_words.json")?;
        
        Ok(())
    }

    pub fn load_word_encounters(&self) -> Result<WordEncounters> {
        let path = self.word_encounters_path();
        
        if !path.exists() {
            return Ok(WordEncounters::new());
        }

        let content = fs::read_to_string(&path)
            .context("Failed to read word_encounters.json")?;
        
        let word_encounters: WordEncounters = serde_json::from_str(&content)
            .context("Failed to parse word_encounters.json")?;
        
        Ok(word_encounters)
    }

    pub fn save_word_encounters(&self, word_encounters: &WordEncounters) -> Result<()> {
        let path = self.word_encounters_path();
        let content = serde_json::to_string_pretty(word_encounters)
            .context("Failed to serialize word encounters")?;
        
        fs::write(&path, content)
            .context("Failed to write word_encounters.json")?;
        
        Ok(())
    }

    pub fn add_word_encounter(&self, word: &str) -> Result<(u32, bool)> {
        let mut encounters = self.load_word_encounters()?;
        let mut known_words = self.load_known_words()?;
        
        // Don't track encounters for already known words
        if known_words.contains(word) {
            return Ok((0, false));
        }
        
        let count = encounters.increment_word(word.to_string());
        let should_promote = count >= PROMOTION_THRESHOLD;
        
        if should_promote {
            // Promote to known words
            known_words.add_word(word.to_string());
            encounters.remove_word(word);
            
            // Save both files
            self.save_known_words(&known_words)?;
            self.save_word_encounters(&encounters)?;
            
            Ok((count, true))
        } else {
            // Just save encounters
            self.save_word_encounters(&encounters)?;
            Ok((count, false))
        }
    }

    pub fn add_known_word(&self, word: &str) -> Result<bool> {
        let mut known_words = self.load_known_words()?;
        let mut encounters = self.load_word_encounters()?;
        
        let was_new = known_words.add_word(word.to_string());
        
        if was_new {
            // Remove from encounters if it exists
            encounters.remove_word(word);
            
            // Save both files
            self.save_known_words(&known_words)?;
            self.save_word_encounters(&encounters)?;
        }
        
        Ok(was_new)
    }

    pub fn remove_known_word(&self, word: &str) -> Result<bool> {
        let mut known_words = self.load_known_words()?;
        let was_removed = known_words.remove_word(word);
        
        if was_removed {
            self.save_known_words(&known_words)?;
        }
        
        Ok(was_removed)
    }

    pub fn get_word_progress(&self, word: &str) -> Result<(u32, bool)> {
        let known_words = self.load_known_words()?;
        
        if known_words.contains(word) {
            return Ok((PROMOTION_THRESHOLD, true));
        }
        
        let encounters = self.load_word_encounters()?;
        let count = encounters.get_count(word);
        
        Ok((count, false))
    }

    pub fn get_known_words_count(&self) -> Result<usize> {
        let known_words = self.load_known_words()?;
        Ok(known_words.count())
    }

    pub fn get_all_known_words(&self) -> Result<Vec<String>> {
        let known_words = self.load_known_words()?;
        Ok(known_words.get_words())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    
    fn create_test_manager() -> (VocabularyManager, TempDir) {
        let temp_dir = TempDir::new().unwrap();
        let manager = VocabularyManager {
            glossia_dir: temp_dir.path().to_path_buf(),
        };
        (manager, temp_dir)
    }
    
    #[test]
    fn test_add_word_encounter() {
        let (manager, _temp) = create_test_manager();
        
        // First encounter
        let (count, promoted) = manager.add_word_encounter("test").unwrap();
        assert_eq!(count, 1);
        assert!(!promoted);
        
        // Multiple encounters
        for i in 2..=11 {
            let (count, promoted) = manager.add_word_encounter("test").unwrap();
            assert_eq!(count, i);
            assert!(!promoted);
        }
        
        // 12th encounter should promote
        let (count, promoted) = manager.add_word_encounter("test").unwrap();
        assert_eq!(count, 12);
        assert!(promoted);
        
        // Verify it's now a known word
        let known_words = manager.load_known_words().unwrap();
        assert!(known_words.contains("test"));
        
        // Verify it's removed from encounters
        let encounters = manager.load_word_encounters().unwrap();
        assert_eq!(encounters.get_count("test"), 0);
    }
    
    #[test]
    fn test_add_known_word_directly() {
        let (manager, _temp) = create_test_manager();
        
        let was_new = manager.add_known_word("hello").unwrap();
        assert!(was_new);
        
        let was_new = manager.add_known_word("hello").unwrap();
        assert!(!was_new);
        
        let known_words = manager.load_known_words().unwrap();
        assert!(known_words.contains("hello"));
    }
}
