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
//! use rustree::core::llm::{LlmConfig, LlmClientFactory, TreePromptFormatter};
//! use rustree::cli::llm::LlmArgs;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // Create LLM configuration from CLI args
//! let args = LlmArgs {
//!     llm_ask: Some("Analyze this project".to_string()),
//!     llm_provider: "openai".to_string(),
//!     ..Default::default()
//! };
//!
//! let config = LlmConfig::from_cli_args(&args)?;
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
pub mod error;
pub mod preview;
pub mod prompt;
pub mod providers;
pub mod response;

pub use client::LlmClientFactory;
pub use error::LlmError;
pub use preview::RequestPreview;
pub use prompt::TreePromptFormatter;
pub use providers::{LlmConfig, LlmProvider};
pub use response::LlmResponseProcessor;
