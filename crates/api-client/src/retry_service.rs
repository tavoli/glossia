use std::time::Duration;
use tokio::time::sleep;
use glossia_shared::AppError;
use rand::Rng;

/// Service for handling retry logic across different API clients
#[derive(Clone)]
pub struct RetryService {
    max_attempts: u32,
    base_delay: Duration,
}

impl RetryService {
    pub fn new(max_attempts: u32, base_delay: Duration) -> Self {
        Self {
            max_attempts,
            base_delay,
        }
    }

    /// Retry a request with exponential backoff
    pub async fn retry<F, Fut, T>(&self, operation: F) -> Result<T, AppError>
    where
        F: FnMut() -> Fut,
        Fut: std::future::Future<Output = Result<T, reqwest::Error>>,
    {
        self.retry_with_max_attempts(self.max_attempts, operation).await
    }

    /// Retry a request with custom max attempts
    pub async fn retry_with_max_attempts<F, Fut, T>(&self, max_attempts: u32, mut operation: F) -> Result<T, AppError>
    where
        F: FnMut() -> Fut,
        Fut: std::future::Future<Output = Result<T, reqwest::Error>>,
    {
        let mut attempts = 0;
        loop {
            match operation().await {
                Ok(response) => return Ok(response),
                Err(e) => {
                    attempts += 1;
                    
                    // Check if we should retry based on error type
                    if !self.should_retry(&e) || attempts >= max_attempts {
                        return Err(AppError::from(e));
                    }
                    
                    // Exponential backoff with overflow protection and jitter
                    let exponential_factor = 2_u32.saturating_pow(attempts - 1);
                    let base_delay = self.base_delay.saturating_mul(exponential_factor);
                    
                    // Add jitter (±20% randomization) to prevent thundering herd
                    let jitter = rand::thread_rng().gen_range(0.8..=1.2);
                    let delay_with_jitter = base_delay.mul_f64(jitter);
                    
                    // Cap maximum delay at 60 seconds
                    let max_delay = Duration::from_secs(60);
                    let final_delay = delay_with_jitter.min(max_delay);
                    
                    sleep(final_delay).await;
                }
            }
        }
    }

    /// Determine if an error should trigger a retry
    fn should_retry(&self, error: &reqwest::Error) -> bool {
        // Retry on network/timeout errors
        if error.is_timeout() || error.is_connect() || error.is_request() {
            return true;
        }
        
        // Retry on specific HTTP status codes (5xx server errors, 429 rate limiting)
        if let Some(status) = error.status() {
            matches!(status.as_u16(), 429 | 500..=599)
        } else {
            // Retry on network-level errors without status codes
            true
        }
    }
}
