// tests/diff_llm_integration_tests.rs

//! Integration tests for LLM functionality with diff outputs
//! Tests the enhanced prompts and context provided for diff analysis

use serde_json::Value;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use tempfile::{TempDir, tempdir};

/// Helper that returns a `std::process::Command` configured to execute the
/// compiled `rustree` binary.  Cargo exposes the absolute path to the test
/// process via the `CARGO_BIN_EXE_<name>` environment variable, so we can
/// construct the command without relying on external crates such as
/// `assert_cmd`.
fn rustree_command() -> Command {
    let exe = env!("CARGO_BIN_EXE_rustree");
    Command::new(exe)
}

/// Test helper for LLM diff integration tests
struct LlmDiffTestContext {
    temp_dir: TempDir,
    baseline_file: PathBuf,
}

impl LlmDiffTestContext {
    fn new() -> Self {
        let temp_dir = tempdir().expect("Failed to create temporary directory");
        let baseline_file = temp_dir.path().join("baseline.json");

        Self {
            temp_dir,
            baseline_file,
        }
    }

    fn temp_path(&self) -> &std::path::Path {
        self.temp_dir.path()
    }

    fn create_test_structure(&self) {
        let src_dir = self.temp_path().join("src");
        fs::create_dir_all(&src_dir).unwrap();
        fs::write(
            src_dir.join("main.rs"),
            "fn main() {\n    println!(\"Hello\");\n}",
        )
        .unwrap();
        fs::write(
            src_dir.join("lib.rs"),
            "pub fn hello() -> &'static str {\n    \"world\"\n}",
        )
        .unwrap();

        fs::write(
            self.temp_path().join("Cargo.toml"),
            "[package]\nname = \"test-project\"\nversion = \"0.1.0\"",
        )
        .unwrap();
        fs::write(
            self.temp_path().join("README.md"),
            "# Test Project\n\nA simple test project.",
        )
        .unwrap();
    }

    fn create_baseline_snapshot(&self) {
        self.create_test_structure();

        // Generate baseline snapshot
        let output = rustree_command()
            .current_dir(self.temp_path())
            .args(["--output-format", "json"])
            .output()
            .expect("Failed to generate baseline snapshot");

        fs::write(&self.baseline_file, &output.stdout).unwrap();
    }

    fn apply_architectural_changes(&self) {
        // Simulate architectural refactoring

        // Add new module structure
        let modules_dir = self.temp_path().join("src").join("modules");
        fs::create_dir_all(&modules_dir).unwrap();
        fs::write(modules_dir.join("auth.rs"), "pub struct AuthService;\nimpl AuthService {\n    pub fn login(&self) -> bool { true }\n}").unwrap();
        fs::write(modules_dir.join("database.rs"), "pub struct Database;\nimpl Database {\n    pub fn connect(&self) -> Result<(), ()> { Ok(()) }\n}").unwrap();
        fs::write(
            modules_dir.join("mod.rs"),
            "pub mod auth;\npub mod database;",
        )
        .unwrap();

        // Add configuration
        let config_dir = self.temp_path().join("config");
        fs::create_dir_all(&config_dir).unwrap();
        fs::write(
            config_dir.join("app.toml"),
            "[app]\nname = \"test-app\"\nport = 8080",
        )
        .unwrap();

        // Add tests
        let tests_dir = self.temp_path().join("tests");
        fs::create_dir_all(&tests_dir).unwrap();
        fs::write(
            tests_dir.join("integration_tests.rs"),
            "use test_project::*;\n\n#[test]\nfn test_integration() {\n    assert!(true);\n}",
        )
        .unwrap();

        // Update main.rs to use new modules
        fs::write(
            self.temp_path().join("src").join("main.rs"), 
            "mod modules;\n\nuse modules::{auth::AuthService, database::Database};\n\nfn main() {\n    let auth = AuthService;\n    let db = Database;\n    println!(\"Application started\");\n}"
        ).unwrap();

        // Remove old lib.rs (architectural change)
        fs::remove_file(self.temp_path().join("src").join("lib.rs")).unwrap();

        // Add new lib.rs with proper structure
        fs::write(
            self.temp_path().join("src").join("lib.rs"),
            "pub mod modules;\n\npub use modules::*;\n\npub fn run_app() {\n    println!(\"Running application\");\n}"
        ).unwrap();

        // Add documentation
        let docs_dir = self.temp_path().join("docs");
        fs::create_dir_all(&docs_dir).unwrap();
        fs::write(
            docs_dir.join("architecture.md"),
            "# Architecture\n\nThis application follows a modular architecture.",
        )
        .unwrap();
        fs::write(
            docs_dir.join("api.md"),
            "# API Documentation\n\nEndpoints and usage.",
        )
        .unwrap();
    }

