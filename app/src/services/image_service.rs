use glossia_shared::{ImageResult, ImageQueryOptimizationRequest};
use glossia_reading_engine::ReadingEngine;
use glossia_image_client::ImageClientFactory;
use glossia_llm_client::LLMClientFactory;
use dioxus::prelude::{Readable, Writable};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use tracing::{instrument, info, debug, warn};

#[derive(Clone, Debug)]
pub enum ImageFetchState {
    Loading,
    Loaded(Vec<ImageResult>),
    Error(String),
}

/// Service for handling image fetching with context-aware optimization
pub struct ImageService;

impl ImageService {
    #[instrument(skip(reading_state), fields(word = %word, word_meaning_len = word_meaning.len(), context_len = sentence_context.len()))]
    pub async fn fetch_images_for_word(
        word: &str,
        word_meaning: &str,
        sentence_context: &str,
        reading_state: &mut dioxus::prelude::Signal<ReadingEngine>,
    ) -> Result<Vec<ImageResult>, glossia_shared::AppError> {
        info!("Fetching images for word: '{}'", word);
        debug!("Word meaning: {}", word_meaning);
        debug!("Context: {}", sentence_context);
        
        // Create clients
        debug!("Creating LLM and image clients");
        let llm_factory = LLMClientFactory::new();
        let llm_client = llm_factory.create_client()?;
        let image_factory = ImageClientFactory::new();
        let image_client = image_factory.create_client()?;
        
        // Extract required data with a short-lived borrow
        let (cached_images, optimized_query_cached) = {
            let state = reading_state.read();
            let context_key = Self::generate_context_key(word, sentence_context);
            
            (
                state.get_images(word),
                state.get_optimized_query(&context_key)
            )
        };
        
        // Check cache first
        if let Some(cached_images) = cached_images {
            info!(word = word, image_count = cached_images.len(), "Using cached images");
            return Ok(cached_images);
        }
        
        debug!("No cached images found for word: '{}'", word);
        
        // Get or generate optimized query
        let optimized_query = if let Some(cached_query) = optimized_query_cached {
            cached_query
        } else {
            debug!("No cached optimized query found, generating new one");
            // Try to optimize the query
            match llm_client.optimize_image_query(ImageQueryOptimizationRequest { 
                word: word.to_string(),
                sentence_context: sentence_context.to_string(),
                word_meaning: word_meaning.to_string(),
            }).await {
                Ok(optimization_response) => {
                    let optimized_query = optimization_response.optimized_query;
                    info!("Generated optimized query: '{}' for word: '{}'", optimized_query, word);
                    
                    // Cache the optimized query
                    let context_key = Self::generate_context_key(word, sentence_context);
                    reading_state.write().cache_optimized_query(context_key, optimized_query.clone());
                    
                    optimized_query
                }
                Err(e) => {
                    warn!("Failed to optimize query for word '{}', using fallback: {}", word, e);
                    word.to_string() // Fallback to original word
                }
            }
        };
        
        // Fetch images using the optimized query
        info!("Fetching images with query: '{}'", optimized_query);
        let start_time = std::time::Instant::now();
        let images = image_client.search_images(&optimized_query, Some(5)).await?;
        let fetch_duration = start_time.elapsed();
        
        info!("Successfully fetched {} images for word '{}' in {:?}", images.len(), word, fetch_duration);
        
        // Cache the results
        reading_state.write().cache_images(word.to_string(), images.clone());
        debug!("Cached {} images for word: '{}'", images.len(), word);
        
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
