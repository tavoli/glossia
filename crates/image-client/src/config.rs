use glossia_shared::AppError;
use std::time::Duration;

/// Supported image search providers
#[derive(Debug, Clone, PartialEq)]
pub enum ImageProvider {
    Brave,
    Mock,
}

impl std::str::FromStr for ImageProvider {
    type Err = AppError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "brave" => Ok(ImageProvider::Brave),
            "mock" => Ok(ImageProvider::Mock),
            _ => Err(AppError::config_error(format!("Unknown image provider: {s}"))),
        }
    }
}

/// Configuration for image search clients
#[derive(Debug, Clone)]
pub struct ImageClientConfig {
    pub provider: ImageProvider,
    pub api_key: Option<String>,
    pub timeout: Duration,
    pub max_retries: usize,
    pub default_count: usize,
    pub max_count: usize,
}

impl Default for ImageClientConfig {
    fn default() -> Self {
        Self {
            provider: ImageProvider::Brave,
            api_key: None,
            timeout: Duration::from_secs(10),
            max_retries: 3,
            default_count: 5,
            max_count: 20,
        }
    }
}

impl ImageClientConfig {
    pub fn new(provider: ImageProvider) -> Self {
        Self {
            provider,
            ..Default::default()
        }
    }

    pub fn from_env() -> Result<Self, AppError> {
        dotenvy::dotenv().ok(); // Load .env file if it exists
        
        let provider_str = std::env::var("IMAGE_PROVIDER").unwrap_or_else(|_| "brave".to_string());
        let provider = provider_str.parse()?;

        let api_key = match provider {
            ImageProvider::Brave => std::env::var("BRAVE_API_KEY").ok(),
            ImageProvider::Mock => None,
        };

        let timeout = std::env::var("IMAGE_TIMEOUT")
            .ok()
            .and_then(|s| s.parse().ok())
            .map(Duration::from_secs)
            .unwrap_or_else(|| Duration::from_secs(10));

        let max_retries = std::env::var("IMAGE_MAX_RETRIES")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(3);

        let default_count = std::env::var("IMAGE_DEFAULT_COUNT")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(5);

        let max_count = std::env::var("IMAGE_MAX_COUNT")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(20);

        Ok(Self {
            provider,
            api_key,
            timeout,
            max_retries,
            default_count,
            max_count,
        })
    }

    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    pub fn with_max_retries(mut self, max_retries: usize) -> Self {
        self.max_retries = max_retries;
        self
    }

    pub fn with_default_count(mut self, default_count: usize) -> Self {
        self.default_count = default_count;
        self
    }

    pub fn with_max_count(mut self, max_count: usize) -> Self {
        self.max_count = max_count;
        self
    }

    pub fn with_api_key(mut self, api_key: String) -> Self {
        self.api_key = Some(api_key);
        self
    }

    /// Validate the configuration
    pub fn validate(&self) -> Result<(), AppError> {
        match self.provider {
            ImageProvider::Brave => {
                if self.api_key.is_none() {
                    return Err(AppError::config_error("API key is required for Brave provider"));
                }
            }
            ImageProvider::Mock => {
                // Mock provider doesn't need validation
            }
        }

        if self.default_count == 0 {
            return Err(AppError::config_error("Default count must be greater than 0"));
        }

        if self.max_count == 0 {
            return Err(AppError::config_error("Max count must be greater than 0"));
        }

        if self.default_count > self.max_count {
            return Err(AppError::config_error("Default count cannot be greater than max count"));
        }

        Ok(())
    }

    /// Clamp a requested count to valid limits
    pub fn clamp_count(&self, requested: Option<usize>) -> usize {
        match requested {
            Some(count) => count.min(self.max_count).max(1),
            None => self.default_count,
        }
    }
}
