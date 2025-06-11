// tests/no_summary_report_tests.rs
//
// Integration tests for the --no-summary-report CLI flag

use anyhow::Result;
use std::fs;
use std::process::Command;
use tempfile::TempDir;

mod common;
use common::common_test_utils;

#[test]
fn test_cli_no_summary_report_flag() -> Result<()> {
    let temp_dir = common_test_utils::setup_test_directory()?;
    let root_path = temp_dir.path();

    // Test default behavior (with summary)
    let output_with_summary = Command::new(env!("CARGO_BIN_EXE_rustree"))
        .arg(root_path.to_str().unwrap())
        .arg("-L")
        .arg("2")
        .output()
        .expect("Failed to execute rustree");

    let stdout_with_summary = String::from_utf8(output_with_summary.stdout)?;

    // Should contain summary line
    assert!(stdout_with_summary.contains("directories"));
    assert!(stdout_with_summary.contains("files"));
    assert!(stdout_with_summary.trim().ends_with("files"));

    // Test with --no-summary-report flag
    let output_no_summary = Command::new(env!("CARGO_BIN_EXE_rustree"))
        .arg(root_path.to_str().unwrap())
        .arg("-L")
        .arg("2")
        .arg("--no-summary-report")
        .output()
        .expect("Failed to execute rustree");

    let stdout_no_summary = String::from_utf8(output_no_summary.stdout)?;

    // Should NOT contain summary line
    assert!(!stdout_no_summary.contains("directories"));
    assert!(!stdout_no_summary.contains("files"));
    assert!(!stdout_no_summary.trim().ends_with("files"));

    // But should still contain the tree structure
    assert!(stdout_no_summary.contains("file1.txt"));
    assert!(stdout_no_summary.contains("sub_dir"));

    Ok(())
}

#[test]
fn test_cli_no_summary_report_with_markdown() -> Result<()> {
    let temp_dir = common_test_utils::setup_test_directory()?;
    let root_path = temp_dir.path();

    // Test markdown format with summary (default)
    let output_with_summary = Command::new(env!("CARGO_BIN_EXE_rustree"))
        .arg(root_path.to_str().unwrap())
        .arg("-L")
        .arg("2")
        .arg("--output-format")
        .arg("markdown")
        .output()
        .expect("Failed to execute rustree");

    let stdout_with_summary = String::from_utf8(output_with_summary.stdout)?;

    // Should contain markdown summary line
    assert!(stdout_with_summary.contains("__"));
    assert!(stdout_with_summary.contains("total__"));
    assert!(stdout_with_summary.contains("director"));

    // Test markdown format with --no-summary-report
    let output_no_summary = Command::new(env!("CARGO_BIN_EXE_rustree"))
        .arg(root_path.to_str().unwrap())
        .arg("-L")
        .arg("2")
        .arg("--output-format")
        .arg("markdown")
        .arg("--no-summary-report")
        .output()
        .expect("Failed to execute rustree");

    let stdout_no_summary = String::from_utf8(output_no_summary.stdout)?;

    // Should NOT contain markdown summary line
    assert!(!stdout_no_summary.contains("total__"));
    assert!(!stdout_no_summary.contains("__"));

    // But should still be markdown format
    assert!(stdout_no_summary.starts_with("#"));
    assert!(stdout_no_summary.contains("* "));

    Ok(())
}

#[test]
fn test_cli_no_summary_report_with_directories_only() -> Result<()> {
    let temp_dir = common_test_utils::setup_test_directory()?;
    let root_path = temp_dir.path();

    // Test directories only with --no-summary-report
    let output = Command::new(env!("CARGO_BIN_EXE_rustree"))
        .arg(root_path.to_str().unwrap())
        .arg("-L")
        .arg("2")
        .arg("-d") // directories only
        .arg("--no-summary-report")
        .output()
        .expect("Failed to execute rustree");

    let stdout = String::from_utf8(output.stdout)?;

    // Should NOT contain summary line
    assert!(!stdout.contains("directories"));
    assert!(!stdout.contains("files"));

    // Should show only directories
    assert!(stdout.contains("sub_dir"));
    assert!(!stdout.contains("file1.txt"));
    assert!(!stdout.contains("file2.log"));

    Ok(())
}

#[test]
fn test_cli_no_summary_report_with_hidden_files() -> Result<()> {
    let temp_dir = common_test_utils::setup_test_directory()?;
    let root_path = temp_dir.path();

    // Test with hidden files and --no-summary-report
    let output = Command::new(env!("CARGO_BIN_EXE_rustree"))
        .arg(root_path.to_str().unwrap())
        .arg("-L")
        .arg("2")
        .arg("-a") // show hidden files
        .arg("--no-summary-report")
        .output()
        .expect("Failed to execute rustree");

    let stdout = String::from_utf8(output.stdout)?;

    // Should NOT contain summary line
    assert!(!stdout.contains("directories"));
    assert!(!stdout.contains("files"));

    // Should show hidden files
    assert!(stdout.contains(".hidden_file"));

    Ok(())
}

#[test]
fn test_cli_no_summary_report_help_text() -> Result<()> {
    // Test that the flag appears in help
    let output = Command::new(env!("CARGO_BIN_EXE_rustree"))
        .arg("--help")
        .output()
        .expect("Failed to execute rustree");

    let stdout = String::from_utf8(output.stdout)?;

    // Should contain the flag and its description
    assert!(stdout.contains("--no-summary-report"));
    assert!(stdout.contains("Omits printing of the file and directory report"));

    Ok(())
}

#[test]
fn test_cli_no_summary_report_single_file() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let root_path = temp_dir.path();

    // Create a single file
    fs::write(root_path.join("single.txt"), "content")?;

    // Test with single file and --no-summary-report
    let output = Command::new(env!("CARGO_BIN_EXE_rustree"))
        .arg(root_path.to_str().unwrap())
        .arg("--no-summary-report")
        .output()
        .expect("Failed to execute rustree");

    let stdout = String::from_utf8(output.stdout)?;

    // Should NOT contain summary line
    assert!(!stdout.contains("directory"));
    assert!(!stdout.contains("file"));

    // Should show the single file
    assert!(stdout.contains("single.txt"));

    Ok(())
}

#[test]
fn test_cli_no_summary_report_empty_directory() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let root_path = temp_dir.path();

    // Test with empty directory and --no-summary-report
    let output = Command::new(env!("CARGO_BIN_EXE_rustree"))
        .arg(root_path.to_str().unwrap())
        .arg("--no-summary-report")
        .output()
        .expect("Failed to execute rustree");

    let stdout = String::from_utf8(output.stdout)?;

    // Should NOT contain summary line
    assert!(!stdout.contains("directories"));
    assert!(!stdout.contains("files"));

    // Output should be minimal - just the root directory name
    let lines: Vec<&str> = stdout.lines().collect();
    assert!(!lines.is_empty());

    Ok(())
}
