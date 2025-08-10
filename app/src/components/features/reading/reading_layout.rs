use dioxus::prelude::*;
use glossia_shared::WordMeaning;
use crate::theme::Theme;
use crate::components::{ReadingContainer, WordMeanings, NavigationControls};
use crate::components::features::reading::WordMeaningSkeleton;

#[component]
pub fn ReadingLayout(
    original: Option<String>,
    simplified: Option<String>,
    is_loading: bool,
    words: Vec<WordMeaning>,
    theme: Theme,
    reading_state: Signal<glossia_reading_engine::ReadingEngine>,
    on_next: EventHandler<()>,
    on_prev: EventHandler<()>,
    on_word_click: EventHandler<String>,
    on_expand_word: EventHandler<String>,
) -> Element {
    let responsive_styles = format!(
        "
        /* Custom scrollbar styles for reading areas */
        .reading-section::-webkit-scrollbar,
        .word-meanings-sidebar::-webkit-scrollbar {{
            width: 8px;
            height: 8px;
        }}
        
        .reading-section::-webkit-scrollbar-track,
        .word-meanings-sidebar::-webkit-scrollbar-track {{
            background: transparent;
            border-radius: 4px;
        }}
        
        .reading-section::-webkit-scrollbar-thumb,
        .word-meanings-sidebar::-webkit-scrollbar-thumb {{
            background: {};
            border-radius: 4px;
            border: 2px solid transparent;
            background-clip: padding-box;
        }}
        
        .reading-section::-webkit-scrollbar-thumb:hover,
        .word-meanings-sidebar::-webkit-scrollbar-thumb:hover {{
            background: {};
            background-clip: padding-box;
        }}
        
        /* Firefox scrollbar styling */
        .reading-section,
        .word-meanings-sidebar {{
            scrollbar-width: thin;
            scrollbar-color: {} transparent;
        }}
        
        /* Smooth scrolling */
        .reading-section,
        .word-meanings-sidebar {{
            scroll-behavior: smooth;
        }}
        
        @media (max-width: 768px) {{
            .reading-content-container {{
                flex-direction: column !important;
            }}
            .word-meanings-sidebar {{
                max-width: 100% !important;
                min-width: 100% !important;
                border-left: none !important;
                border-top: 1px solid {} !important;
            }}
        }}
        ",
        if theme.mode == crate::theme::ThemeMode::Light { "rgba(0,0,0,0.15)" } else { "rgba(255,255,255,0.1)" },
        if theme.mode == crate::theme::ThemeMode::Light { "rgba(0,0,0,0.25)" } else { "rgba(255,255,255,0.2)" },
        if theme.mode == crate::theme::ThemeMode::Light { "rgba(0,0,0,0.2)" } else { "rgba(255,255,255,0.15)" },
        theme.border
    );

    rsx! {
        style { "{responsive_styles}" }
        
        div {
            class: "reading-layout-wrapper",
            style: "display: flex; flex-direction: column; align-items: center; gap: 20px; width: 100%; margin: 0 auto; overflow-x: hidden;",
            
            // Main content container with sidebar layout
            div {
                class: "reading-content-container",
                style: "
                    width: 95%;
                    max-width: 1400px;
                    height: 70vh;
                    min-height: 500px;
                    display: flex;
                    flex-direction: row;
                    background: {theme.surface};
                    border: 1px solid {theme.border};
                    border-radius: 8px;
                    box-shadow: 0 2px 10px {theme.shadow};
                    overflow: hidden;
                ",
                
                // Reading section (main content area - left side)
                div {
                    class: "reading-section",
                    style: "
                        flex: 2;
                        min-width: 0;
                        overflow-y: auto;
                        padding: 40px;
                        display: flex;
                        flex-direction: column;
                        justify-content: center;
                    ",
                    
                    ReadingContainer {
                        original: original.clone(),
                        simplified: simplified.clone(),
                        is_loading,
                        words: Some(words.clone()),
                        theme: theme.clone(),
                        on_word_click: on_word_click
                    }
                }
                
                // Word meanings sidebar (right side) - always show to prevent layout shifts
                div {
                    class: "word-meanings-sidebar",
                    style: "
                        flex: 1;
                        max-width: 450px;
                        min-width: 350px;
                        overflow-y: auto;
                        padding: 30px;
                        border-left: 1px solid {theme.border};
                        background: {theme.surface};
                    ",
                    
                    if !words.is_empty() {
                        WordMeanings { 
                            words: words.clone(),
                            reading_state: reading_state,
                            current_sentence: original.clone().unwrap_or_default(),
                            theme: theme.clone(),
                            on_expand_word: on_expand_word
                        }
                    } else {
                        WordMeaningSkeleton {
                            theme: theme.clone(),
                            count: 4
                        }
                    }
                }
            }
            
            // Navigation controls outside the container
            NavigationControls {
                theme: theme.clone(),
                on_prev: on_prev,
                on_next: on_next
            }
        }
    }
}