use dioxus::prelude::*;
use crate::theme::Theme;

#[component]
pub fn WordMeaningSkeleton(theme: Theme, count: usize) -> Element {
    rsx! {
        style {
            "
            @keyframes shimmer {{
                0% {{
                    background-position: -200% 0;
                }}
                100% {{
                    background-position: 200% 0;
                }}
            }}
            
            .skeleton {{
                background: linear-gradient(
                    90deg,
                    {theme.hover_bg} 25%,
                    {theme.surface} 50%,
                    {theme.hover_bg} 75%
                );
                background-size: 200% 100%;
                animation: shimmer 2s infinite;
                border-radius: 4px;
            }}
            
            .skeleton-item {{
                display: flex;
                flex-direction: column;
                padding: 12px 16px;
                border-bottom: 1px solid {theme.border};
            }}
            
            .skeleton-header {{
                display: flex;
                align-items: flex-start;
                gap: 12px;
            }}
            
            .skeleton-word {{
                width: 80px;
                height: 24px;
                border-radius: 16px;
                flex-shrink: 0;
            }}
            
            .skeleton-meaning {{
                flex: 1;
                display: flex;
                flex-direction: column;
                gap: 6px;
            }}
            
            .skeleton-line {{
                height: 14px;
                border-radius: 4px;
            }}
            
            .skeleton-progress {{
                width: 60px;
                height: 20px;
                margin-left: auto;
                border-radius: 4px;
            }}
            "
        }
        
        div {
            class: "skeleton-container",
            
            for i in 0..count {
                div {
                    class: "skeleton-item",
                    key: "{i}",
                    
                    div {
                        class: "skeleton-header",
                        
                        // Word label skeleton
                        div {
                            class: "skeleton skeleton-word",
                        }
                        
                        // Meaning text skeleton
                        div {
                            class: "skeleton-meaning",
                            
                            div {
                                class: "skeleton skeleton-line",
                                style: "width: 100%;",
                            }
                            div {
                                class: "skeleton skeleton-line",
                                style: if i % 2 == 0 { "width: 75%;" } else { "width: 90%;" },
                            }
                        }
                        
                        // Progress indicator skeleton (show randomly)
                        if i % 3 != 0 {
                            div {
                                class: "skeleton skeleton-progress",
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn SimplifiedTextSkeleton(theme: Theme) -> Element {
    rsx! {
        style {
            "
            .text-skeleton {{
                display: flex;
                flex-direction: column;
                gap: 8px;
                width: 100%;
            }}
            
            .text-skeleton-line {{
                height: 18px;
                background: linear-gradient(
                    90deg,
                    {theme.hover_bg} 25%,
                    {theme.surface} 50%,
                    {theme.hover_bg} 75%
                );
                background-size: 200% 100%;
                animation: shimmer 2s infinite;
                border-radius: 4px;
            }}
            "
        }
        
        div {
            class: "text-skeleton",
            
            div {
                class: "text-skeleton-line",
                style: "width: 95%;",
            }
            div {
                class: "text-skeleton-line",
                style: "width: 85%;",
            }
            div {
                class: "text-skeleton-line",
                style: "width: 75%;",
            }
        }
    }
}

#[component]
pub fn ImageGallerySkeleton(theme: Theme) -> Element {
    rsx! {
        style {
            "
            .image-gallery-skeleton {{
                margin-top: 16px;
                background: {theme.gallery_bg};
                border-radius: 8px;
            }}
            
            .skeleton-images-grid {{
                display: flex;
                gap: 12px;
                overflow-x: hidden;
                padding: 4px;
            }}
            
            .skeleton-image-item {{
                position: relative;
                background: {theme.surface};
                border-radius: 8px;
                overflow: hidden;
                flex: 0 0 150px;
                height: 150px;
            }}
            
            .skeleton-image-placeholder {{
                width: 150px;
                height: 150px;
                background: linear-gradient(
                    90deg,
                    {theme.hover_bg} 25%,
                    {theme.surface} 50%,
                    {theme.hover_bg} 75%
                );
                background-size: 200% 100%;
                animation: shimmer 2s infinite;
            }}
            "
        }
        
        div {
            class: "image-gallery-skeleton",
            
            div {
                class: "skeleton-images-grid",
                
                // Show 3 skeleton image items
                for i in 0..3 {
                    div {
                        class: "skeleton-image-item",
                        key: "skeleton-{i}",
                        
                        div {
                            class: "skeleton-image-placeholder",
                        }
                    }
                }
            }
        }
    }
}
