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
            .nav-controls-container {{
                position: fixed;
                top: 0;
                left: 0;
                right: 0;
                bottom: 0;
                pointer-events: none;
                z-index: 15;
            }}
            
            .nav-chevron {{
                position: fixed;
                top: 50%;
                transform: translateY(-50%);
                background: {theme.surface};
                backdrop-filter: blur(8px);
                border: 1px solid {theme.border};
                color: {theme.text_primary};
                width: 60px;
                height: 140px;
                display: flex;
                align-items: center;
                justify-content: center;
                cursor: pointer;
                transition: all 0.3s ease;
                font-size: 3em;
                font-weight: 200;
                opacity: 0.4;
                pointer-events: all;
                box-shadow: 0 4px 12px {theme.shadow};
            }}
            
            .nav-chevron:hover {{
                opacity: 0.7;
                transform: translateY(-50%) scale(1.02);
            }}
            
            .nav-chevron:active {{
                transform: translateY(-50%) scale(0.98);
            }}
            
            .nav-chevron.prev {{
                left: 20px;
                border-radius: 0 8px 8px 0;
            }}
            
            .nav-chevron.next {{
                right: 20px;
                border-radius: 8px 0 0 8px;
            }}
            
            @media (max-width: 1600px) {{
                .nav-chevron {{
                    width: 50px;
                    height: 120px;
                    font-size: 2.5em;
                }}
                .nav-chevron.prev {{
                    left: 10px;
                }}
                .nav-chevron.next {{
                    right: 10px;
                }}
            }}
            
            @media (max-width: 768px) {{
                .nav-chevron {{
                    width: 40px;
                    height: 100px;
                    font-size: 2em;
                    opacity: 0.3;
                }}
                .nav-chevron.prev {{
                    left: 5px;
                }}
                .nav-chevron.next {{
                    right: 5px;
                }}
            }}
            "
        }
        
        div {
            class: "nav-controls-container",
            
            // Previous button (left chevron)
            button { 
                class: "nav-chevron prev",
                onclick: move |_| on_prev.call(()), 
                "‹" 
            }
            
            // Next button (right chevron)
            button { 
                class: "nav-chevron next",
                onclick: move |_| on_next.call(()), 
                "›" 
            }
        }
    }
}
