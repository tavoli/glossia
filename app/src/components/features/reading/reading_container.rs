use dioxus::prelude::*;
use glossia_shared::WordMeaning;
use crate::theme::Theme;
use crate::components::TextRenderer;

#[component]
pub fn ReadingContainer(
    original: Option<String>,
    simplified: Option<String>,
    is_loading: bool,
    words: Option<Vec<WordMeaning>>,
    theme: Theme,
    on_word_click: EventHandler<String>
) -> Element {

    rsx! {
        div {
            class: "reading-container",
            style: "text-align: center; width: 100%;",
            
            div {
                class: "original-text",
                style: "font-size: 1.5em; color: {theme.text_primary}; line-height: 1.6;",
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
            
            if simplified.is_some() || is_loading {
                div {
                    class: "simplified-text",
                    style: "margin-top: 20px; font-size: 1.2em; color: {theme.text_secondary}; min-height: 50px; line-height: 1.5;",
                    
                    if is_loading {
                        div { 
                            class: "loading-indicator",
                            style: "color: {theme.text_secondary}; opacity: 0.6;",
                            "Loading simplified version..." 
                        }
                    } else if let Some(text) = simplified {
                        TextRenderer {
                            text: text.clone(),
                            word_meanings: words.clone(),
                            theme: theme.clone(),
                            on_word_click: on_word_click
                        }
                    }
                }
            }
        }
    }
}
