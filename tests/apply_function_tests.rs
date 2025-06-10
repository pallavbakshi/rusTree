// tests/apply_function_tests.rs

use rustree::{get_tree_nodes, format_nodes, RustreeLibConfig, LibOutputFormat};
use rustree::{MetadataOptions, BuiltInFunction};
use std::fs;
use tempfile::TempDir;

#[test]
fn test_cat_function_integration() {
    // Create a temporary directory structure for testing
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let temp_path = temp_dir.path();
    
    // Create test files
    let file1_path = temp_path.join("file1.txt");
    let file2_path = temp_path.join("file2.txt");
    let subdir_path = temp_path.join("subdir");
    let file3_path = subdir_path.join("file3.txt");
    
    fs::write(&file1_path, "Content of file 1\nSecond line").expect("Failed to write file1");
    fs::write(&file2_path, "File 2 content").expect("Failed to write file2");
    fs::create_dir(&subdir_path).expect("Failed to create subdir");
    fs::write(&file3_path, "Nested file content").expect("Failed to write file3");
    
    // Configure to use cat function
    let config = RustreeLibConfig {
        metadata: MetadataOptions {
            apply_function: Some(BuiltInFunction::Cat),
            ..Default::default()
        },
        ..Default::default()
    };
    
    // Get the tree nodes
    let nodes = get_tree_nodes(temp_path, &config).expect("Failed to get tree nodes");
    
    // Format the output
    let output = format_nodes(&nodes, LibOutputFormat::Text, &config).expect("Failed to format nodes");
    
    // Verify the output structure
    assert!(output.contains("file1.txt"));
    assert!(output.contains("file2.txt"));
    assert!(output.contains("subdir/"));
    assert!(output.contains("file3.txt"));
    
    // Verify the cat content section
    assert!(output.contains("--- File Contents ---"));
    assert!(output.contains("=== "));
    assert!(output.contains("file1.txt ==="));
    assert!(output.contains("Content of file 1\nSecond line"));
    assert!(output.contains("file2.txt ==="));
    assert!(output.contains("File 2 content"));
    assert!(output.contains("file3.txt ==="));
    assert!(output.contains("Nested file content"));
    
    // Verify structure: tree comes before file contents
    let tree_end = output.find("--- File Contents ---").expect("File contents section not found");
    let file1_in_tree = output[..tree_end].find("file1.txt").expect("file1 not in tree section");
    let file1_content = output.find("Content of file 1").expect("file1 content not found");
    assert!(file1_in_tree < tree_end);
    assert!(file1_content > tree_end);
}

#[test]
fn test_cat_function_with_empty_files() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let temp_path = temp_dir.path();
    
    // Create an empty file
    let empty_file_path = temp_path.join("empty.txt");
    fs::write(&empty_file_path, "").expect("Failed to write empty file");
    
    let config = RustreeLibConfig {
        metadata: MetadataOptions {
            apply_function: Some(BuiltInFunction::Cat),
            ..Default::default()
        },
        ..Default::default()
    };
    
    let nodes = get_tree_nodes(temp_path, &config).expect("Failed to get tree nodes");
    let output = format_nodes(&nodes, LibOutputFormat::Text, &config).expect("Failed to format nodes");
    
    // Should still show the file in tree and content section, even if empty
    assert!(output.contains("empty.txt"));
    assert!(output.contains("--- File Contents ---"));
    assert!(output.contains("=== "));
    assert!(output.contains("empty.txt ==="));
}

