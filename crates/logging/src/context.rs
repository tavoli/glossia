use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;
use once_cell::sync::Lazy;
use std::collections::HashMap;

/// Unique identifier for correlating related log entries
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CorrelationId(String);

impl CorrelationId {
    /// Generate a new random correlation ID
    pub fn new() -> Self {
        Self(Uuid::new_v4().to_string())
    }
    
    /// Create a correlation ID from a string
    pub fn from_string(id: String) -> Self {
        Self(id)
    }
    
    /// Get the correlation ID as a string
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for CorrelationId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Default for CorrelationId {
    fn default() -> Self {
        Self::new()
    }
}

/// Context information for structured logging
#[derive(Debug, Clone)]
pub struct LogContext {
    pub correlation_id: CorrelationId,
    pub user_session: Option<String>,
    pub component: String,
    pub operation: Option<String>,
    pub metadata: HashMap<String, String>,
}

impl LogContext {
    /// Create a new log context with correlation ID
    pub fn new(component: &str) -> Self {
        Self {
            correlation_id: CorrelationId::new(),
            user_session: None,
            component: component.to_string(),
            operation: None,
            metadata: HashMap::new(),
        }
    }
    
    /// Create a new context with a specific correlation ID
    pub fn with_correlation_id(component: &str, correlation_id: CorrelationId) -> Self {
        Self {
            correlation_id,
            user_session: None,
            component: component.to_string(),
            operation: None,
            metadata: HashMap::new(),
        }
    }
    
    /// Set user session information
    pub fn with_user_session(mut self, session: String) -> Self {
        self.user_session = Some(session);
        self
    }
    
    /// Set the current operation
    pub fn with_operation(mut self, operation: String) -> Self {
        self.operation = Some(operation);
        self
    }
    
    /// Add metadata to the context
    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }
    
    /// Get correlation ID for use in tracing spans
    pub fn correlation_id(&self) -> &str {
        self.correlation_id.as_str()
    }
    
    /// Create a child context that inherits the correlation ID
    pub fn child_context(&self, component: &str) -> Self {
        Self {
            correlation_id: self.correlation_id.clone(),
            user_session: self.user_session.clone(),
            component: component.to_string(),
            operation: None,
            metadata: self.metadata.clone(),
        }
    }
}

/// Global context storage for the current thread/request
static CONTEXT_STORAGE: Lazy<Arc<RwLock<Option<LogContext>>>> = 
    Lazy::new(|| Arc::new(RwLock::new(None)));

/// Execute a closure with a specific log context
pub async fn with_context<F, R>(context: LogContext, f: F) -> R 
where
    F: std::future::Future<Output = R>,
{
    // Set the context
    {
        let mut storage = CONTEXT_STORAGE.write().await;
        *storage = Some(context);
    }
    
    // Execute the function
    let result = f.await;
    
    // Clear the context
    {
        let mut storage = CONTEXT_STORAGE.write().await;
        *storage = None;
    }
    
    result
}

/// Get the current log context if available
pub async fn current_context() -> Option<LogContext> {
    let storage = CONTEXT_STORAGE.read().await;
    storage.clone()
}

/// Get the current correlation ID if available
pub async fn current_correlation_id() -> Option<CorrelationId> {
    let storage = CONTEXT_STORAGE.read().await;
    storage.as_ref().map(|ctx| ctx.correlation_id.clone())
}

/// Macros for easier context-aware logging
#[macro_export]
macro_rules! log_with_context {
    ($level:ident, $($fields:tt)*) => {
        if let Some(ctx) = $crate::context::current_context().await {
            tracing::$level!(
                correlation_id = %ctx.correlation_id,
                component = %ctx.component,
                user_session = ?ctx.user_session,
                operation = ?ctx.operation,
                $($fields)*
            );
        } else {
            tracing::$level!($($fields)*);
        }
    };
}

/// Create tracing spans with context information
#[macro_export]
macro_rules! span_with_context {
    ($level:expr, $name:expr) => {
        if let Some(ctx) = $crate::context::current_context().await {
            tracing::span!(
                $level,
                $name,
                correlation_id = %ctx.correlation_id,
                component = %ctx.component,
                user_session = ?ctx.user_session,
                operation = ?ctx.operation
            )
        } else {
            tracing::span!($level, $name)
        }
    };
}