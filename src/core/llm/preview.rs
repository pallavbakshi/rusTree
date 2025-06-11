//! LLM Request Preview (Dry-Run)
//!
//! This module builds a representation of the HTTP request that would be sent
//! to an LLM provider, allowing users to inspect it when the `--dry-run` flag
//! is supplied.  The goal is **zero network traffic** while re-using the same
//! configuration values used for the real call.

use super::providers::LlmConfig;
use serde_json::json;

/// A human-readable preview of the outgoing LLM request.
#[derive(Debug, Clone)]
pub struct RequestPreview {
    pub provider: String,
    pub endpoint: String,
    pub model: String,
    pub temperature: f32,
    pub max_tokens: u32,
    pub headers: Vec<(String, String)>,
    pub body: serde_json::Value,
    pub estimated_prompt_tokens: Option<u32>,
    pub estimated_completion_tokens: Option<u32>,
}

impl RequestPreview {
    /// Build a preview from an [`LlmConfig`] and the prompt text.
    pub fn from_config(cfg: &LlmConfig, prompt: &str) -> Self {
        // Rough heuristic: 1 token ≈ 4 characters. Guard against zero length.
        let prompt_tokens = ((prompt.len() as f32) / 4.0).ceil() as u32;

        // Determine endpoint based on provider / cfg.endpoint
        let endpoint = match cfg.provider {
            super::providers::LlmProvider::OpenAi => cfg
                .endpoint
                .clone()
                .unwrap_or_else(|| "https://api.openai.com/v1".to_string()),
            super::providers::LlmProvider::Anthropic => cfg
                .endpoint
                .clone()
                .unwrap_or_else(|| "https://api.anthropic.com/v1".to_string()),
            super::providers::LlmProvider::Cohere => cfg
                .endpoint
                .clone()
                .unwrap_or_else(|| "https://api.cohere.ai/v1".to_string()),
            super::providers::LlmProvider::OpenRouter => cfg
                .endpoint
                .clone()
                .unwrap_or_else(|| "https://openrouter.ai/api/v1".to_string()),
        };

        // Build provider-specific request body to match actual wire format
        let body = match cfg.provider {
            super::providers::LlmProvider::OpenAi | super::providers::LlmProvider::OpenRouter => {
                json!({
                    "model": cfg.model,
                    "messages": [ { "role": "user", "content": prompt } ],
                    "temperature": cfg.temperature,
                    "max_tokens": cfg.max_tokens
                })
            }
            super::providers::LlmProvider::Anthropic => {
                json!({
                    "model": cfg.model,
                    "max_tokens": cfg.max_tokens,
                    "messages": [ { "role": "user", "content": prompt } ],
                    "temperature": cfg.temperature
                })
            }
            super::providers::LlmProvider::Cohere => {
                json!({
                    "model": cfg.model,
                    "message": prompt,
                    "temperature": cfg.temperature,
                    "max_tokens": cfg.max_tokens
                })
            }
        };

        let masked_key = mask_key(&cfg.api_key);

        // Build provider-specific headers
        let headers = match cfg.provider {
            super::providers::LlmProvider::OpenAi | super::providers::LlmProvider::OpenRouter => {
                vec![
                    (
                        "Authorization".to_string(),
                        format!("Bearer {}", masked_key),
                    ),
                    ("Content-Type".to_string(), "application/json".to_string()),
                ]
            }
            super::providers::LlmProvider::Anthropic => {
                vec![
                    ("x-api-key".to_string(), masked_key),
                    ("Content-Type".to_string(), "application/json".to_string()),
                    ("anthropic-version".to_string(), "2023-06-01".to_string()),
                ]
            }
            super::providers::LlmProvider::Cohere => {
                vec![
                    (
                        "Authorization".to_string(),
                        format!("Bearer {}", masked_key),
                    ),
                    ("Content-Type".to_string(), "application/json".to_string()),
                ]
            }
        };

        RequestPreview {
            provider: cfg.provider.name().to_string(),
            endpoint,
            model: cfg.model.clone(),
            temperature: cfg.temperature,
            max_tokens: cfg.max_tokens,
            headers,
            body,
            estimated_prompt_tokens: Some(prompt_tokens),
            estimated_completion_tokens: Some(cfg.max_tokens),
        }
    }

    /// Render the preview as a colourful, human-friendly text block.
    pub fn pretty_print(&self) -> String {
        self.format_output(false)
    }

    /// Render the preview in markdown format for better readability.
    pub fn pretty_print_markdown(&self) -> String {
        self.format_output(true)
    }