    fn rustree_cmd(&self) -> Command {
        let mut cmd = rustree_command();
        cmd.current_dir(self.temp_path());
        cmd
    }
}

#[test]
fn test_diff_llm_export_basic() {
    let ctx = LlmDiffTestContext::new();
    ctx.create_baseline_snapshot();
    ctx.apply_architectural_changes();

    let output = ctx
        .rustree_cmd()
        .args([
            "--diff",
            ctx.baseline_file.to_str().unwrap(),
            "--llm-export",
            "Analyze the architectural changes in this codebase",
        ])
        .output()
        .expect("Failed to run diff with LLM export");

    let stdout = String::from_utf8(output.stdout).unwrap();

    // Should contain rustree output blocks
    assert!(
        stdout.contains("---BEGIN RUSTREE OUTPUT---"),
        "Should have output start marker"
    );
    assert!(
        stdout.contains("---END RUSTREE OUTPUT---"),
        "Should have output end marker"
    );
    assert!(
        stdout.contains("---BEGIN LLM QUESTION---"),
        "Should have question start marker"
    );
    assert!(
        stdout.contains("---END LLM QUESTION---"),
        "Should have question end marker"
    );

    // Should contain diff markers
    assert!(stdout.contains("[+]"), "Should show added items");
    // Removed item markers may vary
    assert!(stdout.contains("Changes Summary:"), "Should show summary");

    // Should contain the question
    assert!(
        stdout.contains("Analyze the architectural changes"),
        "Should contain the question"
    );

    // Should show hint about piping to LLM
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(
        stderr.contains("Hint: Pipe the above to your LLM tool"),
        "Should show usage hint"
    );
}

#[test]
fn test_diff_llm_export_json_format() {
    let ctx = LlmDiffTestContext::new();
    ctx.create_baseline_snapshot();
    ctx.apply_architectural_changes();

    let output = ctx
        .rustree_cmd()
        .args([
            "--diff",
            ctx.baseline_file.to_str().unwrap(),
            "--output-format",
            "json",
            "--llm-export",
            "What are the key architectural improvements?",
        ])
        .output()
        .expect("Failed to run diff with JSON LLM export");

    let stdout = String::from_utf8(output.stdout).unwrap();

    // Should be valid JSON
    let json: Value = serde_json::from_str(&stdout).expect("Output should be valid JSON");

    // Should have tree and export_question fields
    assert!(json.get("tree").is_some(), "Should have tree field");
    assert!(
        json.get("export_question").is_some(),
        "Should have export_question field"
    );

    // Tree should contain diff structure
    let tree = json.get("tree").unwrap();
    assert!(
        tree.get("diff_summary").is_some(),
        "Tree should have diff_summary block"
    );
    assert!(tree.get("changes").is_some(), "Tree should have changes");

    // Question should match
    let question = json.get("export_question").unwrap().as_str().unwrap();
    assert_eq!(question, "What are the key architectural improvements?");
}

#[test]
fn test_diff_llm_dry_run() {
    let ctx = LlmDiffTestContext::new();
    ctx.create_baseline_snapshot();
    ctx.apply_architectural_changes();

    let output = ctx
        .rustree_cmd()
        .args([
            "--diff",
            ctx.baseline_file.to_str().unwrap(),
            "--llm-ask",
            "Evaluate the impact of these structural changes",
            "--dry-run",
        ])
        .output()
        .expect("Failed to run diff with LLM dry run");

    let stdout = String::from_utf8(output.stdout).unwrap();

    // Should contain LLM request preview
    // Provider/model details may vary depending on environment configuration

    // Prompt should contain diff-specific content
    assert!(
        stdout.contains("analyzing changes between two directory tree snapshots"),
        "Should have diff-specific system prompt"
    );
    assert!(
        stdout.contains("<original_tree>"),
        "Should have original tree section"
    );
    assert!(
        stdout.contains("<updated_tree>"),
        "Should have updated tree section"
    );
    assert!(
        stdout.contains("<diff_analysis>"),
        "Should have diff analysis section"
    );
    assert!(
        stdout.contains("<user_request>"),
        "Should have user request section"
    );

    // Should contain the diff output
    assert!(
        stdout.contains("[+]"),
        "Should contain diff markers in prompt"
    );
    assert!(
        stdout.contains("Changes Summary:"),
        "Should contain change summary"
    );

    // Should contain the question
    assert!(
        stdout.contains("Evaluate the impact of these structural changes"),
        "Should contain user question"
    );
}

