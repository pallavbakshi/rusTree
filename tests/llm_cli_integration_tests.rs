//! Integration tests for LLM CLI functionality

use rustree::cli::llm::LlmArgs;
use rustree::config::llm::LlmOptions;

#[test]
fn test_llm_args_default() {
    let args = LlmArgs {
        llm_export: None,
        llm_ask: None,
        llm_provider: "openai".to_string(),
        llm_model: None,
        llm_api_key: None,
        llm_endpoint: None,
        llm_temperature: None,
        llm_max_tokens: None,
        llm_generate_env: false,
        dry_run: false,
        human_friendly: false,
    };

    // Test that defaults are sensible
    assert_eq!(args.llm_provider, "openai");
    assert!(!args.llm_generate_env);
    assert!(args.llm_export.is_none());
    assert!(args.llm_ask.is_none());
}

#[test]
fn test_llm_options_from_export_args() {
    let args = LlmArgs {
        llm_export: Some("Test export question".to_string()),
        llm_ask: None,
        llm_provider: "openai".to_string(),
        llm_model: None,
        llm_api_key: Some("sk-test-key".to_string()),
        llm_endpoint: None,
        llm_temperature: None,
        llm_max_tokens: None,
        llm_generate_env: false,
        dry_run: false,
        human_friendly: false,
    };

    let options = LlmOptions::from_cli_args(&args).expect("Should create options from export args");

    assert!(options.enabled);
    assert!(options.export_mode);
    assert!(!options.direct_query_mode);
}

#[test]
fn test_llm_options_from_ask_args() {
    let args = LlmArgs {
        llm_export: None,
        llm_ask: Some("Test ask question".to_string()),
        llm_provider: "anthropic".to_string(),
        llm_model: Some("claude-3-sonnet".to_string()),
        llm_api_key: Some("sk-test-key".to_string()),
        llm_endpoint: None,
        llm_temperature: Some(0.5),
        llm_max_tokens: Some(1500),
        llm_generate_env: false,
        dry_run: false,
        human_friendly: false,
    };

    let options = LlmOptions::from_cli_args(&args).expect("Should create options from ask args");

    assert!(options.enabled);
    assert!(!options.export_mode);
    assert!(options.direct_query_mode);
}

#[test]
fn test_llm_options_from_both_args() {
    let args = LlmArgs {
        llm_export: Some("Export question".to_string()),
        llm_ask: Some("Ask question".to_string()),
        llm_provider: "openai".to_string(),
        llm_model: None,
        llm_api_key: Some("sk-test-key".to_string()),
        llm_endpoint: None,
        llm_temperature: None,
        llm_max_tokens: None,
        llm_generate_env: false,
        dry_run: false,
        human_friendly: false,
    };

    let options = LlmOptions::from_cli_args(&args).expect("Should create options from both args");

    // When both are present, both should be true
    assert!(options.enabled);
    assert!(options.export_mode);
    assert!(options.direct_query_mode);
}

#[test]
fn test_llm_options_from_empty_args() {
    let args = LlmArgs {
        llm_export: None,
        llm_ask: None,
        llm_provider: "openai".to_string(),
        llm_model: None,
        llm_api_key: None,
        llm_endpoint: None,
        llm_temperature: None,
        llm_max_tokens: None,
        llm_generate_env: false,
        dry_run: false,
        human_friendly: false,
    };

    let options = LlmOptions::from_cli_args(&args).expect("Should create options from empty args");

    assert!(!options.enabled);
    assert!(!options.export_mode);
    assert!(!options.direct_query_mode);
}

#[test]
fn test_llm_options_default() {
    let options = LlmOptions::default();

    assert!(!options.enabled);
    assert!(!options.export_mode);
    assert!(!options.direct_query_mode);
}

#[test]
fn test_generate_env_flag() {
    let args = LlmArgs {
        llm_export: None,
        llm_ask: None,
        llm_provider: "openai".to_string(),
        llm_model: None,
        llm_api_key: None,
        llm_endpoint: None,
        llm_temperature: None,
        llm_max_tokens: None,
        llm_generate_env: true,
        dry_run: false,
        human_friendly: false,
    };

    assert!(args.llm_generate_env);

    // The generate env flag doesn't affect LLM options enabled state
    let options =
        LlmOptions::from_cli_args(&args).expect("Should create options from generate env args");
    assert!(!options.enabled); // Because no actual LLM query is requested
}

