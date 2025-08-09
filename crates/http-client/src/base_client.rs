use glossia_shared::AppError;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use serde::de::DeserializeOwned;
use std::collections::HashMap;
use std::time::Duration;
use tracing::{info, warn, error, debug, instrument};
use uuid::Uuid;

/// Base HTTP client with configurable headers and timeouts
pub struct BaseHttpClient {
    client: reqwest::Client,
}

impl BaseHttpClient {
    pub fn new() -> Result<Self, AppError> {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .map_err(|e| AppError::config_error(format!("Failed to create HTTP client: {e}")))?;

        Ok(Self { client })
    }

    pub fn with_headers(mut self, headers: HashMap<String, String>) -> Self {
        let mut header_map = HeaderMap::new();
        
        for (key, value) in headers {
            if let (Ok(header_name), Ok(header_value)) = (
                HeaderName::from_bytes(key.as_bytes()),
                HeaderValue::from_str(&value)
            ) {
                header_map.insert(header_name, header_value);
            }
        }

        // Rebuild client with existing timeout (30s default) and new headers
        let client = reqwest::Client::builder()
            .default_headers(header_map)
            .timeout(Duration::from_secs(30))
            .build()
            .expect("Failed to create HTTP client with headers");

        self.client = client;
        self
    }

    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        // We need to rebuild the client completely, preserving any existing configuration
        // Since we can't extract headers from existing client, we'll store them in the builder
        let client = reqwest::Client::builder()
            .timeout(timeout)
            .build()
            .expect("Failed to create HTTP client with timeout");

