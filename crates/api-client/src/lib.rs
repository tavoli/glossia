use reqwest::Client;
use serde::Deserialize;
use serde_json::json;
use glossia_shared::{AppError, SimplificationRequest, SimplificationResponse};
use std::time::Duration;

// Represents the outer shell of the OpenRouter API response
#[derive(Debug, Deserialize)]
struct OpenRouterResponse {
    choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    message: Message,
}

#[derive(Debug, Deserialize)]
struct Message {
    content: String,
}

#[derive(Clone)]
pub struct OpenRouterClient {
    client: Client,
    api_key: String,
    base_url: String,
}

impl Default for OpenRouterClient {
    fn default() -> Self {
        Self::new()
    }
}

impl OpenRouterClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            // IMPORTANT: Load from config/env in a real app
            api_key: "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".to_string(),
            base_url: "https://openrouter.ai/api/v1".to_string(),
        }
    }

    pub async fn simplify(&self, request: SimplificationRequest) -> Result<SimplificationResponse, AppError> {
        println!("Starting API simplify call for: '{}'", request.sentence);
        
        let prompt = self.build_prompt(&request.sentence);
        println!("Built prompt: {}", prompt);
        
        let body = json!({
            "model": "qwen/qwen3-coder:free",
            "messages": [{"role": "user", "content": prompt}],
            "temperature": 0.3,
            "response_format": { "type": "json_object" }
        });
        println!("Request body: {}", serde_json::to_string_pretty(&body).unwrap_or_default());

        // Basic retry logic
        let mut attempts = 0;
        loop {
            println!("Attempt {} - Making HTTP request to: {}/chat/completions", attempts + 1, self.base_url);
            
            let response = self.client
                .post(format!("{}/chat/completions", self.base_url))
                .header("Authorization", format!("Bearer {}", self.api_key))
                .header("Content-Type", "application/json")
                .json(&body)
                .send()
                .await;

            match response {
                Ok(res) => {
                    println!("Received response with status: {}", res.status());
                    
                    if res.status().is_success() {
                        println!("Success response, parsing JSON...");
                        let openrouter_response = res.json::<OpenRouterResponse>().await?;
                        let content = openrouter_response.choices.get(0)
                            .map(|c| c.message.content.clone())
                            .ok_or(AppError::InvalidResponseContent)?;
                        
                        println!("API Response content: {}", content);
                        
                        let simplified: SimplificationResponse = serde_json::from_str(&content)?;
                        println!("Successfully parsed response");
                        return Ok(simplified);
                    } else {
                        let status = res.status();
                        let error_text = res.text().await.unwrap_or_default();
                        println!("API returned error status {}: {}", status, error_text);
                        return Err(AppError::ApiError(format!("HTTP {}: {}", status, error_text)));
                    }
                },
                Err(e) => {
                    println!("Request failed: {}", e);
                    if attempts >= 3 {
                        return Err(AppError::ApiError(e.to_string()));
                    }
                    attempts += 1;
                    println!("Retrying in {} seconds...", attempts);
                    tokio::time::sleep(Duration::from_secs(attempts)).await;
                }
            }
        }
    }
    
    fn build_prompt(&self, sentence: &str) -> String {
        format!(
            r#"Simplify this sentence in modern English without losing detail. 
            Also provide B2-level meanings for difficult words.
            
            Respond ONLY with valid JSON in this exact format:
            {{
                "original": "{}",
                "simplified": "the simplified version",
                "words": [
                    {{"word": "difficult_word", "meaning": "B2-level explanation"}}
                ]
            }}
            
            Sentence to simplify: "{}"
            "#,
            sentence, sentence
        )
    }
}
