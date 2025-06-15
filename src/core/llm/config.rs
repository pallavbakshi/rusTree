//! Core LLM configuration types
//!
//! This module contains pure configuration types for the core LLM functionality.
//! These types have no dependencies on CLI or external modules and represent
//! the minimal configuration needed by the core LLM engine.

use std::time::Duration;

/// Pure core LLM configuration
///
/// This structure contains only the essential configuration data needed by the core
/// LLM functionality. It has no external dependencies and represents validated,
/// ready-to-use configuration values.
#[derive(Debug, Clone)]
pub struct CoreLlmConfig {
    /// The LLM provider to use
    pub provider: CoreLlmProvider,

    /// The specific model to use
    pub model: String,

    /// API key for authentication
    pub api_key: String,

    /// Custom endpoint URL (optional)
    pub endpoint: Option<String>,

    /// Temperature for response randomness (0.0-2.0)
    pub temperature: f32,

    /// Maximum number of tokens in the response
    pub max_tokens: u32,

    /// Timeout for API requests
    pub timeout: Duration,
}

/// Core LLM provider enum
///
/// This is a simplified version of the provider enum that contains only
/// the essential information needed by the core functionality.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CoreLlmProvider {
    /// OpenAI GPT models
    OpenAi,

    /// Anthropic Claude models
    Anthropic,

    /// Cohere Command models
    Cohere,

    /// OpenRouter unified API
    OpenRouter,
}

impl CoreLlmProvider {
    /// Returns the string name of this provider
    pub fn name(&self) -> &'static str {
        match self {
            CoreLlmProvider::OpenAi => "openai",
            CoreLlmProvider::Anthropic => "anthropic",
            CoreLlmProvider::Cohere => "cohere",
            CoreLlmProvider::OpenRouter => "openrouter",
        }
    }
}
