use dioxus::prelude::*;
use crate::hooks::AppState;

/// Handles keyboard navigation for the reading experience
pub fn keyboard_handler(
    app_state: AppState,
    children: Element,
) -> Element {
    let mut navigation_state = app_state.clone();
    
    rsx! {
        div {
            tabindex: 0,
            onkeydown: move |e| {
                // Only handle navigation when modal is not open and we have content
                if !*navigation_state.show_input_modal.read() && 
                   navigation_state.reading_state.read().total_sentences() > 0 {
                    match e.key() {
                        Key::ArrowRight => {
                            navigation_state.reading_state.write().next();
                            if let Some(sentence) = navigation_state.reading_state.read().current_sentence() {
                                navigation_state.sentence_to_fetch.set(sentence);
                            }
                        },
                        Key::ArrowLeft => {
                            navigation_state.reading_state.write().previous();
                            if let Some(sentence) = navigation_state.reading_state.read().current_sentence() {
                                navigation_state.sentence_to_fetch.set(sentence);
                            }
                        },
                        _ => {}
                    }
                }
            },
            
            {children}
        }
    }
}