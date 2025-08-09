use async_trait::async_trait;
use glossia_shared::{AppError, ImageResult};

/// Trait for image search clients
#[async_trait]
pub trait ImageClient: Send + Sync {
    /// Search for images based on a query
    async fn search_images(&self, query: &str, count: Option<usize>) -> Result<Vec<ImageResult>, AppError>;
    
    /// Get provider name for debugging/logging
    fn provider_name(&self) -> &str;
    
    /// Check if the client is properly configured
    async fn health_check(&self) -> Result<(), AppError>;
}

/// Mock implementation for testing
#[derive(Debug, Clone)]
pub struct MockImageClient {
    pub should_fail: bool,
    pub delay_ms: Option<u64>,
    pub custom_results: std::collections::HashMap<String, Vec<ImageResult>>,
}

impl Default for MockImageClient {
    fn default() -> Self {
        Self::new()
    }
}

impl MockImageClient {
    pub fn new() -> Self {
        Self {
            should_fail: false,
            delay_ms: None,
            custom_results: std::collections::HashMap::new(),
        }
    }

    pub fn with_failure(mut self) -> Self {
        self.should_fail = true;
        self
    }

    pub fn with_delay(mut self, delay_ms: u64) -> Self {
        self.delay_ms = Some(delay_ms);
        self
    }

    pub fn with_custom_results(mut self, query: String, results: Vec<ImageResult>) -> Self {
        self.custom_results.insert(query, results);
        self
    }

    fn generate_mock_results(&self, query: &str, count: usize) -> Vec<ImageResult> {
        (0..count)
            .map(|i| ImageResult {
                url: format!("https://example.com/{}_image_{}.jpg", query.replace(" ", "_"), i),
                title: format!("Mock image {} for {}", i + 1, query),
                thumbnail_url: format!("https://example.com/{}_thumb_{}.jpg", query.replace(" ", "_"), i),
                width: Some(800),
                height: Some(600),
            })
            .collect()
    }
}

#[async_trait]
impl ImageClient for MockImageClient {
    async fn search_images(&self, query: &str, count: Option<usize>) -> Result<Vec<ImageResult>, AppError> {
        if let Some(delay) = self.delay_ms {
            tokio::time::sleep(tokio::time::Duration::from_millis(delay)).await;
        }

        if self.should_fail {
            return Err(AppError::api_error("Mock image client configured to fail"));
        }

        let count = count.unwrap_or(5);

        // Return custom results if available
        if let Some(custom_results) = self.custom_results.get(query) {
            Ok(custom_results.clone())
        } else {
            Ok(self.generate_mock_results(query, count))
        }
    }

    fn provider_name(&self) -> &str {
        "Mock"
    }

    async fn health_check(&self) -> Result<(), AppError> {
        if self.should_fail {
            Err(AppError::api_error("Mock image client health check failed"))
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
        let client = MockImageClient::new();
        let results = client.search_images("test", Some(3)).await;
        
        assert!(results.is_ok());
        let images = results.unwrap();
        assert_eq!(images.len(), 3);
        assert!(images[0].url.contains("test"));
    }

    #[tokio::test]
    async fn test_mock_client_failure() {
        let client = MockImageClient::new().with_failure();
        let results = client.search_images("test", Some(3)).await;
        assert!(results.is_err());
    }

    #[tokio::test]
    async fn test_mock_client_custom_results() {
        let custom_results = vec![
            ImageResult {
                url: "https://custom.com/image1.jpg".to_string(),
                title: "Custom Image 1".to_string(),
                thumbnail_url: "https://custom.com/thumb1.jpg".to_string(),
                width: Some(800),
                height: Some(600),
            }
        ];

        let client = MockImageClient::new()
            .with_custom_results("custom".to_string(), custom_results.clone());
        
        let results = client.search_images("custom", None).await.unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].title, "Custom Image 1");
    }

    #[tokio::test]
    async fn test_mock_client_delay() {
        let client = MockImageClient::new().with_delay(50);
        
        let start = std::time::Instant::now();
        let _results = client.search_images("test", Some(1)).await.unwrap();
        let elapsed = start.elapsed();
        
        assert!(elapsed >= std::time::Duration::from_millis(40)); // Allow some tolerance
    }
}
