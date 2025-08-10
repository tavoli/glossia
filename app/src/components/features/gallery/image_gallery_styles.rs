use dioxus::prelude::*;
use crate::theme::Theme;

#[component]
pub fn ImageGalleryStyles(theme: Theme) -> Element {
    rsx! {
        style {
            "
            .image-gallery {{
                margin-top: 12px;
                padding: 0;
                background: {theme.gallery_bg};
                border-radius: 0;
                border: none;
            }}
            
            .gallery-message {{
                text-align: center;
                color: {theme.text_secondary};
                font-style: italic;
            }}
            
            .gallery-message.error {{
                color: {theme.error};
            }}
            
            .gallery-header {{
                margin-bottom: 12px;
                color: {theme.text_primary};
                font-weight: 500;
                font-size: 0.9em;
            }}
            
            .images-grid {{
                display: flex;
                gap: 8px;
                overflow-x: auto;
                padding: 0;
                scroll-behavior: smooth;
                -webkit-overflow-scrolling: touch;
            }}
            
            .images-grid::-webkit-scrollbar {{
                height: 6px;
            }}
            
            .images-grid::-webkit-scrollbar-track {{
                background: {theme.background};
                border-radius: 3px;
            }}
            
            .images-grid::-webkit-scrollbar-thumb {{
                background: {theme.border};
                border-radius: 3px;
            }}
            
            .images-grid::-webkit-scrollbar-thumb:hover {{
                background: {theme.accent};
            }}
            
            .image-item {{
                position: relative;
                background: transparent;
                border-radius: 6px;
                overflow: hidden;
                box-shadow: 0 1px 3px {theme.shadow};
                transition: opacity 0.2s ease;
                flex: 0 0 150px;
                cursor: pointer;
            }}
            
            .image-item:hover {{
                opacity: 0.9;
            }}
            
            .image-item img {{
                width: 150px;
                height: 150px;
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