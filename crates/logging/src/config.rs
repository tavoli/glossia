use serde::{Deserialize, Serialize};
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt, Registry};
use std::env;

/// Configuration for the logging system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    /// Log level filter (e.g., "info", "debug", "glossia=debug")
    pub level: String,
    /// Output format for logs
    pub format: LogFormat,
    /// Whether to include target module names in logs
    pub with_target: bool,
    /// Whether to include thread IDs in logs
    pub with_thread_ids: bool,
    /// Whether to include line numbers in logs
    pub with_line_number: bool,
    /// Whether to include timestamps
    pub with_timestamp: bool,
}

/// Available log output formats
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogFormat {
    /// Human-readable format for development
    Pretty,
    /// Compact format for production
    Compact,
    /// JSON format for log aggregation systems
    Json,
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: "glossia=info,glossia_http_client=debug,glossia_llm_client=info,glossia_image_client=info,glossia_logging=info".to_string(),
            format: LogFormat::Pretty,
            with_target: false,
            with_thread_ids: true,
            with_line_number: true,
            with_timestamp: true,
        }
    }
}

impl LoggingConfig {
    /// Create logging configuration from environment variables
    pub fn from_env() -> Self {
        let mut config = Self::default();
        
        // Override with environment variables if present
        if let Ok(level) = env::var("RUST_LOG") {
            config.level = level;
        }
        
        if let Ok(format) = env::var("LOG_FORMAT") {
            config.format = match format.to_lowercase().as_str() {
                "json" => LogFormat::Json,
                "compact" => LogFormat::Compact,
                "pretty" | _ => LogFormat::Pretty,
            };
        }
        
        if let Ok(target) = env::var("LOG_WITH_TARGET") {
            config.with_target = target.parse().unwrap_or(false);
        }
        
        if let Ok(threads) = env::var("LOG_WITH_THREAD_IDS") {
            config.with_thread_ids = threads.parse().unwrap_or(true);
        }
        
        if let Ok(lines) = env::var("LOG_WITH_LINE_NUMBER") {
            config.with_line_number = lines.parse().unwrap_or(true);
        }
        
        if let Ok(timestamp) = env::var("LOG_WITH_TIMESTAMP") {
            config.with_timestamp = timestamp.parse().unwrap_or(true);
        }
        
        config
    }
    
    /// Create production-ready configuration
    pub fn production() -> Self {
        Self {
            level: "glossia=info,glossia_http_client=warn,glossia_llm_client=info,glossia_image_client=info,glossia_logging=info".to_string(),
            format: LogFormat::Json,
            with_target: true,
            with_thread_ids: false,
            with_line_number: false,
            with_timestamp: true,
        }
    }
    
    /// Create development-friendly configuration  
    pub fn development() -> Self {
        Self {
            level: "glossia=debug,glossia_http_client=debug,glossia_llm_client=debug,glossia_image_client=debug,glossia_logging=debug".to_string(),
            format: LogFormat::Pretty,
            with_target: false,
            with_thread_ids: true,
            with_line_number: true,
            with_timestamp: true,
        }
    }
}

/// Initialize the logging system with the provided configuration
pub fn init_logging(config: &LoggingConfig) -> Result<(), Box<dyn std::error::Error>> {
    let env_filter = EnvFilter::try_new(&config.level)
        .unwrap_or_else(|_| EnvFilter::new("info"));
    
    match config.format {
        LogFormat::Json => {
            let layer = fmt::layer()
                .json()
                .with_target(config.with_target)
                .with_thread_ids(config.with_thread_ids)
                .with_line_number(config.with_line_number);
            
            if config.with_timestamp {
                Registry::default()
                    .with(env_filter)
                    .with(layer.with_timer(fmt::time::ChronoUtc::default()))
                    .init();
            } else {
                Registry::default()
                    .with(env_filter)
                    .with(layer.with_timer(fmt::time::uptime()))
                    .init();
            }
        },
        LogFormat::Compact => {
            let layer = fmt::layer()
                .compact()
                .with_target(config.with_target)
                .with_thread_ids(config.with_thread_ids)
                .with_line_number(config.with_line_number);
            
            if config.with_timestamp {
                Registry::default()
                    .with(env_filter)
                    .with(layer.with_timer(fmt::time::ChronoUtc::default()))
                    .init();
            } else {
                Registry::default()
                    .with(env_filter)
                    .with(layer.with_timer(fmt::time::uptime()))
                    .init();
            }
        },
        LogFormat::Pretty => {
            let layer = fmt::layer()
                .pretty()
                .with_target(config.with_target)
                .with_thread_ids(config.with_thread_ids)
                .with_line_number(config.with_line_number);
            
            if config.with_timestamp {
                Registry::default()
                    .with(env_filter)
                    .with(layer.with_timer(fmt::time::ChronoUtc::default()))
                    .init();
            } else {
                Registry::default()
                    .with(env_filter)
                    .with(layer.with_timer(fmt::time::uptime()))
                    .init();
            }
        }
    }
    
    Ok(())
}