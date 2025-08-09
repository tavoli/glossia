use async_trait::async_trait;
use glossia_shared::{AppError, SimplificationRequest, SimplificationResponse, ImageQueryOptimizationRequest, ImageQueryOptimizationResponse, WordMeaning};
use glossia_http_client::{EnhancedHttpClient, HttpClient};
use crate::{LLMClient, LLMConfig};
use serde_json::{json, Value};
use std::collections::HashMap;
use tracing::{info, debug, error, warn, instrument};

/// OpenAI provider implementation
pub struct OpenAIProvider {
    client: EnhancedHttpClient,
    config: LLMConfig,
}

impl OpenAIProvider {
    pub fn new(config: LLMConfig) -> Result<Self, AppError> {
        info!("Creating OpenAI provider with config: provider={:?}", config.provider);
        debug!("Config validation...");
        config.validate()?;

        let api_key = config.api_key.as_ref()
            .ok_or_else(|| AppError::config_error("OpenAI API key is required"))?;

        info!("OpenAI API key found (length: {})", api_key.len());
        debug!("API key prefix: {}", &api_key[..std::cmp::min(api_key.len(), 10)]);

        let mut headers = HashMap::new();
        headers.insert("Authorization".to_string(), format!("Bearer {api_key}"));
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        
        info!("HTTP headers configured, Authorization header added");

        let client = EnhancedHttpClient::new()?
            .with_timeout(config.timeout)
            .with_headers(headers);

        Ok(Self {
            client,
            config,
        })
    }

    fn get_base_url(&self) -> String {
        self.config.base_url.clone()
            .unwrap_or_else(|| "https://api.openai.com/v1".to_string())
    }

    fn get_model(&self) -> String {
        self.config.model.clone()
            .unwrap_or_else(|| "gpt-4o-mini".to_string())
    }


    #[instrument(skip(self, messages), fields(message_count = messages.len(), model = %self.get_model()))]
    async fn make_completion_request_with_json_format(&self, messages: Vec<Value>) -> Result<String, AppError> {
        let url = format!("{}/chat/completions", self.get_base_url());
        
        info!("Making OpenAI completion request with JSON format");
        debug!("Request URL: {}", url);
        
        let mut request_body = json!({
            "model": self.get_model(),
            "messages": messages,
            "response_format": { "type": "json_object" },
            "temperature": 1,
        });

        if let Some(max_tokens) = self.config.max_tokens {
            request_body["max_completion_tokens"] = json!(max_tokens);
        }

        let response: Value = self.client.post_json(&url, request_body.clone()).await
            .map_err(|e| {
                error!("OpenAI API request failed: {}", e);
                match &e {
                    AppError::AuthenticationError { .. } => {
                        AppError::authentication_error(
                            format!("OpenAI authentication failed. Please check your API key and ensure it's valid. Model: {}", self.get_model()),
                            None,
                            Some("invalid_api_key".to_string()),
                            None,
                        )
                    }
                    AppError::BadRequestError { message, .. } => {
                        AppError::bad_request_error(
                            format!("OpenAI request invalid: {}. Model: {}, URL: {}", message, self.get_model(), url),
                            Some("invalid_request".to_string()),
                            None,
                        )
                    }
                    _ => e
                }
            })?;

        let content = response["choices"][0]["message"]["content"]
            .as_str()
            .ok_or_else(|| {
                error!("Invalid OpenAI response format: missing content field");
                AppError::api_error("Invalid response format from OpenAI - missing content field")
            })?;

        info!("OpenAI completion successful, response length: {} chars", content.len());
        Ok(content.to_string())
    }

