use dioxus::prelude::*;
use arboard::Clipboard;
use std::time::Duration;
use tracing::{debug, error, info};

/// Hook to monitor clipboard changes and detect new text
pub fn use_clipboard_monitor(
    last_clipboard_text: Signal<Option<String>>,
    current_clipboard_text: Signal<Option<String>>,
    show_clipboard_toast: Signal<bool>,
    current_reading_text: String,
) {
    // Use a coroutine instead of thread for better integration with Dioxus
    use_coroutine(move |_: UnboundedReceiver<()>| {
        let mut last_clipboard = last_clipboard_text.clone();
        let mut current_clipboard = current_clipboard_text.clone();
        let mut show_toast = show_clipboard_toast.clone();
        let reading_text = current_reading_text.clone();
        
        async move {
            info!("Starting clipboard monitoring");
            
            loop {
                // Check clipboard every 500ms
                tokio::time::sleep(Duration::from_millis(500)).await;
                
                // Try to get clipboard text in a blocking task
                let clipboard_result = tokio::task::spawn_blocking(|| {
                    match Clipboard::new() {
                        Ok(mut clipboard) => clipboard.get_text(),
                        Err(e) => Err(e),
                    }
                }).await;
                
                match clipboard_result {
                    Ok(Ok(new_text)) => {
                        // Check if this is genuinely new text
                        let last = last_clipboard.read().clone();
                        
                        // Determine if we should show the toast
                        let should_show = !new_text.is_empty() 
                            && Some(new_text.clone()) != last
                            && new_text != reading_text
                            && new_text.len() > 10; // Minimum length to avoid accidental triggers
                        
                        if should_show {
                            debug!(
                                "New clipboard text detected: {} chars", 
                                new_text.len()
                            );
                            
                            // Update current clipboard and show toast
                            current_clipboard.set(Some(new_text.clone()));
                            show_toast.set(true);
                            
                            // Don't auto-hide - let user decide
                        }
                        
                        // Always update last clipboard text
                        if Some(new_text.clone()) != last {
                            last_clipboard.set(Some(new_text));
                        }
                    }
                    Ok(Err(e)) => {
                        // This is normal when clipboard doesn't contain text
                        debug!("Could not get text from clipboard: {}", e);
                    }
                    Err(e) => {
                        error!("Failed to spawn clipboard task: {}", e);
                        // Wait longer before retrying on error
                        tokio::time::sleep(Duration::from_secs(2)).await;
                    }
                }
            }
        }
    });
}