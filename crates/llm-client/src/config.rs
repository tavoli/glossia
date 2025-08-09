use glossia_shared::AppError;
use std::time::Duration;

/// Supported LLM providers
#[derive(Debug, Clone, PartialEq)]
pub enum ProviderType {
    OpenAI,
    Claude,
    Mock,
}

impl std::str::FromStr for ProviderType {
    type Err = AppError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "openai" => Ok(ProviderType::OpenAI),
            "claude" => Ok(ProviderType::Claude),
            "mock" => Ok(ProviderType::Mock),
            _ => Err(AppError::config_error(format!("Unknown LLM provider: {s}"))),
        }
    }
}

/// Configuration for LLM clients
#[derive(Debug, Clone)]
pub struct LLMConfig {
    pub provider: ProviderType,
    pub api_key: Option<String>,
    pub base_url: Option<String>,
    pub model: Option<String>,
    pub timeout: Duration,
    pub max_retries: usize,
    pub temperature: Option<f32>,
    pub max_tokens: Option<u32>,
}

impl Default for LLMConfig {
    fn default() -> Self {
        Self {
            provider: ProviderType::OpenAI,
            api_key: None,
            base_url: None,
            model: None,
            timeout: Duration::from_secs(30),
            max_retries: 3,
            temperature: None,
            max_tokens: None,
        }
    }
}

impl LLMConfig {
    pub fn new(provider: ProviderType) -> Self {
        Self {
            provider,
            ..Default::default()
        }
    }

    pub fn from_env() -> Result<Self, AppError> {
        // Load .env file and provide helpful error if it fails
        match dotenvy::dotenv() {
            Ok(path) => {
                tracing::debug!("Loaded environment variables from: {}", path.display());
            }
            Err(e) => {
                tracing::warn!("Failed to load .env file: {}. Using system environment variables.", e);
            }
        }

        let provider_str = std::env::var("LLM_PROVIDER").unwrap_or_else(|_| "openai".to_string());
        let provider = provider_str.parse()?;

        let api_key = match provider {
            ProviderType::OpenAI => {
                tracing::debug!("Loading OpenAI API key from environment");
                let key = std::env::var("OPENAI_API_KEY")
                    .map_err(|_| AppError::config_error("OPENAI_API_KEY environment variable must be set. Please check your .env file or environment variables."))?;
                tracing::debug!("OpenAI API key loaded successfully (length: {})", key.len());
                Some(key)
            }
            ProviderType::Claude => {
                tracing::debug!("Loading Claude API key from environment");
                let key = std::env::var("CLAUDE_API_KEY")
                    .map_err(|_| AppError::config_error("CLAUDE_API_KEY environment variable must be set. Please check your .env file or environment variables."))?;
                tracing::debug!("Claude API key loaded successfully (length: {})", key.len());
                Some(key)
            }
            ProviderType::Mock => None,
        };

        let base_url = match provider {
            ProviderType::OpenAI => std::env::var("OPENAI_BASE_URL").ok(),
            ProviderType::Claude => std::env::var("CLAUDE_BASE_URL").ok(),
            ProviderType::Mock => None,
        };

        let model = match provider {
            ProviderType::OpenAI => std::env::var("OPENAI_MODEL").ok(),
            ProviderType::Claude => std::env::var("CLAUDE_MODEL").ok(),
            ProviderType::Mock => None,
        };

        let timeout = std::env::var("LLM_TIMEOUT")
            .ok()
            .and_then(|s| s.parse().ok())
            .map(Duration::from_secs)
            .unwrap_or_else(|| Duration::from_secs(30));

        let max_retries = std::env::var("LLM_MAX_RETRIES")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(3);

        let temperature = std::env::var("LLM_TEMPERATURE")
            .ok()
            .and_then(|s| s.parse().ok());

        let max_tokens = std::env::var("LLM_MAX_TOKENS")
            .ok()
            .and_then(|s| s.parse().ok());

        Ok(Self {
            provider,
            api_key,
            base_url,
            model,
            timeout,
            max_retries,
            temperature,
            max_tokens,
        })
    }

    pub fn with_api_key(mut self, api_key: String) -> Self {
        self.api_key = Some(api_key);
        self
    }

    pub fn with_base_url(mut self, base_url: String) -> Self {
        self.base_url = Some(base_url);
        self
    }

    pub fn with_model(mut self, model: String) -> Self {
        self.model = Some(model);
        self
    }

    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    pub fn with_max_retries(mut self, max_retries: usize) -> Self {
        self.max_retries = max_retries;
        self
    }

    pub fn with_temperature(mut self, temperature: f32) -> Self {
        self.temperature = Some(temperature);
        self
    }

    pub fn with_max_tokens(mut self, max_tokens: u32) -> Self {
        self.max_tokens = Some(max_tokens);
        self
    }

    /// Validate the configuration
    pub fn validate(&self) -> Result<(), AppError> {
        match self.provider {
            ProviderType::OpenAI => {
                if self.api_key.is_none() {
                    return Err(AppError::config_error(
                        "OpenAI API key is required. Please set OPENAI_API_KEY environment variable in your .env file or environment."
                    ));
                }
                if let Some(ref key) = self.api_key {
                    if key.trim().is_empty() {
                        return Err(AppError::config_error(
                            "OpenAI API key cannot be empty. Please check your OPENAI_API_KEY environment variable."
                        ));
                    }
                    if !key.starts_with("sk-") {
                        return Err(AppError::config_error(
                            "Invalid OpenAI API key format. OpenAI keys should start with 'sk-'. Please check your OPENAI_API_KEY environment variable."
                        ));
                    }
                }
            }
            ProviderType::Claude => {
                if self.api_key.is_none() {
                    return Err(AppError::config_error(
                        "Claude API key is required. Please set CLAUDE_API_KEY environment variable in your .env file or environment."
                    ));
                }
                if let Some(ref key) = self.api_key {
                    if key.trim().is_empty() {
                        return Err(AppError::config_error(
                            "Claude API key cannot be empty. Please check your CLAUDE_API_KEY environment variable."
                        ));
                    }
                }
            }
            ProviderType::Mock => {
                // Mock provider doesn't need validation
            }
        }

        if let Some(temperature) = self.temperature {
            if !(0.0..=2.0).contains(&temperature) {
                return Err(AppError::config_error(
                    "Temperature must be between 0.0 and 2.0"
                ));
            }
        }

        if let Some(max_tokens) = self.max_tokens {
            if max_tokens == 0 {
                return Err(AppError::config_error(
                    "Max tokens must be greater than 0"
                ));
            }
        }

        Ok(())
    }
}
