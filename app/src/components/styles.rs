use dioxus::prelude::*;

#[component]
pub fn WordMeaningsStyles() -> Element {
    rsx! {
        style {
            "
            .word-meanings-container {{
                margin-top: 24px;
                width: 80%;
                padding: 40px;
                background: #ffffff;
                max-width: 700px;
            }}
            
            .meanings-list {{
                background: #ffffff;
                overflow: hidden;
            }}
            
            .meaning-item {{
                display: flex;
                flex-direction: column;
                padding: 12px 16px;
                transition: background-color 0.15s ease;
            }}
            
            .meaning-item.border-bottom {{
                border-bottom: 1px solid #f1f3f4;
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
                box-shadow: 0 1px 3px rgba(0,0,0,0.15);
            }}
            
            .meaning-definition {{
                color: #495057;
                line-height: 1.4;
                font-size: 0.9em;
                flex: 1;
            }}
            
            .expand-icon {{
                margin-left: 8px;
                color: #6c757d;
                font-size: 0.8em;
                transition: transform 0.2s ease;
            }}
            
            .expand-icon.expanded {{
                transform: rotate(180deg);
            }}
            
            .image-gallery {{
                margin-top: 16px;
                padding: 16px;
                background: #f8f9fa;
                border-radius: 8px;
                border: 1px solid #e9ecef;
            }}
            
            .gallery-message {{
                text-align: center;
                color: #6c757d;
                font-style: italic;
            }}
            
            .gallery-message.error {{
                color: #dc3545;
            }}
            
            .images-grid {{
                display: grid;
                grid-template-columns: repeat(auto-fit, minmax(120px, 1fr));
                gap: 12px;
                max-width: 100%;
            }}
            
            .image-item {{
                position: relative;
                background: white;
                border-radius: 6px;
                overflow: hidden;
                box-shadow: 0 2px 4px rgba(0,0,0,0.1);
                transition: transform 0.2s ease;
            }}
            
            .image-item:hover {{
                transform: translateY(-2px);
                box-shadow: 0 4px 8px rgba(0,0,0,0.15);
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
