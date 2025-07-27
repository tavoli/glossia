use glossia_api_client::OpenRouterClient;
use glossia_shared::{AppError, SimplificationResponse};
use std::collections::HashMap;
use glossia_text_parser::split_into_sentences;

#[derive(Clone, Default)]
pub struct ReadingState {
    pub sentences: Vec<String>,
    pub simplified_cache: HashMap<String, SimplificationResponse>,
    pub position: usize,
    pub total_sentences: usize,
    pub api_client: OpenRouterClient,
}

impl ReadingState {
    pub fn new() -> Self {
        Self {
            api_client: OpenRouterClient::new(),
            ..Default::default()
        }
    }

    pub fn load_text(&mut self, text: &str) {
        let sentences = split_into_sentences(text);
        self.total_sentences = sentences.len();
        self.sentences = sentences;
        self.position = 0;
        self.simplified_cache.clear();
    }

    pub fn current_sentence(&self) -> Option<String> {
        if self.position < self.sentences.len() {
            Some(self.sentences[self.position].clone())
        } else {
            None
        }
    }

    pub fn next(&mut self) {
        if !self.sentences.is_empty() && self.position < self.sentences.len() - 1 {
            self.position += 1;
        }
    }

    pub fn previous(&mut self) {
        if self.position > 0 {
            self.position -= 1;
        }
    }

    pub async fn fetch_simplification(&self, sentence: String) -> Result<SimplificationResponse, AppError> {
        let request = glossia_shared::SimplificationRequest { sentence };
        self.api_client.simplify(request).await
    }
}
