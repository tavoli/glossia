use glossia_text_parser::split_into_sentences;

/// State related to navigation and text content
#[derive(Clone, Default)]
pub struct NavigationState {
    pub sentences: Vec<String>,
    pub position: usize,
    pub total_sentences: usize,
}

impl NavigationState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn load_text(&mut self, text: &str) {
        let sentences = split_into_sentences(text);
        self.total_sentences = sentences.len();
        self.sentences = sentences;
        self.position = 0;
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


}