    /// Internal method to format output in either plain text or markdown.
    fn format_output(&self, markdown: bool) -> String {
        use std::fmt::Write as _;

        let mut out = String::new();

        if markdown {
            writeln!(out, "# LLM Request Preview (dry-run)").ok();
            writeln!(out).ok();
            writeln!(out, "## Configuration").ok();
            writeln!(out).ok();
            writeln!(out, "| Parameter | Value |").ok();
            writeln!(out, "|-----------|-------|").ok();
            writeln!(out, "| Provider | {} |", self.provider).ok();
            writeln!(out, "| Endpoint | `{}` |", self.endpoint).ok();
            writeln!(out, "| Model | {} |", self.model).ok();
            writeln!(out, "| Temperature | {} |", self.temperature).ok();
            writeln!(out, "| Max tokens | {} |", self.max_tokens).ok();
            writeln!(out).ok();

            writeln!(out, "## Headers").ok();
            writeln!(out).ok();
            writeln!(out, "```").ok();
            for (k, v) in &self.headers {
                writeln!(out, "{}: {}", k, v).ok();
            }
            writeln!(out, "```").ok();
            writeln!(out).ok();

            writeln!(out, "## Request Body").ok();
            writeln!(out).ok();

            // Extract and format key fields for better readability
            if let Some(model) = self.body.get("model").and_then(|v| v.as_str()) {
                writeln!(out, "**Model**: `{}`", model).ok();
                writeln!(out).ok();
            }

            if let Some(temp) = self.body.get("temperature").and_then(|v| v.as_f64()) {
                writeln!(out, "**Temperature**: {:.2}", temp).ok();
                writeln!(out).ok();
            }

            if let Some(max_tokens) = self.body.get("max_tokens").and_then(|v| v.as_u64()) {
                writeln!(out, "**Max Tokens**: {}", max_tokens).ok();
                writeln!(out).ok();
            }

            // Format messages section
            if let Some(messages) = self.body.get("messages").and_then(|v| v.as_array()) {
                writeln!(out, "### Messages").ok();
                writeln!(out).ok();

                for (i, msg) in messages.iter().enumerate() {
                    if let (Some(role), Some(content)) = (
                        msg.get("role").and_then(|v| v.as_str()),
                        msg.get("content").and_then(|v| v.as_str()),
                    ) {
                        writeln!(out, "**Message {} ({})**:", i + 1, role).ok();
                        writeln!(out).ok();
                        writeln!(out, "```").ok();
                        writeln!(out, "{}", content).ok();
                        writeln!(out, "```").ok();
                        writeln!(out).ok();
                    }
                }
            }

            // Include full JSON request
            writeln!(out, "### Full JSON Request").ok();
            writeln!(out).ok();
            writeln!(out, "```json").ok();
            let body_pretty = serde_json::to_string_pretty(&self.body).unwrap_or_default();
            write!(out, "{}", body_pretty).ok();
            writeln!(out, "\n```").ok();

            if let Some(prompt_tk) = self.estimated_prompt_tokens {
                let comp_tk = self.estimated_completion_tokens.unwrap_or(0);
                let total = prompt_tk + comp_tk;
                writeln!(out).ok();
                writeln!(out, "## Token Estimation").ok();
                writeln!(out).ok();
                writeln!(out, "- **Prompt tokens**: {}", prompt_tk).ok();
                writeln!(out, "- **Completion tokens**: {}", comp_tk).ok();
                writeln!(out, "- **Total tokens**: ≈ {}", total).ok();
            }
        } else {
            writeln!(out, "─── LLM REQUEST PREVIEW (dry-run) ───").ok();
            writeln!(out, "Provider        : {}", self.provider).ok();
            writeln!(out, "Endpoint        : {}", self.endpoint).ok();
            writeln!(out, "Model           : {}", self.model).ok();
            writeln!(out, "Temperature     : {}", self.temperature).ok();
            writeln!(out, "Max tokens      : {}", self.max_tokens).ok();

            writeln!(out, "\nHeaders:").ok();
            for (k, v) in &self.headers {
                writeln!(out, "  {}: {}", k, v).ok();
            }

            writeln!(out, "\nBody:").ok();
            let body_pretty = serde_json::to_string_pretty(&self.body).unwrap_or_default();
            for line in body_pretty.lines() {
                writeln!(out, "  {}", line).ok();
            }

            if let Some(prompt_tk) = self.estimated_prompt_tokens {
                let comp_tk = self.estimated_completion_tokens.unwrap_or(0);
                let total = prompt_tk + comp_tk;
                writeln!(
                    out,
                    "\nEstimated tokens: {} prompt + {} completion ≈ {} total",
                    prompt_tk, comp_tk, total
                )
                .ok();
            }

            writeln!(out, "──────────────────────────────────────").ok();
        }

        out
    }
}

/// Mask all but first and last 2 chars of the key for display.
fn mask_key(key: &str) -> String {
    if key.len() <= 4 {
        return "****".to_string();
    }
    let (first, last) = key.split_at(2);
    let last2 = &last[last.len().saturating_sub(2)..];
    format!("{}{}{}", first, "*".repeat(8), last2)
}

// Unit tests for preview
#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::llm::providers::{LlmConfig, LlmProvider};
    use std::time::Duration;

    fn dummy_cfg() -> LlmConfig {
        LlmConfig {
            provider: LlmProvider::OpenAi,
            model: "gpt-4".to_string(),
            api_key: "sk-test123456".to_string(),
            endpoint: None,
            temperature: 0.5,
            max_tokens: 100,
            timeout: Duration::from_secs(30),
        }
    }

    #[test]
    fn preview_contains_prompt() {
        let cfg = dummy_cfg();
        let prompt = "Hello world";
        let preview = RequestPreview::from_config(&cfg, prompt);
        let printed = preview.pretty_print();
        assert!(printed.contains("Hello world"));
        assert!(printed.contains("gpt-4"));
        assert!(printed.contains("openai"));
    }
}