#[test]
fn test_complex_args_configuration() {
    let args = LlmArgs {
        llm_export: None,
        llm_ask: Some("Complex analysis question with detailed requirements".to_string()),
        llm_provider: "anthropic".to_string(),
        llm_model: Some("claude-3-opus-20240229".to_string()),
        llm_api_key: Some("test-api-key-12345".to_string()),
        llm_endpoint: Some("https://custom-endpoint.example.com".to_string()),
        llm_temperature: Some(0.2),
        llm_max_tokens: Some(2000),
        llm_generate_env: false,
        dry_run: false,
        human_friendly: false,
    };

    // All fields should be preserved
    assert_eq!(
        args.llm_ask.as_ref().unwrap(),
        "Complex analysis question with detailed requirements"
    );
    assert_eq!(args.llm_provider, "anthropic");
    assert_eq!(args.llm_model.as_ref().unwrap(), "claude-3-opus-20240229");
    assert_eq!(args.llm_api_key.as_ref().unwrap(), "test-api-key-12345");
    assert_eq!(
        args.llm_endpoint.as_ref().unwrap(),
        "https://custom-endpoint.example.com"
    );
    assert_eq!(args.llm_temperature.unwrap(), 0.2);
    assert_eq!(args.llm_max_tokens.unwrap(), 2000);

    let options =
        LlmOptions::from_cli_args(&args).expect("Should create options from complex args");
    assert!(options.enabled);
    assert!(options.direct_query_mode);
    assert!(!options.export_mode);
}

#[test]
fn test_edge_case_empty_strings() {
    let args = LlmArgs {
        llm_export: Some("".to_string()), // Empty string
        llm_ask: None,
        llm_provider: "openai".to_string(),
        llm_model: Some("".to_string()), // Empty string
        llm_api_key: Some("sk-test-key".to_string()),
        llm_endpoint: None,
        llm_temperature: None,
        llm_max_tokens: None,
        llm_generate_env: false,
        dry_run: false,
        human_friendly: false,
    };

    let options =
        LlmOptions::from_cli_args(&args).expect("Should create options from edge case args");

    // Empty string should still count as "some"
    assert!(options.enabled);
    assert!(options.export_mode);
    assert!(!options.direct_query_mode);
}

#[test]
fn test_boundary_values() {
    let args = LlmArgs {
        llm_export: None,
        llm_ask: Some("Test".to_string()),
        llm_provider: "openai".to_string(),
        llm_model: None,
        llm_api_key: None,
        llm_endpoint: None,
        llm_temperature: Some(0.0), // Minimum temperature
        llm_max_tokens: Some(1),    // Minimum tokens
        llm_generate_env: false,
        dry_run: false,
        human_friendly: false,
    };

    assert_eq!(args.llm_temperature.unwrap(), 0.0);
    assert_eq!(args.llm_max_tokens.unwrap(), 1);
}

#[test]
fn test_maximum_boundary_values() {
    let args = LlmArgs {
        llm_export: None,
        llm_ask: Some("Test".to_string()),
        llm_provider: "openai".to_string(),
        llm_model: None,
        llm_api_key: None,
        llm_endpoint: None,
        llm_temperature: Some(2.0),  // Maximum temperature
        llm_max_tokens: Some(32000), // Maximum tokens (for our validation)
        llm_generate_env: false,
        dry_run: false,
        human_friendly: false,
    };

    assert_eq!(args.llm_temperature.unwrap(), 2.0);
    assert_eq!(args.llm_max_tokens.unwrap(), 32000);
}

// Test that the CLI args struct properly implements Debug
#[test]
fn test_debug_implementation() {
    let args = LlmArgs {
        llm_export: Some("test".to_string()),
        llm_ask: None,
        llm_provider: "openai".to_string(),
        llm_model: None,
        llm_api_key: Some("secret-key".to_string()),
        llm_endpoint: None,
        llm_temperature: None,
        llm_max_tokens: None,
        llm_generate_env: false,
        dry_run: false,
        human_friendly: false,
    };

    let debug_string = format!("{:?}", args);
    assert!(debug_string.contains("llm_export"));
    assert!(debug_string.contains("llm_provider"));
    // API key should be included in debug (this is a test, in real usage consider if this is desired)
    assert!(debug_string.contains("secret-key"));
}

#[test]
fn test_provider_case_handling() {
    let providers = vec![
        "openai",
        "OpenAI",
        "OPENAI",
        "anthropic",
        "Anthropic",
        "ANTHROPIC",
        "cohere",
        "Cohere",
        "COHERE",
        "openrouter",
        "OpenRouter",
        "OPENROUTER",
    ];

    for provider in providers {
        let args = LlmArgs {
            llm_export: None,
            llm_ask: Some("test".to_string()),
            llm_provider: provider.to_string(),
            llm_model: None,
            llm_api_key: Some("sk-test-key".to_string()),
            llm_endpoint: None,
            llm_temperature: None,
            llm_max_tokens: None,
            llm_generate_env: false,
            dry_run: false,
            human_friendly: false,
        };

        // Should not panic when creating LlmOptions
        let options =
            LlmOptions::from_cli_args(&args).expect("Should create options for all provider cases");
        assert!(options.enabled);
    }
}
