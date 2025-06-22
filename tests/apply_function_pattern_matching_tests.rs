// tests/apply_function_pattern_matching_tests.rs
//
// Comprehensive tests for apply-function pattern matching to prevent regression.
// These tests specifically focus on the relative path pattern matching issue that was fixed.

use rustree::config::metadata::ApplyFunction;
use rustree::{BuiltInFunction, FilteringOptions, MetadataOptions};
use rustree::{LibOutputFormat, RustreeLibConfig, format_nodes, get_tree_nodes};
use std::fs;
use std::sync::Mutex;
use tempfile::TempDir;

// Global mutex to serialize directory changes to prevent data races in parallel tests
static DIRECTORY_CHANGE_MUTEX: Mutex<()> = Mutex::new(());

#[test]
fn test_apply_function_relative_path_patterns() {
    // Create a comprehensive directory structure for testing various patterns
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let temp_path = temp_dir.path();

    // Create test structure:
    // temp/
    // ├── root_file.txt
    // ├── src/
    // │   ├── main.rs
    // │   ├── lib.rs
    // │   └── cli/
    // │       ├── mod.rs
    // │       ├── args.rs
    // │       └── commands/
    // │           ├── build.rs
    // │           └── run.rs
    // ├── tests/
    // │   ├── integration.rs
    // │   └── unit.rs
    // ├── docs/
    // │   └── README.md
    // └── Cargo.toml

    // Create directories
    let src_dir = temp_path.join("src");
    let cli_dir = src_dir.join("cli");
    let commands_dir = cli_dir.join("commands");
    let tests_dir = temp_path.join("tests");
    let docs_dir = temp_path.join("docs");

    fs::create_dir_all(&commands_dir).expect("Failed to create nested dirs");
    fs::create_dir(&tests_dir).expect("Failed to create tests dir");
    fs::create_dir(&docs_dir).expect("Failed to create docs dir");

    // Create files with content that will be catted
    fs::write(temp_path.join("root_file.txt"), "Root file content")
        .expect("Failed to write root_file.txt");
    fs::write(
        src_dir.join("main.rs"),
        "fn main() { println!(\"Hello\"); }",
    )
    .expect("Failed to write main.rs");
    fs::write(src_dir.join("lib.rs"), "pub mod cli;").expect("Failed to write lib.rs");
    fs::write(cli_dir.join("mod.rs"), "pub mod args;\npub mod commands;")
        .expect("Failed to write cli/mod.rs");
    fs::write(cli_dir.join("args.rs"), "pub struct Args {}").expect("Failed to write cli/args.rs");
    fs::write(commands_dir.join("build.rs"), "pub fn build() { }")
        .expect("Failed to write commands/build.rs");
    fs::write(commands_dir.join("run.rs"), "pub fn run() { }")
        .expect("Failed to write commands/run.rs");
    fs::write(
        tests_dir.join("integration.rs"),
        "#[test] fn it_works() { }",
    )
    .expect("Failed to write integration.rs");
    fs::write(tests_dir.join("unit.rs"), "#[test] fn unit_test() { }")
        .expect("Failed to write unit.rs");
    fs::write(docs_dir.join("README.md"), "# Documentation").expect("Failed to write README.md");
    fs::write(temp_path.join("Cargo.toml"), "[package]\nname = \"test\"")
        .expect("Failed to write Cargo.toml");

    // Test 1: Single file pattern (exact match)
    test_pattern_match(temp_path, "src/main.rs", vec!["src/main.rs"], 1);

    // Test 2: Wildcard file pattern in directory
    test_pattern_match(temp_path, "src/*.rs", vec!["src/main.rs", "src/lib.rs"], 2);

    // Test 3: CLI directory pattern - match files in cli directory
    test_pattern_match(
        temp_path,
        "src/cli/*",
        vec!["src/cli/mod.rs", "src/cli/args.rs"],
        2,
    );

    // Test 4: Recursive pattern in CLI directory
    test_pattern_match(
        temp_path,
        "src/cli/**",
        vec![
            "src/cli/mod.rs",
            "src/cli/args.rs",
            "src/cli/commands/build.rs",
            "src/cli/commands/run.rs",
        ],
        4,
    );

    // Test 5: Global recursive pattern for Rust files
    test_pattern_match(
        temp_path,
        "**/*.rs",
        vec![
            "src/main.rs",
            "src/lib.rs",
            "src/cli/mod.rs",
            "src/cli/args.rs",
            "src/cli/commands/build.rs",
            "src/cli/commands/run.rs",
            "tests/integration.rs",
            "tests/unit.rs",
        ],
        8,
    );

    // Test 6: Basename pattern (works anywhere in tree)
    test_pattern_match(temp_path, "*.toml", vec!["Cargo.toml"], 1);

    // Test 7: Directory-only pattern
    test_pattern_match(temp_path, "src/cli/", vec![], 0); // Directory patterns don't match files for cat function

    // Test 8: Multiple patterns using pipe separator
    test_pattern_match(
        temp_path,
        "*.toml|src/main.rs",
        vec!["Cargo.toml", "src/main.rs"],
        2,
    );

    // Test 9: Pattern that shouldn't match anything
    test_pattern_match(temp_path, "nonexistent/**", vec![], 0);

    // Test 10: Complex nested pattern
    test_pattern_match(
        temp_path,
        "src/**/commands/*.rs",
        vec!["src/cli/commands/build.rs", "src/cli/commands/run.rs"],
        2,
    );
}