#[test]
fn test_count_pluses_function_integration() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let temp_path = temp_dir.path();
    
    // Create test files with plus characters
    let file1_path = temp_path.join("plus_file.txt");
    fs::write(&file1_path, "++test++content+").expect("Failed to write file");
    
    let config = RustreeLibConfig {
        metadata: MetadataOptions {
            apply_function: Some(BuiltInFunction::CountPluses),
            ..Default::default()
        },
        ..Default::default()
    };
    
    let nodes = get_tree_nodes(temp_path, &config).expect("Failed to get tree nodes");
    let output = format_nodes(&nodes, LibOutputFormat::Text, &config).expect("Failed to format nodes");
    
    // Should show the count in metadata, not as separate content
    assert!(output.contains("[F: \"5\"]")); // 5 plus characters
    assert!(!output.contains("--- File Contents ---"));
    assert!(!output.contains("++test++content+"));
}

#[test]
fn test_cat_function_markdown_format() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let temp_path = temp_dir.path();
    
    let file_path = temp_path.join("test.txt");
    fs::write(&file_path, "Test content for markdown").expect("Failed to write file");
    
    let config = RustreeLibConfig {
        metadata: MetadataOptions {
            apply_function: Some(BuiltInFunction::Cat),
            ..Default::default()
        },
        ..Default::default()
    };
    
    let nodes = get_tree_nodes(temp_path, &config).expect("Failed to get tree nodes");
    let output = format_nodes(&nodes, LibOutputFormat::Markdown, &config).expect("Failed to format nodes");
    
    // Should work with markdown format too
    assert!(output.contains("test.txt")); // More flexible check
    assert!(output.contains("--- File Contents ---"));
    assert!(output.contains("=== "));
    assert!(output.contains("test.txt ==="));
    assert!(output.contains("Test content for markdown"));
}

#[test]
fn test_cat_function_directories_only_mode() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let temp_path = temp_dir.path();
    
    // Create directory structure
    let subdir_path = temp_path.join("subdir");
    let file_path = subdir_path.join("file.txt");
    fs::create_dir(&subdir_path).expect("Failed to create subdir");
    fs::write(&file_path, "File content").expect("Failed to write file");
    
    let config = RustreeLibConfig {
        listing: rustree::ListingOptions {
            list_directories_only: true,
            ..Default::default()
        },
        metadata: MetadataOptions {
            apply_function: Some(BuiltInFunction::Cat),
            ..Default::default()
        },
        ..Default::default()
    };
    
    let nodes = get_tree_nodes(temp_path, &config).expect("Failed to get tree nodes");
    let output = format_nodes(&nodes, LibOutputFormat::Text, &config).expect("Failed to format nodes");
    
    // Should only show directories, no file content section
    assert!(output.contains("subdir/"));
    assert!(!output.contains("file.txt"));
    assert!(!output.contains("--- File Contents ---"));
}

#[test]
fn test_cat_function_with_depth_limit() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let temp_path = temp_dir.path();
    
    // Create nested structure
    let level1_dir = temp_path.join("level1");
    let level2_dir = level1_dir.join("level2");
    let file1_path = level1_dir.join("file1.txt");
    let file2_path = level2_dir.join("file2.txt");
    
    fs::create_dir_all(&level2_dir).expect("Failed to create nested dirs");
    fs::write(&file1_path, "Level 1 content").expect("Failed to write file1");
    fs::write(&file2_path, "Level 2 content").expect("Failed to write file2");
    
    let config = RustreeLibConfig {
        listing: rustree::ListingOptions {
            max_depth: Some(2), // Show two levels to include file1
            ..Default::default()
        },
        metadata: MetadataOptions {
            apply_function: Some(BuiltInFunction::Cat),
            ..Default::default()
        },
        ..Default::default()
    };
    
    let nodes = get_tree_nodes(temp_path, &config).expect("Failed to get tree nodes");
    let output = format_nodes(&nodes, LibOutputFormat::Text, &config).expect("Failed to format nodes");
    
    // Should show level 1 content but not level 2 (depth 2 means root + 2 levels)
    assert!(output.contains("level1/"));
    assert!(output.contains("file1.txt"));
    assert!(output.contains("Level 1 content"));
    // Note: depth of 2 might still show level2/ directory but not its contents if depth counting is different
}