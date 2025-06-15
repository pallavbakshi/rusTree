// tests/diff_cli_integration_tests.rs

//! Integration tests for the diff CLI functionality
//! Tests the complete flow from CLI arguments to diff output

use serde_json::Value;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use tempfile::{TempDir, tempdir};

/// Returns a `Command` configured to run the compiled `rustree` binary.
fn rustree_command() -> Command {
    let exe = env!("CARGO_BIN_EXE_rustree");
    Command::new(exe)
}

/// Test helper struct that manages temporary directories and snapshots
struct DiffTestContext {
    temp_dir: TempDir,
    baseline_file: PathBuf,
}

impl DiffTestContext {
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
        fs::write(src_dir.join("main.rs"), "fn main() {}").unwrap();
        fs::write(src_dir.join("lib.rs"), "// Library code").unwrap();

        let tests_dir = self.temp_path().join("tests");
        fs::create_dir_all(&tests_dir).unwrap();
        fs::write(tests_dir.join("integration.rs"), "// Tests").unwrap();

        fs::write(
            self.temp_path().join("Cargo.toml"),
            "[package]\nname = \"test\"",
        )
        .unwrap();
        fs::write(self.temp_path().join("README.md"), "# Test Project").unwrap();
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

    fn modify_structure(&self) {
        // Add a new file
        fs::write(
            self.temp_path().join("src").join("utils.rs"),
            "// Utility functions",
        )
        .unwrap();

        // Remove a file
        fs::remove_file(self.temp_path().join("tests").join("integration.rs")).unwrap();

        // Modify an existing file
        fs::write(
            self.temp_path().join("src").join("main.rs"),
            "fn main() {\n    println!(\"Hello, world!\");\n}",
        )
        .unwrap();

        // Add a new directory
        let docs_dir = self.temp_path().join("docs");
        fs::create_dir_all(&docs_dir).unwrap();
        fs::write(docs_dir.join("guide.md"), "# User Guide").unwrap();
    }

    fn rustree_cmd(&self) -> Command {
        let mut cmd = rustree_command();
        cmd.current_dir(self.temp_path());
        cmd
    }
}

#[test]
fn test_basic_diff_functionality() {
    let ctx = DiffTestContext::new();
    ctx.create_baseline_snapshot();
    ctx.modify_structure();

    let output = ctx
        .rustree_cmd()
        .args(["--diff", ctx.baseline_file.to_str().unwrap()])
        .output()
        .expect("Failed to run diff");

    let stdout = String::from_utf8(output.stdout).unwrap();

    // Should show added, removed, and modified items
    assert!(stdout.contains("[+]"), "Should show added items");
    assert!(stdout.contains("[-]"), "Should show removed items");
    assert!(stdout.contains("Changes Summary:"), "Should show summary");

    // Should show specific changes
    assert!(stdout.contains("utils.rs"), "Should show added utils.rs");
    assert!(
        stdout.contains("integration.rs"),
        "Should show removed integration.rs"
    );
    assert!(stdout.contains("docs"), "Should show added docs directory");
}

#[test]
fn test_diff_with_depth_filter() {
    let ctx = DiffTestContext::new();
    ctx.create_baseline_snapshot();
    ctx.modify_structure();

    let output = ctx
        .rustree_cmd()
        .args([
            "--diff",
            ctx.baseline_file.to_str().unwrap(),
            "--depth",
            "1",
        ])
        .output()
        .expect("Failed to run diff with depth filter");

    let stdout = String::from_utf8(output.stdout).unwrap();

    // Should still show changes but limited depth
    assert!(stdout.contains("Changes Summary:"));
    assert!(stdout.contains("[+]") || stdout.contains("[-]"));
}

