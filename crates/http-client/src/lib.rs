mod base_client;
mod retry_service;
mod rate_limiter;
mod circuit_breaker;
mod request_tracker;

pub use base_client::BaseHttpClient;
pub use retry_service::{RetryService, RetryConfig};
pub use rate_limiter::RateLimiter;
pub use request_tracker::{RequestTracker, RequestTrackingResult, RequestStats, hash_request_body};
pub use circuit_breaker::{CircuitBreaker, CircuitBreakerConfig};

use glossia_shared::AppError;
use async_trait::async_trait;
use serde::de::DeserializeOwned;
use std::collections::HashMap;

/// Trait for HTTP client implementations
#[async_trait]
pub trait HttpClient: Send + Sync {
    async fn get(&self, url: &str) -> Result<reqwest::Response, AppError>;
    async fn post(&self, url: &str, body: serde_json::Value) -> Result<reqwest::Response, AppError>;
    async fn put(&self, url: &str, body: serde_json::Value) -> Result<reqwest::Response, AppError>;
    async fn delete(&self, url: &str) -> Result<reqwest::Response, AppError>;
    
    /// Convenience method for JSON responses
    async fn get_json<T: DeserializeOwned>(&self, url: &str) -> Result<T, AppError>;
    async fn post_json<T: DeserializeOwned>(&self, url: &str, body: serde_json::Value) -> Result<T, AppError>;
}

/// Enhanced HTTP client with retry logic, rate limiting, circuit breaker, and better error handling
pub struct EnhancedHttpClient {
    base_client: BaseHttpClient,
    retry_service: RetryService,
    rate_limiter: RateLimiter,
    request_tracker: RequestTracker,
    circuit_breaker: CircuitBreaker,
}

impl EnhancedHttpClient {
    pub fn new() -> Result<Self, AppError> {
        Ok(Self {
            base_client: BaseHttpClient::new()?,
            retry_service: RetryService::new(RetryConfig::default()),
            rate_limiter: RateLimiter::new(10, std::time::Duration::from_secs(1)), // 10 requests per second
            request_tracker: RequestTracker::new(),
            circuit_breaker: CircuitBreaker::new(CircuitBreakerConfig::default()),
        })
    }

    pub fn with_config(retry_config: RetryConfig, rate_limit: (usize, std::time::Duration)) -> Result<Self, AppError> {
        Ok(Self {
            base_client: BaseHttpClient::new()?,
            retry_service: RetryService::new(retry_config),
            rate_limiter: RateLimiter::new(rate_limit.0, rate_limit.1),
            request_tracker: RequestTracker::new(),
            circuit_breaker: CircuitBreaker::new(CircuitBreakerConfig::default()),
        })
    }

    pub fn with_headers(mut self, headers: HashMap<String, String>) -> Self {
        self.base_client = self.base_client.with_headers(headers);
        self
    }

    pub fn with_timeout(mut self, timeout: std::time::Duration) -> Self {
        self.base_client = self.base_client.with_timeout(timeout);
        self
    }

    /// Get request statistics for analyzing API usage patterns
    pub fn get_request_stats(&self) -> RequestStats {
        self.request_tracker.get_stats()
    }

    /// Clear request tracking history
    pub fn clear_request_history(&self) {
        self.request_tracker.clear();
    }
}

#[async_trait]
impl HttpClient for EnhancedHttpClient {
    async fn get(&self, url: &str) -> Result<reqwest::Response, AppError> {
        // Track the request for duplicate detection
        let _tracking_result = self.request_tracker.track_request("GET", url, None);
        
        self.rate_limiter.wait_for_permit().await;
        
        self.retry_service.execute(|| async {
            self.base_client.get(url).await
        }).await
    }

    async fn post(&self, url: &str, body: serde_json::Value) -> Result<reqwest::Response, AppError> {
        // Track the request for duplicate detection
        let body_hash = Some(hash_request_body(&body));
        let _tracking_result = self.request_tracker.track_request("POST", url, body_hash);
        
        self.rate_limiter.wait_for_permit().await;
        
        // Use circuit breaker to prevent cascading failures
        self.circuit_breaker.call(|| async {
            self.retry_service.execute(|| async {
                self.base_client.post(url, body.clone()).await
            }).await
        }).await
    }

    async fn put(&self, url: &str, body: serde_json::Value) -> Result<reqwest::Response, AppError> {
        self.rate_limiter.wait_for_permit().await;
        
        self.retry_service.execute(|| async {
            self.base_client.put(url, body.clone()).await
        }).await
    }

    async fn delete(&self, url: &str) -> Result<reqwest::Response, AppError> {
        self.rate_limiter.wait_for_permit().await;
        
        self.retry_service.execute(|| async {
            self.base_client.delete(url).await
        }).await
    }

    async fn get_json<T: DeserializeOwned>(&self, url: &str) -> Result<T, AppError> {
        let response = self.get(url).await?;
        self.base_client.parse_json_response(response).await
    }

    async fn post_json<T: DeserializeOwned>(&self, url: &str, body: serde_json::Value) -> Result<T, AppError> {
        let response = self.post(url, body).await?;
        self.base_client.parse_json_response(response).await
    }
}

impl Default for EnhancedHttpClient {
    fn default() -> Self {
        Self::new().expect("Failed to create EnhancedHttpClient")
    }
}
