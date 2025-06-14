// src/config/llm.rs

/// Configuration for LLM integration
///
/// This module contains configuration structures specific to LLM functionality.
/// The main LLM configuration logic is in the core::llm::providers module.
#[derive(Debug, Clone, Default)]
pub struct LlmOptions {
    pub enabled: bool,
    pub export_mode: bool,
    pub direct_query_mode: bool,
}
impl LlmOptions {
    pub fn from_cli_args(llm_args: &crate::cli::llm::LlmArgs) -> Self {
        Self {
            enabled: llm_args.llm_export.is_some() || llm_args.llm_ask.is_some(),
            export_mode: llm_args.llm_export.is_some(),
            direct_query_mode: llm_args.llm_ask.is_some(),
        }
    }
}
