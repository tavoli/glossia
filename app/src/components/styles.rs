use dioxus::prelude::*;
use crate::theme::Theme;

#[component]
pub fn WordMeaningsStyles(theme: Theme) -> Element {
    rsx! {
        style {
            "
            .word-meanings-container {{
                margin-top: 24px;
                width: 80%;
                padding: 40px;
                background: {theme.surface};
                max-width: 700px;
                border: 1px solid {theme.border};
                border-radius: 8px;
                box-shadow: 0 2px 10px {theme.shadow};
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
            
            .image-gallery {{
                margin-top: 16px;
                padding: 16px;
                background: {theme.gallery_bg};
                border-radius: 8px;
                border: 1px solid {theme.gallery_border};
            }}
            
            .gallery-message {{
                text-align: center;
                color: {theme.text_secondary};
                font-style: italic;
            }}
            
            .gallery-message.error {{
                color: {theme.error};
            }}
            
            .images-grid {{
                display: grid;
                grid-template-columns: repeat(auto-fit, minmax(120px, 1fr));
                gap: 12px;
                max-width: 100%;
            }}
            
            .image-item {{
                position: relative;
                background: {theme.surface};
                border-radius: 6px;
                overflow: hidden;
                box-shadow: 0 2px 4px {theme.shadow};
                transition: transform 0.2s ease;
            }}
            
            .image-item:hover {{
                transform: translateY(-2px);
                box-shadow: 0 4px 8px {theme.shadow};
            }}
            
            .image-item img {{
                width: 100%;
                height: 120px;
                object-fit: cover;
                display: block;
            }}
            
            .image-title {{
                position: absolute;
                bottom: 0;
                left: 0;
                right: 0;
                background: linear-gradient(transparent, rgba(0,0,0,0.7));
                color: white;
                padding: 8px 6px 4px;
                font-size: 0.75em;
                line-height: 1.2;
                max-height: 2.4em;
                overflow: hidden;
            }}
            "
        }
    }
}
