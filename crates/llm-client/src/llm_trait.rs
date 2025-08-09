use async_trait::async_trait;
use glossia_shared::{AppError, SimplificationRequest, SimplificationResponse, ImageQueryOptimizationRequest, ImageQueryOptimizationResponse};
use std::collections::HashMap;

/// Trait for Language Model clients that can simplify text and define words
#[async_trait]
pub trait LLMClient: Send + Sync {
    /// Simplify a sentence and identify difficult words
    async fn simplify(&self, request: SimplificationRequest) -> Result<SimplificationResponse, AppError>;
    
    /// Get the meaning of a word in context
    async fn get_word_meaning(&self, word: &str, context: &str) -> Result<String, AppError>;
    
    /// Optimize image search queries based on word context
    async fn optimize_image_query(&self, request: ImageQueryOptimizationRequest) -> Result<ImageQueryOptimizationResponse, AppError>;
    
    /// Get provider name for debugging/logging
    fn provider_name(&self) -> &str;
    
    /// Check if the client is properly configured
    async fn health_check(&self) -> Result<(), AppError>;
}

/// Mock implementation for testing
#[derive(Debug, Clone)]
pub struct MockLLMClient {
    pub should_fail: bool,
    pub custom_responses: HashMap<String, String>,
    pub delay_ms: Option<u64>,
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
            custom_responses: HashMap::new(),
            delay_ms: None,
        }
    }

    pub fn with_failure(mut self) -> Self {
        self.should_fail = true;
        self
    }

    pub fn with_custom_response(mut self, input: String, output: String) -> Self {
        self.custom_responses.insert(input, output);
        self
    }

    pub fn with_delay(mut self, delay_ms: u64) -> Self {
        self.delay_ms = Some(delay_ms);
        self
    }
}

#[async_trait]
impl LLMClient for MockLLMClient {
    async fn simplify(&self, request: SimplificationRequest) -> Result<SimplificationResponse, AppError> {
        if let Some(delay) = self.delay_ms {
            tokio::time::sleep(tokio::time::Duration::from_millis(delay)).await;
        }

        if self.should_fail {
            return Err(AppError::api_error("Mock client configured to fail"));
        }

        let simplified = if let Some(custom_response) = self.custom_responses.get(&request.sentence) {
            custom_response.clone()
        } else {
            format!("Simplified: {}", request.sentence)
        };

        Ok(SimplificationResponse {
            original: request.sentence.clone(),
            simplified,
            words: vec![],
        })
    }

    async fn get_word_meaning(&self, word: &str, _context: &str) -> Result<String, AppError> {
        if let Some(delay) = self.delay_ms {
            tokio::time::sleep(tokio::time::Duration::from_millis(delay)).await;
        }

        if self.should_fail {
            return Err(AppError::api_error("Mock client configured to fail"));
        }

        Ok(format!("Mock meaning for '{word}'"))
    }

    async fn optimize_image_query(&self, request: ImageQueryOptimizationRequest) -> Result<ImageQueryOptimizationResponse, AppError> {
        if let Some(delay) = self.delay_ms {
            tokio::time::sleep(tokio::time::Duration::from_millis(delay)).await;
        }

        if self.should_fail {
            return Err(AppError::api_error("Mock client configured to fail"));
        }

        Ok(ImageQueryOptimizationResponse {
            optimized_query: format!("optimized {}", request.word),
        })
    }

    fn provider_name(&self) -> &str {
        "Mock"
    }

    async fn health_check(&self) -> Result<(), AppError> {
        if self.should_fail {
            Err(AppError::api_error("Mock client health check failed"))
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mock_client_success() {
        let client = MockLLMClient::new();
        let request = SimplificationRequest {
            sentence: "Test sentence".to_string(),
        };

        let result = client.simplify(request).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_mock_client_failure() {
        let client = MockLLMClient::new().with_failure();
        let request = SimplificationRequest {
            sentence: "Test sentence".to_string(),
        };

        let result = client.simplify(request).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_mock_client_custom_response() {
        let client = MockLLMClient::new()
            .with_custom_response("hello".to_string(), "custom response".to_string());
        
        let request = SimplificationRequest {
            sentence: "hello".to_string(),
        };

        let result = client.simplify(request).await.unwrap();
        assert_eq!(result.simplified, "custom response");
    }
}
