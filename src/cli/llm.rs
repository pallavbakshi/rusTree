//! # LLM CLI Arguments
//!
//! This module defines the command-line interface for LLM integration features.
//! It provides both export functionality (for use with external LLM tools) and
//! direct LLM integration (using the Rig library).
//!
//! ## Usage Examples
//!
//! ```bash
//! # Export formatted query for external tools
//! rustree --llm-export "What's the architecture of this project?"
//!
//! # Direct LLM integration
//! rustree --llm-ask "Analyze this codebase" --llm-provider openai
//!
//! # With custom settings
//! rustree --llm-ask "Security review" \
//!   --llm-provider anthropic \
//!   --llm-model claude-3-sonnet \
//!   --llm-temperature 0.3 \
//!   --llm-max-tokens 1500
//!
//! # Generate .env template
//! rustree --llm-generate-env > .env
//! ```

use clap::Args;

/// Command-line arguments for LLM integration
///
/// This structure defines all the CLI options related to LLM functionality,
/// including both export mode (for external tools) and direct integration mode.
///
/// ## Modes of Operation
///
/// 1. **Export Mode**: Use `--llm-export` to generate formatted queries for external LLM tools
/// 2. **Direct Mode**: Use `--llm-ask` to send queries directly to LLM providers
/// 3. **Template Mode**: Use `--llm-generate-env` to create .env file templates
///
/// ## API Key Resolution
///
/// API keys are resolved in order of priority:
/// 1. `--llm-api-key` CLI argument (highest priority)
/// 2. Environment variables (e.g., `OPENAI_API_KEY`)
/// 3. `.env` file (lowest priority)
#[derive(Args, Debug, Default, Clone)]
pub struct LlmArgs {
    /// Export a formatted query for external LLM tools (preserves current behavior)
    #[arg(long)]
    pub llm_export: Option<String>,

    /// Ask a question directly to an LLM service
    #[arg(long)]
    pub llm_ask: Option<String>,

    /// LLM provider (openai, anthropic, cohere, openrouter)
    #[arg(long, default_value = "openai")]
    pub llm_provider: String,

    /// Model name (e.g., gpt-4, claude-3-sonnet)
    #[arg(long)]
    pub llm_model: Option<String>,

    /// API key (or use environment variables)
    #[arg(long)]
    pub llm_api_key: Option<String>,

    /// Custom endpoint URL
    #[arg(long)]
    pub llm_endpoint: Option<String>,

    /// Temperature for response randomness (0.0-2.0)
    #[arg(long)]
    pub llm_temperature: Option<f32>,

    /// Maximum tokens in response
    #[arg(long)]
    pub llm_max_tokens: Option<u32>,

    /// Generate a sample .env file template for LLM API keys
    #[arg(long)]
    pub llm_generate_env: bool,
}
