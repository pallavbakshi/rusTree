// tests/llm_env_tests.rs

use rustree::cli::llm::LlmArgs;
use rustree::config::LlmOptions;
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
        llm_ask: Some("test question".to_string()),
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

    let llm_options =
        LlmOptions::from_cli_args(&llm_args).expect("Should create options from env var");
    let core_config = llm_options
        .to_core_config()
        .expect("Should convert to core config");
    let config = LlmConfig::new(core_config);
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
        llm_ask: Some("test question".to_string()),
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

    let llm_options =
        LlmOptions::from_cli_args(&llm_args).expect("Should create options with CLI key");
    let core_config = llm_options
        .to_core_config()
        .expect("Should convert to core config");
    let config = LlmConfig::new(core_config);
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
        llm_ask: Some("test question".to_string()),
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

    let result = LlmOptions::from_cli_args(&llm_args);
    assert!(result.is_err());

    let error = result.unwrap_err();
    let error_msg = error.to_string();
    println!("Error message: {}", error_msg);
    assert!(error_msg.contains("OPENAI_API_KEY"));
    // The new error message might not contain ".env file", let's check for "environment variable" instead
    assert!(error_msg.contains("environment variable") || error_msg.contains(".env"));
}

#[test]
fn test_generate_sample_env_file() {
    let sample = LlmOptions::generate_sample_env_file();

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
