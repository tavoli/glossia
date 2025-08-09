use crate::{LLMClient, LLMConfig, ProviderType, OpenAIProvider, ClaudeProvider, MockLLMClient};
use glossia_shared::AppError;

/// Factory for creating LLM clients based on configuration
pub struct LLMClientFactory;

impl LLMClientFactory {
    /// Create a new factory instance
    pub fn new() -> Self {
        Self
    }
}

impl Default for LLMClientFactory {
    fn default() -> Self {
        Self::new()
    }
}

impl LLMClientFactory {

    /// Create an LLM client with default configuration
    pub fn create_client(&self) -> Result<Box<dyn LLMClient>, AppError> {
        Self::from_env()
    }

    /// Create an LLM client based on the provided configuration
    pub fn create(config: LLMConfig) -> Result<Box<dyn LLMClient>, AppError> {
        match config.provider {
            ProviderType::OpenAI => {
                let provider = OpenAIProvider::new(config)?;
                Ok(Box::new(provider))
            }
            ProviderType::Claude => {
                let provider = ClaudeProvider::new(config)?;
                Ok(Box::new(provider))
            }
            ProviderType::Mock => {
                let mock_client = MockLLMClient::new();
                Ok(Box::new(mock_client))
            }
        }
    }

    /// Create an LLM client from environment variables
    pub fn from_env() -> Result<Box<dyn LLMClient>, AppError> {
        let config = LLMConfig::from_env()?;
        Self::create(config)
    }

    /// Create a mock client for testing
    pub fn create_mock() -> Box<dyn LLMClient> {
        Box::new(MockLLMClient::new())
    }

    /// Create a mock client with custom configuration
    pub fn create_mock_with_config(should_fail: bool, delay_ms: Option<u64>) -> Box<dyn LLMClient> {
        let mut mock = MockLLMClient::new();
        if should_fail {
            mock = mock.with_failure();
        }
        if let Some(delay) = delay_ms {
            mock = mock.with_delay(delay);
        }
        Box::new(mock)
    }

    /// List available providers
    pub fn available_providers() -> Vec<ProviderType> {
        vec![
            ProviderType::OpenAI,
            ProviderType::Claude,
            ProviderType::Mock,
        ]
    }

    /// Check if a provider is available (has necessary configuration)
    pub async fn check_provider_availability(provider: ProviderType) -> Result<bool, AppError> {
        let config = match provider {
            ProviderType::OpenAI => {
                if std::env::var("OPENAI_API_KEY").is_ok() {
                    LLMConfig::new(ProviderType::OpenAI)
                        .with_api_key(std::env::var("OPENAI_API_KEY").unwrap())
                } else {
                    return Ok(false);
                }
            }
            ProviderType::Claude => {
                if std::env::var("CLAUDE_API_KEY").is_ok() {
                    LLMConfig::new(ProviderType::Claude)
                        .with_api_key(std::env::var("CLAUDE_API_KEY").unwrap())
                } else {
                    return Ok(false);
                }
            }
            ProviderType::Mock => {
                return Ok(true); // Mock is always available
            }
        };

        let client = Self::create(config)?;
        match client.health_check().await {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_available_providers() {
        let providers = LLMClientFactory::available_providers();
        assert!(providers.contains(&ProviderType::OpenAI));
        assert!(providers.contains(&ProviderType::Claude));
        assert!(providers.contains(&ProviderType::Mock));
    }

    #[test]
    fn test_create_mock() {
        let client = LLMClientFactory::create_mock();
        assert_eq!(client.provider_name(), "Mock");
    }

    #[tokio::test]
    async fn test_mock_provider_availability() {
        let available = LLMClientFactory::check_provider_availability(ProviderType::Mock).await;
        assert!(available.is_ok());
        assert!(available.unwrap());
    }

    #[test]
    fn test_create_mock_with_config() {
        let client = LLMClientFactory::create_mock_with_config(true, Some(100));
        assert_eq!(client.provider_name(), "Mock");
    }
}
