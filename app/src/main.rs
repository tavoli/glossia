#![allow(non_snake_case)]
mod components;
mod theme;
mod utils;
mod hooks;
mod services;

use components::App;
use glossia_logging;
use tracing::{info, warn, error};

fn main() {
    // Load environment variables from .env file
    dotenvy::dotenv().ok();
    
    // Initialize centralized logging system
    if let Err(e) = glossia_logging::setup_application_logging() {
        eprintln!("Failed to initialize logging: {}", e);
        std::process::exit(1);
    }
    
    glossia_logging::log_startup("Glossia", env!("CARGO_PKG_VERSION"));
    
    // Perform basic health checks
    tokio::runtime::Runtime::new().unwrap().block_on(perform_health_checks());

    // Set up graceful shutdown handler
    let app_name = "Glossia";
    let app_name_clone = app_name.to_string();
    
    let shutdown_handler = move || {
        glossia_logging::log_shutdown(&app_name_clone);
    };
    
    // Set up signal handlers for graceful shutdown
    #[cfg(unix)]
    {
        use std::sync::Arc;
        let shutdown_handler = Arc::new(std::sync::Mutex::new(shutdown_handler));
        let handler_clone = shutdown_handler.clone();
        
        ctrlc::set_handler(move || {
            if let Ok(handler) = handler_clone.lock() {
                handler();
            }
            std::process::exit(0);
        }).expect("Error setting Ctrl-C handler");
    }
    
    // Initialize and launch app
    info!("Launching desktop application");
    dioxus_desktop::launch::launch(App, vec![], Default::default());
    
    // This will only be reached if the app exits normally
    glossia_logging::log_shutdown(app_name);
}

/// Perform basic health checks on application startup
async fn perform_health_checks() {
    info!("Starting application health checks");
    
    // Check if .env file is available
    if std::path::Path::new(".env").exists() {
        info!("Configuration file (.env) found");
    } else {
        warn!("No .env configuration file found, using environment variables");
    }
    
    // Check data directories
    if let Some(home_dir) = dirs::home_dir() {
        let app_data_dir = home_dir.join(".glossia");
        if !app_data_dir.exists() {
            info!("Creating application data directory: {:?}", app_data_dir);
            if let Err(e) = std::fs::create_dir_all(&app_data_dir) {
                error!("Failed to create application data directory: {}", e);
            } else {
                info!("Application data directory created successfully");
            }
        } else {
            info!("Application data directory exists: {:?}", app_data_dir);
        }
    }
    
    // Try to initialize core services to check for major configuration issues
    match glossia_llm_client::LLMClientFactory::new().create_client() {
        Ok(_) => info!("LLM client initialization successful"),
        Err(e) => warn!("LLM client initialization failed: {}. Features requiring LLM may not work.", e),
    }
    
    match glossia_image_client::ImageClientFactory::new().create_client() {
        Ok(_) => info!("Image client initialization successful"),
        Err(e) => warn!("Image client initialization failed: {}. Image features may not work.", e),
    }
    
    info!("Health checks completed");
}