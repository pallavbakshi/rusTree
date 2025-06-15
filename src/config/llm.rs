// src/config/llm.rs

//! Configuration for LLM integration
//!
//! This module handles all LLM configuration including provider settings, API keys,
//! environment variable resolution, and validation. It serves as the bridge between
//! CLI arguments and the core LLM functionality.

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
    pub fn default_model(&self) -> &'static str {
        match self {
            LlmProvider::OpenAi => "gpt-4",
            LlmProvider::Anthropic => "claude-3-sonnet-20240229",
            LlmProvider::Cohere => "command-r",
            LlmProvider::OpenRouter => "openai/gpt-4",
        }
    }

    /// Returns the environment variable name for this provider's API key
    pub fn env_var(&self) -> &'static str {
        match self {
            LlmProvider::OpenAi => "OPENAI_API_KEY",
            LlmProvider::Anthropic => "ANTHROPIC_API_KEY",
            LlmProvider::Cohere => "COHERE_API_KEY",
            LlmProvider::OpenRouter => "OPENROUTER_API_KEY",
        }
    }

    /// Returns the string name of this provider
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

/// Configuration for LLM integration
///
/// This structure contains all LLM-related configuration options, including both
/// operational flags (enabled, export_mode, etc.) and provider-specific settings
/// (provider, model, API key, etc.).
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

/// Error types for LLM configuration
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

impl LlmOptions {
    /// Creates LLM options from CLI arguments with full configuration resolution
    ///
    /// This method:
    /// 1. Loads .env file if present
    /// 2. Resolves API key from CLI args, environment, or .env file
    /// 3. Validates all configuration parameters
    /// 4. Sets sensible defaults for optional parameters
    pub fn from_cli_args(llm_args: &crate::cli::llm::LlmArgs) -> Result<Self, LlmConfigError> {
        // Load .env file (silently ignore if not found)
        let _ = dotenvy::dotenv();

        let enabled = llm_args.llm_export.is_some() || llm_args.llm_ask.is_some();
        let export_mode = llm_args.llm_export.is_some();
        let direct_query_mode = llm_args.llm_ask.is_some();

        // If LLM is not enabled, return minimal config
        if !enabled {
            return Ok(Self {
                enabled: false,
                export_mode: false,
                direct_query_mode: false,
                ..Default::default()
            });
        }

        // For export-only mode, we only need basic validation since no API calls are made
        if export_mode && !direct_query_mode {
            return Ok(Self {
                enabled: true,
                export_mode: true,
                direct_query_mode: false,
                provider: None,    // Not needed for export
                model: None,       // Not needed for export
                api_key: None,     // Not needed for export
                endpoint: None,    // Not needed for export
                temperature: None, // Not needed for export
                max_tokens: None,  // Not needed for export
                timeout: None,     // Not needed for export
            });
        }

        // Parse provider (only required for direct query mode)
        let provider = LlmProvider::from_str(&llm_args.llm_provider).map_err(|_| {
            LlmConfigError::InvalidProvider {
                provider: llm_args.llm_provider.clone(),
            }
        })?;

        // Resolve API key from CLI args, environment variables, or .env file
        // For dry-run mode, we allow missing API keys and use a placeholder
        let api_key = if let Some(key) = llm_args
            .llm_api_key
            .clone()
            .or_else(|| std::env::var(provider.env_var()).ok())
        {
            key
        } else if llm_args.dry_run {
            // For dry-run mode, use a placeholder API key
            "dry-run-placeholder-key".to_string()
        } else {
            eprintln!("💡 Tip: Create a .env file in your project root with:");
            eprintln!("   {}=your-api-key-here", provider.env_var());
            return Err(LlmConfigError::MissingApiKey {
                provider: provider.name().to_string(),
                env_var: provider.env_var().to_string(),
            });
        };

        // Use provided model or default
        let model = llm_args
            .llm_model
            .clone()
            .unwrap_or_else(|| provider.default_model().to_string());

        // Validate temperature
        let temperature = llm_args.llm_temperature.unwrap_or(0.7);
        if !(0.0..=2.0).contains(&temperature) {
            return Err(LlmConfigError::InvalidTemperature { temp: temperature });
        }

        // Validate max tokens
        let max_tokens = llm_args.llm_max_tokens.unwrap_or(1000);
        if !(1..=32000).contains(&max_tokens) {
            return Err(LlmConfigError::InvalidMaxTokens { tokens: max_tokens });
        }

        Ok(Self {
            enabled,
            export_mode,
            direct_query_mode,
            provider: Some(provider),
            model: Some(model),
            api_key: Some(api_key),
            endpoint: llm_args.llm_endpoint.clone(),
            temperature: Some(temperature),
            max_tokens: Some(max_tokens),
            timeout: Some(Duration::from_secs(60)),
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

    /// Convert to core LLM configuration
    ///
    /// This method converts the config-layer LlmOptions into the pure core configuration
    /// type. It only succeeds if the LLM is enabled and has valid configuration.
    pub fn to_core_config(&self) -> Result<crate::core::llm::CoreLlmConfig, LlmConfigError> {
        if !self.enabled {
            return Err(LlmConfigError::InvalidProvider {
                provider: "LLM not enabled".to_string(),
            });
        }

        let provider = self
            .provider
            .as_ref()
            .ok_or_else(|| LlmConfigError::InvalidProvider {
                provider: "No provider specified".to_string(),
            })?;

        let model = self
            .model
            .as_ref()
            .ok_or_else(|| LlmConfigError::InvalidProvider {
                provider: "No model specified".to_string(),
            })?
            .clone();

        let api_key = self
            .api_key
            .as_ref()
            .ok_or_else(|| LlmConfigError::MissingApiKey {
                provider: provider.name().to_string(),
                env_var: provider.env_var().to_string(),
            })?
            .clone();

        let core_provider = match provider {
            LlmProvider::OpenAi => crate::core::llm::CoreLlmProvider::OpenAi,
            LlmProvider::Anthropic => crate::core::llm::CoreLlmProvider::Anthropic,
            LlmProvider::Cohere => crate::core::llm::CoreLlmProvider::Cohere,
            LlmProvider::OpenRouter => crate::core::llm::CoreLlmProvider::OpenRouter,
        };

        Ok(crate::core::llm::CoreLlmConfig {
            provider: core_provider,
            model,
            api_key,
            endpoint: self.endpoint.clone(),
            temperature: self.temperature.unwrap_or(0.7),
            max_tokens: self.max_tokens.unwrap_or(1000),
            timeout: self.timeout.unwrap_or(Duration::from_secs(60)),
        })
    }
}
