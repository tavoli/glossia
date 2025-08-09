use std::sync::Arc;
use tokio::sync::RwLock;
use std::time::{Duration, Instant};
use glossia_shared::AppError;
use tracing::{instrument, info, warn, debug};

/// Circuit breaker states
#[derive(Debug, Clone, PartialEq)]
pub enum CircuitState {
    Closed,
    Open,
    HalfOpen,
}

/// Configuration for circuit breaker
#[derive(Clone, Debug)]
pub struct CircuitBreakerConfig {
    pub failure_threshold: u32,
    pub recovery_timeout: Duration,
    pub success_threshold: u32,
}

impl Default for CircuitBreakerConfig {
    fn default() -> Self {
        Self {
            failure_threshold: 5,
            recovery_timeout: Duration::from_secs(60),
            success_threshold: 2,
        }
    }
}

/// Circuit breaker for preventing cascading failures
#[derive(Debug)]
pub struct CircuitBreaker {
    config: CircuitBreakerConfig,
    state: Arc<RwLock<CircuitState>>,
    failure_count: Arc<RwLock<u32>>,
    success_count: Arc<RwLock<u32>>,
    last_failure_time: Arc<RwLock<Option<Instant>>>,
}

impl CircuitBreaker {
    pub fn new(config: CircuitBreakerConfig) -> Self {
        Self {
            config,
            state: Arc::new(RwLock::new(CircuitState::Closed)),
            failure_count: Arc::new(RwLock::new(0)),
            success_count: Arc::new(RwLock::new(0)),
            last_failure_time: Arc::new(RwLock::new(None)),
        }
    }

    #[instrument(skip(self, operation), fields(failure_threshold = %self.config.failure_threshold))]
    pub async fn call<F, Fut, T>(&self, operation: F) -> Result<T, AppError>
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = Result<T, AppError>>,
    {
        // Check if circuit is open
        if self.is_circuit_open().await {
            debug!("Circuit breaker is open, rejecting request");
            return Err(AppError::api_error("Circuit breaker is open - too many authentication failures"));
        }

        // Execute operation
        match operation().await {
            Ok(result) => {
                self.on_success().await;
                Ok(result)
            }
            Err(error) => {
                if self.should_trigger_circuit_breaker(&error) {
                    self.on_failure().await;
                }
                Err(error)
            }
        }
    }

    async fn is_circuit_open(&self) -> bool {
        let state = self.state.read().await;
        match *state {
            CircuitState::Open => {
                // Check if we should transition to half-open
                if let Some(last_failure) = *self.last_failure_time.read().await {
                    if last_failure.elapsed() >= self.config.recovery_timeout {
                        drop(state);
                        self.transition_to_half_open().await;
                        false
                    } else {
                        true
                    }
                } else {
                    true
                }
            }
            CircuitState::Closed | CircuitState::HalfOpen => false,
        }
    }

    async fn on_success(&self) {
        let state = self.state.read().await.clone();
        
        match state {
            CircuitState::HalfOpen => {
                let mut success_count = self.success_count.write().await;
                *success_count += 1;
                
                if *success_count >= self.config.success_threshold {
                    self.transition_to_closed().await;
                }
            }
            CircuitState::Closed => {
                // Reset failure count on success
                *self.failure_count.write().await = 0;
            }
            CircuitState::Open => {}
        }
    }

    async fn on_failure(&self) {
        let mut failure_count = self.failure_count.write().await;
        *failure_count += 1;
        *self.last_failure_time.write().await = Some(Instant::now());

        if *failure_count >= self.config.failure_threshold {
            drop(failure_count);
            self.transition_to_open().await;
        }
    }

    async fn transition_to_open(&self) {
        *self.state.write().await = CircuitState::Open;
        *self.success_count.write().await = 0;
        let failure_count = *self.failure_count.read().await;
        warn!(
            event = "circuit_breaker_opened",
            component = "circuit_breaker", 
            failure_count = %failure_count,
            failure_threshold = %self.config.failure_threshold,
            "Circuit breaker opened due to repeated failures"
        );
    }

    async fn transition_to_half_open(&self) {
        *self.state.write().await = CircuitState::HalfOpen;
        *self.success_count.write().await = 0;
        info!(
            event = "circuit_breaker_half_open",
            component = "circuit_breaker",
            "Circuit breaker transitioned to half-open state"
        );
    }

    async fn transition_to_closed(&self) {
        *self.state.write().await = CircuitState::Closed;
        *self.failure_count.write().await = 0;
        *self.success_count.write().await = 0;
        info!(
            event = "circuit_breaker_closed",
            component = "circuit_breaker",
            "Circuit breaker closed - service recovered"
        );
    }

    fn should_trigger_circuit_breaker(&self, error: &AppError) -> bool {
        match error {
            AppError::HttpError { status, .. } => {
                // Trigger circuit breaker for authentication and authorization errors
                *status == 401 || *status == 403
            }
            _ => false,
        }
    }

    pub async fn get_state(&self) -> CircuitState {
        self.state.read().await.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::sleep;

    #[tokio::test]
    async fn test_circuit_breaker_opens_after_failures() {
        let config = CircuitBreakerConfig {
            failure_threshold: 2,
            recovery_timeout: Duration::from_millis(100),
            success_threshold: 1,
        };
        let circuit_breaker = CircuitBreaker::new(config);

        // First failure
        let result = circuit_breaker.call(|| async {
            Err(AppError::HttpError { status: 401, message: "Unauthorized".to_string() })
        }).await;
        assert!(result.is_err());
        assert_eq!(circuit_breaker.get_state().await, CircuitState::Closed);

        // Second failure - should open circuit
        let result = circuit_breaker.call(|| async {
            Err(AppError::HttpError { status: 401, message: "Unauthorized".to_string() })
        }).await;
        assert!(result.is_err());
        assert_eq!(circuit_breaker.get_state().await, CircuitState::Open);

        // Third call should be rejected by circuit breaker
        let result = circuit_breaker.call(|| async {
            Ok("should not reach here")
        }).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Circuit breaker is open"));
    }

    #[tokio::test]
    async fn test_circuit_breaker_recovers() {
        let config = CircuitBreakerConfig {
            failure_threshold: 1,
            recovery_timeout: Duration::from_millis(50),
            success_threshold: 1,
        };
        let circuit_breaker = CircuitBreaker::new(config);

        // Trigger circuit breaker
        let _result = circuit_breaker.call(|| async {
            Err(AppError::HttpError { status: 401, message: "Unauthorized".to_string() })
        }).await;
        assert_eq!(circuit_breaker.get_state().await, CircuitState::Open);

        // Wait for recovery timeout
        sleep(Duration::from_millis(60)).await;

        // Should transition to half-open and allow request
        let result = circuit_breaker.call(|| async {
            Ok("success")
        }).await;
        assert!(result.is_ok());
        assert_eq!(circuit_breaker.get_state().await, CircuitState::Closed);
    }
}