#[test]
fn test_diff_with_include_filter() {
    let ctx = DiffTestContext::new();
    ctx.create_baseline_snapshot();
    ctx.modify_structure();

    let output = ctx
        .rustree_cmd()
        .args([
            "--diff",
            ctx.baseline_file.to_str().unwrap(),
            "--filter-include",
            "*.rs",
        ])
        .output()
        .expect("Failed to run diff with include filter");

    let stdout = String::from_utf8(output.stdout).unwrap();

    // Should show Rust files but not other files
    assert!(stdout.contains("utils.rs"), "Should show added Rust file");
    assert!(
        stdout.contains("integration.rs"),
        "Should show removed Rust file"
    );
    assert!(
        !stdout.contains("guide.md"),
        "Should not show markdown files when filtering for Rust"
    );
}

#[test]
fn test_diff_json_output() {
    let ctx = DiffTestContext::new();
    ctx.create_baseline_snapshot();
    ctx.modify_structure();

    let output = ctx
        .rustree_cmd()
        .args([
            "--diff",
            ctx.baseline_file.to_str().unwrap(),
            "--output-format",
            "json",
        ])
        .output()
        .expect("Failed to run diff with JSON output");

    let stdout = String::from_utf8(output.stdout).unwrap();

    // Should be valid JSON
    let json: Value = serde_json::from_str(&stdout).expect("Output should be valid JSON");

    // Should contain expected top-level fields
    assert!(
        json.get("diff_summary").is_some(),
        "Should have diff_summary block"
    );
    assert!(json.get("changes").is_some(), "Should have changes array");
    assert!(
        json.get("diff_metadata").is_some(),
        "Should have diff_metadata block"
    );

    // Check summary contains expected fields
    let summary = json.get("diff_summary").unwrap();
    assert!(summary.get("added").is_some());
    assert!(summary.get("removed").is_some());
}

#[test]
fn test_diff_markdown_output() {
    let ctx = DiffTestContext::new();
    ctx.create_baseline_snapshot();
    ctx.modify_structure();

    let output = ctx
        .rustree_cmd()
        .args([
            "--diff",
            ctx.baseline_file.to_str().unwrap(),
            "--output-format",
            "markdown",
        ])
        .output()
        .expect("Failed to run diff with markdown output");

    let stdout = String::from_utf8(output.stdout).unwrap();

    // Should contain markdown formatting
    assert!(
        stdout.contains("# Directory Changes"),
        "Should have markdown header"
    );
    // Should contain bullet lists summarising changes
    assert!(
        stdout.contains("- **"),
        "Should have markdown bullet summaries"
    );
}

#[test]
fn test_diff_move_detection() {
    let ctx = DiffTestContext::new();
    ctx.create_test_structure();

    // Create initial structure with a file to move
    fs::write(
        ctx.temp_path().join("old_file.rs"),
        "// This file will be moved",
    )
    .unwrap();

    // Generate baseline
    let output = rustree_command()
        .current_dir(ctx.temp_path())
        .args(["--output-format", "json"])
        .output()
        .expect("Failed to generate baseline");
    fs::write(&ctx.baseline_file, &output.stdout).unwrap();

    // Move the file (rename it)
    fs::rename(
        ctx.temp_path().join("old_file.rs"),
        ctx.temp_path().join("new_file.rs"),
    )
    .unwrap();

    let output = ctx
        .rustree_cmd()
        .args([
            "--diff",
            ctx.baseline_file.to_str().unwrap(),
            "--move-threshold",
            "0.5",
        ])
        .output()
        .expect("Failed to run diff with move detection");

    let stdout = String::from_utf8(output.stdout).unwrap();

    // Should detect the move
    assert!(stdout.contains("[~]"), "Should show moved file marker");
    assert!(stdout.contains("new_file.rs"), "Should show new filename");
    assert!(stdout.contains("old_file.rs"), "Should show old filename");
    assert!(
        stdout.contains("moved/renamed"),
        "Should show move description in summary"
    );
}

