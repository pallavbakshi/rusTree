// tests/llm_env_tests.rs

use rustree::cli::llm::LlmArgs;
use rustree::core::llm::LlmConfig;
use std::env;
use std::sync::{Mutex, OnceLock};

static ENV_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

fn env_lock<'a>() -> std::sync::MutexGuard<'a, ()> {
    ENV_LOCK.get_or_init(|| Mutex::new(())).lock().unwrap()
}

#[test]
fn test_env_variable_loading() {
    // Set up test environment variable
    let _g = env_lock();
    unsafe {
        env::set_var("OPENAI_API_KEY", "test-key-12345");
    }

    let llm_args = LlmArgs {
        llm_export: None,
        llm_ask: None,
        llm_provider: "openai".to_string(),
        llm_model: None,
        llm_api_key: None, // No CLI key provided
        llm_endpoint: None,
        llm_temperature: None,
        llm_max_tokens: None,
        llm_generate_env: false,
        dry_run: false,
        human_friendly: false,
    };

    let config = LlmConfig::from_cli_args(&llm_args).expect("Should create config from env var");
    assert_eq!(config.api_key, "test-key-12345");
    assert_eq!(config.model, "gpt-4"); // Default model

    // Clean up
    unsafe {
        env::remove_var("OPENAI_API_KEY");
    }
}

#[test]
fn test_cli_key_overrides_env() {
    // Set up test environment variable
    let _g = env_lock();
    unsafe {
        env::set_var("OPENAI_API_KEY", "env-key");
    }

    let llm_args = LlmArgs {
        llm_export: None,
        llm_ask: None,
        llm_provider: "openai".to_string(),
        llm_model: None,
        llm_api_key: Some("cli-key".to_string()), // CLI key provided
        llm_endpoint: None,
        llm_temperature: None,
        llm_max_tokens: None,
        llm_generate_env: false,
        dry_run: false,
        human_friendly: false,
    };

    let config = LlmConfig::from_cli_args(&llm_args).expect("Should create config with CLI key");
    assert_eq!(config.api_key, "cli-key"); // CLI key should override env

    // Clean up
    unsafe {
        env::remove_var("OPENAI_API_KEY");
    }
}

#[test]
fn test_missing_api_key_error() {
    // Ensure no env var is set
    let _g = env_lock();
    unsafe {
        env::remove_var("OPENAI_API_KEY");
    }

    let llm_args = LlmArgs {
        llm_export: None,
        llm_ask: None,
        llm_provider: "openai".to_string(),
        llm_model: None,
        llm_api_key: None, // No key provided
        llm_endpoint: None,
        llm_temperature: None,
        llm_max_tokens: None,
        llm_generate_env: false,
        dry_run: false,
        human_friendly: false,
    };

    let result = LlmConfig::from_cli_args(&llm_args);
    assert!(result.is_err());

    let error = result.unwrap_err();
    assert!(error.to_string().contains("OPENAI_API_KEY"));
    assert!(error.to_string().contains(".env file"));
}

#[test]
fn test_generate_sample_env_file() {
    let sample = LlmConfig::generate_sample_env_file();

    // no env changes; no lock needed but acquire anyway for consistency
    let _g = env_lock();

    // Check that all supported providers are included
    assert!(sample.contains("OPENAI_API_KEY"));
    assert!(sample.contains("ANTHROPIC_API_KEY"));
    assert!(sample.contains("COHERE_API_KEY"));
    assert!(sample.contains("OPENROUTER_API_KEY"));

    // Check that it's properly commented out
    assert!(sample.contains("# OPENAI_API_KEY="));
}
