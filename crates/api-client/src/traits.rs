use async_trait::async_trait;
use glossia_shared::{AppError, SimplificationRequest, SimplificationResponse, ImageQueryOptimizationRequest, ImageQueryOptimizationResponse};

/// Trait for Language Model clients that can simplify text and define words
#[async_trait]
pub trait LLMClient: Send + Sync {
    /// Simplify a sentence and identify difficult words
    async fn simplify(&self, request: SimplificationRequest) -> Result<SimplificationResponse, AppError>;
    
    /// Get the meaning of a word in context
    async fn get_word_meaning(&self, word: &str, context: &str) -> Result<String, AppError>;
    
    /// Optimize image search queries based on word context
    async fn optimize_image_query(&self, request: ImageQueryOptimizationRequest) -> Result<ImageQueryOptimizationResponse, AppError>;
}

/// Mock implementation for testing
#[derive(Debug, Clone)]
pub struct MockLLMClient {
    pub should_fail: bool,
    pub custom_responses: std::collections::HashMap<String, String>,
}

impl Default for MockLLMClient {
    fn default() -> Self {
        Self::new()
    }
}

impl MockLLMClient {
    pub fn new() -> Self {
        Self {
            should_fail: false,
            custom_responses: std::collections::HashMap::new(),
        }
    }
    
    pub fn with_failure() -> Self {
        Self {
            should_fail: true,
            custom_responses: std::collections::HashMap::new(),
        }
    }
    
    pub fn with_response(mut self, key: impl Into<String>, response: impl Into<String>) -> Self {
        self.custom_responses.insert(key.into(), response.into());
        self
    }
}

#[async_trait]
impl LLMClient for MockLLMClient {
    async fn simplify(&self, request: SimplificationRequest) -> Result<SimplificationResponse, AppError> {
        if self.should_fail {
            return Err(AppError::api_error("Mock failure"));
        }
        
        if let Some(custom_response) = self.custom_responses.get(&request.sentence) {
            return Ok(serde_json::from_str(custom_response)?);
        }
        
        // Default mock response
        Ok(SimplificationResponse {
            original: request.sentence.clone(),
            simplified: format!("Simplified: {}", request.sentence),
            words: vec![],
        })
    }
    
    async fn get_word_meaning(&self, word: &str, _context: &str) -> Result<String, AppError> {
        if self.should_fail {
            return Err(AppError::api_error("Mock failure"));
        }
        
        if let Some(custom_response) = self.custom_responses.get(word) {
            return Ok(custom_response.clone());
        }
        
        Ok(format!("Mock meaning for '{}'", word))
    }
    
    async fn optimize_image_query(&self, request: ImageQueryOptimizationRequest) -> Result<ImageQueryOptimizationResponse, AppError> {
        if self.should_fail {
            return Err(AppError::api_error("Mock failure"));
        }
        
        if let Some(custom_response) = self.custom_responses.get(&request.word) {
            return Ok(serde_json::from_str(custom_response)?);
        }
        
        Ok(ImageQueryOptimizationResponse {
            optimized_query: format!("mock query {}", request.word),
        })
    }
}