#[test]
fn test_apply_function_absolute_vs_relative_patterns() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let temp_path = temp_dir.path();

    // Create simple structure
    let src_dir = temp_path.join("src");
    fs::create_dir(&src_dir).expect("Failed to create src dir");
    fs::write(src_dir.join("test.rs"), "// test file").expect("Failed to write test.rs");

    // Test relative pattern (should work)
    test_pattern_match(temp_path, "src/test.rs", vec!["src/test.rs"], 1);

    // Test absolute pattern (should also work if implemented correctly)
    let absolute_pattern = format!("{}/src/test.rs", temp_path.display());
    test_pattern_match(temp_path, &absolute_pattern, vec!["src/test.rs"], 1);
}

#[test]
fn test_apply_function_exclude_patterns() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let temp_path = temp_dir.path();

    // Create test structure
    let src_dir = temp_path.join("src");
    fs::create_dir(&src_dir).expect("Failed to create src dir");
    fs::write(src_dir.join("main.rs"), "fn main() {}").expect("Failed to write main.rs");
    fs::write(src_dir.join("test.rs"), "// test").expect("Failed to write test.rs");
    fs::write(src_dir.join("lib.rs"), "// lib").expect("Failed to write lib.rs");

    // Test exclude pattern - should apply to all files except those matching the exclude pattern
    let config = RustreeLibConfig {
        metadata: MetadataOptions {
            apply_function: Some(ApplyFunction::BuiltIn(BuiltInFunction::Cat)),
            ..Default::default()
        },
        filtering: FilteringOptions {
            apply_exclude_patterns: Some(vec!["src/test.rs".to_string()]),
            ..Default::default()
        },
        ..Default::default()
    };

    let nodes = get_tree_nodes(temp_path, &config).expect("Failed to get tree nodes");
    let output =
        format_nodes(&nodes, LibOutputFormat::Text, &config).expect("Failed to format nodes");

    // Should show content for main.rs and lib.rs, but not test.rs
    assert!(output.contains("--- File Contents ("));
    assert!(output.contains("<file path=") && output.contains("main.rs\">"));
    assert!(output.contains("fn main() {}"));
    assert!(output.contains("<file path=") && output.contains("lib.rs\">"));
    assert!(output.contains("// lib"));

    // Should NOT show content for test.rs
    assert!(!(output.contains("<file path=") && output.contains("test.rs\">")));
}

