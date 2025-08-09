use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;

/// Token bucket rate limiter
pub struct RateLimiter {
    bucket: Arc<Mutex<TokenBucket>>,
}

struct TokenBucket {
    tokens: usize,
    max_tokens: usize,
    refill_rate: usize,
    last_refill: Instant,
    refill_interval: Duration,
}

impl RateLimiter {
    /// Create a new rate limiter
    /// `max_requests` - maximum number of requests
    /// `window` - time window for the requests
    pub fn new(max_requests: usize, window: Duration) -> Self {
        let bucket = TokenBucket {
            tokens: max_requests,
            max_tokens: max_requests,
            refill_rate: max_requests,
            last_refill: Instant::now(),
            refill_interval: window,
        };

        Self {
            bucket: Arc::new(Mutex::new(bucket)),
        }
    }

    /// Wait for a permit to make a request
    pub async fn wait_for_permit(&self) {
        loop {
            {
                let mut bucket = self.bucket.lock().await;
                bucket.refill_tokens();
                
                if bucket.tokens > 0 {
                    bucket.tokens -= 1;
                    return;
                }
            }

            // Wait a bit before trying again
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    }

    /// Try to acquire a permit without waiting
    pub async fn try_acquire(&self) -> bool {
        let mut bucket = self.bucket.lock().await;
        bucket.refill_tokens();
        
        if bucket.tokens > 0 {
            bucket.tokens -= 1;
            true
        } else {
            false
        }
    }

    /// Get current number of available tokens
    pub async fn available_tokens(&self) -> usize {
        let mut bucket = self.bucket.lock().await;
        bucket.refill_tokens();
        bucket.tokens
    }
}

impl TokenBucket {
    fn refill_tokens(&mut self) {
        let now = Instant::now();
        let time_passed = now.duration_since(self.last_refill);

        if time_passed >= self.refill_interval {
            // Calculate how many full intervals have passed
            let intervals = time_passed.as_millis() / self.refill_interval.as_millis();
            
            if intervals > 0 {
                let tokens_to_add = (intervals as usize) * self.refill_rate;
                self.tokens = (self.tokens + tokens_to_add).min(self.max_tokens);
                self.last_refill = now;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_rate_limiter_allows_requests_within_limit() {
        let rate_limiter = RateLimiter::new(5, Duration::from_secs(1));

        // Should be able to make 5 requests immediately
        for _ in 0..5 {
            assert!(rate_limiter.try_acquire().await);
        }

        // 6th request should be denied
        assert!(!rate_limiter.try_acquire().await);
    }

    #[tokio::test]
    async fn test_rate_limiter_refills_tokens() {
        let rate_limiter = RateLimiter::new(2, Duration::from_millis(100));

        // Use up all tokens
        assert!(rate_limiter.try_acquire().await);
        assert!(rate_limiter.try_acquire().await);
        assert!(!rate_limiter.try_acquire().await);

        // Wait for refill
        tokio::time::sleep(Duration::from_millis(150)).await;

        // Should have tokens again
        assert!(rate_limiter.try_acquire().await);
        assert!(rate_limiter.try_acquire().await);
    }

    #[tokio::test]
    async fn test_wait_for_permit() {
        let rate_limiter = RateLimiter::new(1, Duration::from_millis(50));

        // Use the one available token
        rate_limiter.wait_for_permit().await;

        // This should wait and eventually succeed
        let start = Instant::now();
        rate_limiter.wait_for_permit().await;
        let elapsed = start.elapsed();

        // Should have waited at least 50ms for token refill
        assert!(elapsed >= Duration::from_millis(40)); // Allow some tolerance
    }
}
