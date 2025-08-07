use serde::{Deserialize, Serialize};
use std::time::Duration;
use crate::AppError;

/// Configuration settings for the Glossia application
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlossiaConfig {
    /// OpenAI API configuration
    pub openai: OpenAIConfig,
    /// Retry service configuration
    pub retry: RetryConfig,
    /// Application behavior settings
    pub app: AppConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenAIConfig {
    /// OpenAI API key (loaded from environment)
    #[serde(skip)]
    pub api_key: String,
    /// OpenAI API base URL
    pub base_url: String,
    /// Default model to use
    pub model: String,
    /// Default temperature for requests
    pub temperature: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryConfig {
    /// Maximum number of retry attempts
    pub max_attempts: u32,
    /// Base delay between retries in milliseconds
    pub base_delay_ms: u64,
    /// Maximum delay cap in seconds
    pub max_delay_secs: u64,
    /// Whether to use jitter for retry delays
    pub use_jitter: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    /// Minimum encounters needed to promote a word to "known"
    pub word_promotion_threshold: u32,
    /// Whether to automatically track word encounters
    pub auto_track_encounters: bool,
    /// Default theme mode
    pub default_theme: String,
}

impl Default for GlossiaConfig {
    fn default() -> Self {
        Self {
            openai: OpenAIConfig::default(),
            retry: RetryConfig::default(),
            app: AppConfig::default(),
        }
    }
}

impl Default for OpenAIConfig {
    fn default() -> Self {
        Self {
            api_key: String::new(), // Will be loaded from environment
            base_url: "https://api.openai.com/v1".to_string(),
            model: "gpt-4o-mini".to_string(),
            temperature: 0.3,
        }
    }
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            base_delay_ms: 1000,
            max_delay_secs: 60,
            use_jitter: true,
        }
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            word_promotion_threshold: 3,
            auto_track_encounters: true,
            default_theme: "light".to_string(),
        }
    }
}

impl GlossiaConfig {
    /// Load configuration from environment variables and default values
    pub fn from_env() -> Result<Self, AppError> {
        dotenvy::dotenv().ok(); // Load .env file if present
        
        let mut config = Self::default();
        
        // Load OpenAI configuration from environment
        config.openai.api_key = std::env::var("OPENAI_API_KEY")
            .map_err(|_| AppError::config_error("OPENAI_API_KEY environment variable must be set"))?;
        
        if let Ok(base_url) = std::env::var("OPENAI_BASE_URL") {
            config.openai.base_url = base_url;
        }
        
        if let Ok(model) = std::env::var("OPENAI_MODEL") {
            config.openai.model = model;
        }
        
        // Load retry configuration from environment
        if let Ok(max_attempts) = std::env::var("RETRY_MAX_ATTEMPTS") {
            if let Ok(attempts) = max_attempts.parse() {
                config.retry.max_attempts = attempts;
            }
        }
        
        if let Ok(base_delay) = std::env::var("RETRY_BASE_DELAY_MS") {
            if let Ok(delay) = base_delay.parse() {
                config.retry.base_delay_ms = delay;
            }
        }
        
        // Load app configuration from environment
        if let Ok(threshold) = std::env::var("WORD_PROMOTION_THRESHOLD") {
            if let Ok(t) = threshold.parse() {
                config.app.word_promotion_threshold = t;
            }
        }
        
        if let Ok(theme) = std::env::var("DEFAULT_THEME") {
            config.app.default_theme = theme;
        }
        
        Ok(config)
    }
    
    /// Get base delay as Duration
    pub fn base_delay(&self) -> Duration {
        Duration::from_millis(self.retry.base_delay_ms)
    }
    
    /// Get max delay as Duration
    pub fn max_delay(&self) -> Duration {
        Duration::from_secs(self.retry.max_delay_secs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_default_config() {
        let config = GlossiaConfig::default();
        assert_eq!(config.openai.model, "gpt-4o-mini");
        assert_eq!(config.retry.max_attempts, 3);
        assert_eq!(config.app.word_promotion_threshold, 3);
        assert!(config.retry.use_jitter);
    }
    
    #[test]
    fn test_config_durations() {
        let config = GlossiaConfig::default();
        assert_eq!(config.base_delay(), Duration::from_millis(1000));
        assert_eq!(config.max_delay(), Duration::from_secs(60));
    }
}
