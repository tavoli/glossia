use dioxus::prelude::*;
use crate::components::features::vocabulary::known_words_modal_styles::KnownWordsModalStyles;
use crate::theme::Theme;

#[component]
pub fn SearchBar(
    search_query: Signal<String>,
    theme: Theme,
) -> Element {
    let styles = KnownWordsModalStyles::new(&theme);
    
    rsx! {
        div {
            class: "modal-search",
            style: "{styles.search_section()}",
            
            input {
                style: "{styles.search_input()}",
                r#type: "text",
                placeholder: "Search words...",
                value: "{search_query}",
                oninput: move |e| {
                    search_query.set(e.value());
                }
            }
        }
    }
}