use dioxus::prelude::*;
use glossia_shared::AppError;
use crate::theme::Theme;

/// Displays user-friendly error messages based on AppError type
#[component]
pub fn ErrorDisplay(error: AppError, theme: Theme) -> Element {
    let error_message = user_friendly_error(&error);
    
    rsx! {
        div {
            class: "error-message",
            style: "background: {theme.error_bg}; color: {theme.error}; padding: 15px; border-radius: 8px; margin-bottom: 20px; border: 1px solid {theme.border}; text-align: center;",
            "⚠️ {error_message}"
        }
    }
}

fn user_friendly_error(error: &AppError) -> String {
    match error {
        AppError::ApiError { message } if message.contains("404") => {
            "The AI service is temporarily unavailable. Please try again later.".to_string()
        },
        AppError::ApiError { message } if message.contains("401") || message.contains("403") => {
            "Authentication error with the AI service. Please check your connection.".to_string()
        },
        AppError::ApiError { message } if message.contains("timeout") || message.contains("network") => {
            "Network connection issue. Please check your internet connection.".to_string()
        },
        AppError::HttpError { status: 404, .. } => {
            "The AI service is temporarily unavailable. Please try again later.".to_string()
        },
        AppError::HttpError { status: 401..=403, .. } => {
            "Authentication error with the AI service. Please check your connection.".to_string()
        },
        AppError::HttpError { status: 429, .. } => {
            "Too many requests. Please wait a moment and try again.".to_string()
        },
        AppError::HttpError { status: 500..=599, .. } => {
            "Server error. Please try again later.".to_string()
        },
        AppError::ParseError { .. } => {
            "The AI response couldn't be processed. Please try again.".to_string()
        },
        AppError::NetworkError { .. } => {
            "Network connection issue. Please check your internet connection.".to_string()
        },
        AppError::InvalidResponseContent => {
            "The AI service returned an unexpected response. Please try again.".to_string()
        },
        AppError::EmptyBook => {
            "No text to process. Please add some text first.".to_string()
        },
        AppError::ConfigError { message } => {
            format!("Configuration error: {}", message)
        },
        _ => {
            "Something went wrong. Please try again.".to_string()
        }
    }
}