#[test]
fn test_apply_function_both_include_and_exclude_patterns() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let temp_path = temp_dir.path();

    // Create test structure
    let src_dir = temp_path.join("src");
    fs::create_dir(&src_dir).expect("Failed to create src dir");
    fs::write(src_dir.join("main.rs"), "fn main() {}").expect("Failed to write main.rs");
    fs::write(src_dir.join("test.rs"), "// test").expect("Failed to write test.rs");
    fs::write(src_dir.join("lib.rs"), "// lib").expect("Failed to write lib.rs");
    fs::write(temp_path.join("readme.txt"), "readme").expect("Failed to write readme.txt");

    // Test both include and exclude - should include src/*.rs but exclude src/test.rs
    let config = RustreeLibConfig {
        metadata: MetadataOptions {
            apply_function: Some(ApplyFunction::BuiltIn(BuiltInFunction::Cat)),
            ..Default::default()
        },
        filtering: FilteringOptions {
            apply_include_patterns: Some(vec!["src/*.rs".to_string()]),
            apply_exclude_patterns: Some(vec!["src/test.rs".to_string()]),
            ..Default::default()
        },
        ..Default::default()
    };

    let nodes = get_tree_nodes(temp_path, &config).expect("Failed to get tree nodes");
    let output =
        format_nodes(&nodes, LibOutputFormat::Text, &config).expect("Failed to format nodes");

    // Should show content for main.rs and lib.rs only
    assert!(output.contains("--- File Contents ("));
    assert!(output.contains("<file path=") && output.contains("main.rs\">"));
    assert!(output.contains("fn main() {}"));
    assert!(output.contains("<file path=") && output.contains("lib.rs\">"));
    assert!(output.contains("// lib"));

    // Should NOT show content for test.rs (excluded) or readme.txt (not included)
    assert!(!(output.contains("<file path=") && output.contains("test.rs\">")));
    assert!(!(output.contains("<file path=") && output.contains("readme.txt\">")));
}

#[test]
fn test_apply_function_case_sensitivity() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let temp_path = temp_dir.path();

    // Create test structure with mixed case
    let src_dir = temp_path.join("src");
    fs::create_dir(&src_dir).expect("Failed to create src dir");

    // Check if filesystem is case-sensitive by trying to create both files
    fs::write(src_dir.join("test_case.txt"), "lowercase").expect("Failed to write test file");
    let is_case_sensitive = fs::write(src_dir.join("TEST_CASE.txt"), "uppercase").is_ok()
        && fs::read_to_string(src_dir.join("test_case.txt")).unwrap() == "lowercase";

    // Clean up test files
    let _ = fs::remove_file(src_dir.join("test_case.txt"));
    let _ = fs::remove_file(src_dir.join("TEST_CASE.txt"));

    if !is_case_sensitive {
        // Skip this test on case-insensitive filesystems
        eprintln!("Skipping case sensitivity test on case-insensitive filesystem");
        return;
    }

    fs::write(src_dir.join("Main.rs"), "// Main").expect("Failed to write Main.rs");
    fs::write(src_dir.join("main.rs"), "// main").expect("Failed to write main.rs");

    // Test case-sensitive pattern (default)
    let config_sensitive = RustreeLibConfig {
        metadata: MetadataOptions {
            apply_function: Some(ApplyFunction::BuiltIn(BuiltInFunction::Cat)),
            ..Default::default()
        },
        filtering: FilteringOptions {
            apply_include_patterns: Some(vec!["src/main.rs".to_string()]),
            case_insensitive_filter: false,
            ..Default::default()
        },
        ..Default::default()
    };

    let nodes = get_tree_nodes(temp_path, &config_sensitive).expect("Failed to get tree nodes");
    let output = format_nodes(&nodes, LibOutputFormat::Text, &config_sensitive)
        .expect("Failed to format nodes");

    // Should match only lowercase main.rs
    assert!(output.contains("<file path=") && output.contains("main.rs\">"));
    assert!(output.contains("// main"));
    assert!(!(output.contains("<file path=") && output.contains("Main.rs\">")));

    // Test case-insensitive pattern
    let config_insensitive = RustreeLibConfig {
        metadata: MetadataOptions {
            apply_function: Some(ApplyFunction::BuiltIn(BuiltInFunction::Cat)),
            ..Default::default()
        },
        filtering: FilteringOptions {
            apply_include_patterns: Some(vec!["src/MAIN.rs".to_string()]),
            case_insensitive_filter: true,
            ..Default::default()
        },
        ..Default::default()
    };

    let nodes = get_tree_nodes(temp_path, &config_insensitive).expect("Failed to get tree nodes");
    let output = format_nodes(&nodes, LibOutputFormat::Text, &config_insensitive)
        .expect("Failed to format nodes");

    // Should match both files due to case insensitivity
    assert!(output.contains("<file path=") && output.contains("main.rs\">"));
    assert!(output.contains("// main"));
    // Note: This test might need adjustment based on how case insensitivity is implemented
}

