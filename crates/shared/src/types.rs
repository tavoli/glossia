use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SimplificationRequest {
    pub sentence: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SimplificationResponse {
    pub original: String,
    pub simplified: String,
    pub words: Vec<WordMeaning>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WordMeaning {
    pub word: String,
    pub meaning: String,
    #[serde(default)]
    pub is_phrase: bool,
}

impl WordMeaning {
    pub fn new_word(word: String, meaning: String) -> Self {
        Self {
            word,
            meaning,
            is_phrase: false,
        }
    }
    
    pub fn new_phrase(phrase: String, meaning: String) -> Self {
        Self {
            word: phrase,
            meaning,
            is_phrase: true,
        }
    }
    
    pub fn word_count(&self) -> usize {
        if self.is_phrase {
            self.word.split_whitespace().count()
        } else {
            1
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ImageResult {
    pub url: String,
    pub thumbnail_url: String,
    pub title: String,
    pub width: Option<u32>,
    pub height: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ImageSearchRequest {
    pub query: String,
    pub count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ImageQueryOptimizationRequest {
    pub word: String,
    pub sentence_context: String,
    pub word_meaning: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ImageQueryOptimizationResponse {
    pub optimized_query: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct KnownWords {
    pub words: HashSet<String>,
}

impl KnownWords {
    pub fn new() -> Self {
        Self {
            words: HashSet::new(),
        }
    }

    pub fn add_word(&mut self, word: String) -> bool {
        self.words.insert(word.to_lowercase())
    }

    pub fn remove_word(&mut self, word: &str) -> bool {
        self.words.remove(&word.to_lowercase())
    }

    pub fn contains(&self, word: &str) -> bool {
        self.words.contains(&word.to_lowercase())
    }

    pub fn count(&self) -> usize {
        self.words.len()
    }

    pub fn get_words(&self) -> Vec<String> {
        let mut words: Vec<String> = self.words.iter().cloned().collect();
        words.sort();
        words
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct WordEncounters {
    pub encounters: HashMap<String, u32>,
}

impl WordEncounters {
    pub fn new() -> Self {
        Self {
            encounters: HashMap::new(),
        }
    }

    pub fn increment_word(&mut self, word: String) -> u32 {
        let word_lower = word.to_lowercase();
        let count = self.encounters.entry(word_lower).or_insert(0);
        *count += 1;
        *count
    }

    pub fn get_count(&self, word: &str) -> u32 {
        self.encounters.get(&word.to_lowercase()).copied().unwrap_or(0)
    }

    pub fn remove_word(&mut self, word: &str) -> Option<u32> {
        self.encounters.remove(&word.to_lowercase())
    }

    pub fn get_words_by_count(&self) -> Vec<(String, u32)> {
        let mut words: Vec<(String, u32)> = self.encounters.iter()
            .map(|(word, count)| (word.clone(), *count))
            .collect();
        words.sort_by(|a, b| b.1.cmp(&a.1)); // Sort by count descending
        words
    }

    pub fn should_promote(&self, word: &str, threshold: u32) -> bool {
        self.get_count(word) >= threshold
    }
}
