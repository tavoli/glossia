use std::time::Duration;
use tokio::time::sleep;
use glossia_shared::AppError;

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
                Ok(result) => return Ok(result),
                Err(e) => {
                    attempts += 1;
                    if attempts >= max_attempts {
                        return Err(AppError::ApiError(e.to_string()));
                    }
                    // Exponential backoff: base_delay * 2^(attempts-1)
                    let delay = self.base_delay * (2_u32.pow(attempts - 1));
                    sleep(delay).await;
                }
            }
        }
    }
}
