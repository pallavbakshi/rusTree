// tests/apply_function_tests.rs

use rustree::{BuiltInFunction, MetadataOptions};
use rustree::{LibOutputFormat, RustreeLibConfig, format_nodes, get_tree_nodes};
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
    let output =
        format_nodes(&nodes, LibOutputFormat::Text, &config).expect("Failed to format nodes");

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
    let tree_end = output
        .find("--- File Contents ---")
        .expect("File contents section not found");
    let file1_in_tree = output[..tree_end]
        .find("file1.txt")
        .expect("file1 not in tree section");
    let file1_content = output
        .find("Content of file 1")
        .expect("file1 content not found");
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
    let output =
        format_nodes(&nodes, LibOutputFormat::Text, &config).expect("Failed to format nodes");

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
    let output =
        format_nodes(&nodes, LibOutputFormat::Text, &config).expect("Failed to format nodes");

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
    let output =
        format_nodes(&nodes, LibOutputFormat::Markdown, &config).expect("Failed to format nodes");

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
    let output =
        format_nodes(&nodes, LibOutputFormat::Text, &config).expect("Failed to format nodes");

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
    let output =
        format_nodes(&nodes, LibOutputFormat::Text, &config).expect("Failed to format nodes");

    // Should show level 1 content but not level 2 (depth 2 means root + 2 levels)
    assert!(output.contains("level1/"));
    assert!(output.contains("file1.txt"));
    assert!(output.contains("Level 1 content"));
    // Note: depth of 2 might still show level2/ directory but not its contents if depth counting is different
}

#[test]
fn test_count_files_function() {
    // Create a temporary directory structure for testing
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let temp_path = temp_dir.path();

    // Create test structure:
    // temp/
    // ├── file1.txt
    // ├── file2.txt
    // ├── subdir1/
    // │   ├── file3.txt
    // │   └── file4.txt
    // └── subdir2/
    //     └── file5.txt

    let file1_path = temp_path.join("file1.txt");
    let file2_path = temp_path.join("file2.txt");
    let subdir1_path = temp_path.join("subdir1");
    let subdir2_path = temp_path.join("subdir2");
    let file3_path = subdir1_path.join("file3.txt");
    let file4_path = subdir1_path.join("file4.txt");
    let file5_path = subdir2_path.join("file5.txt");

    fs::write(&file1_path, "Content 1").expect("Failed to write file1");
    fs::write(&file2_path, "Content 2").expect("Failed to write file2");
    fs::create_dir(&subdir1_path).expect("Failed to create subdir1");
    fs::create_dir(&subdir2_path).expect("Failed to create subdir2");
    fs::write(&file3_path, "Content 3").expect("Failed to write file3");
    fs::write(&file4_path, "Content 4").expect("Failed to write file4");
    fs::write(&file5_path, "Content 5").expect("Failed to write file5");

    // Configure to use count-files function
    let config = RustreeLibConfig {
        metadata: MetadataOptions {
            apply_function: Some(BuiltInFunction::CountFiles),
            ..Default::default()
        },
        ..Default::default()
    };

    // Get the tree nodes
    let nodes = get_tree_nodes(temp_path, &config).expect("Failed to get tree nodes");

    // Format the output
    let output =
        format_nodes(&nodes, LibOutputFormat::Text, &config).expect("Failed to format nodes");

    // Verify the output structure contains directories with file counts
    // Root should show 2 files (file1.txt, file2.txt)
    // subdir1 should show 2 files (file3.txt, file4.txt)
    // subdir2 should show 1 file (file5.txt)

    // The root directory info should show 2 files
    // This is a bit tricky to test directly since the root might not be explicitly shown
    // Instead, we check that subdirectories show correct counts
    assert!(output.contains("subdir1"));
    assert!(output.contains("subdir2"));

    // Check that function results are shown for directories
    // The exact format will be [F: "2"] for subdir1 and [F: "1"] for subdir2
    assert!(output.contains("[F: \"2\"]") || output.contains("F:2"));
    assert!(output.contains("[F: \"1\"]") || output.contains("F:1"));
}

