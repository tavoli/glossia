use thiserror::Error;

#[derive(Debug, Error, Clone)]
pub enum AppError {
    #[error("API request failed: {0}")]
    ApiError(String),

    #[error("Failed to parse JSON response: {0}")]
    ParseError(String),

    #[error("API response content is missing or invalid")]
    InvalidResponseContent,

    #[error("Book is empty or could not be loaded")]
    EmptyBook,
}

impl From<reqwest::Error> for AppError {
    fn from(e: reqwest::Error) -> Self {
        AppError::ApiError(e.to_string())
    }
}

impl From<serde_json::Error> for AppError {
    fn from(e: serde_json::Error) -> Self {
        AppError::ParseError(e.to_string())
    }
}
