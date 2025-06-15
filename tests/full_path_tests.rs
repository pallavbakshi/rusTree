// tests/full_path_tests.rs

#![allow(clippy::needless_update)]

use anyhow::Result;
use rustree::{
    LibOutputFormat, ListingOptions, MetadataOptions, MiscOptions, RustreeLibConfig, format_nodes,
    get_tree_nodes,
};
use std::fs::{self, File};
use std::io::Write;
use tempfile::TempDir;

/// Helper function to create a test directory structure for full-path testing
fn setup_full_path_test_directory() -> Result<TempDir> {
    let temp_dir = tempfile::TempDir::new()?;
    let root = temp_dir.path();

    // Create nested structure:
    // test_root/
    //   ├── file1.txt
    //   ├── dir1/
    //   │   ├── file2.txt
    //   │   └── subdir/
    //   │       └── file3.txt
    //   └── dir2/
    //       └── file4.txt

    let dir1 = root.join("dir1");
    let dir2 = root.join("dir2");
    let subdir = dir1.join("subdir");

    fs::create_dir_all(&dir1)?;
    fs::create_dir_all(&dir2)?;
    fs::create_dir_all(&subdir)?;

    File::create(root.join("file1.txt"))?.write_all(b"content1")?;
    File::create(dir1.join("file2.txt"))?.write_all(b"content2")?;
    File::create(subdir.join("file3.txt"))?.write_all(b"content3")?;
    File::create(dir2.join("file4.txt"))?.write_all(b"content4")?;

    Ok(temp_dir)
}

#[test]
fn test_full_path_flag_disabled_shows_names_only() -> Result<()> {
    let temp_dir = setup_full_path_test_directory()?;
    let root_path = temp_dir.path();

    let config = RustreeLibConfig {
        listing: ListingOptions {
            show_full_path: false, // Disabled
            ..Default::default()
        },
        ..Default::default()
    };

    let nodes = get_tree_nodes(root_path, &config)?;
    let output = format_nodes(&nodes, LibOutputFormat::Text, &config)?;

    // Should show only filenames, not full paths
    assert!(output.contains("file1.txt"));
    assert!(output.contains("file2.txt"));
    assert!(output.contains("file3.txt"));
    assert!(output.contains("file4.txt"));

    // Should NOT contain full paths like "dir1/file2.txt"
    assert!(!output.contains("dir1/file2.txt"));
    assert!(!output.contains("dir1/subdir/file3.txt"));
    assert!(!output.contains("dir2/file4.txt"));

    Ok(())
}

#[test]
fn test_full_path_flag_enabled_shows_full_paths() -> Result<()> {
    let temp_dir = setup_full_path_test_directory()?;
    let root_path = temp_dir.path();

    let config = RustreeLibConfig {
        listing: ListingOptions {
            show_full_path: true, // Enabled
            ..Default::default()
        },
        ..Default::default()
    };

    let nodes = get_tree_nodes(root_path, &config)?;
    let output = format_nodes(&nodes, LibOutputFormat::Text, &config)?;

    // Should show full paths for nested files
    assert!(output.contains("dir1/file2.txt"));
    assert!(output.contains("dir1/subdir/file3.txt"));
    assert!(output.contains("dir2/file4.txt"));

    // Root-level files should still show just their name (no path prefix needed)
    assert!(output.contains("file1.txt"));

    Ok(())
}

#[test]
fn test_full_path_with_markdown_format() -> Result<()> {
    let temp_dir = setup_full_path_test_directory()?;
    let root_path = temp_dir.path();

    let config = RustreeLibConfig {
        listing: ListingOptions {
            show_full_path: true,
            ..Default::default()
        },
        ..Default::default()
    };

    let nodes = get_tree_nodes(root_path, &config)?;
    let output = format_nodes(&nodes, LibOutputFormat::Markdown, &config)?;

    // Should show full paths in markdown format
    assert!(output.contains("* dir1/file2.txt"));
    assert!(output.contains("* dir1/subdir/file3.txt"));
    assert!(output.contains("* dir2/file4.txt"));

    // Should contain markdown list structure
    assert!(output.contains("* file1.txt"));
    assert!(output.contains("* dir1/"));
    assert!(output.contains("  * dir1/subdir/"));

    Ok(())
}

#[test]
fn test_full_path_with_depth_limit() -> Result<()> {
    let temp_dir = setup_full_path_test_directory()?;
    let root_path = temp_dir.path();

    let config = RustreeLibConfig {
        listing: ListingOptions {
            show_full_path: true,
            max_depth: Some(2), // Limit to 2 levels
            ..Default::default()
        },
        ..Default::default()
    };

    let nodes = get_tree_nodes(root_path, &config)?;
    let output = format_nodes(&nodes, LibOutputFormat::Text, &config)?;

    // Should show paths up to depth 2
    assert!(output.contains("dir1/file2.txt"));
    assert!(output.contains("dir2/file4.txt"));

    // Should NOT show depth 3 files
    assert!(!output.contains("dir1/subdir/file3.txt"));

    Ok(())
}