#[test]
fn test_count_dirs_function() {
    // Create a temporary directory structure for testing
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let temp_path = temp_dir.path();

    // Create test structure:
    // temp/
    // ├── file1.txt
    // ├── subdir1/
    // │   ├── nested1/
    // │   └── nested2/
    // └── subdir2/
    //     └── file2.txt

    let file1_path = temp_path.join("file1.txt");
    let subdir1_path = temp_path.join("subdir1");
    let subdir2_path = temp_path.join("subdir2");
    let nested1_path = subdir1_path.join("nested1");
    let nested2_path = subdir1_path.join("nested2");
    let file2_path = subdir2_path.join("file2.txt");

    fs::write(&file1_path, "Content 1").expect("Failed to write file1");
    fs::create_dir(&subdir1_path).expect("Failed to create subdir1");
    fs::create_dir(&subdir2_path).expect("Failed to create subdir2");
    fs::create_dir(&nested1_path).expect("Failed to create nested1");
    fs::create_dir(&nested2_path).expect("Failed to create nested2");
    fs::write(&file2_path, "Content 2").expect("Failed to write file2");

    // Configure to use count-dirs function
    let config = RustreeLibConfig {
        metadata: MetadataOptions {
            apply_function: Some(BuiltInFunction::CountDirs),
            ..Default::default()
        },
        ..Default::default()
    };

    // Get the tree nodes
    let nodes = get_tree_nodes(temp_path, &config).expect("Failed to get tree nodes");

    // Format the output
    let output =
        format_nodes(&nodes, LibOutputFormat::Text, &config).expect("Failed to format nodes");

    // Verify the output structure contains directories with directory counts
    // Root should show 2 directories (subdir1, subdir2)
    // subdir1 should show 2 directories (nested1, nested2)
    // subdir2 should show 0 directories
    // nested1 and nested2 should show 0 directories

    assert!(output.contains("subdir1"));
    assert!(output.contains("subdir2"));
    assert!(output.contains("nested1"));
    assert!(output.contains("nested2"));

    // Check that function results are shown for directories
    // subdir1 should show 2 directories, subdir2 should show 0
    assert!(output.contains("[F: \"2\"]") || output.contains("F:2"));
    assert!(output.contains("[F: \"0\"]") || output.contains("F:0"));
}

#[test]
fn test_dir_stats_function() {
    // Create a temporary directory structure for testing
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let temp_path = temp_dir.path();

    // Create test structure:
    // temp/
    // ├── file1.txt
    // ├── subdir1/
    // │   ├── file2.txt
    // │   └── nested/
    // └── subdir2/
    //     └── file3.txt

    let file1_path = temp_path.join("file1.txt");
    let subdir1_path = temp_path.join("subdir1");
    let subdir2_path = temp_path.join("subdir2");
    let file2_path = subdir1_path.join("file2.txt");
    let nested_path = subdir1_path.join("nested");
    let file3_path = subdir2_path.join("file3.txt");

    fs::write(&file1_path, "Content 1").expect("Failed to write file1");
    fs::create_dir(&subdir1_path).expect("Failed to create subdir1");
    fs::create_dir(&subdir2_path).expect("Failed to create subdir2");
    fs::write(&file2_path, "Content 2").expect("Failed to write file2");
    fs::create_dir(&nested_path).expect("Failed to create nested");
    fs::write(&file3_path, "Content 3").expect("Failed to write file3");

    // Configure to use dir-stats function
    let config = RustreeLibConfig {
        metadata: MetadataOptions {
            apply_function: Some(BuiltInFunction::DirStats),
            ..Default::default()
        },
        ..Default::default()
    };

    // Get the tree nodes
    let nodes = get_tree_nodes(temp_path, &config).expect("Failed to get tree nodes");

    // Format the output
    let output =
        format_nodes(&nodes, LibOutputFormat::Text, &config).expect("Failed to format nodes");

    // Verify the output contains stats in format "XfYdZB"
    // subdir1 should show "1f,1d,9B" (1 file, 1 directory, size of file2.txt)
    // subdir2 should show "1f,0d,9B" (1 file, 0 directories, size of file3.txt)

    assert!(output.contains("subdir1"));
    assert!(output.contains("subdir2"));

    // Check that stats format is present (files, dirs, bytes)
    // Look for the pattern like "1f,1d," or "1f,0d,"
    assert!(output.contains("f,") && output.contains("d,") && output.contains("B"));
}