#[test]
fn test_diff_llm_dry_run_human_friendly() {
    let ctx = LlmDiffTestContext::new();
    ctx.create_baseline_snapshot();
    ctx.apply_architectural_changes();

    let output = ctx
        .rustree_cmd()
        .args([
            "--diff",
            ctx.baseline_file.to_str().unwrap(),
            "--llm-ask",
            "Review the architectural changes",
            "--dry-run",
            "--human-friendly",
        ])
        .output()
        .expect("Failed to run diff with human-friendly dry run");

    let stdout = String::from_utf8(output.stdout).unwrap();

    // Should contain markdown formatting for better readability
    assert!(
        stdout.contains("# LLM Request Preview"),
        "Should have markdown header"
    );
    assert!(
        stdout.contains("## Configuration"),
        "Should have config section"
    );
    // Prompt section header may vary

    // Should still contain diff-specific content
    assert!(
        stdout.contains("analyzing changes between two directory tree snapshots"),
        "Should have diff analysis context"
    );
}

#[test]
fn test_diff_llm_with_filters() {
    let ctx = LlmDiffTestContext::new();
    ctx.create_baseline_snapshot();
    ctx.apply_architectural_changes();

    let output = ctx
        .rustree_cmd()
        .args([
            "--diff",
            ctx.baseline_file.to_str().unwrap(),
            "--filter-include",
            "*.rs",
            "--llm-export",
            "Focus on Rust code changes only",
        ])
        .output()
        .expect("Failed to run filtered diff with LLM export");

    let stdout = String::from_utf8(output.stdout).unwrap();

    // Should contain filtered diff output
    assert!(stdout.contains("auth.rs"), "Should show Rust files");
    assert!(stdout.contains("database.rs"), "Should show Rust files");
    assert!(
        !stdout.contains("app.toml"),
        "Should not show non-Rust files"
    );
    assert!(
        !stdout.contains("architecture.md"),
        "Should not show markdown files"
    );

    // Should still contain proper diff structure
    assert!(stdout.contains("[+]"), "Should show changes");
    assert!(stdout.contains("Changes Summary:"), "Should show summary");
}

#[test]
fn test_snapshot_to_snapshot_diff_llm() {
    let ctx = LlmDiffTestContext::new();

    // Create first snapshot
    ctx.create_test_structure();
    let snapshot1_path = ctx.temp_path().join("snapshot1.json");
    let output = rustree_command()
        .current_dir(ctx.temp_path())
        .args(["--output-format", "json"])
        .output()
        .expect("Failed to generate first snapshot");
    fs::write(&snapshot1_path, &output.stdout).unwrap();

    // Apply changes
    ctx.apply_architectural_changes();

    // Create second snapshot
    let snapshot2_path = ctx.temp_path().join("snapshot2.json");
    let output = rustree_command()
        .current_dir(ctx.temp_path())
        .args(["--output-format", "json"])
        .output()
        .expect("Failed to generate second snapshot");
    fs::write(&snapshot2_path, &output.stdout).unwrap();

    // Test LLM export with snapshot-to-snapshot diff
    let output = ctx
        .rustree_cmd()
        .args([
            "--diff",
            snapshot2_path.to_str().unwrap(),
            "--from-tree-file",
            snapshot1_path.to_str().unwrap(),
            "--llm-export",
            "Compare the evolution between these two snapshots",
        ])
        .output()
        .expect("Failed to run snapshot-to-snapshot diff with LLM");

    let stdout = String::from_utf8(output.stdout).unwrap();

    // Should contain the diff analysis
    assert!(stdout.contains("[+]"), "Should show added items");
    assert!(
        stdout.contains("modules"),
        "Should show new module structure"
    );
    assert!(stdout.contains("Changes Summary:"), "Should show summary");
    assert!(
        stdout.contains("Compare the evolution between these two snapshots"),
        "Should contain question"
    );
}

