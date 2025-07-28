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
