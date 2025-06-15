// tests/apply_function_edge_cases.rs

use rustree::config::metadata::ApplyFunction;
use rustree::{BuiltInFunction, FilteringOptions, MetadataOptions};
use rustree::{LibOutputFormat, RustreeLibConfig, format_nodes, get_tree_nodes};
use std::fs;
use tempfile::TempDir;

#[test]
fn test_size_total_function() {
    // Create a temporary directory structure for testing
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let temp_path = temp_dir.path();

    // Create test structure:
    // temp/
    // ├── small.txt (10 bytes)
    // ├── large.txt (100 bytes)
    // ├── subdir/
    // │   ├── file1.txt (20 bytes)
    // │   └── file2.txt (30 bytes)
    // └── empty_dir/

    let small_file = temp_path.join("small.txt");
    let large_file = temp_path.join("large.txt");
    let subdir = temp_path.join("subdir");
    let empty_dir = temp_path.join("empty_dir");
    let file1 = subdir.join("file1.txt");
    let file2 = subdir.join("file2.txt");

    fs::write(&small_file, "0123456789").expect("Failed to write small.txt"); // 10 bytes
    fs::write(&large_file, "a".repeat(100)).expect("Failed to write large.txt"); // 100 bytes
    fs::create_dir(&subdir).expect("Failed to create subdir");
    fs::create_dir(&empty_dir).expect("Failed to create empty_dir");
    fs::write(&file1, "01234567890123456789").expect("Failed to write file1.txt"); // 20 bytes
    fs::write(&file2, "012345678901234567890123456789").expect("Failed to write file2.txt"); // 30 bytes

    let config = RustreeLibConfig {
        metadata: MetadataOptions {
            apply_function: Some(ApplyFunction::BuiltIn(BuiltInFunction::SizeTotal)),
            show_size_bytes: true, // Enable size collection for SizeTotal to work
            ..Default::default()
        },
        ..Default::default()
    };

    let nodes = get_tree_nodes(temp_path, &config).expect("Failed to get tree nodes");
    let output =
        format_nodes(&nodes, LibOutputFormat::Text, &config).expect("Failed to format nodes");

    // subdir should show total size of its files: 20 + 30 = 50 bytes
    // empty_dir should show 0 bytes
    assert!(output.contains("subdir"));
    assert!(output.contains("empty_dir"));

    // Look for size total results
    assert!(output.contains("[F: \"50\"]") || output.contains("F:50"));
    assert!(output.contains("[F: \"0\"]") || output.contains("F:0"));
}