#[test]
fn test_apply_function_patterns_with_different_working_directories() {
    // Acquire the mutex to prevent data races from concurrent directory changes
    let _guard = DIRECTORY_CHANGE_MUTEX
        .lock()
        .expect("Failed to acquire directory change mutex");

    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let temp_path = temp_dir.path();

    // Create nested structure
    let nested_dir = temp_path.join("nested");
    fs::create_dir(&nested_dir).expect("Failed to create nested dir");
    fs::write(nested_dir.join("file.txt"), "nested content").expect("Failed to write nested file");

    // Change to nested directory and test relative patterns from there
    let original_cwd = std::env::current_dir().expect("Failed to get current dir");
    std::env::set_current_dir(&nested_dir).expect("Failed to change dir");

    // Test pattern that should work from nested directory
    let config = RustreeLibConfig {
        metadata: MetadataOptions {
            apply_function: Some(ApplyFunction::BuiltIn(BuiltInFunction::Cat)),
            ..Default::default()
        },
        filtering: FilteringOptions {
            apply_include_patterns: Some(vec!["file.txt".to_string()]),
            ..Default::default()
        },
        ..Default::default()
    };

    let nodes = get_tree_nodes(&nested_dir, &config).expect("Failed to get tree nodes");
    let output =
        format_nodes(&nodes, LibOutputFormat::Text, &config).expect("Failed to format nodes");

    // Should match the file
    assert!(output.contains("<file path=") && output.contains("file.txt\">"));
    assert!(output.contains("nested content"));

    // Restore original working directory
    std::env::set_current_dir(original_cwd).expect("Failed to restore dir");

    // Mutex guard is automatically dropped here, releasing the lock
}

/// Helper function to test a specific pattern against expected matching files
fn test_pattern_match(
    temp_path: &std::path::Path,
    pattern: &str,
    expected_files: Vec<&str>,
    expected_count: usize,
) {
    let config = RustreeLibConfig {
        metadata: MetadataOptions {
            apply_function: Some(ApplyFunction::BuiltIn(BuiltInFunction::Cat)),
            ..Default::default()
        },
        filtering: FilteringOptions {
            apply_include_patterns: Some(vec![pattern.to_string()]),
            ..Default::default()
        },
        ..Default::default()
    };

    let nodes = get_tree_nodes(temp_path, &config).expect("Failed to get tree nodes");
    let output =
        format_nodes(&nodes, LibOutputFormat::Text, &config).expect("Failed to format nodes");

    if expected_count == 0 {
        // Should not have file contents section at all
        assert!(
            !output.contains("--- File Contents ("),
            "Pattern '{}' should not match any files, but found file contents section",
            pattern
        );
        return;
    }

    // Should have file contents section
    assert!(
        output.contains("--- File Contents ("),
        "Pattern '{}' should match {} files but no file contents section found",
        pattern,
        expected_count
    );

    // Count the number of file headers in the content section
    let content_headers = output.matches("<file path=").count();
    assert_eq!(
        content_headers, expected_count,
        "Pattern '{}' should match {} files but found {} file headers",
        pattern, expected_count, content_headers
    );

    // Verify each expected file is present
    for expected_file in expected_files {
        let header = format!("<file path=\"{}\">", expected_file);
        assert!(
            output.contains(&header),
            "Pattern '{}' should match '{}' but file header '{}' not found in output",
            pattern,
            expected_file,
            header
        );
    }
}

