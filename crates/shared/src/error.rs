use thiserror::Error;

#[derive(Debug, Error, Clone, PartialEq)]
pub enum AppError {
    #[error("API request failed: {message}")]
    ApiError { message: String },

    #[error("Authentication failed: {message}")]
    AuthenticationError { 
        message: String,
        status_code: Option<u16>,
        error_type: Option<String>,
        error_code: Option<String>,
    },

    #[error("Rate limit exceeded: {message}")]
    RateLimitError { 
        message: String,
        retry_after: Option<u64>,
    },

    #[error("Invalid API request: {message}")]
    BadRequestError { 
        message: String,
        error_type: Option<String>,
        error_code: Option<String>,
    },

    #[error("HTTP {status} - {message}")]
    HttpError { 
        status: u16, 
        message: String,
        headers: Option<std::collections::HashMap<String, String>>,
        body: Option<String>,
    },

    #[error("Failed to parse JSON response: {message}")]
    ParseError { message: String },

    #[error("Network request failed: {message}")]
    NetworkError { message: String },

    #[error("API response content is missing or invalid")]
    InvalidResponseContent,

    #[error("Book is empty or could not be loaded")]
    EmptyBook,

    #[error("Configuration error: {message}")]
    ConfigError { message: String },
}

// Implement From traits for automatic conversion
impl From<serde_json::Error> for AppError {
    fn from(e: serde_json::Error) -> Self {
        Self::ParseError { message: e.to_string() }
    }
}

impl From<reqwest::Error> for AppError {
    fn from(e: reqwest::Error) -> Self {
        Self::NetworkError { message: e.to_string() }
    }
}

// Custom conversion for cases where we want to provide context
impl AppError {
    pub fn api_error(message: impl Into<String>) -> Self {
        Self::ApiError { message: message.into() }
    }

    pub fn authentication_error(
        message: impl Into<String>, 
        status_code: Option<u16>,
        error_type: Option<String>,
        error_code: Option<String>,
    ) -> Self {
        Self::AuthenticationError { 
            message: message.into(),
            status_code,
            error_type,
            error_code,
        }
    }

    pub fn rate_limit_error(message: impl Into<String>, retry_after: Option<u64>) -> Self {
        Self::RateLimitError { 
            message: message.into(),
            retry_after,
        }
    }

    pub fn bad_request_error(
        message: impl Into<String>,
        error_type: Option<String>,
        error_code: Option<String>,
    ) -> Self {
        Self::BadRequestError { 
            message: message.into(),
            error_type,
            error_code,
        }
    }

    pub fn http_error(status: u16, message: impl Into<String>) -> Self {
        Self::HttpError { 
            status, 
            message: message.into(),
            headers: None,
            body: None,
        }
    }

    pub fn http_error_with_details(
        status: u16, 
        message: impl Into<String>,
        headers: Option<std::collections::HashMap<String, String>>,
        body: Option<String>,
    ) -> Self {
        Self::HttpError { 
            status, 
            message: message.into(),
            headers,
            body,
        }
    }

    pub fn config_error(message: impl Into<String>) -> Self {
        Self::ConfigError { message: message.into() }
    }

    /// Check if this error is related to authentication
    pub fn is_authentication_error(&self) -> bool {
        matches!(self, Self::AuthenticationError { .. }) ||
        matches!(self, Self::HttpError { status, .. } if *status == 401 || *status == 403)
    }

    /// Check if this error is retryable
    pub fn is_retryable(&self) -> bool {
        match self {
            Self::RateLimitError { .. } => true,
            Self::HttpError { status, .. } => matches!(*status, 429 | 500..=599),
            Self::NetworkError { .. } => true,
            _ => false,
        }
    }

    /// Get user-friendly error message with actionable advice
    pub fn user_friendly_message(&self) -> String {
        match self {
            Self::AuthenticationError { message, error_type, error_code, .. } => {
                let base_msg = "Authentication failed with the AI service.";
                let advice = match (error_type.as_deref(), error_code.as_deref()) {
                    (Some("invalid_api_key"), _) | (_, Some("invalid_api_key")) => {
                        "Please check that your API key is correct and has not expired. You can verify your key at https://platform.openai.com/api-keys"
                    }
                    (Some("insufficient_quota"), _) | (_, Some("insufficient_quota")) => {
                        "Your API quota has been exceeded. Please check your billing and usage at https://platform.openai.com/usage"
                    }
                    (Some("invalid_request_error"), _) if message.contains("model") => {
                        "The specified model is not available or invalid. Please check your model configuration."
                    }
                    _ => "Please check your connection and API key configuration."
                };
                format!("{} {}", base_msg, advice)
            }

            Self::BadRequestError { message, .. } => {
                if message.contains("model") || message.contains("Model") {
                    format!("Invalid model configuration: {}. Please check that the model name is correct and available to your API key.", message)
                } else if message.contains("token") {
                    format!("Request too large: {}. Try reducing the input text size.", message)
                } else {
                    format!("Request error: {}. Please check your request parameters.", message)
                }
            }

            Self::RateLimitError { retry_after, .. } => {
                if let Some(seconds) = retry_after {
                    format!("Rate limit exceeded. Please wait {} seconds before trying again.", seconds)
                } else {
                    "Rate limit exceeded. Please wait a moment before trying again.".to_string()
                }
            }

            Self::NetworkError { message } => {
                format!("Network connection failed: {}. Please check your internet connection.", message)
            }

            Self::HttpError { status, message, .. } => {
                match *status {
                    401 => "Authentication failed. Please check your API key.".to_string(),
                    403 => "Access denied. Please check your API key permissions.".to_string(),
                    404 => "API endpoint not found. Please check your configuration.".to_string(),
                    429 => "Too many requests. Please wait before trying again.".to_string(),
                    500..=599 => "Server error. Please try again later.".to_string(),
                    _ => format!("Request failed (HTTP {}): {}", status, message),
                }
            }

            Self::ConfigError { message } => {
                format!("Configuration error: {}. Please check your environment variables and settings.", message)
            }

            _ => self.to_string(),
        }
    }

    /// Get error category for telemetry and logging
    pub fn category(&self) -> &'static str {
        match self {
            Self::AuthenticationError { .. } => "authentication",
            Self::BadRequestError { .. } => "bad_request",
            Self::RateLimitError { .. } => "rate_limit",
            Self::NetworkError { .. } => "network",
            Self::HttpError { .. } => "http",
            Self::ParseError { .. } => "parse",
            Self::ConfigError { .. } => "config",
            Self::ApiError { .. } => "api",
            Self::InvalidResponseContent => "invalid_response",
            Self::EmptyBook => "empty_book",
        }
    }

    /// Check if error suggests immediate retry (vs backoff)
    pub fn should_retry_immediately(&self) -> bool {
        matches!(self, Self::NetworkError { .. })
    }

    /// Get suggested retry delay in seconds
    pub fn suggested_retry_delay(&self) -> Option<u64> {
        match self {
            Self::RateLimitError { retry_after, .. } => *retry_after,
            Self::HttpError { status, .. } if *status == 429 => Some(60),
            Self::HttpError { status, .. } if matches!(*status, 500..=599) => Some(5),
            Self::NetworkError { .. } => Some(1),
            _ => None,
        }
    }
}
