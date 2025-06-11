// src/core/llm/error.rs

use thiserror::Error;

#[derive(Debug, Error)]
pub enum LlmError {
    #[error(
        "API key not found for provider '{provider}'. Set {env_var} environment variable or add to .env file"
    )]
    MissingApiKey { provider: String, env_var: String },

    #[error("Invalid model '{model}' for provider '{provider}'")]
    InvalidModel { model: String, provider: String },

    #[error("Invalid provider '{provider}'. Supported: openai, anthropic, cohere, openrouter")]
    InvalidProvider { provider: String },

    #[error("Network error: {0}")]
    Network(String),

    #[error("Provider API error: {0}")]
    Api(String),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Timeout after {seconds}s")]
    Timeout { seconds: u64 },

    #[error("Invalid temperature {temp}. Must be between 0.0 and 2.0")]
    InvalidTemperature { temp: f32 },

    #[error("Invalid max tokens {tokens}. Must be between 1 and 32000")]
    InvalidMaxTokens { tokens: u32 },

    #[error("Rig client error: {0}")]
    RigClient(String),

    #[error("Unsupported feature: {0}")]
    UnsupportedFeature(String),
}
