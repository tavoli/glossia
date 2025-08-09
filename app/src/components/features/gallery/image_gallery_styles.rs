use dioxus::prelude::*;
use crate::theme::Theme;

#[component]
pub fn ImageGalleryStyles(theme: Theme) -> Element {
    rsx! {
        style {
            "
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