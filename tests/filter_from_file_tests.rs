// tests/filter_from_file_tests.rs
use anyhow::Result;
use std::fs;
use std::path::Path;
use tempfile::TempDir;

mod common;
use common::common_test_utils::setup_complex_test_directory;

// Helper to create a pattern file
fn create_pattern_file(
    dir: &Path,
    filename: &str,
    patterns: &[&str],
) -> Result<std::path::PathBuf> {
    let file_path = dir.join(filename);
    let content = patterns.join("\n");
    fs::write(&file_path, content)?;
    Ok(file_path)
}

#[test]
fn test_filter_include_from_file() -> Result<()> {
    let temp_dir = setup_complex_test_directory()?;
    let root_path = temp_dir.path();

    // Create include pattern file
    let pattern_file = create_pattern_file(
        root_path,
        "include_patterns.txt",
        &["*.txt", "*.log", "# This is a comment", "", "sub_dir/"],
    )?;

    // Test with CLI args simulation
    use std::process::Command;
    let output = Command::new("cargo")
        .args([
            "run",
            "--quiet",
            "--",
            "--filter-include-from",
            pattern_file.to_str().unwrap(),
            root_path.to_str().unwrap(),
        ])
        .output()?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    // Debug output
    if !output.status.success() {
        eprintln!("Command failed with stderr: {}", stderr);
    }

    // Check that only .txt and .log files are shown
    assert!(
        stdout.contains("file_a.txt"),
        "Expected file_a.txt in output:\n{}",
        stdout
    );
    assert!(
        stdout.contains("file_b.log"),
        "Expected file_b.log in output:\n{}",
        stdout
    );
    assert!(
        stdout.contains("sub_dir"),
        "Expected sub_dir in output:\n{}",
        stdout
    );

    // Check that other files are not shown
    assert!(
        !stdout.contains("image.JPG"),
        "Unexpected image.JPG in output:\n{}",
        stdout
    );
    assert!(
        !stdout.contains("script.sh"),
        "Unexpected script.sh in output:\n{}",
        stdout
    );
    assert!(
        !stdout.contains("sub_file.rs"),
        "Unexpected sub_file.rs in output:\n{}",
        stdout
    );

    Ok(())
}

#[test]
fn test_filter_exclude_from_file() -> Result<()> {
    let temp_dir = setup_complex_test_directory()?;
    let root_path = temp_dir.path();

    // Create exclude pattern file
    let pattern_file = create_pattern_file(
        root_path,
        "exclude_patterns.txt",
        &["*.log", "# Exclude log files", "", "empty_dir/", "*.JPG"],
    )?;

    // Test with CLI args simulation
    use std::process::Command;
    let output = Command::new("cargo")
        .args([
            "run",
            "--quiet",
            "--",
            "--filter-exclude-from",
            pattern_file.to_str().unwrap(),
            root_path.to_str().unwrap(),
        ])
        .output()?;

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Check that excluded files are not shown
    assert!(
        !stdout.contains("file_b.log"),
        "file_b.log should be excluded"
    );
    assert!(
        !stdout.contains("image.JPG"),
        "image.JPG should be excluded"
    );
    assert!(
        !stdout.contains("empty_dir"),
        "empty_dir should be excluded"
    );

    // Check that other files are shown
    assert!(stdout.contains("file_a.txt"), "file_a.txt should be shown");
    assert!(stdout.contains("script.sh"), "script.sh should be shown");
    assert!(stdout.contains("sub_dir"), "sub_dir should be shown");

    Ok(())
}

#[test]
fn test_combined_include_exclude_from_files() -> Result<()> {
    let temp_dir = setup_complex_test_directory()?;
    let root_path = temp_dir.path();

    // Create include pattern file
    let include_file = create_pattern_file(root_path, "include.txt", &["*.txt", "*.log", "*.sh"])?;

    // Create exclude pattern file
    let exclude_file = create_pattern_file(root_path, "exclude.txt", &["*.log"])?;

    // Test with CLI args simulation
    use std::process::Command;
    let output = Command::new("cargo")
        .args([
            "run",
            "--",
            "--filter-include-from",
            include_file.to_str().unwrap(),
            "--filter-exclude-from",
            exclude_file.to_str().unwrap(),
            "-L",
            "1",
            root_path.to_str().unwrap(),
        ])
        .output()?;

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should show .txt and .sh files, but not .log files
    assert!(stdout.contains("file_a.txt"));
    assert!(stdout.contains("script.sh"));
    assert!(!stdout.contains("file_b.log"));
    assert!(!stdout.contains("image.JPG"));

    Ok(())
}

