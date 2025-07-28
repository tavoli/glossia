use glossia_api_client::{OpenAIClient, BraveImageClient};
use glossia_shared::{AppError, SimplificationResponse, ImageSearchRequest, ImageResult, ImageQueryOptimizationRequest};
use std::collections::{HashMap, hash_map::DefaultHasher};
use std::hash::{Hash, Hasher};
use glossia_text_parser::split_into_sentences;

#[derive(Clone, Default)]
pub struct ReadingState {
    pub sentences: Vec<String>,
    pub simplified_cache: HashMap<String, SimplificationResponse>,
    pub image_cache: HashMap<String, Vec<ImageResult>>,
    pub optimized_query_cache: HashMap<String, String>, // context_key -> optimized query
    pub position: usize,
    pub total_sentences: usize,
    pub api_client: OpenAIClient,
    pub image_client: BraveImageClient,
}

impl ReadingState {
    pub fn new() -> Self {
        Self {
            api_client: OpenAIClient::new(),
            image_client: BraveImageClient::new(),
            image_cache: HashMap::new(),
            optimized_query_cache: HashMap::new(),
            ..Default::default()
        }
    }

    pub fn load_text(&mut self, text: &str) {
        let sentences = split_into_sentences(text);
        self.total_sentences = sentences.len();
        self.sentences = sentences;
        self.position = 0;
        self.simplified_cache.clear();
        self.image_cache.clear();
        self.optimized_query_cache.clear();
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

    pub async fn fetch_images(&self, word: String) -> Result<Vec<ImageResult>, AppError> {
        let request = ImageSearchRequest { 
            query: word,
            count: 5 
        };
        self.image_client.search_images(request).await
    }

    // Helper method to generate a context-aware cache key
    fn generate_context_key(&self, word: &str, sentence_context: &str) -> String {
        let mut hasher = DefaultHasher::new();
        word.hash(&mut hasher);
        sentence_context.hash(&mut hasher);
        format!("{}_{:x}", word, hasher.finish())
    }

    pub async fn optimize_and_fetch_images(&mut self, word: String, sentence_context: String, word_meaning: String) -> Result<Vec<ImageResult>, AppError> {
        let context_key = self.generate_context_key(&word, &sentence_context);
        
        // Check if we already have images cached for this word (context-agnostic cache)
        if let Some(cached_images) = self.image_cache.get(&word) {
            println!("Using cached images for word: '{}'", word);
            return Ok(cached_images.clone());
        }

        // Get optimized query (from context-aware cache or by calling OpenAI)
        let optimized_query = if let Some(cached_query) = self.optimized_query_cache.get(&context_key) {
            println!("Using cached optimized query for '{}' in context: '{}'", word, cached_query);
            cached_query.clone()
        } else {
            println!("Optimizing query for word: '{}' in context: '{}'", word, sentence_context);
            match self.api_client.optimize_image_query(ImageQueryOptimizationRequest { 
                word: word.clone(),
                sentence_context: sentence_context.clone(),
                word_meaning: word_meaning.clone(),
            }).await {
                Ok(optimization_response) => {
                    let optimized_query = optimization_response.optimized_query;
                    println!("Context-aware query optimized: '{}' -> '{}'", word, optimized_query);
                    // Cache the optimized query with context key
                    self.optimized_query_cache.insert(context_key, optimized_query.clone());
                    optimized_query
                }
                Err(e) => {
                    println!("Query optimization failed for '{}': {}. Falling back to original word", word, e);
                    // Fallback to original word if optimization fails
                    word.clone()
                }
            }
        };

        // Fetch images using the optimized query
        println!("Fetching images with optimized query: '{}'", optimized_query);
        let request = ImageSearchRequest { 
            query: optimized_query,
            count: 5 
        };
        
        match self.image_client.search_images(request).await {
            Ok(images) => {
                // Cache the results (still using word as key for broader reusability)
                self.image_cache.insert(word.clone(), images.clone());
                println!("Successfully fetched and cached {} images for '{}'", images.len(), word);
                Ok(images)
            }
            Err(e) => {
                println!("Failed to fetch images for '{}': {}", word, e);
                Err(e)
            }
        }
    }
}
