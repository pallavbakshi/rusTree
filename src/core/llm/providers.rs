//! # LLM Provider Configuration
//!
//! This module contains configuration types and logic for different LLM providers.
//! It handles provider-specific settings, API key management, and configuration validation.
//!
//! ## Supported Providers
//!
//! - **OpenAI**: GPT models (gpt-4, gpt-3.5-turbo, etc.)
//! - **Anthropic**: Claude models (claude-3-sonnet, claude-3-haiku, etc.)
//! - **Cohere**: Command models (command-r, command-r-plus, etc.)
//! - **OpenRouter**: Various models through OpenRouter's unified API
//!
//! ## Configuration Priority
//!
//! API keys are resolved in the following order (highest to lowest priority):
//! 1. CLI argument (`--llm-api-key`)
//! 2. Environment variable (e.g., `OPENAI_API_KEY`)
//! 3. `.env` file (same variable names)
//!
//! ## Examples
//!
//! ```rust,no_run
//! use rustree::core::llm::{LlmProvider, LlmConfig};
//! use rustree::cli::llm::LlmArgs;
//! use std::str::FromStr;
//!
//! // Parse provider from string
//! let provider = LlmProvider::from_str("openai")?;
//! assert_eq!(provider.default_model(), "gpt-4");
//! assert_eq!(provider.env_var(), "OPENAI_API_KEY");
//!
//! // Create configuration from CLI args
//! let args = LlmArgs {
//!     llm_provider: "anthropic".to_string(),
//!     llm_model: Some("claude-3-sonnet".to_string()),
//!     llm_temperature: Some(0.7),
//!     ..Default::default()
//! };
//!
//! # unsafe { std::env::set_var("ANTHROPIC_API_KEY", "test-key"); }
//! let config = LlmConfig::from_cli_args(&args)?;
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

use crate::core::llm::error::LlmError;
use std::str::FromStr;
use std::time::Duration;

/// Supported LLM providers
///
/// Each provider has specific default models and API key environment variables.
/// The provider determines which API endpoint and authentication method to use.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LlmProvider {
    /// OpenAI GPT models (gpt-4, gpt-3.5-turbo, etc.)
    /// Requires: OPENAI_API_KEY environment variable
    OpenAi,

    /// Anthropic Claude models (claude-3-sonnet, claude-3-haiku, etc.)
    /// Requires: ANTHROPIC_API_KEY environment variable
    Anthropic,

    /// Cohere Command models (command-r, command-r-plus, etc.)
    /// Requires: COHERE_API_KEY environment variable
    Cohere,

    /// OpenRouter unified API for various models
    /// Requires: OPENROUTER_API_KEY environment variable
    OpenRouter,
}

impl LlmProvider {
    /// Returns the default model for this provider
    ///
    /// These are sensible defaults that work well for code analysis tasks.
    /// Users can override these with the `--llm-model` CLI argument.
    pub fn default_model(&self) -> &'static str {
        match self {
            LlmProvider::OpenAi => "gpt-4",
            LlmProvider::Anthropic => "claude-3-sonnet-20240229",
            LlmProvider::Cohere => "command-r",
            LlmProvider::OpenRouter => "openai/gpt-4",
        }
    }

    /// Returns the environment variable name for this provider's API key
    ///
    /// These environment variables are checked automatically when creating
    /// an [`LlmConfig`]. They can also be defined in a `.env` file.
    pub fn env_var(&self) -> &'static str {
        match self {
            LlmProvider::OpenAi => "OPENAI_API_KEY",
            LlmProvider::Anthropic => "ANTHROPIC_API_KEY",
            LlmProvider::Cohere => "COHERE_API_KEY",
            LlmProvider::OpenRouter => "OPENROUTER_API_KEY",
        }
    }

    /// Returns the string name of this provider
    ///
    /// This is used for CLI argument parsing and error messages.
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
    type Err = LlmError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "openai" => Ok(LlmProvider::OpenAi),
            "anthropic" => Ok(LlmProvider::Anthropic),
            "cohere" => Ok(LlmProvider::Cohere),
            "openrouter" => Ok(LlmProvider::OpenRouter),
            _ => Err(LlmError::InvalidProvider {
                provider: s.to_string(),
            }),
        }
    }
}