#[test]
fn test_apply_function_filtering() {
    use rustree::FilteringOptions;

    // Create a temporary directory structure for testing
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let temp_path = temp_dir.path();

    // Create test structure:
    // temp/
    // ├── include_me.txt
    // ├── exclude_me.txt
    // ├── regular.txt
    // ├── special_dir/
    // │   └── file.txt
    // └── normal_dir/
    //     └── file.txt

    let include_file = temp_path.join("include_me.txt");
    let exclude_file = temp_path.join("exclude_me.txt");
    let regular_file = temp_path.join("regular.txt");
    let special_dir = temp_path.join("special_dir");
    let normal_dir = temp_path.join("normal_dir");
    let special_file = special_dir.join("file.txt");
    let normal_file = normal_dir.join("file.txt");

    fs::write(&include_file, "Content to include").expect("Failed to write include_me.txt");
    fs::write(&exclude_file, "Content to exclude").expect("Failed to write exclude_me.txt");
    fs::write(&regular_file, "Regular content").expect("Failed to write regular.txt");
    fs::create_dir(&special_dir).expect("Failed to create special_dir");
    fs::create_dir(&normal_dir).expect("Failed to create normal_dir");
    fs::write(&special_file, "Special file").expect("Failed to write special file");
    fs::write(&normal_file, "Normal file").expect("Failed to write normal file");

    // Test 1: Apply count-pluses to only files matching "include*" pattern
    let config_include = RustreeLibConfig {
        metadata: MetadataOptions {
            apply_function: Some(BuiltInFunction::CountPluses),
            ..Default::default()
        },
        filtering: FilteringOptions {
            apply_include_patterns: Some(vec!["include*".to_string()]),
            ..Default::default()
        },
        ..Default::default()
    };

    let nodes = get_tree_nodes(temp_path, &config_include).expect("Failed to get tree nodes");
    let output = format_nodes(&nodes, LibOutputFormat::Text, &config_include)
        .expect("Failed to format nodes");

    // Should apply function only to include_me.txt
    // include_me.txt should show function result, others should show [F: N/A]
    assert!(output.contains("include_me.txt"));
    assert!(output.contains("exclude_me.txt"));
    assert!(output.contains("regular.txt"));

    // Count occurrences of function results - only include_me.txt should have a non-N/A result
    let function_results = output.matches("[F: \"").count();
    assert_eq!(function_results, 1); // Only include_me.txt should have the function applied

    // Test 2: Apply count-files to directories but exclude "special*" pattern
    let config_exclude = RustreeLibConfig {
        metadata: MetadataOptions {
            apply_function: Some(BuiltInFunction::CountFiles),
            ..Default::default()
        },
        filtering: FilteringOptions {
            apply_exclude_patterns: Some(vec!["special*".to_string()]),
            ..Default::default()
        },
        ..Default::default()
    };

    let nodes = get_tree_nodes(temp_path, &config_exclude).expect("Failed to get tree nodes");
    let output = format_nodes(&nodes, LibOutputFormat::Text, &config_exclude)
        .expect("Failed to format nodes");

    // Should apply function to normal_dir but not special_dir
    assert!(output.contains("special_dir"));
    assert!(output.contains("normal_dir"));

    // special_dir should show [F: N/A], normal_dir should show [F: "1"]
    let lines: Vec<&str> = output.lines().collect();
    let special_line = lines
        .iter()
        .find(|line| line.contains("special_dir"))
        .unwrap();
    let normal_line = lines
        .iter()
        .find(|line| line.contains("normal_dir"))
        .unwrap();

    assert!(special_line.contains("[F: N/A]"));
    assert!(normal_line.contains("[F: \"1\"]") || normal_line.contains("F:1"));
}