#[test]
fn test_apply_function_with_mixed_patterns() {
    use tempfile::NamedTempFile;

    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let temp_path = temp_dir.path();

    // Create test structure
    let test_file = temp_path.join("test.txt");
    let special_file = temp_path.join("special_file.txt");
    let normal_dir = temp_path.join("normal");
    let test_dir = temp_path.join("test_dir");

    fs::write(&test_file, "content").expect("Failed to write test.txt");
    fs::write(&special_file, "content").expect("Failed to write special_file.txt");
    fs::create_dir(&normal_dir).expect("Failed to create normal dir");
    fs::create_dir(&test_dir).expect("Failed to create test_dir");

    // Create pattern files with overlapping patterns
    let mut include_patterns_file =
        NamedTempFile::new().expect("Failed to create temp include file");
    let mut exclude_patterns_file =
        NamedTempFile::new().expect("Failed to create temp exclude file");

    use std::io::Write;
    writeln!(include_patterns_file, "test*").expect("Failed to write to include file");
    writeln!(include_patterns_file, "special*").expect("Failed to write to include file");

    writeln!(exclude_patterns_file, "*_file*").expect("Failed to write to exclude file");

    // Test combined include and exclude patterns
    let config = RustreeLibConfig {
        metadata: MetadataOptions {
            apply_function: Some(ApplyFunction::BuiltIn(BuiltInFunction::CountPluses)),
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

    let nodes = get_tree_nodes(temp_path, &config).expect("Failed to get tree nodes");
    let output =
        format_nodes(&nodes, LibOutputFormat::Text, &config).expect("Failed to format nodes");

    // test.txt should match include but not exclude, so function applies
    // special_file.txt should match include but also match exclude, so function doesn't apply
    // test_dir should match include but not exclude, so function applies (for directories)
    let function_results = output.matches("[F: \"").count();
    assert!(function_results >= 1); // At least test.txt and test_dir should have function applied

    // Verify special_file.txt shows N/A due to exclusion
    let lines: Vec<&str> = output.lines().collect();
    let special_line = lines.iter().find(|line| line.contains("special_file.txt"));
    if let Some(line) = special_line {
        assert!(line.contains("[F: N/A]"));
    }
}

#[test]
fn test_error_handling_for_nonexistent_pattern_files() {
    use rustree::cli::filtering::apply_function::ApplyFunctionFilterArgs;
    use std::path::PathBuf;

    let args = ApplyFunctionFilterArgs {
        apply_include: None,
        apply_exclude: None,
        apply_include_from: Some(PathBuf::from("/nonexistent/path/include.txt")),
        apply_exclude_from: Some(PathBuf::from("/nonexistent/path/exclude.txt")),
    };

    // Should return IO error for nonexistent files
    assert!(args.get_all_include_patterns().is_err());
    assert!(args.get_all_exclude_patterns().is_err());
}

#[test]
fn test_empty_pattern_files() {
    use tempfile::NamedTempFile;

    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let temp_path = temp_dir.path();

    let test_file = temp_path.join("test.txt");
    fs::write(&test_file, "content").expect("Failed to write test.txt");

    // Create empty pattern files
    let include_patterns_file = NamedTempFile::new().expect("Failed to create temp include file");
    let exclude_patterns_file = NamedTempFile::new().expect("Failed to create temp exclude file");

    // Don't write anything to the files - they remain empty

    let config = RustreeLibConfig {
        metadata: MetadataOptions {
            apply_function: Some(ApplyFunction::BuiltIn(BuiltInFunction::CountPluses)),
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

    let nodes = get_tree_nodes(temp_path, &config).expect("Failed to get tree nodes");
    let output =
        format_nodes(&nodes, LibOutputFormat::Text, &config).expect("Failed to format nodes");

    // With empty include patterns, the function is still applied (no restrictions)
    // With empty exclude patterns, all files should have functions applied
    assert!(output.contains("test.txt"));

    // Since both pattern files are empty, function should apply normally
    assert!(output.contains("[F: \"0\"]")); // CountPluses should return 0 for "content"
}

#[test]
fn test_directory_functions_with_symlinks() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let temp_path = temp_dir.path();

    // Create test structure with symlinks
    let real_file = temp_path.join("real_file.txt");
    let real_dir = temp_path.join("real_dir");
    let real_subfile = real_dir.join("subfile.txt");
    let symlink_file = temp_path.join("symlink_to_file.txt");
    let symlink_dir = temp_path.join("symlink_to_dir");

    fs::write(&real_file, "content").expect("Failed to write real_file.txt");
    fs::create_dir(&real_dir).expect("Failed to create real_dir");
    fs::write(&real_subfile, "subcontent").expect("Failed to write subfile.txt");

    // Create symlinks (ignore errors on platforms that don't support them)
    #[cfg(unix)]
    {
        let _ = std::os::unix::fs::symlink(&real_file, &symlink_file);
        let _ = std::os::unix::fs::symlink(&real_dir, &symlink_dir);
    }

    let config = RustreeLibConfig {
        metadata: MetadataOptions {
            apply_function: Some(ApplyFunction::BuiltIn(BuiltInFunction::CountFiles)),
            ..Default::default()
        },
        ..Default::default()
    };

    let nodes = get_tree_nodes(temp_path, &config).expect("Failed to get tree nodes");
    let output =
        format_nodes(&nodes, LibOutputFormat::Text, &config).expect("Failed to format nodes");

    // Should handle symlinks gracefully
    assert!(output.contains("real_dir"));
    // The function should work whether symlinks are followed or not
    assert!(output.contains("[F:") || output.contains("F:"));
}
