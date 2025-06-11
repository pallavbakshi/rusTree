//! Integration tests for LLM prompt formatting

use rustree::config::RustreeLibConfig;
use rustree::core::llm::TreePromptFormatter;

#[test]
fn test_basic_prompt_formatting() {
    let tree_output = "src/\n├── main.rs\n└── lib.rs";
    let question = "What is this project structure?";
    let config = RustreeLibConfig::default();

    let prompt = TreePromptFormatter::format_prompt(tree_output, question, &config);

    // Check that prompt contains expected components
    assert!(prompt.contains("You are analyzing a directory tree structure"));
    assert!(prompt.contains(tree_output));
    assert!(prompt.contains(question));
    assert!(prompt.contains("architectural patterns"));
    assert!(prompt.contains("actionable insights"));
}

#[test]
fn test_prompt_with_depth_limit() {
    let tree_output = "src/\n├── main.rs\n└── lib.rs";
    let question = "Analyze the structure";

    let mut config = RustreeLibConfig::default();
    config.listing.max_depth = Some(3);

    let prompt = TreePromptFormatter::format_prompt(tree_output, question, &config);

    assert!(prompt.contains("Tree depth limited to 3 levels"));
}

#[test]
fn test_prompt_with_directories_only() {
    let tree_output = "src/\n└── core/";
    let question = "What directories exist?";

    let mut config = RustreeLibConfig::default();
    config.listing.list_directories_only = true;

    let prompt = TreePromptFormatter::format_prompt(tree_output, question, &config);

    assert!(prompt.contains("Showing directories only"));
}

#[test]
fn test_prompt_with_summary_extraction() {
    let tree_output =
        "src/\n├── main.rs\n└── lib.rs\n\nSummary:\nTotal files: 2\nTotal directories: 1";
    let question = "Tell me about this project";
    let config = RustreeLibConfig::default();

    let prompt = TreePromptFormatter::format_prompt(tree_output, question, &config);

    assert!(prompt.contains("Project Statistics:"));
    assert!(prompt.contains("Summary:\nTotal files: 2"));
}

#[test]
fn test_prompt_with_complex_config() {
    let tree_output = "src/\n├── main.rs\n├── lib.rs\n└── tests/";
    let question = "Detailed analysis please";

    let mut config = RustreeLibConfig::default();
    config.listing.max_depth = Some(5);
    config.listing.list_directories_only = true;

    let prompt = TreePromptFormatter::format_prompt(tree_output, question, &config);

    assert!(prompt.contains("Tree depth limited to 5 levels"));
    assert!(prompt.contains("Showing directories only"));
    assert!(prompt.contains(tree_output));
    assert!(prompt.contains("Detailed analysis please"));
}

#[test]
fn test_prompt_with_no_metadata() {
    let tree_output = "simple/\n└── file.txt";
    let question = "What is this?";
    let config = RustreeLibConfig::default();

    let prompt = TreePromptFormatter::format_prompt(tree_output, question, &config);

    // Should not contain any metadata notes
    assert!(!prompt.contains("Tree depth limited"));
    assert!(!prompt.contains("Showing directories only"));
    assert!(!prompt.contains("Project Statistics"));

    // But should still contain the basics
    assert!(prompt.contains("You are analyzing"));
    assert!(prompt.contains(tree_output));
    assert!(prompt.contains(question));
}

#[test]
fn test_prompt_with_special_characters() {
    let tree_output = "src/\n├── file-with-dashes.rs\n└── file_with_underscores.rs";
    let question = "How are files named in this project?";
    let config = RustreeLibConfig::default();

    let prompt = TreePromptFormatter::format_prompt(tree_output, question, &config);

    // Should properly escape/include special characters
    assert!(prompt.contains("file-with-dashes.rs"));
    assert!(prompt.contains("file_with_underscores.rs"));
    assert!(prompt.contains("How are files named"));
}

#[test]
fn test_prompt_with_unicode() {
    let tree_output = "src/\n├── 文件.rs\n└── файл.rs";
    let question = "What about these files with Unicode names?";
    let config = RustreeLibConfig::default();

    let prompt = TreePromptFormatter::format_prompt(tree_output, question, &config);

    assert!(prompt.contains("文件.rs"));
    assert!(prompt.contains("файл.rs"));
    assert!(prompt.contains("Unicode names"));
}

#[test]
fn test_system_prompt() {
    let system_prompt = TreePromptFormatter::format_system_prompt();

    assert!(system_prompt.contains("expert software architect"));
    assert!(system_prompt.contains("Code organization"));
    assert!(system_prompt.contains("architectural patterns"));
    assert!(system_prompt.contains("maintenance and scalability"));
    assert!(system_prompt.contains("Technology stack insights"));
    assert!(system_prompt.contains("concise, actionable insights"));
}

#[test]
fn test_long_tree_output() {
    let tree_output = (0..100)
        .map(|i| format!("├── file_{}.rs", i))
        .collect::<Vec<_>>()
        .join("\n");

    let question = "Analyze this large codebase";
    let config = RustreeLibConfig::default();

    let prompt = TreePromptFormatter::format_prompt(&tree_output, question, &config);

    // Should handle large inputs gracefully
    assert!(prompt.contains("file_0.rs"));
    assert!(prompt.contains("file_99.rs"));
    assert!(prompt.contains("Analyze this large codebase"));
    assert!(prompt.len() > tree_output.len()); // Should add additional context
}

#[test]
fn test_empty_inputs() {
    let prompt = TreePromptFormatter::format_prompt("", "", &RustreeLibConfig::default());

    // Should handle empty inputs gracefully
    assert!(prompt.contains("You are analyzing"));
    assert!(prompt.contains("Question:"));
    // Should not crash or panic
}

#[test]
fn test_metadata_extraction_edge_cases() {
    // Test with multiple "Summary:" occurrences
    let tree_output = "Summary: Not the real one\nsrc/\n├── main.rs\n\nSummary:\nReal summary here";
    let question = "Test";
    let config = RustreeLibConfig::default();

    let prompt = TreePromptFormatter::format_prompt(tree_output, question, &config);

    // Should extract the last occurrence in the metadata section
    assert!(prompt.contains("Real summary here"));
    // The entire tree output is included in the code block, so the first summary will be there
    assert!(prompt.contains("Not the real one"));

    // But the metadata section should only contain the real summary
    assert!(prompt.contains("Project Statistics:\nSummary:\nReal summary here"));
}

#[test]
fn test_prompt_structure() {
    let tree_output = "src/\n└── main.rs";
    let question = "Simple test";
    let config = RustreeLibConfig::default();

    let prompt = TreePromptFormatter::format_prompt(tree_output, question, &config);

    // Check the overall structure
    let lines: Vec<&str> = prompt.lines().collect();

    // Should start with the system instruction
    assert!(lines[0].contains("You are analyzing"));

    // Should have code block markers
    assert!(prompt.contains("```"));

    // Should end with guidance
    assert!(prompt.contains("Focus on architectural patterns"));
}
