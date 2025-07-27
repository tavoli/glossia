use serde::{Deserialize, Serialize};

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
}
