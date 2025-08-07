#[cfg(test)]
mod tests {
    use crate::traits::{LLMClient, MockLLMClient};
    use glossia_shared::{SimplificationRequest, ImageQueryOptimizationRequest};

    #[tokio::test]
    async fn test_mock_llm_client_simplify() {
        let client = MockLLMClient::new();
        let request = SimplificationRequest {
            sentence: "This is a complex sentence.".to_string(),
        };
        
        let result = client.simplify(request).await;
        assert!(result.is_ok());
        
        let response = result.unwrap();
        assert_eq!(response.original, "This is a complex sentence.");
        assert!(response.simplified.contains("Simplified:"));
    }

    #[tokio::test]
    async fn test_mock_llm_client_word_meaning() {
        let client = MockLLMClient::new()
            .with_response("complex", "difficult to understand");
        
        let result = client.get_word_meaning("complex", "This is complex").await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "difficult to understand");
    }

    #[tokio::test]
    async fn test_mock_llm_client_failure() {
        let client = MockLLMClient::with_failure();
        let request = SimplificationRequest {
            sentence: "Test sentence".to_string(),
        };
        
        let result = client.simplify(request).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_mock_llm_client_image_optimization() {
        let client = MockLLMClient::new();
        let request = ImageQueryOptimizationRequest {
            word: "castle".to_string(),
            sentence_context: "The old castle stood on the hill".to_string(),
            word_meaning: "A large fortified building".to_string(),
        };
        
        let result = client.optimize_image_query(request).await;
        assert!(result.is_ok());
        
        let response = result.unwrap();
        assert!(response.optimized_query.contains("castle"));
    }

    // Test trait object usage for polymorphism
    async fn test_trait_object(client: &dyn LLMClient) -> Result<String, glossia_shared::AppError> {
        client.get_word_meaning("test", "test context").await
    }

    #[tokio::test]
    async fn test_polymorphism() {
        let mock_client = MockLLMClient::new();
        let result = test_trait_object(&mock_client).await;
        assert!(result.is_ok());
    }
}