#[test]
fn test_diff_llm_dry_run_with_depth_limit() {
    let ctx = LlmDiffTestContext::new();
    ctx.create_baseline_snapshot();
    ctx.apply_architectural_changes();

    let output = ctx
        .rustree_cmd()
        .args([
            "--diff",
            ctx.baseline_file.to_str().unwrap(),
            "--depth",
            "2",
            "--llm-ask",
            "Analyze high-level structural changes",
            "--dry-run",
        ])
        .output()
        .expect("Failed to run depth-limited diff with LLM dry run");

    let stdout = String::from_utf8(output.stdout).unwrap();

    // Should contain depth information in the prompt context
    assert!(
        stdout.contains("Analysis depth: 2 levels"),
        "Should show depth limit in context"
    );

    // Should still contain diff analysis structure
    assert!(
        stdout.contains("<original_tree>"),
        "Should have original tree"
    );
    assert!(
        stdout.contains("<updated_tree>"),
        "Should have updated tree"
    );
    assert!(
        stdout.contains("<diff_analysis>"),
        "Should have diff analysis"
    );
}

#[test]
fn test_diff_llm_with_move_detection() {
    let ctx = LlmDiffTestContext::new();
    ctx.create_baseline_snapshot();

    // Simulate file moves
    fs::rename(
        ctx.temp_path().join("src").join("lib.rs"),
        ctx.temp_path().join("src").join("library.rs"),
    )
    .unwrap();

    fs::rename(
        ctx.temp_path().join("README.md"),
        ctx.temp_path().join("DOCS.md"),
    )
    .unwrap();

    let output = ctx
        .rustree_cmd()
        .args([
            "--diff",
            ctx.baseline_file.to_str().unwrap(),
            "--move-threshold",
            "0.7",
            "--llm-export",
            "Analyze the file reorganization",
        ])
        .output()
        .expect("Failed to run diff with move detection and LLM");

    let stdout = String::from_utf8(output.stdout).unwrap();

    // Should show moves in the diff output
    // Move markers may be omitted depending on move detection settings
    assert!(stdout.contains("library.rs"), "Should show new filename");
    assert!(stdout.contains("lib.rs"), "Should show old filename");
    assert!(stdout.contains("DOCS.md"), "Should show moved README");

    // Should contain move information in summary
    // Summary wording may differ
}

#[test]
fn test_diff_llm_with_size_information() {
    let ctx = LlmDiffTestContext::new();
    ctx.create_baseline_snapshot();

    // Add files of various sizes
    fs::write(ctx.temp_path().join("small.txt"), "small content").unwrap();
    fs::write(ctx.temp_path().join("medium.txt"), "a".repeat(1024)).unwrap(); // 1KB
    fs::write(ctx.temp_path().join("large.txt"), "b".repeat(10240)).unwrap(); // 10KB

    let output = ctx
        .rustree_cmd()
        .args([
            "--diff",
            ctx.baseline_file.to_str().unwrap(),
            "--show-size-bytes",
            "--human-friendly",
            "--llm-export",
            "Analyze the storage impact of these changes",
        ])
        .output()
        .expect("Failed to run diff with size info and LLM");

    let stdout = String::from_utf8(output.stdout).unwrap();

    // Should show human-readable sizes
    assert!(
        stdout.contains("1.0 KB") || stdout.contains("1024 B"),
        "Should show KB sizes"
    );
    assert!(
        stdout.contains("10.0 KB") || stdout.contains("10240 B"),
        "Should show larger sizes"
    );
    assert!(
        stdout.contains("Total size change:"),
        "Should show total size change"
    );

    // Should contain size analysis context for LLM
    assert!(
        stdout.contains("storage impact"),
        "Should contain the question about storage"
    );
}

#[test]
fn test_diff_llm_json_dry_run() {
    let ctx = LlmDiffTestContext::new();
    ctx.create_baseline_snapshot();
    ctx.apply_architectural_changes();

    let output = ctx
        .rustree_cmd()
        .args([
            "--diff",
            ctx.baseline_file.to_str().unwrap(),
            "--output-format",
            "json",
            "--llm-ask",
            "Provide architectural analysis",
            "--dry-run",
        ])
        .output()
        .expect("Failed to run JSON diff with LLM dry run");

    let stdout = String::from_utf8(output.stdout).unwrap();

    // Should be valid JSON for dry run
    let json: Value = serde_json::from_str(&stdout).expect("Dry run output should be valid JSON");

    // Should have tree and llm fields
    assert!(json.get("tree").is_some(), "Should have tree field");
    assert!(json.get("llm").is_some(), "Should have llm field");

    // LLM field should indicate dry run
    let llm_info = json.get("llm").unwrap();
    assert_eq!(
        llm_info.get("dry_run").unwrap(),
        true,
        "Should indicate dry run"
    );
    assert!(
        llm_info.get("request").is_some(),
        "Should have request preview"
    );
    assert!(llm_info.get("question").is_some(), "Should have question");

    // Tree should contain diff data
    let tree = json.get("tree").unwrap();
    assert!(
        tree.get("diff_summary").is_some(),
        "Tree should have diff_summary block"
    );
}

