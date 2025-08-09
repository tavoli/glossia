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
            
            .gallery-header {{
                margin-bottom: 12px;
                color: {theme.text_primary};
                font-weight: 500;
                font-size: 0.9em;
            }}
            
            .images-grid {{
                display: flex;
                gap: 12px;
                overflow-x: auto;
                padding: 4px;
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
                background: {theme.surface};
                border-radius: 8px;
                overflow: hidden;
                box-shadow: 0 2px 6px {theme.shadow};
                transition: all 0.3s ease;
                flex: 0 0 150px;
                cursor: pointer;
            }}
            
            .image-item:hover {{
                transform: scale(1.05);
                box-shadow: 0 4px 12px {theme.shadow};
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