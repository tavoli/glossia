mod openai_provider;
mod claude_provider;
mod llm_trait;
mod config;
mod factory;

pub use openai_provider::OpenAIProvider;
pub use claude_provider::ClaudeProvider;
pub use llm_trait::{LLMClient, MockLLMClient};
pub use config::{LLMConfig, ProviderType};
pub use factory::LLMClientFactory;

// Re-export commonly used types
pub use glossia_shared::{
    SimplificationRequest, SimplificationResponse, 
    ImageQueryOptimizationRequest, ImageQueryOptimizationResponse,
    AppError
};
