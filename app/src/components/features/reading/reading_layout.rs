use dioxus::prelude::*;
use glossia_shared::WordMeaning;
use crate::theme::Theme;
use crate::components::{ReadingContainer, WordMeanings, NavigationControls};

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
    rsx! {
        div {
            class: "reading-layout-wrapper",
            style: "display: flex; flex-direction: column; align-items: center; gap: 20px; width: 100%; max-width: 800px; margin: 0 auto;",
            
            // Main content container with fixed heights
            div {
                class: "reading-content-container",
                style: "
                    width: 90%;
                    max-width: 900px;
                    height: 70vh;
                    min-height: 500px;
                    display: flex;
                    flex-direction: column;
                    background: {theme.surface};
                    border: 1px solid {theme.border};
                    border-radius: 8px;
                    box-shadow: 0 2px 10px {theme.shadow};
                    overflow: hidden;
                ",
                
                // Reading section (top half)
                div {
                    class: "reading-section",
                    style: "
                        flex: 1;
                        overflow-y: auto;
                        padding: 30px;
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
                
                // Word meanings section (bottom half) - only show if there are words
                if !words.is_empty() {
                    div {
                        class: "word-meanings-section",
                        style: "
                            flex: 1;
                            overflow-y: auto;
                            padding: 30px;
                            border-top: 1px solid {theme.border};
                        ",
                        
                        WordMeanings { 
                            words: words.clone(),
                            reading_state: reading_state,
                            current_sentence: original.clone().unwrap_or_default(),
                            theme: theme.clone(),
                            on_expand_word: on_expand_word
                        }
                    }
                }
            }
            
            // Navigation controls outside the main content
            NavigationControls {
                theme: theme.clone(),
                on_prev: on_prev,
                on_next: on_next
            }
        }
    }
}