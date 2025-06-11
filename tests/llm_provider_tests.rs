//! Integration tests for LLM provider functionality

use rustree::cli::llm::LlmArgs;
use rustree::core::llm::{LlmConfig, LlmError, LlmProvider};
use std::env;
use std::str::FromStr;
use std::sync::{Mutex, OnceLock};

static ENV_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

fn env_lock<'a>() -> std::sync::MutexGuard<'a, ()> {
    ENV_LOCK.get_or_init(|| Mutex::new(())).lock().unwrap()
}

#[test]
fn test_provider_from_str() {
    // Test valid providers
    assert_eq!(
        LlmProvider::from_str("openai").unwrap(),
        LlmProvider::OpenAi
    );
    assert_eq!(
        LlmProvider::from_str("OpenAI").unwrap(),
        LlmProvider::OpenAi
    );
    assert_eq!(
        LlmProvider::from_str("OPENAI").unwrap(),
        LlmProvider::OpenAi
    );

    assert_eq!(
        LlmProvider::from_str("anthropic").unwrap(),
        LlmProvider::Anthropic
    );
    assert_eq!(
        LlmProvider::from_str("cohere").unwrap(),
        LlmProvider::Cohere
    );
    assert_eq!(
        LlmProvider::from_str("openrouter").unwrap(),
        LlmProvider::OpenRouter
    );

    // Test invalid provider
    assert!(LlmProvider::from_str("invalid").is_err());
    let error = LlmProvider::from_str("invalid").unwrap_err();
    assert!(matches!(error, LlmError::InvalidProvider { .. }));
    assert!(error.to_string().contains("invalid"));
}

#[test]
fn test_provider_properties() {
    let openai = LlmProvider::OpenAi;
    assert_eq!(openai.default_model(), "gpt-4");
    assert_eq!(openai.env_var(), "OPENAI_API_KEY");
    assert_eq!(openai.name(), "openai");

    let anthropic = LlmProvider::Anthropic;
    assert_eq!(anthropic.default_model(), "claude-3-sonnet-20240229");
    assert_eq!(anthropic.env_var(), "ANTHROPIC_API_KEY");
    assert_eq!(anthropic.name(), "anthropic");

    let cohere = LlmProvider::Cohere;
    assert_eq!(cohere.default_model(), "command-r");
    assert_eq!(cohere.env_var(), "COHERE_API_KEY");
    assert_eq!(cohere.name(), "cohere");

    let openrouter = LlmProvider::OpenRouter;
    assert_eq!(openrouter.default_model(), "openai/gpt-4");
    assert_eq!(openrouter.env_var(), "OPENROUTER_API_KEY");
    assert_eq!(openrouter.name(), "openrouter");
}

#[test]
fn test_config_with_custom_model() {
    let _g = env_lock();
    unsafe {
        env::set_var("OPENAI_API_KEY", "test-key");
    }

    let args = LlmArgs {
        llm_export: None,
        llm_ask: None,
        llm_provider: "openai".to_string(),
        llm_model: Some("gpt-3.5-turbo".to_string()),
        llm_api_key: None,
        llm_endpoint: None,
        llm_temperature: None,
        llm_max_tokens: None,
        llm_generate_env: false,
        dry_run: false,
        human_friendly: false,
    };

    let config = LlmConfig::from_cli_args(&args).expect("Should create config");
    assert_eq!(config.model, "gpt-3.5-turbo");

    unsafe {
        env::remove_var("OPENAI_API_KEY");
    }
}

#[test]
fn test_config_validation_temperature() {
    let _g = env_lock();
    unsafe {
        env::set_var("OPENAI_API_KEY", "test-key");
    }

    // Valid temperature
    let args = LlmArgs {
        llm_export: None,
        llm_ask: None,
        llm_provider: "openai".to_string(),
        llm_model: None,
        llm_api_key: None,
        llm_endpoint: None,
        llm_temperature: Some(1.5),
        llm_max_tokens: None,
        llm_generate_env: false,
        dry_run: false,
        human_friendly: false,
    };

    let config = LlmConfig::from_cli_args(&args).expect("Should create config");
    assert_eq!(config.temperature, 1.5);

    // Invalid temperature - too low
    let args_low = LlmArgs {
        llm_temperature: Some(-0.1),
        ..args.clone()
    };

    let result = LlmConfig::from_cli_args(&args_low);
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        LlmError::InvalidTemperature { .. }
    ));

    // Invalid temperature - too high
    let args_high = LlmArgs {
        llm_temperature: Some(2.1),
        ..args
    };

    let result = LlmConfig::from_cli_args(&args_high);
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        LlmError::InvalidTemperature { .. }
    ));

    unsafe {
        env::remove_var("OPENAI_API_KEY");
    }
}

