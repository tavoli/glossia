mod cache_manager;
mod navigation_state;

pub use cache_manager::CacheManager;
pub use navigation_state::NavigationState;

use glossia_api_client::{OpenAIClient, BraveImageClient};

#[derive(Clone)]
pub struct ReadingState {
    pub navigation: NavigationState,
    pub cache: CacheManager,
    pub api_client: OpenAIClient,
    pub image_client: BraveImageClient,
}

impl Default for ReadingState {
    fn default() -> Self {
        Self::new()
    }
}

impl ReadingState {
    pub fn new() -> Self {
        Self {
            navigation: NavigationState::new(),
            cache: CacheManager::new(),
            api_client: OpenAIClient::new(),
            image_client: BraveImageClient::new(),
        }
    }

    pub fn load_text(&mut self, text: &str) {
        self.navigation.load_text(text);
        self.cache.clear_text_caches(); // Keep image cache for reuse
    }

    pub fn current_sentence(&self) -> Option<String> {
        self.navigation.current_sentence()
    }

    pub fn next(&mut self) {
        self.navigation.next();
    }

    pub fn previous(&mut self) {
        self.navigation.previous();
    }

    // Convenience getters for backward compatibility
    pub fn sentences(&self) -> &[String] {
        &self.navigation.sentences
    }

    pub fn position(&self) -> usize {
        self.navigation.position
    }

    pub fn total_sentences(&self) -> usize {
        self.navigation.total_sentences
    }








}