    #[instrument(skip(self, messages), fields(message_count = messages.len(), model = %self.get_model()))]
    async fn make_completion_request_with_options(&self, messages: Vec<Value>, temperature: Option<i8>, max_tokens: Option<i32>) -> Result<String, AppError> {
        let url = format!("{}/chat/completions", self.get_base_url());
        
        info!("Making OpenAI completion request with custom options");
        debug!("Request URL: {}", url);
        
        let mut request_body = json!({
            "model": self.get_model(),
            "messages": messages,
        });

        if let Some(temp) = temperature {
            request_body["temperature"] = json!(temp);
            debug!("Temperature: {}", temp);
        }

        if let Some(tokens) = max_tokens {
            request_body["max_completion_tokens"] = json!(tokens);
            debug!("Max tokens: {}", tokens);
        }

        let response: Value = self.client.post_json(&url, request_body.clone()).await
            .map_err(|e| {
                error!("OpenAI API request failed: {}", e);
                match &e {
                    AppError::AuthenticationError { .. } => {
                        AppError::authentication_error(
                            format!("OpenAI authentication failed. Please check your API key and ensure it's valid. Model: {}", self.get_model()),
                            None,
                            Some("invalid_api_key".to_string()),
                            None,
                        )
                    }
                    AppError::BadRequestError { message, .. } => {
                        AppError::bad_request_error(
                            format!("OpenAI request invalid: {}. Model: {}, URL: {}", message, self.get_model(), url),
                            Some("invalid_request".to_string()),
                            None,
                        )
                    }
                    _ => e
                }
            })?;

        let content = response["choices"][0]["message"]["content"]
            .as_str()
            .ok_or_else(|| {
                error!("Invalid OpenAI response format: missing content field");
                AppError::api_error("Invalid response format from OpenAI - missing content field")
            })?;

        info!("OpenAI completion successful, response length: {} chars", content.len());
        Ok(content.trim().to_string())
    }

    fn build_simplification_prompt(&self, sentence: &str) -> String {
        format!(
            r#"
You are a language assistant helping advanced English learners (3+ years experience) understand sophisticated text.

Simplify the sentence below using clear and modern English, without losing important meaning.

Then identify words AND phrases that would be challenging for learners with intermediate-advanced English (C1/C2 level). Focus ONLY on:
- Advanced academic vocabulary (sophisticated, nuanced terms)
- Professional/technical terminology
- Literary and formal expressions
- Complex idioms and phrasal verbs
- Sophisticated collocations
- Words rarely used in everyday conversation

DO NOT include basic or intermediate words that 3+ year learners already know (common verbs, everyday adjectives, basic prepositions, etc.).

For each challenging word or phrase, provide a clear definition using simpler English.

Respond ONLY in this exact JSON format:
{{
  "original": "{sentence}",
  "simplified": "the simplified version",
  "words": [
    {{ "word": "sophisticated_word", "meaning": "simple explanation", "is_phrase": false }},
    {{ "word": "complex phrasal expression", "meaning": "simple explanation", "is_phrase": true }}
  ]
}}

Sentence to analyze: "{sentence}"
"#,
            sentence = sentence.replace('"', "\\\"")
        )
    }

    fn build_word_meaning_prompt(&self, word: &str, context: &str) -> String {
        format!(
            r#"Define the word "{}" in simple English using maximum 15 words.

Context: "{}"

Provide a clear, concise definition that helps someone understand the word's meaning in this context.

Respond with ONLY the definition, no extra formatting or quotes."#,
            word, context
        )
    }