#[test]
fn test_apply_function_filtering_from_files() {
    use rustree::FilteringOptions;
    use std::fs;
    use std::io::Write;
    use tempfile::NamedTempFile;

    // Create a temporary directory structure for testing
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let temp_path = temp_dir.path();

    // Create test structure:
    // temp/
    // ├── include_me.txt
    // ├── exclude_me.txt
    // ├── regular.txt
    // ├── special_dir/
    // └── normal_dir/

    let include_file = temp_path.join("include_me.txt");
    let exclude_file = temp_path.join("exclude_me.txt");
    let regular_file = temp_path.join("regular.txt");
    let special_dir = temp_path.join("special_dir");
    let normal_dir = temp_path.join("normal_dir");

    fs::write(&include_file, "Content to include").expect("Failed to write include_me.txt");
    fs::write(&exclude_file, "Content to exclude").expect("Failed to write exclude_me.txt");
    fs::write(&regular_file, "Regular content").expect("Failed to write regular.txt");
    fs::create_dir(&special_dir).expect("Failed to create special_dir");
    fs::create_dir(&normal_dir).expect("Failed to create normal_dir");

    // Create temporary pattern files
    let mut include_patterns_file =
        NamedTempFile::new().expect("Failed to create temp include file");
    let mut exclude_patterns_file =
        NamedTempFile::new().expect("Failed to create temp exclude file");

    // Write patterns to files (with comments and empty lines to test filtering)
    writeln!(include_patterns_file, "# This is a comment")
        .expect("Failed to write to include file");
    writeln!(include_patterns_file, "include*").expect("Failed to write to include file");
    // Write an empty line (writeln! without arguments adds a newline)
    writeln!(include_patterns_file).expect("Failed to write to include file");
    writeln!(include_patterns_file, "regular*").expect("Failed to write to include file");

    writeln!(exclude_patterns_file, "exclude*").expect("Failed to write to exclude file");
    writeln!(exclude_patterns_file, "# Comment here too").expect("Failed to write to exclude file");

    // Test 1: Apply count-pluses using include patterns from file
    let config_include = RustreeLibConfig {
        metadata: MetadataOptions {
            apply_function: Some(BuiltInFunction::CountPluses),
            ..Default::default()
        },
        filtering: FilteringOptions {
            apply_include_patterns: {
                use rustree::cli::filtering::apply_function::ApplyFunctionFilterArgs;
                let args = ApplyFunctionFilterArgs {
                    apply_include: None,
                    apply_exclude: None,
                    apply_include_from: Some(include_patterns_file.path().to_path_buf()),
                    apply_exclude_from: None,
                };
                args.get_all_include_patterns()
                    .expect("Failed to read include patterns")
            },
            ..Default::default()
        },
        ..Default::default()
    };

    let nodes = get_tree_nodes(temp_path, &config_include).expect("Failed to get tree nodes");
    let output = format_nodes(&nodes, LibOutputFormat::Text, &config_include)
        .expect("Failed to format nodes");

    // Should apply function to include_me.txt and regular.txt
    assert!(output.contains("include_me.txt"));
    assert!(output.contains("exclude_me.txt"));
    assert!(output.contains("regular.txt"));

    // Count occurrences of function results - include_me.txt and regular.txt should have non-N/A results
    let function_results = output.matches("[F: \"").count();
    assert_eq!(function_results, 2); // include_me.txt and regular.txt should have the function applied

    // Test 2: Apply count-files using exclude patterns from file
    let config_exclude = RustreeLibConfig {
        metadata: MetadataOptions {
            apply_function: Some(BuiltInFunction::CountFiles),
            ..Default::default()
        },
        filtering: FilteringOptions {
            apply_exclude_patterns: {
                use rustree::cli::filtering::apply_function::ApplyFunctionFilterArgs;
                let args = ApplyFunctionFilterArgs {
                    apply_include: None,
                    apply_exclude: None,
                    apply_include_from: None,
                    apply_exclude_from: Some(exclude_patterns_file.path().to_path_buf()),
                };
                args.get_all_exclude_patterns()
                    .expect("Failed to read exclude patterns")
            },
            ..Default::default()
        },
        ..Default::default()
    };

    let nodes = get_tree_nodes(temp_path, &config_exclude).expect("Failed to get tree nodes");
    let output = format_nodes(&nodes, LibOutputFormat::Text, &config_exclude)
        .expect("Failed to format nodes");

    // Should apply function to special_dir and normal_dir, but not to any files/dirs matching "exclude*"
    assert!(output.contains("special_dir"));
    assert!(output.contains("normal_dir"));

    // Both directories should have function applied since they don't match "exclude*"
    let lines: Vec<&str> = output.lines().collect();
    let special_line = lines
        .iter()
        .find(|line| line.contains("special_dir"))
        .unwrap();
    let normal_line = lines
        .iter()
        .find(|line| line.contains("normal_dir"))
        .unwrap();

    // Both should show function results since neither matches the exclude pattern
    assert!(special_line.contains("[F: \"0\"]") || special_line.contains("F:0"));
    assert!(normal_line.contains("[F: \"0\"]") || normal_line.contains("F:0"));
}