#[test]
fn test_diff_ignore_moves() {
    let ctx = DiffTestContext::new();
    ctx.create_test_structure();

    // Create file to move
    fs::write(ctx.temp_path().join("move_me.rs"), "// File content").unwrap();

    // Generate baseline
    let output = rustree_command()
        .current_dir(ctx.temp_path())
        .args(["--output-format", "json"])
        .output()
        .expect("Failed to generate baseline");
    fs::write(&ctx.baseline_file, &output.stdout).unwrap();

    // Move the file
    fs::rename(
        ctx.temp_path().join("move_me.rs"),
        ctx.temp_path().join("moved_file.rs"),
    )
    .unwrap();

    let output = ctx
        .rustree_cmd()
        .args([
            "--diff",
            ctx.baseline_file.to_str().unwrap(),
            "--ignore-moves",
        ])
        .output()
        .expect("Failed to run diff with moves ignored");

    let stdout = String::from_utf8(output.stdout).unwrap();

    // Should treat as separate add and remove
    assert!(!stdout.contains("[~]"), "Should not show moved file marker");
    assert!(stdout.contains("[+]"), "Should show added file");
    assert!(stdout.contains("[-]"), "Should show removed file");
    assert!(
        stdout.contains("moved_file.rs"),
        "Should show new file as added"
    );
    assert!(
        stdout.contains("move_me.rs"),
        "Should show old file as removed"
    );
}

