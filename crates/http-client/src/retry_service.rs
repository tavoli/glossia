use glossia_shared::AppError;
use std::future::Future;
use std::time::Duration;
use rand::Rng;

/// Configuration for retry behavior
#[derive(Clone)]
pub struct RetryConfig {
    pub max_retries: usize,
    pub base_delay: Duration,
    pub max_delay: Duration,
    pub backoff_multiplier: f64,
    pub jitter: bool,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_retries: 3,
            base_delay: Duration::from_millis(100),
            max_delay: Duration::from_secs(5),
            backoff_multiplier: 2.0,
            jitter: true,
        }
    }
}

/// Service that handles retry logic with exponential backoff
pub struct RetryService {
    config: RetryConfig,
}

impl RetryService {
    pub fn new(config: RetryConfig) -> Self {
        Self { config }
    }

    /// Execute a function with retry logic
    pub async fn execute<F, Fut, T>(&self, mut operation: F) -> Result<T, AppError>
    where
        F: FnMut() -> Fut,
        Fut: Future<Output = Result<T, AppError>>,
    {
        let mut last_error = None;

        for attempt in 0..=self.config.max_retries {
            match operation().await {
                Ok(result) => return Ok(result),
                Err(error) => {
                    last_error = Some(error.clone());

                    // Don't retry certain types of errors
                    if !self.should_retry(&error) {
                        return Err(error);
                    }

                    // Don't delay after the last attempt
                    if attempt < self.config.max_retries {
                        let delay = self.calculate_delay(attempt);
                        tokio::time::sleep(delay).await;
                    }
                }
            }
        }

        // Return the last error if all retries failed
        Err(last_error.unwrap_or_else(|| AppError::api_error("Retry service failed without error")))
    }

    /// Determine if an error should be retried
    fn should_retry(&self, error: &AppError) -> bool {
        match error {
            AppError::NetworkError { .. } => true,
            AppError::HttpError { status, .. } => {
                // Retry on server errors (5xx) and rate limiting (429)
                *status >= 500 || *status == 429
            },
            AppError::RateLimitError { .. } => true, // Always retry rate limit errors with backoff
            AppError::AuthenticationError { .. } => false, // Don't retry auth errors
            AppError::BadRequestError { .. } => false, // Don't retry bad requests
            AppError::ApiError { .. } => false, // Usually don't retry API errors
            AppError::ParseError { .. } => false, // Don't retry parse errors
            AppError::ConfigError { .. } => false, // Don't retry config errors
            AppError::InvalidResponseContent => false,
            AppError::EmptyBook => false,
        }
    }

    /// Calculate delay with exponential backoff and optional jitter
    fn calculate_delay(&self, attempt: usize) -> Duration {
        let base_delay_ms = self.config.base_delay.as_millis() as f64;
        let exponential_delay = base_delay_ms * self.config.backoff_multiplier.powi(attempt as i32);
        
        let mut delay_ms = exponential_delay.min(self.config.max_delay.as_millis() as f64);

        // Add jitter to prevent thundering herd
        if self.config.jitter {
            let mut rng = rand::thread_rng();
            let jitter_factor: f64 = rng.gen_range(0.8..1.2);
            delay_ms *= jitter_factor;
        }

        Duration::from_millis(delay_ms as u64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_retry_eventually_succeeds() {
        let config = RetryConfig {
            max_retries: 3,
            base_delay: Duration::from_millis(1),
            ..Default::default()
        };
        let retry_service = RetryService::new(config);

        let attempt_count = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let attempt_count_clone = attempt_count.clone();
        
        let result: Result<String, AppError> = retry_service.execute(move || {
            let count = attempt_count_clone.clone();
            async move {
                let current = count.fetch_add(1, std::sync::atomic::Ordering::SeqCst) + 1;
                if current < 3 {
                    Err(AppError::NetworkError { message: "Test error".to_string() })
                } else {
                    Ok("Success".to_string())
                }
            }
        }).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Success");
        assert_eq!(attempt_count.load(std::sync::atomic::Ordering::SeqCst), 3);
    }

    #[tokio::test]
    async fn test_retry_fails_after_max_attempts() {
        let config = RetryConfig {
            max_retries: 2,
            base_delay: Duration::from_millis(1),
            ..Default::default()
        };
        let retry_service = RetryService::new(config);

        let attempt_count = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let attempt_count_clone = attempt_count.clone();
        
        let result: Result<String, AppError> = retry_service.execute(move || {
            let count = attempt_count_clone.clone();
            async move {
                count.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                Err(AppError::NetworkError { message: "Test error".to_string() })
            }
        }).await;

        assert!(result.is_err());
        assert_eq!(attempt_count.load(std::sync::atomic::Ordering::SeqCst), 3); // Initial attempt + 2 retries
    }

    #[tokio::test]
    async fn test_no_retry_for_non_retryable_errors() {
        let config = RetryConfig::default();
        let retry_service = RetryService::new(config);

        let attempt_count = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let attempt_count_clone = attempt_count.clone();
        
        let result: Result<String, AppError> = retry_service.execute(move || {
            let count = attempt_count_clone.clone();
            async move {
                count.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                Err(AppError::ParseError { message: "Parse error".to_string() })
            }
        }).await;

        assert!(result.is_err());
        assert_eq!(attempt_count.load(std::sync::atomic::Ordering::SeqCst), 1); // Should not retry parse errors
    }
}
