use dioxus::prelude::*;
use crate::theme::Theme;

#[component]
pub fn WordMeaningsStyles(theme: Theme) -> Element {
    rsx! {
        style {
            "
            .word-meanings-container {{
                width: 100%;
                background: {theme.surface};
            }}
            
            .meanings-list {{
                background: {theme.surface};
                overflow: hidden;
            }}
            
            .meaning-item {{
                display: flex;
                flex-direction: column;
                padding: 12px 16px;
                transition: background-color 0.15s ease;
            }}
            
            .meaning-item:hover {{
                background: {theme.hover_bg};
            }}
            
            .meaning-item.border-bottom {{
                border-bottom: 1px solid {theme.border};
            }}
            
            .word-header {{
                display: flex;
                align-items: flex-start;
                cursor: pointer;
            }}
            
            .word-label {{
                color: white;
                padding: 3px 8px;
                border-radius: 16px;
                font-size: 0.8em;
                font-weight: 600;
                min-width: fit-content;
                margin-right: 12px;
                margin-top: 2px;
                box-shadow: 0 1px 3px {theme.shadow};
            }}
            
            .meaning-definition {{
                color: {theme.text_secondary};
                line-height: 1.4;
                font-size: 0.9em;
                flex: 1;
            }}
            
            .expand-icon {{
                margin-left: 8px;
                color: {theme.text_secondary};
                font-size: 0.8em;
                transition: transform 0.2s ease;
            }}
            
            .expand-icon.expanded {{
                transform: rotate(180deg);
            }}
            "
        }
    }
}