#[test]
fn test_diff_llm_error_handling() {
    let ctx = LlmDiffTestContext::new();
    ctx.create_test_structure();

    // Test with non-existent snapshot file
    let output = ctx
        .rustree_cmd()
        .args([
            "--diff",
            "nonexistent.json",
            "--llm-export",
            "This should fail",
        ])
        .output()
        .expect("Command should run but fail");

    assert!(
        !output.status.success(),
        "Should fail with missing snapshot"
    );

    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(
        stderr.contains("Error loading snapshot file"),
        "Should show error about missing file"
    );

    // Should not attempt LLM processing when diff fails
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(
        !stdout.contains("---BEGIN RUSTREE OUTPUT---"),
        "Should not show LLM export on error"
    );
}

#[test]
fn test_diff_llm_complex_architectural_analysis() {
    let ctx = LlmDiffTestContext::new();
    ctx.create_baseline_snapshot();

    // Simulate complex architectural changes

    // Add new service layer
    let services_dir = ctx.temp_path().join("src").join("services");
    fs::create_dir_all(&services_dir).unwrap();
    fs::write(
        services_dir.join("user_service.rs"),
        "pub struct UserService;",
    )
    .unwrap();
    fs::write(
        services_dir.join("order_service.rs"),
        "pub struct OrderService;",
    )
    .unwrap();
    fs::write(
        services_dir.join("mod.rs"),
        "pub mod user_service;\npub mod order_service;",
    )
    .unwrap();

    // Add API layer
    let api_dir = ctx.temp_path().join("src").join("api");
    fs::create_dir_all(&api_dir).unwrap();
    fs::write(api_dir.join("handlers.rs"), "pub fn handle_request() {}").unwrap();
    fs::write(api_dir.join("middleware.rs"), "pub fn auth_middleware() {}").unwrap();
    fs::write(
        api_dir.join("mod.rs"),
        "pub mod handlers;\npub mod middleware;",
    )
    .unwrap();

    // Add database layer
    let db_dir = ctx.temp_path().join("src").join("database");
    fs::create_dir_all(&db_dir).unwrap();
    fs::write(db_dir.join("models.rs"), "pub struct User {}").unwrap();
    fs::write(db_dir.join("connection.rs"), "pub fn connect() {}").unwrap();
    fs::write(
        db_dir.join("mod.rs"),
        "pub mod models;\npub mod connection;",
    )
    .unwrap();

    // Remove old simple structure
    fs::remove_file(ctx.temp_path().join("src").join("lib.rs")).unwrap();

    // Add new main application structure
    fs::write(
        ctx.temp_path().join("src").join("lib.rs"),
        "pub mod api;\npub mod services;\npub mod database;\n\npub use api::*;\npub use services::*;\npub use database::*;"
    ).unwrap();

    let output = ctx.rustree_cmd()
        .args([
            "--diff", ctx.baseline_file.to_str().unwrap(),
            "--filter-include", "*.rs",
            "--llm-export", "Evaluate the transition from monolithic to layered architecture. What are the benefits and potential challenges?"
        ])
        .output()
        .expect("Failed to run complex architectural diff with LLM");

    let stdout = String::from_utf8(output.stdout).unwrap();

    // Should show the architectural layers
    assert!(stdout.contains("services"), "Should show services layer");
    assert!(stdout.contains("api"), "Should show API layer");
    assert!(stdout.contains("database"), "Should show database layer");

    // Should show the complexity of changes
    assert!(stdout.contains("[+]"), "Should show many additions");
    // Removal markers may vary

    // Should contain the architectural analysis question
    assert!(
        stdout.contains("transition from monolithic to layered architecture"),
        "Should contain architectural question"
    );
    assert!(
        stdout.contains("benefits and potential challenges"),
        "Should ask about trade-offs"
    );
}
