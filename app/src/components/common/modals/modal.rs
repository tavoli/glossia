use dioxus::prelude::*;
use crate::theme::Theme;

#[derive(Clone)]
pub struct ModalStyles<'a> {
    pub theme: &'a Theme,
}

impl<'a> ModalStyles<'a> {
    pub fn new(theme: &'a Theme) -> Self {
        Self { theme }
    }
    
    pub fn overlay(&self) -> String {
        format!(
            "position: fixed; \
             top: 0; \
             left: 0; \
             right: 0; \
             bottom: 0; \
             background: rgba(0, 0, 0, 0.5); \
             display: flex; \
             align-items: center; \
             justify-content: center; \
             z-index: 1000; \
             backdrop-filter: blur(5px);"
        )
    }
    
    pub fn container(&self, width: &str) -> String {
        format!(
            "background: {}; \
             border-radius: 12px; \
             box-shadow: 0 20px 40px rgba(0, 0, 0, 0.3); \
             width: 90%; \
             max-width: {}; \
             max-height: 80vh; \
             display: flex; \
             flex-direction: column; \
             border: 1px solid {};",
            self.theme.surface, width, self.theme.border
        )
    }
}

#[component]
pub fn Modal(
    children: Element,
    theme: Theme,
    on_close: Option<EventHandler<()>>,
    max_width: Option<String>,
) -> Element {
    let styles = ModalStyles::new(&theme);
    let width = max_width.unwrap_or_else(|| "600px".to_string());
    
    rsx! {
        div {
            class: "modal-overlay",
            style: "{styles.overlay()}",
            onclick: move |_| {
                if let Some(handler) = &on_close {
                    handler.call(());
                }
            },
            
            div {
                class: "modal-container",
                style: "{styles.container(&width)}",
                onclick: move |e| {
                    // Prevent clicks inside the modal from closing it
                    e.stop_propagation();
                },
                
                {children}
            }
        }
    }
}