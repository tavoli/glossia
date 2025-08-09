use dioxus::prelude::*;
use glossia_shared::WordMeaning;
use crate::theme::Theme;
use crate::components::{TextRenderer, NavigationControls};

#[component]
pub fn ReadingContainer(
    original: Option<String>,
    simplified: Option<String>,
    is_loading: bool,
    words: Option<Vec<WordMeaning>>,
    theme: Theme,
    on_next: EventHandler<()>, 
    on_prev: EventHandler<()>,
    on_word_click: EventHandler<String>
) -> Element {

    rsx! {
        div {
            class: "reading-container",
            style: "background: {theme.surface}; padding: 40px; border-radius: 8px; box-shadow: 0 2px 10px {theme.shadow}; width: 80%; max-width: 700px; text-align: center; z-index: 20; border: 1px solid {theme.border};",
            
            div {
                class: "original-text",
                style: "font-size: 1.5em; color: {theme.text_primary}; line-height: 1.4;",
                if let Some(ref text) = original {
                    TextRenderer {
                        text: text.clone(),
                        word_meanings: words.clone(),
                        theme: theme.clone(),
                        on_word_click: on_word_click
                    }
                } else {
                    "Paste text to start reading."
                }
            }
            
            div {
                class: "simplified-text",
                style: "margin-top: 20px; font-size: 1.2em; color: {theme.text_secondary}; min-height: 50px; line-height: 1.4;",
                
                if is_loading {
                    div { class: "loading-indicator", "Loading..." }
                } else if let Some(text) = simplified {
                    TextRenderer {
                        text: text.clone(),
                        word_meanings: words.clone(),
                        theme: theme.clone(),
                        on_word_click: on_word_click
                    }
                }
            }
            
            NavigationControls {
                theme: theme.clone(),
                on_prev: on_prev,
                on_next: on_next
            }
        }
    }
}
