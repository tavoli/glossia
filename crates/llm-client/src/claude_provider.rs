use async_trait::async_trait;
use glossia_shared::{AppError, SimplificationRequest, SimplificationResponse, ImageQueryOptimizationRequest, ImageQueryOptimizationResponse, WordMeaning};
use glossia_http_client::{EnhancedHttpClient, HttpClient};
use crate::{LLMClient, LLMConfig};
use serde_json::{json, Value};
use std::collections::HashMap;

/// Claude provider implementation (Anthropic)
pub struct ClaudeProvider {
    client: EnhancedHttpClient,
    config: LLMConfig,
}

impl ClaudeProvider {
    pub fn new(config: LLMConfig) -> Result<Self, AppError> {
        config.validate()?;

        let api_key = config.api_key.as_ref()
            .ok_or_else(|| AppError::config_error("Claude API key is required"))?;

        let mut headers = HashMap::new();
        headers.insert("x-api-key".to_string(), api_key.clone());
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        headers.insert("anthropic-version".to_string(), "2023-06-01".to_string());

        let client = EnhancedHttpClient::new()?
            .with_headers(headers)
            .with_timeout(config.timeout);

        Ok(Self {
            client,
            config,
        })
    }

    fn get_base_url(&self) -> String {
        self.config.base_url.clone()
            .unwrap_or_else(|| "https://api.anthropic.com/v1".to_string())
    }

    fn get_model(&self) -> String {
        self.config.model.clone()
            .unwrap_or_else(|| "claude-3-haiku-20240307".to_string())
    }

    async fn make_completion_request(&self, prompt: &str) -> Result<String, AppError> {
        let url = format!("{}/messages", self.get_base_url());
        
        let mut request_body = json!({
            "model": self.get_model(),
            "max_tokens": self.config.max_tokens.unwrap_or(1024),
            "messages": [
                {
                    "role": "user",
                    "content": prompt
                }
            ]
        });

        // Add optional parameters
        if let Some(temperature) = self.config.temperature {
            request_body["temperature"] = json!(temperature);
        }

        let response: Value = self.client.post_json(&url, request_body).await?;

        // Extract the response content
        let content = response["content"][0]["text"]
            .as_str()
            .ok_or_else(|| AppError::api_error("Invalid response format from Claude"))?;

        Ok(content.to_string())
    }

    fn parse_simplification_response(&self, content: &str, original: &str) -> Result<SimplificationResponse, AppError> {
        // Try to parse as JSON first
        if let Ok(parsed) = serde_json::from_str::<Value>(content) {
            let simplified = parsed["simplified"]
                .as_str()
                .unwrap_or(original)
                .to_string();

            let words = if let Some(words_array) = parsed["words"].as_array() {
                words_array.iter()
                    .filter_map(|word_obj| {
                        let word = word_obj["word"].as_str()?;
                        let meaning = word_obj["meaning"].as_str()?;
                        let is_phrase = word_obj["is_phrase"].as_bool().unwrap_or(false);
                        
                        Some(WordMeaning {
                            word: word.to_string(),
                            meaning: meaning.to_string(),
                            is_phrase,
                            timestamp: None,
                        })
                    })
                    .collect()
            } else {
                Vec::new()
            };

            Ok(SimplificationResponse {
                original: original.to_string(),
                simplified,
                words,
            })
        } else {
            // Fallback: treat entire response as simplified text
            Ok(SimplificationResponse {
                original: original.to_string(),
                simplified: content.to_string(),
                words: Vec::new(),
            })
        }
    }
}

#[async_trait]
impl LLMClient for ClaudeProvider {
    async fn simplify(&self, request: SimplificationRequest) -> Result<SimplificationResponse, AppError> {
        let prompt = format!(
            "You are a helpful assistant that simplifies text and identifies difficult words. \
            Respond with JSON in this format: {{\"simplified\": \"simplified text\", \"words\": [{{\"word\": \"word\", \"meaning\": \"definition\", \"is_phrase\": false}}]}}\n\n\
            Simplify this sentence and identify difficult words: {}",
            request.sentence
        );

        let response_content = self.make_completion_request(&prompt).await?;
        self.parse_simplification_response(&response_content, &request.sentence)
    }

    async fn get_word_meaning(&self, word: &str, context: &str) -> Result<String, AppError> {
        let prompt = format!(
            "What does the word '{word}' mean in this context: '{context}'? Provide a brief definition."
        );

        self.make_completion_request(&prompt).await
    }

    async fn optimize_image_query(&self, request: ImageQueryOptimizationRequest) -> Result<ImageQueryOptimizationResponse, AppError> {
        let prompt = format!(
            "Optimize this word for image search: '{}'. Context: '{}'. \
            Make it more specific and visual. Respond with just the optimized query.",
            request.word,
            request.sentence_context
        );

        let optimized_query = self.make_completion_request(&prompt).await?;
        
        Ok(ImageQueryOptimizationResponse {
            optimized_query: optimized_query.trim().to_string(),
        })
    }

    fn provider_name(&self) -> &str {
        "Claude"
    }

    async fn health_check(&self) -> Result<(), AppError> {
        // Claude doesn't have a simple health check endpoint like OpenAI
        // We'll do a minimal completion request instead
        let prompt = "Hello";
        let _response = self.make_completion_request(prompt).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{LLMConfig, ProviderType};

    #[tokio::test]
    async fn test_claude_provider_creation() {
        let config = LLMConfig::new(ProviderType::Claude)
            .with_api_key("test-key".to_string());

        let provider = ClaudeProvider::new(config);
        assert!(provider.is_ok());
    }

    #[tokio::test]
    async fn test_claude_provider_without_api_key() {
        let config = LLMConfig::new(ProviderType::Claude);
        let provider = ClaudeProvider::new(config);
        assert!(provider.is_err());
    }
}
