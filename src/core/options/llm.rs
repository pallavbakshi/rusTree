//! Basic LLM configuration *types* that are required by the core crate.
//!
//! Higher-level convenience functions that translate CLI arguments or config
//! files into these structures live in the higher-level `src/config/llm.rs`
//! module.  Those
//! helpers depend on the CLI layer and therefore cannot reside in the core
//! crate.

use std::str::FromStr;
use std::time::Duration;

/// Supported LLM providers.
///
/// Each provider has specific default models and API key environment
/// variables.  The provider determines which API endpoint and authentication
/// method to use.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LlmProvider {
    /// OpenAI GPT models (gpt-4, gpt-3.5-turbo, …)
    OpenAi,
    /// Anthropic Claude models (claude-3-sonnet, …)
    Anthropic,
    /// Cohere Command models (command-r, …)
    Cohere,
    /// OpenRouter unified API for various models.
    OpenRouter,
}

impl LlmProvider {
    /// Returns the default model for this provider.
    pub fn default_model(&self) -> &'static str {
        match self {
            LlmProvider::OpenAi => "gpt-4",
            LlmProvider::Anthropic => "claude-3-sonnet-20240229",
            LlmProvider::Cohere => "command-r",
            LlmProvider::OpenRouter => "openai/gpt-4",
        }
    }

    /// Returns the environment variable name for this provider's API key.
    pub fn env_var(&self) -> &'static str {
        match self {
            LlmProvider::OpenAi => "OPENAI_API_KEY",
            LlmProvider::Anthropic => "ANTHROPIC_API_KEY",
            LlmProvider::Cohere => "COHERE_API_KEY",
            LlmProvider::OpenRouter => "OPENROUTER_API_KEY",
        }
    }

    /// Returns the canonical name of this provider (lower-case).
    pub fn name(&self) -> &'static str {
        match self {
            LlmProvider::OpenAi => "openai",
            LlmProvider::Anthropic => "anthropic",
            LlmProvider::Cohere => "cohere",
            LlmProvider::OpenRouter => "openrouter",
        }
    }
}

impl FromStr for LlmProvider {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "openai" => Ok(LlmProvider::OpenAi),
            "anthropic" => Ok(LlmProvider::Anthropic),
            "cohere" => Ok(LlmProvider::Cohere),
            "openrouter" => Ok(LlmProvider::OpenRouter),
            _ => Err(format!("Invalid provider: {}", s)),
        }
    }
}

/// Error types for LLM configuration.
#[derive(Debug, thiserror::Error)]
pub enum LlmConfigError {
    #[error(
        "Missing API key for provider '{provider}'. Set {env_var} environment variable or use --llm-api-key"
    )]
    MissingApiKey { provider: String, env_var: String },

    #[error("Invalid temperature {temp}. Must be between 0.0 and 2.0")]
    InvalidTemperature { temp: f32 },

    #[error("Invalid max tokens {tokens}. Must be between 1 and 32000")]
    InvalidMaxTokens { tokens: u32 },

    #[error("Invalid provider: {provider}")]
    InvalidProvider { provider: String },
}

/// Configuration for LLM integration.
#[derive(Debug, Clone, Default)]
pub struct LlmOptions {
    // Operational flags
    pub enabled: bool,
    pub export_mode: bool,
    pub direct_query_mode: bool,

    // Provider configuration
    pub provider: Option<LlmProvider>,
    pub model: Option<String>,
    pub api_key: Option<String>,
    pub endpoint: Option<String>,
    pub temperature: Option<f32>,
    pub max_tokens: Option<u32>,
    pub timeout: Option<Duration>,
}

/* ----------------------------------------------------------------------- */
/*  Note on helper functions                                                */
/* ----------------------------------------------------------------------- */
// Helper methods that *parse* CLI flags or environment variables are **not**
// implemented here because they would introduce a dependency on the higher
// layers (config/CLI).  See `src/config/llm.rs` for those convenience
// constructors.