    fn build_image_optimization_prompt(&self, request: &ImageQueryOptimizationRequest) -> String {
        format!(
            r#"Generate an image search query for the word '{}' based on its contextual meaning.

Context: "{}"
Definition: {}

RULES:
1. If the word itself is already visually descriptive (e.g., "hermit", "lighthouse", "castle"), use it directly
2. Output ONLY valid JSON: {{"optimized_query": "your query"}}
3. Maximum 4 words
4. Add context words that enhance, not distract
5. AVOID extracting unrelated or inappropriate descriptors from context
6. Focus on the PRIMARY subject and its relevant setting

PROHIBITED:
- NO nudity, body parts, or clothing state descriptors (naked, nude, bare, etc.)
- NO sexual or suggestive content
- NO inappropriate physical descriptions

Examples:
- "hermits" + "sea hermits issuing from" → {{"optimized_query": "hermit on sea"}}
- "lighthouse" + "the old lighthouse keeper" → {{"optimized_query": "lighthouse coastal tower"}}
- "crown" + "heavy crown of responsibility" → {{"optimized_query": "royal crown gold"}}
- "bank" + "river bank was muddy" → {{"optimized_query": "river bank shore"}}

Word: '{}'
Context: '{}'
Meaning: {}"#,
            request.word, 
            request.sentence_context,
            request.word_meaning,
            request.word,
            request.sentence_context,
            request.word_meaning
        )
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
impl LLMClient for OpenAIProvider {
    #[instrument(skip(self), fields(sentence_length = request.sentence.len()))]
    async fn simplify(&self, request: SimplificationRequest) -> Result<SimplificationResponse, AppError> {
        info!("Simplifying sentence: {} chars", request.sentence.len());
        debug!("Sentence: {}", request.sentence);
        
        let prompt = self.build_simplification_prompt(&request.sentence);
        
        let messages = vec![
            json!({
                "role": "user",
                "content": prompt
            })
        ];

        let response_content = self.make_completion_request_with_json_format(messages).await?;
        let result = self.parse_simplification_response(&response_content, &request.sentence)?;
        
        info!("Simplification complete: {} words identified", result.words.len());
        Ok(result)
    }

    #[instrument(skip(self, context), fields(word = word, context_length = context.len()))]
    async fn get_word_meaning(&self, word: &str, context: &str) -> Result<String, AppError> {
        info!("Getting meaning for word: '{}'", word);
        debug!("Context: {}", context);
        
        let prompt = self.build_word_meaning_prompt(word, context);
        
        let messages = vec![
            json!({
                "role": "user",
                "content": prompt
            })
        ];

        let result = self.make_completion_request_with_options(messages, Some(1), Some(30)).await?;
        info!("Word meaning retrieved for: '{}'", word);
        Ok(result)
    }

    #[instrument(skip(self), fields(word = %request.word, context_length = request.sentence_context.len()))]
    async fn optimize_image_query(&self, request: ImageQueryOptimizationRequest) -> Result<ImageQueryOptimizationResponse, AppError> {
        info!("Optimizing image query for word: '{}'", request.word);
        debug!("Context: {}", request.sentence_context);
        let prompt = self.build_image_optimization_prompt(&request);
        
        let messages = vec![
            json!({
                "role": "user",
                "content": prompt
            })
        ];

        let response_content = self.make_completion_request_with_json_format(messages).await?;
        let optimization_response: ImageQueryOptimizationResponse = serde_json::from_str(&response_content)
            .map_err(|e| {
                error!("Failed to parse image query optimization response: {}", e);
                AppError::ParseError { message: format!("Invalid JSON response for image query optimization: {}", e) }
            })?;
        
        info!("Image query optimization complete for: '{}', optimized query: '{}'", request.word, optimization_response.optimized_query);
        Ok(optimization_response)
    }

    fn provider_name(&self) -> &str {
        "OpenAI"
    }

    #[instrument(skip(self), fields(provider = "OpenAI", model = %self.get_model()))]
    async fn health_check(&self) -> Result<(), AppError> {
        let url = format!("{}/models", self.get_base_url());
        info!("Performing OpenAI health check at: {}", url);
        
        let response: Value = self.client.get_json(&url).await
            .map_err(|e| {
                error!("OpenAI health check failed: {}", e);
                match &e {
                    AppError::AuthenticationError { .. } => {
                        AppError::authentication_error(
                            "OpenAI health check failed: Invalid API key or insufficient permissions",
                            None,
                            Some("invalid_api_key".to_string()),
                            None,
                        )
                    }
                    AppError::HttpError { status, message, .. } => {
                        AppError::api_error(
                            format!("OpenAI health check failed with HTTP {}: {}. Check your base URL and network connectivity.", status, message)
                        )
                    }
                    _ => AppError::api_error(format!("OpenAI health check failed: {}", e))
                }
            })?;

        // Verify the response contains expected data
        if let Some(data) = response.get("data") {
            if data.as_array().map_or(false, |arr| !arr.is_empty()) {
                info!("OpenAI health check successful - {} models available", data.as_array().unwrap().len());
                
                // Check if our configured model is available
                let target_model = self.get_model();
                let model_found = data.as_array().unwrap()
                    .iter()
                    .any(|model| {
                        model.get("id")
                            .and_then(|id| id.as_str())
                            .map_or(false, |id| id == target_model)
                    });
                
                if !model_found {
                    warn!("Configured model '{}' not found in available models. This may cause API requests to fail.", target_model);
                    debug!("Available models: {:?}", 
                        data.as_array().unwrap()
                            .iter()
                            .filter_map(|m| m.get("id").and_then(|id| id.as_str()))
                            .collect::<Vec<_>>()
                    );
                }
                
                Ok(())
            } else {
                Err(AppError::api_error("OpenAI health check returned empty model list"))
            }
        } else {
            Err(AppError::api_error("OpenAI health check returned unexpected response format"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{LLMConfig, ProviderType};

    #[tokio::test]
    async fn test_openai_provider_creation() {
        let config = LLMConfig::new(ProviderType::OpenAI)
            .with_api_key("test-key".to_string());

        let provider = OpenAIProvider::new(config);
        assert!(provider.is_ok());
    }

    #[tokio::test]
    async fn test_openai_provider_without_api_key() {
        let config = LLMConfig::new(ProviderType::OpenAI);
        let provider = OpenAIProvider::new(config);
        assert!(provider.is_err());
    }
}
