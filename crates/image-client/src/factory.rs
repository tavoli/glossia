use crate::{ImageClient, ImageClientConfig, ImageProvider, BraveProvider, MockImageClient};
use glossia_shared::AppError;

/// Factory for creating image search clients based on configuration
pub struct ImageClientFactory;

impl ImageClientFactory {
    /// Create a new factory instance
    pub fn new() -> Self {
        Self
    }
}

impl Default for ImageClientFactory {
    fn default() -> Self {
        Self::new()
    }
}

impl ImageClientFactory {
    /// Create an image client with default configuration
    pub fn create_client(&self) -> Result<Box<dyn ImageClient>, AppError> {
        Self::from_env()
    }

    /// Create an image client based on the provided configuration
    pub fn create(config: ImageClientConfig) -> Result<Box<dyn ImageClient>, AppError> {
        match config.provider {
            ImageProvider::Brave => {
                let provider = BraveProvider::new(config)?;
                Ok(Box::new(provider))
            }
            ImageProvider::Mock => {
                let mock_client = MockImageClient::new();
                Ok(Box::new(mock_client))
            }
        }
    }

    /// Create an image client from environment variables
    pub fn from_env() -> Result<Box<dyn ImageClient>, AppError> {
        let config = ImageClientConfig::from_env()?;
        Self::create(config)
    }

    /// Create a mock client for testing
    pub fn create_mock() -> Box<dyn ImageClient> {
        Box::new(MockImageClient::new())
    }

    /// Create a mock client with custom configuration
    pub fn create_mock_with_config(should_fail: bool, delay_ms: Option<u64>) -> Box<dyn ImageClient> {
        let mut mock = MockImageClient::new();
        if should_fail {
            mock = mock.with_failure();
        }
        if let Some(delay) = delay_ms {
            mock = mock.with_delay(delay);
        }
        Box::new(mock)
    }

    /// List available providers
    pub fn available_providers() -> Vec<ImageProvider> {
        vec![
            ImageProvider::Brave,
            ImageProvider::Mock,
        ]
    }

    /// Check if a provider is available
    pub async fn check_provider_availability(provider: ImageProvider) -> Result<bool, AppError> {
        let config = ImageClientConfig::new(provider);
        let client = Self::create(config)?;
        
        match client.health_check().await {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    /// Create a client with custom retry and timeout settings
    pub fn create_with_custom_settings(
        provider: ImageProvider,
        timeout_secs: u64,
        max_retries: usize,
        default_count: usize,
    ) -> Result<Box<dyn ImageClient>, AppError> {
        let config = ImageClientConfig::new(provider)
            .with_timeout(std::time::Duration::from_secs(timeout_secs))
            .with_max_retries(max_retries)
            .with_default_count(default_count);
        
        Self::create(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_available_providers() {
        let providers = ImageClientFactory::available_providers();
        assert!(providers.contains(&ImageProvider::Brave));
        assert!(providers.contains(&ImageProvider::Mock));
    }

    #[test]
    fn test_create_mock() {
        let client = ImageClientFactory::create_mock();
        assert_eq!(client.provider_name(), "Mock");
    }

    #[tokio::test]
    async fn test_mock_provider_availability() {
        let available = ImageClientFactory::check_provider_availability(ImageProvider::Mock).await;
        assert!(available.is_ok());
        assert!(available.unwrap());
    }

    #[test]
    fn test_create_mock_with_config() {
        let client = ImageClientFactory::create_mock_with_config(true, Some(100));
        assert_eq!(client.provider_name(), "Mock");
    }

    #[test]
    fn test_create_with_custom_settings() {
        let client = ImageClientFactory::create_with_custom_settings(
            ImageProvider::Mock,
            15,  // timeout_secs
            5,   // max_retries  
            10,  // default_count
        );
        assert!(client.is_ok());
    }
}
