use dioxus::prelude::*;
use crate::components::features::vocabulary::known_words_modal_styles::KnownWordsModalStyles;
use crate::theme::Theme;

#[component]
pub fn ModalHeader(
    title: String,
    count: usize,
    theme: Theme,
    on_close: EventHandler<()>,
) -> Element {
    let styles = KnownWordsModalStyles::new(&theme);
    
    rsx! {
        div {
            class: "modal-header",
            style: "{styles.header()}",
            
            h2 {
                style: "{styles.title()}",
                "{title} ({count})"
            }
            
            button {
                style: "{styles.close_button()}",
                onclick: move |_| on_close.call(()),
                title: "Close",
                "×"
            }
        }
    }
}