#[cfg(test)]
mod tests {
    use crate::{GlossiaService, ImprovedReadingState, PureReadingState, ReadingState};
    use glossia_api_client::MockLLMClient;

    #[tokio::test]
    async fn test_improved_reading_state_with_mock() {
        // Create a mock LLM client for testing
        let mock_client = MockLLMClient::new()
            .with_response("test sentence", r#"{"original":"test sentence","simplified":"simple test","words":[]}"#);
        
        // Create service with mock client
        let service = GlossiaService::with_llm_client(Box::new(mock_client));
        
        // Create improved reading state
        let mut state = ImprovedReadingState::with_service(service);
        
        // Load text
        state.load_text("This is a test sentence. Here is another one.");
        
        assert_eq!(state.total_sentences(), 2);
        assert_eq!(state.position(), 0);
        
        // Test navigation
        let current = state.current_sentence().unwrap();
        assert!(current.contains("test"));
        
        // Test manual words
        state.add_manual_word("test".to_string());
        assert!(state.is_manual_word("test"));
        
        // Test sentence simplification (with mock)
        let result = state.simplify_sentence(&current).await;
        assert!(result.is_ok());
        
        let simplified = result.unwrap();
        assert_eq!(simplified.original, "test sentence");
        assert_eq!(simplified.simplified, "simple test");
    }

    #[tokio::test]
    async fn test_service_caching() {
        let mock_client = MockLLMClient::new();
        let mut service = GlossiaService::with_llm_client(Box::new(mock_client));
        
        // First call should hit the API
        let result1 = service.get_word_meaning("test", "test context").await;
        assert!(result1.is_ok());
        
        // Second call should hit the cache
        let result2 = service.get_cached_word_meaning("test");
        assert!(result2.is_some());
        assert_eq!(result1.unwrap(), result2.unwrap());
    }

    #[test]
    fn test_pure_reading_state() {
        let mut pure_state = PureReadingState::new();
        
        // Test manual words
        pure_state.add_manual_word("difficult".to_string());
        pure_state.add_manual_word("Complex".to_string()); // Should be lowercased
        
        assert!(pure_state.is_manual_word("difficult"));
        assert!(pure_state.is_manual_word("complex")); // Case insensitive
        assert!(pure_state.is_manual_word("COMPLEX")); // Case insensitive
        assert!(!pure_state.is_manual_word("easy"));
        
        // Test removal
        pure_state.remove_manual_word("difficult");
        assert!(!pure_state.is_manual_word("difficult"));
        assert!(pure_state.is_manual_word("complex"));
        
        // Test clear
        pure_state.clear();
        assert!(!pure_state.is_manual_word("complex"));
        assert_eq!(pure_state.get_manual_words().len(), 0);
    }

    #[test]
    fn test_backward_compatibility() {
        // Set up environment for the test
        std::env::set_var("OPENAI_API_KEY", "test-key");
        
        // The old ReadingState should still work
        let result = ReadingState::new();
        assert!(result.is_ok());
        
        let mut state = result.unwrap();
        state.load_text("Test sentence.");
        assert_eq!(state.total_sentences(), 1);
    }
}