#[test]
fn test_full_path_with_directories_only_mode() -> Result<()> {
    let temp_dir = setup_full_path_test_directory()?;
    let root_path = temp_dir.path();

    let config = RustreeLibConfig {
        listing: ListingOptions {
            show_full_path: true,
            list_directories_only: true,
            ..Default::default()
        },
        ..Default::default()
    };

    let nodes = get_tree_nodes(root_path, &config)?;
    let output = format_nodes(&nodes, LibOutputFormat::Text, &config)?;

    // Should show directory paths
    assert!(output.contains("dir1/subdir/"));

    // Should NOT show any files (directories only mode)
    assert!(!output.contains("file1.txt"));
    assert!(!output.contains("dir1/file2.txt"));
    assert!(!output.contains("dir1/subdir/file3.txt"));
    assert!(!output.contains("dir2/file4.txt"));

    Ok(())
}

#[test]
fn test_full_path_preserves_directory_suffix() -> Result<()> {
    let temp_dir = setup_full_path_test_directory()?;
    let root_path = temp_dir.path();

    let config = RustreeLibConfig {
        listing: ListingOptions {
            show_full_path: true,
            ..Default::default()
        },
        ..Default::default()
    };

    let nodes = get_tree_nodes(root_path, &config)?;
    let output = format_nodes(&nodes, LibOutputFormat::Text, &config)?;

    // Directories should still have trailing slash
    assert!(output.contains("dir1/"));
    assert!(output.contains("dir2/"));
    assert!(output.contains("dir1/subdir/"));

    // Files should not have trailing slash
    assert!(output.contains("dir1/file2.txt"));
    assert!(!output.contains("dir1/file2.txt/"));

    Ok(())
}

#[test]
fn test_full_path_with_metadata() -> Result<()> {
    let temp_dir = setup_full_path_test_directory()?;
    let root_path = temp_dir.path();

    let config = RustreeLibConfig {
        listing: ListingOptions {
            show_full_path: true,
            ..Default::default()
        },
        metadata: MetadataOptions {
            show_size_bytes: true,
            ..Default::default()
        },
        ..Default::default()
    };

    let nodes = get_tree_nodes(root_path, &config)?;
    let output = format_nodes(&nodes, LibOutputFormat::Text, &config)?;

    // Should show both metadata and full paths
    // Look for pattern like "[     8B] dir1/file2.txt"
    let lines: Vec<&str> = output.lines().collect();
    let file2_line = lines.iter().find(|line| line.contains("dir1/file2.txt"));
    assert!(
        file2_line.is_some(),
        "Should contain full path for file2.txt"
    );

    let file2_line = file2_line.unwrap();
    assert!(
        file2_line.contains("[") && file2_line.contains("B]"),
        "Should show size metadata with full path: {}",
        file2_line
    );

    Ok(())
}

#[test]
fn test_full_path_empty_directory() -> Result<()> {
    let temp_dir = tempfile::TempDir::new()?;
    let root_path = temp_dir.path();

    let config = RustreeLibConfig {
        listing: ListingOptions {
            show_full_path: true,
            ..Default::default()
        },
        ..Default::default()
    };

    let nodes = get_tree_nodes(root_path, &config)?;
    let output = format_nodes(&nodes, LibOutputFormat::Text, &config)?;

    // Empty directory should still work
    assert!(output.contains("0 directories, 0 files"));

    Ok(())
}

#[test]
fn test_full_path_single_file() -> Result<()> {
    let temp_dir = tempfile::TempDir::new()?;
    let root_path = temp_dir.path();

    File::create(root_path.join("single.txt"))?.write_all(b"content")?;

    let config = RustreeLibConfig {
        listing: ListingOptions {
            show_full_path: true,
            ..Default::default()
        },
        ..Default::default()
    };

    let nodes = get_tree_nodes(root_path, &config)?;
    let output = format_nodes(&nodes, LibOutputFormat::Text, &config)?;

    // Single file at root should show just filename (no path prefix needed)
    assert!(output.contains("single.txt"));
    assert!(!output.contains("/single.txt"));

    Ok(())
}

#[test]
fn test_full_path_no_summary_report() -> Result<()> {
    let temp_dir = setup_full_path_test_directory()?;
    let root_path = temp_dir.path();

    let config = RustreeLibConfig {
        listing: ListingOptions {
            show_full_path: true,
            ..Default::default()
        },
        misc: MiscOptions {
            no_summary_report: true,
            human_friendly: false,
            no_color: false,
            verbose: false,
        },
        ..Default::default()
    };

    let nodes = get_tree_nodes(root_path, &config)?;
    let output = format_nodes(&nodes, LibOutputFormat::Text, &config)?;

    // Should show full paths but no summary
    assert!(output.contains("dir1/file2.txt"));
    assert!(output.contains("dir1/subdir/file3.txt"));
    assert!(!output.contains("directories,"));
    assert!(!output.contains("files"));

    Ok(())
}
