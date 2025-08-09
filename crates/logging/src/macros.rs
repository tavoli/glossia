/// Convenience macros for common logging patterns

/// Log an error with structured context
#[macro_export]
macro_rules! log_error {
    ($error:expr, $component:expr) => {
        tracing::error!(
            event = "error",
            component = $component,
            error = %$error,
            "Error occurred"
        );
    };
    ($error:expr, $component:expr, $($fields:tt)*) => {
        tracing::error!(
            event = "error", 
            component = $component,
            error = %$error,
            $($fields)*
        );
    };
}

/// Log a warning with structured context
#[macro_export]
macro_rules! log_warn {
    ($message:expr, $component:expr) => {
        tracing::warn!(
            event = "warning",
            component = $component,
            message = $message,
            "Warning occurred"
        );
    };
    ($message:expr, $component:expr, $($fields:tt)*) => {
        tracing::warn!(
            event = "warning",
            component = $component, 
            message = $message,
            $($fields)*
        );
    };
}

/// Log an info message with structured context
#[macro_export]
macro_rules! log_info {
    ($message:expr, $component:expr) => {
        tracing::info!(
            event = "info",
            component = $component,
            message = $message
        );
    };
    ($message:expr, $component:expr, $($fields:tt)*) => {
        tracing::info!(
            event = "info",
            component = $component,
            message = $message,
            $($fields)*
        );
    };
}

/// Log a debug message with structured context
#[macro_export] 
macro_rules! log_debug {
    ($message:expr, $component:expr) => {
        tracing::debug!(
            event = "debug",
            component = $component,
            message = $message
        );
    };
    ($message:expr, $component:expr, $($fields:tt)*) => {
        tracing::debug!(
            event = "debug",
            component = $component,
            message = $message,
            $($fields)*
        );
    };
}

/// Log the start of an operation
#[macro_export]
macro_rules! log_operation_start {
    ($operation:expr, $component:expr) => {
        tracing::info!(
            event = "operation_start",
            operation = $operation,
            component = $component,
            "Operation started"
        );
    };
    ($operation:expr, $component:expr, $($fields:tt)*) => {
        tracing::info!(
            event = "operation_start",
            operation = $operation,
            component = $component,
            $($fields)*,
            "Operation started"
        );
    };
}

/// Log the completion of an operation
#[macro_export]
macro_rules! log_operation_complete {
    ($operation:expr, $component:expr, $duration_ms:expr) => {
        tracing::info!(
            event = "operation_complete",
            operation = $operation,
            component = $component,
            duration_ms = $duration_ms,
            "Operation completed successfully"
        );
    };
    ($operation:expr, $component:expr, $duration_ms:expr, $($fields:tt)*) => {
        tracing::info!(
            event = "operation_complete",
            operation = $operation,
            component = $component,
            duration_ms = $duration_ms,
            $($fields)*,
            "Operation completed successfully"
        );
    };
}

/// Log a failed operation
#[macro_export]
macro_rules! log_operation_failed {
    ($operation:expr, $component:expr, $error:expr, $duration_ms:expr) => {
        tracing::error!(
            event = "operation_failed",
            operation = $operation,
            component = $component,
            error = %$error,
            duration_ms = $duration_ms,
            "Operation failed"
        );
    };
    ($operation:expr, $component:expr, $error:expr, $duration_ms:expr, $($fields:tt)*) => {
        tracing::error!(
            event = "operation_failed",
            operation = $operation,
            component = $component,
            error = %$error,
            duration_ms = $duration_ms,
            $($fields)*,
            "Operation failed"
        );
    };
}

/// Log HTTP request information
#[macro_export]
macro_rules! log_http_request {
    ($method:expr, $url:expr, $status:expr, $duration_ms:expr) => {
        tracing::info!(
            event = "http_request",
            component = "http_client",
            method = $method,
            url = $url,
            status = $status,
            duration_ms = $duration_ms,
            "HTTP request completed"
        );
    };
    ($method:expr, $url:expr, $status:expr, $duration_ms:expr, $($fields:tt)*) => {
        tracing::info!(
            event = "http_request",
            component = "http_client",
            method = $method,
            url = $url, 
            status = $status,
            duration_ms = $duration_ms,
            $($fields)*,
            "HTTP request completed"
        );
    };
}

/// Log cache operations
#[macro_export]
macro_rules! log_cache_operation {
    ($operation:expr, $cache_key:expr, $hit:expr) => {
        if $hit {
            tracing::debug!(
                event = "cache_hit",
                component = "cache",
                operation = $operation,
                cache_key = $cache_key,
                "Cache hit"
            );
        } else {
            tracing::debug!(
                event = "cache_miss",
                component = "cache", 
                operation = $operation,
                cache_key = $cache_key,
                "Cache miss"
            );
        }
    };
}