use dioxus::prelude::*;
use glossia_shared::AppError;
use std::time::{Duration, Instant};
use tracing::{instrument, info, warn, debug, error};

/// Configuration for error boundary behavior
#[derive(Clone, Debug)]
pub struct ErrorBoundaryConfig {
    pub max_consecutive_errors: u32,
    pub error_cooldown: Duration,
    pub circuit_breaker_threshold: u32,
}

impl Default for ErrorBoundaryConfig {
    fn default() -> Self {
        Self {
            max_consecutive_errors: 3,
            error_cooldown: Duration::from_secs(30),
            circuit_breaker_threshold: 5,
        }
    }
}

/// State for tracking errors and preventing cascading failures
#[derive(Clone, Debug, PartialEq)]
pub enum ErrorState {
    Normal,
    CooldownMode { until: Instant, error_count: u32 },
    CircuitBreaker { error: AppError },
}

/// Hook for managing error boundaries and preventing infinite retry loops
pub fn use_error_boundary(_config: ErrorBoundaryConfig) -> Signal<ErrorState> {
    let error_state = use_signal(|| ErrorState::Normal);
    let _error_count = use_signal(|| 0u32);
    let _last_error_time = use_signal(|| Option::<Instant>::None);

    error_state
}

/// Separate function to handle error processing
#[instrument(skip(error_state, error_count, last_error_time, config), fields(error_type = ?error))]
pub fn handle_error_boundary(
    error: &AppError,
    mut error_state: Signal<ErrorState>,
    mut error_count: Signal<u32>,
    mut last_error_time: Signal<Option<Instant>>,
    config: &ErrorBoundaryConfig,
) -> bool {
        let now = Instant::now();
        let current_state = error_state.read().clone();
        
        match current_state {
            ErrorState::Normal => {
                if is_authentication_error(error) {
                    let mut count = error_count.write();
                    *count += 1;
                    *last_error_time.write() = Some(now);
                    
                    if *count >= config.circuit_breaker_threshold {
                        *error_state.write() = ErrorState::CircuitBreaker { error: error.clone() };
                        error!(
                            event = "error_boundary_activated",
                            component = "error_boundary",
                            consecutive_errors = *count,
                            threshold = config.circuit_breaker_threshold,
                            "Error boundary activated - too many authentication failures"
                        );
                        return false;
                    } else if *count >= config.max_consecutive_errors {
                        let cooldown_until = now + config.error_cooldown;
                        *error_state.write() = ErrorState::CooldownMode { 
                            until: cooldown_until, 
                            error_count: *count 
                        };
                        warn!(
                            event = "cooldown_mode_activated",
                            component = "error_boundary",
                            consecutive_errors = *count,
                            cooldown_duration_secs = config.error_cooldown.as_secs(),
                            "Entering cooldown mode due to consecutive errors"
                        );
                        return false;
                    }
                }
                true
            }
            ErrorState::CooldownMode { until, error_count: count } => {
                if now >= until {
                    // Cooldown period over, reset state
                    *error_state.write() = ErrorState::Normal;
                    *error_count.write() = 0;
                    info!(
                        event = "cooldown_mode_exited",
                        component = "error_boundary",
                        previous_error_count = count,
                        "Exiting cooldown mode - requests allowed again"
                    );
                    true
                } else {
                    debug!(
                        event = "request_blocked_cooldown",
                        component = "error_boundary",
                        remaining_cooldown_ms = until.duration_since(now).as_millis(),
                        "Request blocked - still in cooldown mode"
                    );
                    false
                }
            }
            ErrorState::CircuitBreaker { .. } => {
                debug!(
                    event = "request_blocked_circuit_breaker",
                    component = "error_boundary",
                    "Request blocked - circuit breaker active"
                );
                false
            }
        }
}

/// Check if an error is an authentication/authorization error
fn is_authentication_error(error: &AppError) -> bool {
    match error {
        AppError::HttpError { status, .. } => *status == 401 || *status == 403,
        AppError::ApiError { message } => {
            message.to_lowercase().contains("unauthorized") ||
            message.to_lowercase().contains("authentication") ||
            message.to_lowercase().contains("circuit breaker")
        }
        _ => false,
    }
}

/// Hook specifically for managing API request states with error boundary
pub fn use_protected_api_call<T, F, Fut>(
    api_call: F,
    dependency: impl PartialEq + Clone + 'static,
) -> Resource<Option<Result<T, AppError>>>
where
    T: 'static,
    F: Fn() -> Fut + 'static,
    Fut: std::future::Future<Output = Result<T, AppError>> + 'static,
{
    let mut error_state = use_error_boundary(ErrorBoundaryConfig::default());
    let mut error_count = use_signal(|| 0u32);
    let mut last_error_time = use_signal(|| Option::<Instant>::None);
    let api_call = std::sync::Arc::new(api_call);
    
    use_resource(move || {
        let _dependency = dependency.clone();
        let api_call = api_call.clone();
        async move {
            // Check error boundary before making request
            let current_state = error_state.read().clone();
            match current_state {
                ErrorState::CooldownMode { until, error_count } => {
                    if Instant::now() < until {
                        return Some(Err(AppError::api_error(
                            format!("API calls temporarily disabled due to {} consecutive errors. Try again in {:?}", 
                                error_count, until.duration_since(Instant::now()))
                        )));
                    }
                }
                ErrorState::CircuitBreaker { error } => {
                    return Some(Err(AppError::api_error(
                        format!("API calls disabled due to repeated failures: {}", error)
                    )));
                }
                ErrorState::Normal => {}
            }

            let result = api_call().await;
            
            match &result {
                Ok(_) => {
                    // Reset error state on success
                    if !matches!(*error_state.read(), ErrorState::Normal) {
                        *error_state.write() = ErrorState::Normal;
                    }
                }
                Err(error) => {
                    // Update error boundary state
                    let config = ErrorBoundaryConfig::default();
                    handle_error_boundary(error, error_state, error_count, last_error_time, &config);
                }
            }
            
            Some(result)
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_authentication_error_detection() {
        let auth_error = AppError::HttpError { status: 401, message: "Unauthorized".to_string() };
        assert!(is_authentication_error(&auth_error));

        let forbidden_error = AppError::HttpError { status: 403, message: "Forbidden".to_string() };
        assert!(is_authentication_error(&forbidden_error));

        let server_error = AppError::HttpError { status: 500, message: "Internal Server Error".to_string() };
        assert!(!is_authentication_error(&server_error));

        let circuit_breaker_error = AppError::ApiError { message: "Circuit breaker is open".to_string() };
        assert!(is_authentication_error(&circuit_breaker_error));
    }
}