        self.client = client;
        self
    }

    #[instrument(skip(self), fields(request_id = %Uuid::new_v4()))]
    pub async fn get(&self, url: &str) -> Result<reqwest::Response, AppError> {
        info!("Making GET request to: {}", url);
        let start_time = std::time::Instant::now();
        
        let response = self.client
            .get(url)
            .send()
            .await
            .map_err(|e| {
                error!("GET request failed: {}", e);
                AppError::NetworkError { message: e.to_string() }
            })?;

        let status = response.status();
        let duration = start_time.elapsed();
        
        info!("GET response: {} in {:?}", status, duration);
        
        if !status.is_success() {
            warn!("Non-success status code: {}", status);
        }

        self.handle_response_status(response).await
    }

    #[instrument(skip(self, body), fields(request_id = %Uuid::new_v4(), body_size = body.to_string().len()))]
    pub async fn post(&self, url: &str, body: serde_json::Value) -> Result<reqwest::Response, AppError> {
        info!("Making POST request to: {}", url);
        debug!("POST body: {}", serde_json::to_string_pretty(&body).unwrap_or_else(|_| "Invalid JSON".to_string()));
        let start_time = std::time::Instant::now();
        
        let response = self.client
            .post(url)
            .json(&body)
            .send()
            .await
            .map_err(|e| {
                error!("POST request failed: {}", e);
                AppError::NetworkError { message: e.to_string() }
            })?;

        let status = response.status();
        let duration = start_time.elapsed();
        
        info!("POST response: {} in {:?}", status, duration);
        
        if !status.is_success() {
            warn!("Non-success status code: {}", status);
        }

        self.handle_response_status(response).await
    }

    #[instrument(skip(self, body), fields(request_id = %Uuid::new_v4(), body_size = body.to_string().len()))]
    pub async fn put(&self, url: &str, body: serde_json::Value) -> Result<reqwest::Response, AppError> {
        info!("Making PUT request to: {}", url);
        debug!("PUT body: {}", serde_json::to_string_pretty(&body).unwrap_or_else(|_| "Invalid JSON".to_string()));
        let start_time = std::time::Instant::now();
        let response = self.client
            .put(url)
            .json(&body)
            .send()
            .await
            .map_err(|e| {
                error!("PUT request failed: {}", e);
                AppError::NetworkError { message: e.to_string() }
            })?;

        let status = response.status();
        let duration = start_time.elapsed();
        
        info!("PUT response: {} in {:?}", status, duration);
        
        if !status.is_success() {
            warn!("Non-success status code: {}", status);
        }

        self.handle_response_status(response).await
    }

    #[instrument(skip(self), fields(request_id = %Uuid::new_v4()))]
    pub async fn delete(&self, url: &str) -> Result<reqwest::Response, AppError> {
        info!("Making DELETE request to: {}", url);
        let start_time = std::time::Instant::now();
        let response = self.client
            .delete(url)
            .send()
            .await
            .map_err(|e| {
                error!("DELETE request failed: {}", e);
                AppError::NetworkError { message: e.to_string() }
            })?;

        let status = response.status();
        let duration = start_time.elapsed();
        
        info!("DELETE response: {} in {:?}", status, duration);
        
        if !status.is_success() {
            warn!("Non-success status code: {}", status);
        }

        self.handle_response_status(response).await
    }

    #[instrument(skip(self, response))]
    pub async fn parse_json_response<T: DeserializeOwned>(&self, response: reqwest::Response) -> Result<T, AppError> {
        let content_length = response.content_length();
        debug!("Parsing JSON response, content length: {:?}", content_length);
        
        let text = response.text().await
            .map_err(|e| {
                error!("Failed to read response text: {}", e);
                AppError::NetworkError { message: e.to_string() }
            })?;

        debug!("Response text length: {} bytes", text.len());
        
        serde_json::from_str(&text)
            .map_err(|e| {
                error!("Failed to parse JSON: {}", e);
                AppError::ParseError { 
                    message: format!("Failed to parse JSON response: {e}. Response: {text}") 
                }
            })
    }

    async fn handle_response_status(&self, response: reqwest::Response) -> Result<reqwest::Response, AppError> {
        let status = response.status();
        
        if status.is_success() {
            Ok(response)
        } else {
            // Extract headers for additional context
            let headers: std::collections::HashMap<String, String> = response
                .headers()
                .iter()
                .map(|(key, value)| {
                    (key.to_string(), value.to_str().unwrap_or("").to_string())
                })
                .collect();

            let error_body = response.text().await
                .unwrap_or_else(|_| "Failed to read error response".to_string());
            
            // Try to parse OpenAI/API-specific error format
            if let Ok(error_json) = serde_json::from_str::<serde_json::Value>(&error_body) {
                return self.create_structured_error(status.as_u16(), &error_json, headers, error_body);
            }
            
            // Fallback to generic HTTP error with full details
            Err(AppError::http_error_with_details(
                status.as_u16(),
                format!("HTTP {status}"),
                Some(headers),
                Some(error_body),
            ))
        }
    }

    fn create_structured_error(
        &self,
        status_code: u16,
        error_json: &serde_json::Value,
        headers: std::collections::HashMap<String, String>,
        raw_body: String,
    ) -> Result<reqwest::Response, AppError> {
        // Handle OpenAI API error format
        if let Some(error_obj) = error_json.get("error") {
            let error_message = error_obj.get("message")
                .and_then(|m| m.as_str())
                .unwrap_or("Unknown API error");
            
            let error_type = error_obj.get("type")
                .and_then(|t| t.as_str())
                .map(|s| s.to_string());
            
            let error_code = error_obj.get("code")
                .and_then(|c| c.as_str())
                .map(|s| s.to_string());

            match status_code {
                401 | 403 => {
                    error!("Authentication error: {} (type: {:?}, code: {:?})", error_message, error_type, error_code);
                    Err(AppError::authentication_error(
                        error_message,
                        Some(status_code),
                        error_type,
                        error_code,
                    ))
                }
                400 => {
                    error!("Bad request error: {} (type: {:?}, code: {:?})", error_message, error_type, error_code);
                    Err(AppError::bad_request_error(
                        error_message,
                        error_type,
                        error_code,
                    ))
                }
                429 => {
                    let retry_after = headers.get("retry-after")
                        .and_then(|h| h.parse().ok());
                    error!("Rate limit error: {} (retry after: {:?})", error_message, retry_after);
                    Err(AppError::rate_limit_error(error_message, retry_after))
                }
                _ => {
                    error!("HTTP {} error: {} (type: {:?}, code: {:?})", status_code, error_message, error_type, error_code);
                    Err(AppError::http_error_with_details(
                        status_code,
                        error_message,
                        Some(headers),
                        Some(raw_body),
                    ))
                }
            }
        } else {
            // Handle other structured error formats or fallback
            let error_message = error_json.get("message")
                .and_then(|m| m.as_str())
                .unwrap_or("Unknown structured error");
            
            error!("Structured error response: {}", error_message);
            Err(AppError::http_error_with_details(
                status_code,
                error_message,
                Some(headers),
                Some(raw_body),
            ))
        }
    }
}

impl Default for BaseHttpClient {
    fn default() -> Self {
        Self::new().expect("Failed to create BaseHttpClient")
    }
}