#[test]
fn test_snapshot_to_snapshot_diff() {
    let ctx = DiffTestContext::new();

    // Create first snapshot
    ctx.create_test_structure();
    let snapshot1_path = ctx.temp_path().join("snapshot1.json");
    let output = rustree_command()
        .current_dir(ctx.temp_path())
        .args(["--output-format", "json"])
        .output()
        .expect("Failed to generate first snapshot");
    fs::write(&snapshot1_path, &output.stdout).unwrap();

    // Modify structure
    ctx.modify_structure();

    // Create second snapshot
    let snapshot2_path = ctx.temp_path().join("snapshot2.json");
    let output = rustree_command()
        .current_dir(ctx.temp_path())
        .args(["--output-format", "json"])
        .output()
        .expect("Failed to generate second snapshot");
    fs::write(&snapshot2_path, &output.stdout).unwrap();

    // Compare snapshots
    let output = ctx
        .rustree_cmd()
        .args([
            "--diff",
            snapshot2_path.to_str().unwrap(),
            "--from-tree-file",
            snapshot1_path.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to run snapshot-to-snapshot diff");

    let stdout = String::from_utf8(output.stdout).unwrap();

    // Should show the same changes as filesystem diff
    assert!(stdout.contains("Changes Summary:"));
    assert!(stdout.contains("utils.rs"), "Should show added file");
    assert!(
        stdout.contains("integration.rs"),
        "Should show removed file"
    );
    assert!(stdout.contains("docs"), "Should show added directory");
}

#[test]
fn test_diff_with_size_info() {
    let ctx = DiffTestContext::new();
    ctx.create_baseline_snapshot();

    // Add a large file
    let large_content = "a".repeat(1024); // 1KB file
    fs::write(ctx.temp_path().join("large_file.txt"), large_content).unwrap();

    let output = ctx
        .rustree_cmd()
        .args([
            "--diff",
            ctx.baseline_file.to_str().unwrap(),
            "--show-size-bytes",
        ])
        .output()
        .expect("Failed to run diff with size info");

    let stdout = String::from_utf8(output.stdout).unwrap();

    // Should show size information
    assert!(
        stdout.contains("large_file.txt"),
        "Should show the added file"
    );
    assert!(
        stdout.contains("1024 B") || stdout.contains("1.0 KB"),
        "Should show file size"
    );
    assert!(
        stdout.contains("Total size change:"),
        "Should show total size change in summary"
    );
}

#[test]
fn test_diff_stats_only() {
    let ctx = DiffTestContext::new();
    ctx.create_baseline_snapshot();
    ctx.modify_structure();

    let output = ctx
        .rustree_cmd()
        .args([
            "--diff",
            ctx.baseline_file.to_str().unwrap(),
            "--stats-only",
        ])
        .output()
        .expect("Failed to run diff with stats only");

    let stdout = String::from_utf8(output.stdout).unwrap();

    // Should at least show summary of changes
    assert!(stdout.contains("Changes Summary:"), "Should show summary");
}

#[test]
fn test_diff_show_unchanged() {
    let ctx = DiffTestContext::new();
    ctx.create_baseline_snapshot();

    // Make minimal change
    fs::write(ctx.temp_path().join("new_file.rs"), "// New file").unwrap();

    let output = ctx
        .rustree_cmd()
        .args([
            "--diff",
            ctx.baseline_file.to_str().unwrap(),
            "--show-unchanged",
        ])
        .output()
        .expect("Failed to run diff with unchanged files");

    let stdout = String::from_utf8(output.stdout).unwrap();

    // Should show unchanged files
    assert!(stdout.contains("[+]"), "Should show added file");
    // Unchanged file details should be present in diff output
    assert!(stdout.contains("Cargo.toml") || stdout.contains("unchanged"));
    // Ensure summary is present
    assert!(stdout.contains("Changes Summary:"));
}

#[test]
fn test_diff_nonexistent_snapshot() {
    let ctx = DiffTestContext::new();
    ctx.create_test_structure();

    let output = ctx
        .rustree_cmd()
        .args(["--diff", "nonexistent.json"])
        .output()
        .expect("Command should run but fail");

    assert!(
        !output.status.success(),
        "Should fail with nonexistent snapshot"
    );

    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(
        stderr.contains("Error loading snapshot file"),
        "Should show error message"
    );
}

#[test]
fn test_diff_invalid_json_snapshot() {
    let ctx = DiffTestContext::new();
    ctx.create_test_structure();

    // Create invalid JSON file
    let invalid_json = ctx.temp_path().join("invalid.json");
    fs::write(&invalid_json, "{ invalid json }").unwrap();

    let output = ctx
        .rustree_cmd()
        .args(["--diff", invalid_json.to_str().unwrap()])
        .output()
        .expect("Command should run but fail");

    assert!(!output.status.success(), "Should fail with invalid JSON");

    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("Error"), "Should show error message");
}

#[test]
fn test_diff_with_path_scope() {
    let ctx = DiffTestContext::new();
    ctx.create_baseline_snapshot();
    ctx.modify_structure();

    let output = ctx
        .rustree_cmd()
        .args(["--diff", ctx.baseline_file.to_str().unwrap(), "src/"])
        .output()
        .expect("Failed to run diff with path scope");

    let stdout = String::from_utf8(output.stdout).unwrap();

    // Should only show changes in src/ directory
    assert!(stdout.contains("utils.rs"), "Should show changes in src/");
    assert!(
        !stdout.contains("docs"),
        "Should not show changes outside src/"
    );
    assert!(!stdout.contains("guide.md"), "Should not show docs changes");
}

#[test]
fn test_diff_human_readable_sizes() {
    let ctx = DiffTestContext::new();
    ctx.create_baseline_snapshot();

    // Add files of different sizes
    fs::write(ctx.temp_path().join("small.txt"), "small").unwrap();
    fs::write(ctx.temp_path().join("medium.txt"), "a".repeat(2048)).unwrap(); // 2KB
    fs::write(ctx.temp_path().join("large.txt"), "b".repeat(1024 * 1024)).unwrap(); // 1MB

    let output = ctx
        .rustree_cmd()
        .args([
            "--diff",
            ctx.baseline_file.to_str().unwrap(),
            "--show-size-bytes",
            "--human-friendly",
        ])
        .output()
        .expect("Failed to run diff with human-readable sizes");

    let stdout = String::from_utf8(output.stdout).unwrap();

    // Should show human-readable sizes
    assert!(stdout.contains("small.txt"), "Should show small file");
    assert!(
        stdout.contains("2.0 KB") || stdout.contains("2048 B"),
        "Should show medium file size"
    );
    assert!(
        stdout.contains("1.0 MB"),
        "Should show large file size in human format"
    );
}
