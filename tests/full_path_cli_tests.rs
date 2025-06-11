// tests/full_path_cli_tests.rs

#![allow(clippy::needless_update)]

use anyhow::Result;
use std::fs::{self, File};
use std::io::Write;
use std::process::{Command, Stdio};
use tempfile::TempDir;

/// Helper function to create a test directory structure for CLI testing
fn setup_cli_test_directory() -> Result<TempDir> {
    let temp_dir = tempfile::TempDir::new()?;
    let root = temp_dir.path();

    // Create structure:
    // test_root/
    //   ├── file_a.txt
    //   ├── nested/
    //   │   ├── file_b.txt
    //   │   └── deep/
    //   │       └── file_c.txt
    //   └── other/
    //       └── file_d.txt

    let nested = root.join("nested");
    let deep = nested.join("deep");
    let other = root.join("other");

    fs::create_dir_all(&nested)?;
    fs::create_dir_all(&deep)?;
    fs::create_dir_all(&other)?;

    File::create(root.join("file_a.txt"))?.write_all(b"content a")?;
    File::create(nested.join("file_b.txt"))?.write_all(b"content b")?;
    File::create(deep.join("file_c.txt"))?.write_all(b"content c")?;
    File::create(other.join("file_d.txt"))?.write_all(b"content d")?;

    Ok(temp_dir)
}

/// Helper function to run the rustree binary with given arguments
fn run_rustree_binary(args: &[&str]) -> Result<String> {
    let mut cmd_args = vec!["run", "--"];
    cmd_args.extend_from_slice(args);

    let output = Command::new("cargo")
        .args(&cmd_args)
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Binary execution failed: {}", stderr);
    }

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

#[test]
fn test_cli_full_path_basic_functionality() -> Result<()> {
    let temp_dir = setup_cli_test_directory()?;
    let root_path = temp_dir.path();

    // Test without full-path flag
    let output_normal = run_rustree_binary(&[root_path.to_str().unwrap()])?;

    // Test with full-path flag
    let output_full_path = run_rustree_binary(&["-f", root_path.to_str().unwrap()])?;

    // Normal output should show just filenames
    assert!(output_normal.contains("file_b.txt"));
    assert!(output_normal.contains("file_c.txt"));
    assert!(output_normal.contains("file_d.txt"));

    // Full-path output should show relative paths
    assert!(output_full_path.contains("nested/file_b.txt"));
    assert!(output_full_path.contains("nested/deep/file_c.txt"));
    assert!(output_full_path.contains("other/file_d.txt"));

    // Both should show root-level files the same way
    assert!(output_normal.contains("file_a.txt"));
    assert!(output_full_path.contains("file_a.txt"));

    Ok(())
}

#[test]
fn test_cli_full_path_with_depth_limit() -> Result<()> {
    let temp_dir = setup_cli_test_directory()?;
    let root_path = temp_dir.path();

    let output = run_rustree_binary(&["-f", "-L", "2", root_path.to_str().unwrap()])?;

    // Should show paths up to depth 2
    assert!(output.contains("nested/file_b.txt"));
    assert!(output.contains("other/file_d.txt"));

    // Should NOT show depth 3 files
    assert!(!output.contains("nested/deep/file_c.txt"));

    Ok(())
}

#[test]
fn test_cli_full_path_with_markdown_format() -> Result<()> {
    let temp_dir = setup_cli_test_directory()?;
    let root_path = temp_dir.path();

    let output = run_rustree_binary(&[
        "-f",
        "--output-format",
        "markdown",
        root_path.to_str().unwrap(),
    ])?;

    // Should be markdown format with full paths
    assert!(output.starts_with("#"));
    assert!(output.contains("* nested/file_b.txt"));
    assert!(output.contains("* nested/deep/file_c.txt"));
    assert!(output.contains("* other/file_d.txt"));

    Ok(())
}

#[test]
fn test_cli_full_path_with_directories_only() -> Result<()> {
    let temp_dir = setup_cli_test_directory()?;
    let root_path = temp_dir.path();

    let output = run_rustree_binary(&["-f", "-d", root_path.to_str().unwrap()])?;

    // Should show directory paths only
    assert!(output.contains("nested/deep/"));

    // Should NOT show any files
    assert!(!output.contains("file_a.txt"));
    assert!(!output.contains("nested/file_b.txt"));
    assert!(!output.contains("nested/deep/file_c.txt"));
    assert!(!output.contains("other/file_d.txt"));

    Ok(())
}

#[test]
fn test_cli_full_path_with_size_metadata() -> Result<()> {
    let temp_dir = setup_cli_test_directory()?;
    let root_path = temp_dir.path();

    let output = run_rustree_binary(&["-f", "-s", root_path.to_str().unwrap()])?;

    // Should show both size metadata and full paths
    // Look for pattern like "[     9B] nested/file_b.txt"
    let lines: Vec<&str> = output.lines().collect();
    let file_b_line = lines.iter().find(|line| line.contains("nested/file_b.txt"));
    assert!(
        file_b_line.is_some(),
        "Should contain full path for file_b.txt"
    );

    let file_b_line = file_b_line.unwrap();
    assert!(
        file_b_line.contains("[") && file_b_line.contains("B]"),
        "Should show size metadata with full path: {}",
        file_b_line
    );

    Ok(())
}

#[test]
fn test_cli_help_contains_full_path_option() -> Result<()> {
    let output = Command::new("cargo")
        .args(["run", "--", "--help"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()?;

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should contain our full-path option
    assert!(stdout.contains("--full-path") || stdout.contains("-f"));
    assert!(stdout.contains("Print the full path prefix"));

    Ok(())
}

#[test]
fn test_cli_full_path_short_and_long_flags() -> Result<()> {
    let temp_dir = setup_cli_test_directory()?;
    let root_path = temp_dir.path();

    // Test short flag
    let output_short = run_rustree_binary(&["-f", root_path.to_str().unwrap()])?;

    // Test long flag
    let output_long = run_rustree_binary(&["--full-path", root_path.to_str().unwrap()])?;

    // Both should produce the same output
    assert_eq!(output_short, output_long);

    // Both should show full paths
    assert!(output_short.contains("nested/file_b.txt"));
    assert!(output_long.contains("nested/file_b.txt"));

    Ok(())
}

#[test]
fn test_cli_full_path_with_multiple_flags() -> Result<()> {
    let temp_dir = setup_cli_test_directory()?;
    let root_path = temp_dir.path();

    // Test combination with hidden files and size
    let output = run_rustree_binary(&["-f", "-a", "-s", root_path.to_str().unwrap()])?;

    // Should work without errors and show full paths
    assert!(output.contains("nested/file_b.txt"));
    assert!(output.contains("[") && output.contains("B]")); // Size info

    Ok(())
}

#[test]
fn test_cli_full_path_empty_directory() -> Result<()> {
    let temp_dir = tempfile::TempDir::new()?;
    let root_path = temp_dir.path();

    let output = run_rustree_binary(&["-f", root_path.to_str().unwrap()])?;

    // Empty directory should still work (root directory counts as 1 directory)
    assert!(output.contains("1 directory, 0 files"));

    Ok(())
}
