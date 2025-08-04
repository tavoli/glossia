use glossia_shared::{ImageResult, ImageSearchRequest, ImageQueryOptimizationRequest};
use glossia_book_reader::ReadingState;
use dioxus::prelude::{Readable, Writable};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Clone, Debug)]
pub enum ImageFetchState {
    Loading,
    Loaded(Vec<ImageResult>),
    Error(String),
}

/// Service for handling image fetching with context-aware optimization
pub struct ImageService;

impl ImageService {
    pub async fn fetch_images_for_word(
        word: &str,
        word_meaning: &str,
        sentence_context: &str,
        reading_state: &mut dioxus::prelude::Signal<ReadingState>,
    ) -> Result<Vec<ImageResult>, glossia_shared::AppError> {
        // Extract required data with a short-lived borrow
        let (api_client, image_client, cached_images, optimized_query_cached) = {
            let state = reading_state.read();
            let context_key = Self::generate_context_key(word, sentence_context);
            
            (
                state.api_client.clone(),
                state.image_client.clone(),
                state.cache.get_images(word),
                state.cache.get_optimized_query(&context_key)
            )
        };
        
        // Check cache first
        if let Some(cached_images) = cached_images {
            return Ok(cached_images);
        }
        
        // Get or generate optimized query
        let optimized_query = if let Some(cached_query) = optimized_query_cached {
            cached_query
        } else {
            // Try to optimize the query
            match api_client.optimize_image_query(ImageQueryOptimizationRequest { 
                word: word.to_string(),
                sentence_context: sentence_context.to_string(),
                word_meaning: word_meaning.to_string(),
            }).await {
                Ok(optimization_response) => {
                    let optimized_query = optimization_response.optimized_query;
                    
                    // Cache the optimized query
                    let context_key = Self::generate_context_key(word, sentence_context);
                    reading_state.write().cache.cache_optimized_query(context_key, optimized_query.clone());
                    
                    optimized_query
                }
                Err(_) => word.to_string() // Fallback to original word
            }
        };
        
        // Fetch images using the optimized query
        let request = ImageSearchRequest { 
            query: optimized_query,
            count: 5 
        };
        
        let images = image_client.search_images(request).await?;
        
        // Cache the results
        reading_state.write().cache.cache_images(word.to_string(), images.clone());
        
        Ok(images)
    }
    
    /// Generate a consistent context key for caching
    fn generate_context_key(word: &str, sentence_context: &str) -> String {
        let mut hasher = DefaultHasher::new();
        word.hash(&mut hasher);
        sentence_context.hash(&mut hasher);
        format!("{}_{}_{:x}", word, sentence_context, hasher.finish())
    }
}
