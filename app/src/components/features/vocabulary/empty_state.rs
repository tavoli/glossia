use dioxus::prelude::*;
use crate::components::features::vocabulary::known_words_modal_styles::KnownWordsModalStyles;
use crate::theme::Theme;

#[component]
pub fn EmptyState(
    is_searching: bool,
    theme: Theme,
) -> Element {
    let styles = KnownWordsModalStyles::new(&theme);
    
    let message = if is_searching {
        "No words found matching your search."
    } else {
        "No known words yet. Start reading and double-click words to add them!"
    };
    
    rsx! {
        div {
            style: "{styles.empty_state()}",
            "{message}"
        }
    }
}