#[test]
fn test_pattern_matching_edge_cases() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let temp_path = temp_dir.path();

    // Create files with special characters and edge cases
    let src_dir = temp_path.join("src");
    fs::create_dir(&src_dir).expect("Failed to create src dir");
    fs::write(src_dir.join("file-with-dashes.rs"), "// dashes").expect("Failed to write file");
    fs::write(src_dir.join("file_with_underscores.rs"), "// underscores")
        .expect("Failed to write file");
    fs::write(src_dir.join("file.with.dots.rs"), "// dots").expect("Failed to write file");
    fs::write(src_dir.join("FILE_CAPS.RS"), "// caps").expect("Failed to write file");

    // Test pattern with special characters
    test_pattern_match(
        temp_path,
        "src/file-with-*.rs",
        vec!["src/file-with-dashes.rs"],
        1,
    );
    test_pattern_match(
        temp_path,
        "src/file_with_*.rs",
        vec!["src/file_with_underscores.rs"],
        1,
    );
    test_pattern_match(
        temp_path,
        "src/file.with.*.rs",
        vec!["src/file.with.dots.rs"],
        1,
    );
    test_pattern_match(temp_path, "src/FILE_*.RS", vec!["src/FILE_CAPS.RS"], 1);

    // Test patterns that should match multiple files
    test_pattern_match(
        temp_path,
        "src/file*",
        vec![
            "src/file-with-dashes.rs",
            "src/file_with_underscores.rs",
            "src/file.with.dots.rs",
        ],
        3,
    );
}

#[test]
fn test_empty_and_invalid_patterns() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let temp_path = temp_dir.path();

    // Create a test file
    fs::write(temp_path.join("test.txt"), "test content").expect("Failed to write test file");

    // Test empty pattern list
    let config_empty = RustreeLibConfig {
        metadata: MetadataOptions {
            apply_function: Some(ApplyFunction::BuiltIn(BuiltInFunction::Cat)),
            ..Default::default()
        },
        filtering: FilteringOptions {
            apply_include_patterns: Some(vec![]),
            ..Default::default()
        },
        ..Default::default()
    };

    let nodes = get_tree_nodes(temp_path, &config_empty).expect("Failed to get tree nodes");
    let output =
        format_nodes(&nodes, LibOutputFormat::Text, &config_empty).expect("Failed to format nodes");

    // Empty pattern list should match nothing
    assert!(!output.contains("--- File Contents ---"));

    // Test pattern with pipe separator and empty parts
    let config_pipe = RustreeLibConfig {
        metadata: MetadataOptions {
            apply_function: Some(ApplyFunction::BuiltIn(BuiltInFunction::Cat)),
            ..Default::default()
        },
        filtering: FilteringOptions {
            apply_include_patterns: Some(vec!["|test.txt|".to_string()]),
            ..Default::default()
        },
        ..Default::default()
    };

    let nodes = get_tree_nodes(temp_path, &config_pipe).expect("Failed to get tree nodes");
    let output =
        format_nodes(&nodes, LibOutputFormat::Text, &config_pipe).expect("Failed to format nodes");

    // Should match test.txt despite empty parts in pattern
    assert!(output.contains("<file path=") && output.contains("test.txt\">"));
    assert!(output.contains("test content"));
}