#[test]
fn test_config_validation_max_tokens() {
    let _g = env_lock();
    unsafe {
        env::set_var("OPENAI_API_KEY", "test-key");
    }

    // Valid max tokens
    let args = LlmArgs {
        llm_export: None,
        llm_ask: None,
        llm_provider: "openai".to_string(),
        llm_model: None,
        llm_api_key: None,
        llm_endpoint: None,
        llm_temperature: None,
        llm_max_tokens: Some(500),
        llm_generate_env: false,
        dry_run: false,
        human_friendly: false,
    };

    let config = LlmConfig::from_cli_args(&args).expect("Should create config");
    assert_eq!(config.max_tokens, 500);

    // Invalid max tokens - too low
    let args_low = LlmArgs {
        llm_max_tokens: Some(0),
        ..args.clone()
    };

    let result = LlmConfig::from_cli_args(&args_low);
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        LlmError::InvalidMaxTokens { .. }
    ));

    // Invalid max tokens - too high
    let args_high = LlmArgs {
        llm_max_tokens: Some(50000),
        ..args
    };

    let result = LlmConfig::from_cli_args(&args_high);
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        LlmError::InvalidMaxTokens { .. }
    ));

    unsafe {
        env::remove_var("OPENAI_API_KEY");
    }
}

#[test]
fn test_config_defaults() {
    let _g = env_lock();
    unsafe {
        env::set_var("ANTHROPIC_API_KEY", "test-key");
    }

    let args = LlmArgs {
        llm_export: None,
        llm_ask: None,
        llm_provider: "anthropic".to_string(),
        llm_model: None,
        llm_api_key: None,
        llm_endpoint: None,
        llm_temperature: None,
        llm_max_tokens: None,
        llm_generate_env: false,
        dry_run: false,
        human_friendly: false,
    };

    let config = LlmConfig::from_cli_args(&args).expect("Should create config");

    // Check defaults
    assert_eq!(config.provider, LlmProvider::Anthropic);
    assert_eq!(config.model, "claude-3-sonnet-20240229"); // Default for Anthropic
    assert_eq!(config.temperature, 0.7); // Default temperature
    assert_eq!(config.max_tokens, 1000); // Default max tokens
    assert_eq!(config.timeout.as_secs(), 60); // Default timeout

    unsafe {
        env::remove_var("ANTHROPIC_API_KEY");
    }
}

#[test]
fn test_provider_equality() {
    assert_eq!(LlmProvider::OpenAi, LlmProvider::OpenAi);
    assert_ne!(LlmProvider::OpenAi, LlmProvider::Anthropic);

    // Test FromStr consistency
    let provider1 = LlmProvider::from_str("openai").unwrap();
    let provider2 = LlmProvider::from_str("OPENAI").unwrap();
    assert_eq!(provider1, provider2);
}

#[derive(Clone)]
struct LlmArgsBuilder {
    args: LlmArgs,
}

impl LlmArgsBuilder {
    fn new() -> Self {
        Self {
            args: LlmArgs {
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
            },
        }
    }

    fn provider(mut self, provider: &str) -> Self {
        self.args.llm_provider = provider.to_string();
        self
    }

    fn temperature(mut self, temp: f32) -> Self {
        self.args.llm_temperature = Some(temp);
        self
    }

    fn max_tokens(mut self, tokens: u32) -> Self {
        self.args.llm_max_tokens = Some(tokens);
        self
    }

    fn build(self) -> LlmArgs {
        self.args
    }
}

#[test]
fn test_builder_pattern_usage() {
    let _g = env_lock();
    unsafe {
        env::set_var("COHERE_API_KEY", "test-key");
    }

    let args = LlmArgsBuilder::new()
        .provider("cohere")
        .temperature(0.5)
        .max_tokens(2000)
        .build();

    let config = LlmConfig::from_cli_args(&args).expect("Should create config");
    assert_eq!(config.provider, LlmProvider::Cohere);
    assert_eq!(config.temperature, 0.5);
    assert_eq!(config.max_tokens, 2000);

    unsafe {
        env::remove_var("COHERE_API_KEY");
    }
}

#[test]
fn test_config_with_custom_endpoint() {
    let _g = env_lock();
    unsafe {
        env::set_var("OPENAI_API_KEY", "test-key");
    }

    let args = LlmArgs {
        llm_provider: "openai".to_string(),
        llm_endpoint: Some("https://custom.openai.example.com".to_string()),
        ..Default::default()
    };

    let config = LlmConfig::from_cli_args(&args).unwrap();
    assert_eq!(
        config.endpoint,
        Some("https://custom.openai.example.com".to_string())
    );

    unsafe {
        env::remove_var("OPENAI_API_KEY");
    }
}

#[test]
fn test_openrouter_default_endpoint() {
    let _g = env_lock();
    unsafe {
        env::set_var("OPENROUTER_API_KEY", "test-key");
    }

    let args = LlmArgs {
        llm_provider: "openrouter".to_string(),
        llm_endpoint: None, // Should use default OpenRouter endpoint
        ..Default::default()
    };

    let config = LlmConfig::from_cli_args(&args).unwrap();
    assert_eq!(config.endpoint, None); // Config stores None, client provides default

    unsafe {
        env::remove_var("OPENROUTER_API_KEY");
    }
}
