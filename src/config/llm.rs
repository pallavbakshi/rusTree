//! Configuration helper bridging the CLI layer with the *core* LLM types.

// Re-export the basic data structures from the core crate so external users
// can continue to access them via `rustree::config::llm::*`.

pub use crate::core::options::llm::{LlmConfigError, LlmOptions, LlmProvider};

use std::str::FromStr;

/// Build an [`LlmOptions`] instance from parsed CLI arguments.
///
/// This helper is kept in the config layer because it depends on
/// `crate::cli::llm::LlmArgs`, tying it to the CLI crate.  The resulting
/// *plain* options structure lives in the core layer and therefore **does not
/// reference the CLI**.
impl LlmOptions {
    #[allow(clippy::too_many_lines)]
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
        if let Some(max_tokens) = llm_args.llm_max_tokens {
            if max_tokens == 0 || max_tokens > 32_000 {
                return Err(LlmConfigError::InvalidMaxTokens { tokens: max_tokens });
            }
        }

        Ok(Self {
            enabled: true,
            export_mode,
            direct_query_mode,
            provider: Some(provider),
            model: Some(model),
            api_key: Some(api_key),
            endpoint: llm_args.llm_endpoint.clone(),
            temperature: Some(temperature),
            max_tokens: llm_args.llm_max_tokens,
            timeout: None,
        })
    }

    /// Convert the *configuration-layer* [`LlmOptions`] into a *core* [`CoreLlmConfig`].
    ///
    /// This strips away all `Option<>` wrappers and applies sane defaults so
    /// that the returned structure is ready for immediate use by the core
    /// engine.
    pub fn to_core_config(
        &self,
    ) -> Result<crate::core::llm::config::CoreLlmConfig, LlmConfigError> {
        use crate::core::llm::config::{CoreLlmConfig, CoreLlmProvider};

        // Ensure the basic fields are present. All invariants should already
        // be enforced by `from_cli_args`, so we unwrap here.
        let provider = self
            .provider
            .clone()
            .ok_or_else(|| LlmConfigError::InvalidProvider {
                provider: "unknown".to_string(),
            })?;

        let model = self
            .model
            .clone()
            .unwrap_or_else(|| provider.default_model().to_string());

        let api_key = self.api_key.clone().unwrap_or_default();

        let core_provider = match provider {
            LlmProvider::OpenAi => CoreLlmProvider::OpenAi,
            LlmProvider::Anthropic => CoreLlmProvider::Anthropic,
            LlmProvider::Cohere => CoreLlmProvider::Cohere,
            LlmProvider::OpenRouter => CoreLlmProvider::OpenRouter,
        };

        Ok(CoreLlmConfig {
            provider: core_provider,
            model,
            api_key,
            endpoint: self.endpoint.clone(),
            temperature: self.temperature.unwrap_or(0.7),
            max_tokens: self.max_tokens.unwrap_or(1000),
            timeout: self
                .timeout
                .unwrap_or_else(|| std::time::Duration::from_secs(60)),
        })
    }

    /// Generate a commented `.env` template containing all supported provider
    /// variables.
    pub fn generate_sample_env_file() -> String {
        [
            "# LLM provider API keys",
            "# Uncomment the variables for the providers you intend to use and",
            "# paste your actual API key after the '=' sign.",
            "#",
            "# OPENAI_API_KEY=",
            "# ANTHROPIC_API_KEY=",
            "# COHERE_API_KEY=",
            "# OPENROUTER_API_KEY=",
            "",
        ]
        .join("\n")
    }
}
