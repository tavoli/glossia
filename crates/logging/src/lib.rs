//! Centralized logging configuration and utilities for Glossia
//! 
//! This crate provides a unified logging interface with:
//! - Structured logging with correlation IDs
//! - Environment-specific configuration  
//! - Request/operation tracing
//! - Performance and resource monitoring

pub mod config;
pub mod context;
pub mod macros;

pub use config::{LoggingConfig, LogFormat, init_logging};
pub use context::{CorrelationId, LogContext, with_context};

use tracing::{info, warn};

/// Initialize the logging system for the application
pub fn setup_application_logging() -> Result<(), Box<dyn std::error::Error>> {
    let config = LoggingConfig::from_env();
    init_logging(&config)?;
    
    info!(
        component = "logging",
        version = env!("CARGO_PKG_VERSION"),
        "Logging system initialized"
    );
    
    Ok(())
}

/// Log application startup with system information
pub fn log_startup(app_name: &str, version: &str) {
    info!(
        event = "app_startup",
        app_name = app_name,
        version = version,
        component = "lifecycle",
        "Application starting"
    );
}

/// Log application shutdown
pub fn log_shutdown(app_name: &str) {
    info!(
        event = "app_shutdown", 
        app_name = app_name,
        component = "lifecycle",
        "Application shutting down"
    );
}

/// Log critical errors that may require immediate attention
pub fn log_critical_error(error: &str, component: &str, context: Option<&str>) {
    tracing::error!(
        event = "critical_error",
        component = component,
        error = error,
        context = context,
        "Critical error occurred"
    );
}

/// Log performance metrics for operations
pub fn log_performance_metric(
    operation: &str,
    component: &str, 
    duration_ms: u64,
    success: bool,
    context: Option<&str>
) {
    if success {
        info!(
            event = "performance_metric",
            operation = operation,
            component = component,
            duration_ms = duration_ms,
            success = success,
            context = context,
            "Operation completed"
        );
    } else {
        warn!(
            event = "performance_metric",
            operation = operation,
            component = component,
            duration_ms = duration_ms,
            success = success,
            context = context,
            "Operation failed"
        );
    }
}