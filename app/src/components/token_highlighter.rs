use dioxus::prelude::*;
use glossia_shared::WordMeaning;
use crate::theme::Theme;
use crate::utils::generate_word_color_themed;
use crate::components::ClickableWord;

#[component]
pub fn TokenHighlighter(
    text: String,
    start_index: usize,
    end_index: usize,
    is_highlighted: bool,
    word_meanings: Option<Vec<WordMeaning>>,
    theme: Theme,
    on_word_click: EventHandler<String>,
) -> Element {
    if is_highlighted {
        // Generate color and styling for highlighted text
        let color = generate_word_color_themed(&text, &theme);
        let style = format!("color: {color}; font-weight: 600;");
        
        rsx! {
            ClickableWord {
                text: text.clone(),
                index: start_index,
                is_clickable: true,
                style: style,
                on_click: on_word_click
            }
        }
    } else {
        // Normal text styling
        let style = "".to_string();
        
        rsx! {
            ClickableWord {
                text: text.clone(),
                index: start_index,
                is_clickable: true,
                style: style,
                on_click: on_word_click
            }
        }
    }
}
