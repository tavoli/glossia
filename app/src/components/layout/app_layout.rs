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
            
            // Theme-aware grid background
            div {
                class: "app-background",
                style: "position: absolute; inset: 0; z-index: 1; background: {theme.background}; background-image: linear-gradient(to right, {theme.border} 1px, transparent 1px), linear-gradient(to bottom, {theme.border} 1px, transparent 1px), radial-gradient(circle at 50% 60%, rgba(236,72,153,0.15) 0%, rgba(168,85,247,0.05) 40%, transparent 70%); background-size: 40px 40px, 40px 40px, 100% 100%;",
            }
            
            // Main content with proper z-index to appear above background
            div {
                style: "position: relative; z-index: 2; flex: 1; display: flex; flex-direction: column;",
                {children}
            }
        }
    }
}