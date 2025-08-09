use dioxus::prelude::*;
use crate::theme::Theme;

#[component]
pub fn NavigationControls(
    theme: Theme,
    on_prev: EventHandler<()>,
    on_next: EventHandler<()>,
) -> Element {
    rsx! {
        style {
            "
            .nav-btn {{
                background: {theme.accent};
                color: white;
                border: none;
                padding: 12px 24px;
                border-radius: 6px;
                cursor: pointer;
                font-size: 1em;
                transition: all 0.2s ease;
                box-shadow: 0 2px 4px {theme.shadow};
            }}
            .nav-btn:hover {{
                transform: translateY(-2px);
                box-shadow: 0 4px 8px {theme.shadow};
            }}
            .nav-btn:active {{
                transform: translateY(0);
            }}
            "
        }
        
        div {
            class: "navigation-controls",
            style: "display: flex; gap: 15px; justify-content: center;",
            
            button { 
                class: "nav-btn",
                onclick: move |_| on_prev.call(()), 
                "Previous" 
            }
            
            button { 
                class: "nav-btn",
                onclick: move |_| on_next.call(()), 
                "Next" 
            }
        }
    }
}
