use thiserror::Error;

#[derive(Debug, Error, Clone, PartialEq)]
pub enum AppError {
    #[error("API request failed: {message}")]
    ApiError { message: String },

    #[error("HTTP {status} - {message}")]
    HttpError { status: u16, message: String },

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

    pub fn http_error(status: u16, message: impl Into<String>) -> Self {
        Self::HttpError { status, message: message.into() }
    }

    pub fn config_error(message: impl Into<String>) -> Self {
        Self::ConfigError { message: message.into() }
    }
}
