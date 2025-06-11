// src/core/llm/client.rs

use crate::core::llm::{LlmConfig, LlmError, LlmProvider};
use rig::completion::Prompt;
use serde_json::json;

pub struct LlmClientFactory;

impl LlmClientFactory {
    pub async fn create_and_query(config: &LlmConfig, prompt: &str) -> Result<String, LlmError> {
        match config.provider {
            LlmProvider::OpenAi => Self::query_openai(config, prompt).await,
            LlmProvider::Anthropic => Self::query_anthropic(config, prompt).await,
            LlmProvider::Cohere => Self::query_cohere(config, prompt).await,
            LlmProvider::OpenRouter => Self::query_openrouter(config, prompt).await,
        }
    }

    async fn query_openai(config: &LlmConfig, prompt: &str) -> Result<String, LlmError> {
        // If custom endpoint is specified, use reqwest for full control
        if let Some(endpoint) = &config.endpoint {
            return Self::query_openai_compatible(config, prompt, endpoint, "OpenAI").await;
        }

        // Otherwise use Rig for standard OpenAI API
        // SAFETY: Setting environment variables with valid UTF-8 strings is safe
        // We're only setting the key for this process and the values are controlled
        unsafe {
            std::env::set_var("OPENAI_API_KEY", &config.api_key);
        }

        let client = rig::providers::openai::Client::from_env();
        let agent = client
            .agent(&config.model)
            .temperature(config.temperature as f64)
            .max_tokens(config.max_tokens as u64)
            .build();

        let response = agent
            .prompt(prompt)
            .await
            .map_err(|e| LlmError::RigClient(e.to_string()))?;

        Ok(response)
    }

    async fn query_anthropic(config: &LlmConfig, prompt: &str) -> Result<String, LlmError> {
        // Custom endpoints for Anthropic would require different API format than OpenAI
        if config.endpoint.is_some() {
            return Err(LlmError::UnsupportedFeature(
                "Custom endpoints for Anthropic not yet implemented (requires different API format than OpenAI)".to_string()
            ));
        }

        // Use Rig for standard Anthropic API
        // SAFETY: Setting environment variables with valid UTF-8 strings is safe
        unsafe {
            std::env::set_var("ANTHROPIC_API_KEY", &config.api_key);
        }

        let client = rig::providers::anthropic::Client::from_env();
        let agent = client
            .agent(&config.model)
            .temperature(config.temperature as f64)
            .max_tokens(config.max_tokens as u64)
            .build();

        let response = agent
            .prompt(prompt)
            .await
            .map_err(|e| LlmError::RigClient(e.to_string()))?;

        Ok(response)
    }

    async fn query_cohere(config: &LlmConfig, prompt: &str) -> Result<String, LlmError> {
        // Custom endpoints for Cohere would require different API format than OpenAI
        if config.endpoint.is_some() {
            return Err(LlmError::UnsupportedFeature(
                "Custom endpoints for Cohere not yet implemented (requires different API format than OpenAI)".to_string()
            ));
        }

        // Use Rig for standard Cohere API
        // SAFETY: Setting environment variables with valid UTF-8 strings is safe
        unsafe {
            std::env::set_var("COHERE_API_KEY", &config.api_key);
        }

        let client = rig::providers::cohere::Client::from_env();
        let agent = client
            .agent(&config.model)
            .temperature(config.temperature as f64)
            .max_tokens(config.max_tokens as u64)
            .build();

        let response = agent
            .prompt(prompt)
            .await
            .map_err(|e| LlmError::RigClient(e.to_string()))?;

        Ok(response)
    }

    async fn query_openrouter(config: &LlmConfig, prompt: &str) -> Result<String, LlmError> {
        // OpenRouter uses OpenAI-compatible API but with different endpoint
        let default_endpoint = "https://openrouter.ai/api/v1".to_string();
        let endpoint = config.endpoint.as_ref().unwrap_or(&default_endpoint);

        // Use reqwest for custom endpoint support since Rig may not support it directly
        Self::query_openai_compatible(config, prompt, endpoint, "OpenRouter").await
    }

    // Helper function for OpenAI-compatible APIs with custom endpoints
    async fn query_openai_compatible(
        config: &LlmConfig,
        prompt: &str,
        endpoint: &str,
        provider_name: &str,
    ) -> Result<String, LlmError> {
        let client = reqwest::Client::new();

        let url = format!("{}/chat/completions", endpoint.trim_end_matches('/'));

        let request_body = json!({
            "model": config.model,
            "messages": [
                {
                    "role": "user",
                    "content": prompt
                }
            ],
            "temperature": config.temperature,
            "max_tokens": config.max_tokens
        });

        let response = client
            .post(&url)
            .header("Authorization", format!("Bearer {}", config.api_key))
            .header("Content-Type", "application/json")
            .timeout(config.timeout)
            .json(&request_body)
            .send()
            .await
            .map_err(|e| LlmError::Network(format!("{} request failed: {}", provider_name, e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(LlmError::Api(format!(
                "{} API error {}: {}",
                provider_name, status, error_text
            )));
        }

        let response_json: serde_json::Value = response.json().await.map_err(|e| {
            LlmError::Api(format!("{} response parsing failed: {}", provider_name, e))
        })?;

        // Extract the response text from OpenAI-compatible format
        let content = response_json
            .get("choices")
            .and_then(|choices| choices.get(0))
            .and_then(|choice| choice.get("message"))
            .and_then(|message| message.get("content"))
            .and_then(|content| content.as_str())
            .ok_or_else(|| {
                LlmError::Api(format!(
                    "{} response format unexpected: {}",
                    provider_name, response_json
                ))
            })?;

        Ok(content.to_string())
    }
}
