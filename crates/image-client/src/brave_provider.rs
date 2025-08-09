use async_trait::async_trait;
use glossia_shared::{AppError, ImageResult};
use glossia_http_client::{EnhancedHttpClient, HttpClient};
use crate::{ImageClient, ImageClientConfig};
use serde_json::Value;
use urlencoding::encode;
use tracing::{info, warn, debug, instrument};

/// Brave Search API provider for image search
pub struct BraveProvider {
    client: EnhancedHttpClient,
    config: ImageClientConfig,
}

impl BraveProvider {
    pub fn new(config: ImageClientConfig) -> Result<Self, AppError> {
        config.validate()?;

        let mut client = EnhancedHttpClient::new()?
            .with_timeout(config.timeout);

        // Add API key as header if provided
        if let Some(ref api_key) = config.api_key {
            let mut headers = std::collections::HashMap::new();
            headers.insert("X-Subscription-Token".to_string(), api_key.clone());
            client = client.with_headers(headers);
        }

        Ok(Self {
            client,
            config,
        })
    }

    fn build_search_url(&self, query: &str, count: usize) -> String {
        let encoded_query = encode(query);
        let clamped_count = self.config.clamp_count(Some(count));
        
        format!(
            "https://api.search.brave.com/res/v1/images/search?q={encoded_query}&count={clamped_count}"
        )
    }

    fn parse_brave_response(&self, response: Value) -> Result<Vec<ImageResult>, AppError> {
        let results = response["results"]
            .as_array()
            .ok_or_else(|| AppError::api_error("Invalid response format from Brave Search"))?;

        let images = results
            .iter()
            .filter_map(|item| {
                // Try to get URL from different possible locations in the API response
                let url = item["properties"]["url"].as_str()
                    .or_else(|| item["url"].as_str())
                    .or_else(|| item["src"].as_str())?
                    .to_string();
                let title = item["title"].as_str().unwrap_or("Untitled").to_string();
                let thumbnail_url = item["thumbnail"]["src"].as_str()
                    .unwrap_or(&url)
                    .to_string();

                Some(ImageResult {
                    url,
                    title,
                    thumbnail_url,
                    width: None,
                    height: None,
                })
            })
            .collect();

        Ok(images)
    }
}

#[async_trait]
impl ImageClient for BraveProvider {
    #[instrument(skip(self), fields(query = query, count = count))]
    async fn search_images(&self, query: &str, count: Option<usize>) -> Result<Vec<ImageResult>, AppError> {
        info!("Searching images for query: '{}'", query);
        
        if query.trim().is_empty() {
            warn!("Empty search query provided");
            return Err(AppError::api_error("Search query cannot be empty"));
        }

        let count = self.config.clamp_count(count);
        let url = self.build_search_url(query, count);
        
        debug!("Brave search URL: {}", url);

        let response: Value = self.client.get_json(&url).await?;
        let results = self.parse_brave_response(response)?;
        
        info!("Found {} images for query: '{}'", results.len(), query);
        Ok(results)
    }

    fn provider_name(&self) -> &str {
        "Brave"
    }

    async fn health_check(&self) -> Result<(), AppError> {
        // Do a minimal search to test if the API is working
        let test_results = self.search_images("test", Some(1)).await?;
        
        if test_results.is_empty() {
            Err(AppError::api_error("Brave Search API returned no results for test query"))
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_brave_provider_creation() {
        let config = ImageClientConfig::new(crate::ImageProvider::Brave)
            .with_api_key("test_key".to_string());
        let provider = BraveProvider::new(config);
        assert!(provider.is_ok());
    }

    #[test]
    fn test_build_search_url() {
        let config = ImageClientConfig::new(crate::ImageProvider::Brave)
            .with_api_key("test_key".to_string());
        let provider = BraveProvider::new(config).unwrap();
        
        let url = provider.build_search_url("test query", 5);
        assert!(url.contains("test%20query"));
        assert!(url.contains("count=5"));
    }

    #[test]
    fn test_parse_brave_response() {
        let config = ImageClientConfig::new(crate::ImageProvider::Brave)
            .with_api_key("test_key".to_string());
        let provider = BraveProvider::new(config).unwrap();
        
        let mock_response = serde_json::json!({
            "results": [
                {
                    "url": "https://example.com/image1.jpg",
                    "title": "Test Image 1",
                    "thumbnail": {
                        "src": "https://example.com/thumb1.jpg"
                    },
                    "properties": {
                        "url": "https://example.com/image1.jpg"
                    }
                },
                {
                    "url": "https://example.com/image2.jpg",
                    "title": "Test Image 2",
                    "thumbnail": {
                        "src": "https://example.com/thumb2.jpg"
                    },
                    "properties": {
                        "url": "https://example.com/image2.jpg"
                    }
                }
            ]
        });

        let results = provider.parse_brave_response(mock_response).unwrap();
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].title, "Test Image 1");
        assert_eq!(results[1].url, "https://example.com/image2.jpg");
    }

    #[test]
    fn test_parse_brave_response_missing_thumbnail() {
        let config = ImageClientConfig::new(crate::ImageProvider::Brave)
            .with_api_key("test_key".to_string());
        let provider = BraveProvider::new(config).unwrap();
        
        let mock_response = serde_json::json!({
            "results": [
                {
                    "url": "https://example.com/image1.jpg",
                    "title": "Test Image 1",
                    "properties": {
                        "url": "https://example.com/image1.jpg"
                    }
                    // No thumbnail field
                }
            ]
        });

        let results = provider.parse_brave_response(mock_response).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].thumbnail_url, "https://example.com/image1.jpg"); // Should fallback to main URL
    }
}
