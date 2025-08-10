use dioxus::prelude::*;
use crate::theme::Theme;

/// Provides the main application layout structure
#[component]
pub fn AppLayout(
    theme: Theme,
    children: Element,
) -> Element {
    rsx! {
        div {
            class: "app-container",
            style: "min-height: 100vh; width: 100%; background: {theme.background}; font-family: 'Alegreya', Palatino, 'Book Antiqua', serif; display: flex; flex-direction: column; position: relative;",
            
            // Theme-aware subtle gradient background
            div {
                class: "app-background",
                style: match theme.mode {
                    crate::theme::ThemeMode::Light => 
                        "position: absolute; inset: 0; z-index: 1; background: linear-gradient(135deg, #fafafa 0%, #f5f5f5 50%, #fafafa 100%); background-size: 100% 100%;",
                    crate::theme::ThemeMode::Dark => 
                        "position: absolute; inset: 0; z-index: 1; background: linear-gradient(135deg, #1a1a1a 0%, #1f1f1f 50%, #1a1a1a 100%); background-size: 100% 100%;",
                },
            }
            
            // Main content with proper z-index to appear above background
            div {
                style: "position: relative; z-index: 2; flex: 1; display: flex; flex-direction: column;",
                {children}
            }
        }
    }
}