#[test]
fn test_multiple_filter_from_files() -> Result<()> {
    let temp_dir = setup_complex_test_directory()?;
    let root_path = temp_dir.path();

    // Create multiple include pattern files
    let include_file1 = create_pattern_file(root_path, "include1.txt", &["*.txt"])?;

    let include_file2 = create_pattern_file(root_path, "include2.txt", &["*.sh", "*.log"])?;

    // Test with CLI args simulation
    use std::process::Command;
    let output = Command::new("cargo")
        .args([
            "run",
            "--",
            "--filter-include-from",
            include_file1.to_str().unwrap(),
            "--filter-include-from",
            include_file2.to_str().unwrap(),
            "-L",
            "1",
            root_path.to_str().unwrap(),
        ])
        .output()?;

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should show files matching any of the patterns
    assert!(stdout.contains("file_a.txt"));
    assert!(stdout.contains("script.sh"));
    assert!(stdout.contains("file_b.log"));
    assert!(!stdout.contains("image.JPG"));

    Ok(())
}

#[test]
fn test_filter_from_file_with_comments_and_empty_lines() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let root_path = temp_dir.path();

    // Create test files
    fs::write(root_path.join("test.txt"), "content")?;
    fs::write(root_path.join("test.rs"), "content")?;
    fs::write(root_path.join("test.md"), "content")?;

    // Create pattern file with comments and empty lines
    let pattern_file = create_pattern_file(
        root_path,
        "patterns.txt",
        &[
            "# This is a comment",
            "*.txt",
            "",
            "  # Another comment with spaces",
            "*.rs",
            "   ", // Just spaces
            "# *.md is commented out",
        ],
    )?;

    // Test with CLI args simulation
    use std::process::Command;
    let output = Command::new("cargo")
        .args([
            "run",
            "--",
            "--filter-include-from",
            pattern_file.to_str().unwrap(),
            root_path.to_str().unwrap(),
        ])
        .output()?;

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should show .txt and .rs files, but not .md
    assert!(stdout.contains("test.txt"));
    assert!(stdout.contains("test.rs"));
    assert!(!stdout.contains("test.md"));

    Ok(())
}

#[test]
fn test_filter_from_nonexistent_file() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let root_path = temp_dir.path();

    // Test with CLI args simulation
    use std::process::Command;
    let output = Command::new("cargo")
        .args([
            "run",
            "--",
            "--filter-include-from",
            "/nonexistent/file.txt",
            root_path.to_str().unwrap(),
        ])
        .output()?;

    let stderr = String::from_utf8_lossy(&output.stderr);

    // Should show error message
    assert!(stderr.contains("Error reading pattern files"));
    assert!(stderr.contains("No such file or directory"));

    Ok(())
}

#[test]
fn test_filter_from_file_with_cli_patterns() -> Result<()> {
    let temp_dir = setup_complex_test_directory()?;
    let root_path = temp_dir.path();

    // Create include pattern file with just .txt
    let pattern_file = create_pattern_file(root_path, "include.txt", &["*.txt"])?;

    // Test combining file patterns with CLI patterns
    use std::process::Command;
    let output = Command::new("cargo")
        .args([
            "run",
            "--",
            "--filter-include-from",
            pattern_file.to_str().unwrap(),
            "--filter-include",
            "*.sh",
            "-L",
            "1",
            root_path.to_str().unwrap(),
        ])
        .output()?;

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should show both .txt (from file) and .sh (from CLI) files
    assert!(stdout.contains("file_a.txt"));
    assert!(stdout.contains("script.sh"));
    assert!(!stdout.contains("file_b.log"));
    assert!(!stdout.contains("image.JPG"));

    Ok(())
}
