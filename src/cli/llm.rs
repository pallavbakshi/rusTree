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

    /// Preview the LLM request without actually sending it.
    ///
    /// When used together with `--llm-ask`, RusTree will build the full HTTP
    /// request that would be sent to the provider, print it to stdout, and
    /// exit without incurring any API cost. Supplying `--dry-run` without
    /// `--llm-ask` has no effect other than a short notice.
    #[arg(long)]
    pub dry_run: bool,

    /// If present, format certain outputs in a more human-readable style.
    ///
    /// Originally this flag controlled the LLM `--dry-run` pretty printer.
    /// It is now reused by the core listing functionality to display file
    /// sizes in a human-readable form (e.g. `1.2 MB` instead of `1234567B`).
    /// Retains its original role for LLM dry-run but no longer requires
    /// `--dry-run` to be passed.
    #[arg(long)]
    pub human_friendly: bool,
}
