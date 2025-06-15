//! # LLM Integration Module
//!
//! This module provides comprehensive LLM (Large Language Model) integration for RusTree,
//! enabling direct analysis of directory structures through various AI providers.
//!
//! ## Features
//!
//! - **Multiple Providers**: Support for OpenAI, Anthropic, Cohere, and OpenRouter
//! - **Flexible Configuration**: Environment variables, .env files, and CLI arguments
//! - **Smart Prompting**: Context-aware prompt generation with tree metadata
//! - **Error Handling**: Comprehensive error types with helpful messages
//! - **Async Operations**: Non-blocking LLM API calls with proper timeout handling
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use rustree::core::llm::{CoreLlmConfig, CoreLlmProvider, LlmConfig, LlmClientFactory};
//! use std::time::Duration;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // Create pure core LLM configuration
//! let core_config = CoreLlmConfig {
//!     provider: CoreLlmProvider::OpenAi,
//!     model: "gpt-4".to_string(),
//!     api_key: "sk-your-api-key".to_string(),
//!     endpoint: None,
//!     temperature: 0.7,
//!     max_tokens: 1000,
//!     timeout: Duration::from_secs(60),
//! };
//!
//! let config = LlmConfig::new(core_config);
//!
//! // Query the LLM
//! let response = LlmClientFactory::create_and_query(&config, "What is this project about?").await?;
//! println!("LLM Response: {}", response);
//! # Ok(())
//! # }
//! ```
//!
//! ## Architecture
//!
//! - [`LlmProvider`]: Enum representing supported LLM providers
//! - [`LlmConfig`]: Configuration structure for LLM settings
//! - [`LlmClientFactory`]: Factory for creating and querying LLM clients
//! - [`TreePromptFormatter`]: Formats tree output into LLM-optimized prompts
//! - [`LlmResponseProcessor`]: Processes and formats LLM responses
//! - [`LlmError`]: Comprehensive error types for LLM operations

pub mod client;
pub mod config;
pub mod error;
pub mod preview;
pub mod prompt;
pub mod providers;
pub mod response;

pub use client::LlmClientFactory;
pub use config::{CoreLlmConfig, CoreLlmProvider};
pub use error::LlmError;
pub use preview::RequestPreview;
pub use prompt::TreePromptFormatter;
pub use providers::{LlmConfig, LlmProvider};
pub use response::LlmResponseProcessor;
