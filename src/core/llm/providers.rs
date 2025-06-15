//! # LLM Provider Configuration (Core)
//!
//! This module contains the core LLM provider functionality without any CLI dependencies.
//! It provides the minimal interface needed by the core LLM functionality using pure
//! configuration types.
//!
//! ## Architecture
//!
//! This module is part of the core layer and contains only business logic.
//! All configuration, validation, and external dependencies are handled by
//! the higher-level configuration layer.
//!
//! ## Examples
//!
//! ```rust,no_run
//! use rustree::core::llm::{CoreLlmConfig, CoreLlmProvider, LlmConfig};
//! use std::time::Duration;
//!
//! // Create core configuration (normally done by config layer)
//! let core_config = CoreLlmConfig {
//!     provider: CoreLlmProvider::OpenAi,
//!     model: "gpt-4".to_string(),
//!     api_key: "sk-...".to_string(),
//!     endpoint: None,
//!     temperature: 0.7,
//!     max_tokens: 1000,
//!     timeout: Duration::from_secs(60),
//! };
//!
//! // Create LLM config for core operations
//! let config = LlmConfig::new(core_config);
//! ```

use crate::core::llm::config::{CoreLlmConfig, CoreLlmProvider};
use std::time::Duration;

/// Configuration for LLM operations (Core)
///
/// This structure contains validated configuration data for LLM operations.
/// Unlike the config-layer types, this has no external dependencies and
/// represents ready-to-use configuration values.
#[derive(Debug, Clone)]
pub struct LlmConfig {
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

/// Legacy LLM provider enum for backward compatibility
///
/// This enum is maintained for compatibility with existing code that
/// imports LlmProvider from core. New code should use CoreLlmProvider.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LlmProvider {
    /// OpenAI GPT models
    OpenAi,
    /// Anthropic Claude models
    Anthropic,
    /// Cohere Command models
    Cohere,
    /// OpenRouter unified API
    OpenRouter,
}

impl From<CoreLlmProvider> for LlmProvider {
    fn from(core_provider: CoreLlmProvider) -> Self {
        match core_provider {
            CoreLlmProvider::OpenAi => LlmProvider::OpenAi,
            CoreLlmProvider::Anthropic => LlmProvider::Anthropic,
            CoreLlmProvider::Cohere => LlmProvider::Cohere,
            CoreLlmProvider::OpenRouter => LlmProvider::OpenRouter,
        }
    }
}

impl LlmConfig {
    /// Creates an LLM configuration from core configuration data
    ///
    /// This is the pure core constructor that takes validated configuration
    /// data. All validation and external dependency resolution should be
    /// done by the config layer before calling this method.
    ///
    /// # Arguments
    ///
    /// * `core_config` - Validated core configuration from the config layer
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use rustree::core::llm::{CoreLlmConfig, CoreLlmProvider, LlmConfig};
    /// use std::time::Duration;
    ///
    /// let core_config = CoreLlmConfig {
    ///     provider: CoreLlmProvider::OpenAi,
    ///     model: "gpt-4".to_string(),
    ///     api_key: "sk-...".to_string(),
    ///     endpoint: None,
    ///     temperature: 0.7,
    ///     max_tokens: 1000,
    ///     timeout: Duration::from_secs(60),
    /// };
    ///
    /// let config = LlmConfig::new(core_config);
    /// ```
    pub fn new(core_config: CoreLlmConfig) -> Self {
        Self {
            provider: core_config.provider,
            model: core_config.model,
            api_key: core_config.api_key,
            endpoint: core_config.endpoint,
            temperature: core_config.temperature,
            max_tokens: core_config.max_tokens,
            timeout: core_config.timeout,
        }
    }
}