/// Configuration for LLM operations
///
/// This structure contains all the settings needed to communicate with an LLM provider.
/// It includes authentication, model selection, and generation parameters.
///
/// # Example
///
/// ```rust,no_run
/// use rustree::core::llm::LlmConfig;
/// use rustree::cli::llm::LlmArgs;
///
/// let args = LlmArgs {
///     llm_provider: "openai".to_string(),
///     llm_model: Some("gpt-4".to_string()),
///     llm_temperature: Some(0.7),
///     llm_max_tokens: Some(1000),
///     ..Default::default()
/// };
///
/// # unsafe { std::env::set_var("OPENAI_API_KEY", "test-key"); }
/// let config = LlmConfig::from_cli_args(&args)?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
#[derive(Debug, Clone)]
pub struct LlmConfig {
    /// The LLM provider to use (OpenAI, Anthropic, etc.)
    pub provider: LlmProvider,

    /// The specific model to use (e.g., "gpt-4", "claude-3-sonnet")
    pub model: String,

    /// API key for authentication with the provider
    pub api_key: String,

    /// Custom endpoint URL (optional, for self-hosted or proxy services)
    /// Currently supported for: OpenAI (custom endpoints), OpenRouter (default: openrouter.ai)
    /// Not yet supported for: Anthropic, Cohere (will return error if specified)
    pub endpoint: Option<String>,

    /// Temperature for response randomness (0.0 = deterministic, 2.0 = very random)
    pub temperature: f32,

    /// Maximum number of tokens in the response
    pub max_tokens: u32,

    /// Timeout for API requests
    pub timeout: Duration,
}

impl LlmConfig {
    /// Creates an LLM configuration from CLI arguments
    ///
    /// This method handles the complete configuration flow:
    /// 1. Loads `.env` file if present
    /// 2. Resolves API key from CLI args, environment, or .env file
    /// 3. Validates all configuration parameters
    /// 4. Sets sensible defaults for optional parameters
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - No API key is found for the specified provider
    /// - Temperature is outside the valid range (0.0-2.0)
    /// - Max tokens is outside the valid range (1-32000)
    /// - Provider name is invalid
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use rustree::core::llm::LlmConfig;
    /// use rustree::cli::llm::LlmArgs;
    ///
    /// let args = LlmArgs {
    ///     llm_provider: "anthropic".to_string(),
    ///     llm_temperature: Some(0.3),
    ///     ..Default::default()
    /// };
    ///
    /// # unsafe { std::env::set_var("ANTHROPIC_API_KEY", "test-key"); }
    /// let config = LlmConfig::from_cli_args(&args)?;
    /// assert_eq!(config.temperature, 0.3);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn from_cli_args(args: &crate::cli::llm::LlmArgs) -> Result<Self, LlmError> {
        // Try to load .env file (silently ignore if not found)
        let _ = dotenvy::dotenv();

        let provider = LlmProvider::from_str(&args.llm_provider)?;

        // Get API key from CLI args, environment variables, or .env file
        let api_key = args
            .llm_api_key
            .clone()
            .or_else(|| std::env::var(provider.env_var()).ok())
            .ok_or_else(|| {
                eprintln!("💡 Tip: Create a .env file in your project root with:");
                eprintln!("   {}=your-api-key-here", provider.env_var());
                LlmError::MissingApiKey {
                    provider: provider.name().to_string(),
                    env_var: provider.env_var().to_string(),
                }
            })?;

        // Use provided model or default
        let model = args
            .llm_model
            .clone()
            .unwrap_or_else(|| provider.default_model().to_string());

        // Validate temperature
        let temperature = args.llm_temperature.unwrap_or(0.7);
        if !(0.0..=2.0).contains(&temperature) {
            return Err(LlmError::InvalidTemperature { temp: temperature });
        }

        // Validate max tokens
        let max_tokens = args.llm_max_tokens.unwrap_or(1000);
        if !(1..=32000).contains(&max_tokens) {
            return Err(LlmError::InvalidMaxTokens { tokens: max_tokens });
        }

        Ok(LlmConfig {
            provider,
            model,
            api_key,
            endpoint: args.llm_endpoint.clone(),
            temperature,
            max_tokens,
            timeout: Duration::from_secs(60),
        })
    }

    /// Generate a sample .env file content with all supported API keys
    pub fn generate_sample_env_file() -> String {
        "# LLM API Keys for rustree\n\
         # Uncomment and add your API keys as needed\n\n\
         # OpenAI\n\
         # OPENAI_API_KEY=sk-your-openai-key-here\n\n\
         # Anthropic\n\
         # ANTHROPIC_API_KEY=sk-your-anthropic-key-here\n\n\
         # Cohere\n\
         # COHERE_API_KEY=your-cohere-key-here\n\n\
         # OpenRouter\n\
         # OPENROUTER_API_KEY=sk-your-openrouter-key-here\n"
            .to_string()
    }
}
