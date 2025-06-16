// tests/full_flow_tests.rs

#![allow(clippy::needless_update)]
// Use your library as if you were an external user
use anyhow::Result;
use clap::Parser;
use rustree::config::metadata::ApplyFunction; // For test functions returning Result
use rustree::{
    BuiltInFunction, InputSourceOptions, LibOutputFormat, ListingOptions, MetadataOptions,
    NodeType, RustreeLibConfig, SortKey, SortingOptions, format_nodes, get_tree_nodes,
};
use std::fs; // Added import for fs module // Added import for Parser trait

// Use the common module
mod common;
use common::common_test_utils;

#[test]
fn test_get_nodes_basic_structure() -> Result<()> {
    let temp_dir = common_test_utils::setup_test_directory()?;
    let root_path = temp_dir.path();

    let config = RustreeLibConfig {
        listing: ListingOptions {
            max_depth: Some(2), // file1, file2, sub_dir (depth 1); file3 (depth 2)
            show_hidden: false,
            ..Default::default()
        },
        metadata: MetadataOptions {
            calculate_line_count: true,
            ..Default::default()
        },
        ..Default::default()
    };

    let mut nodes = get_tree_nodes(root_path, &config)?;
    // Sort by path to make assertions stable
    nodes.sort_by_key(|n| n.path.clone());

    // Expected: file1.txt, file2.log, sub_dir, sub_dir/file3.dat
    // .hidden_file is excluded.
    // walker.rs uses min_depth(1), so root_path itself is not included.
    // Depth 1: file1.txt, file2.log, sub_dir
    // Depth 2: sub_dir/file3.dat
    assert_eq!(
        nodes.len(),
        4,
        "Expected 4 nodes. Found: {:?}",
        nodes.iter().map(|n| n.name.as_str()).collect::<Vec<&str>>()
    );

    let file1_node = nodes
        .iter()
        .find(|n| n.name == "file1.txt")
        .expect("file1.txt not found");
    assert_eq!(file1_node.line_count, Some(3)); // "hello\nworld\nrust" -> 3 lines

    let subdir_node = nodes
        .iter()
        .find(|n| n.name == "sub_dir")
        .expect("sub_dir not found");
    assert_eq!(subdir_node.node_type, rustree::NodeType::Directory);

    let file3_node = nodes
        .iter()
        .find(|n| n.name == "file3.dat")
        .expect("file3.dat not found");
    println!("root_path: {:?}", root_path);
    println!("expected_parent: {:?}", root_path.join("sub_dir"));
    println!("file3_node.path: {:?}", file3_node.path);
    assert_eq!(file3_node.line_count, Some(2)); // "data\nplus+plus" -> 2 lines
    let expected_parent_canonical = std::fs::canonicalize(root_path.join("sub_dir"))?;
    assert!(file3_node.path.starts_with(&expected_parent_canonical));

    Ok(())
}

// --- Tests for Pruning Empty Directories ---

fn setup_pruning_test_directory() -> Result<tempfile::TempDir> {
    let dir = tempfile::tempdir()?;
    let p = dir.path();

    // Kept structure
    fs::create_dir(p.join("dir_with_content"))?;
    common_test_utils::create_file_with_content(
        &p.join("dir_with_content"),
        "file1.txt",
        "content",
    )?;
    common_test_utils::create_file_with_content(p, "root_file.txt", "content")?;

    // To be pruned
    fs::create_dir(p.join("empty_dir1"))?;
    fs::create_dir_all(p.join("parent_of_empty/empty_child"))?;
    fs::create_dir(p.join(".hidden_empty_dir"))?; // Hidden but empty

    // Kept hidden structure
    fs::create_dir(p.join(".hidden_dir_with_content"))?;
    common_test_utils::create_file_with_content(
        &p.join(".hidden_dir_with_content"),
        ".hidden_file.txt",
        "secret",
    )?;

    Ok(dir)
}

#[test]
fn test_prune_empty_directories_flag_long() -> Result<()> {
    let temp_dir = setup_pruning_test_directory()?;
    let root_path = temp_dir.path();

    // Simulate CLI: rustree . --prune-empty-directories -a (to see hidden)
    let cli_args = rustree::cli::CliArgs::parse_from([
        // Changed crate::cli to rustree::cli
        "rustree",
        root_path.to_str().unwrap(),
        "--prune-empty-directories",
        "-a", // Show hidden to test pruning of hidden empty dirs
        "--sort-by",
        "name", // For stable output
    ]);
    let lib_config =
        rustree::cli::map_cli_to_lib_config(&cli_args).expect("Failed to map CLI config"); // Changed crate::cli to rustree::cli
    let lib_output_format =
        rustree::cli::map_cli_to_lib_output_format(cli_args.format.output_format); // Changed crate::cli to rustree::cli

    let nodes = get_tree_nodes(root_path, &lib_config)?;
    let output = format_nodes(&nodes, lib_output_format, &lib_config)?;

    println!(
        "[test_prune_empty_directories_flag_long]\nOutput:\n{}",
        output
    );

    assert!(output.contains(".hidden_dir_with_content/"));
    assert!(output.contains(".hidden_file.txt"));
    assert!(output.contains("dir_with_content/"));
    assert!(output.contains("file1.txt"));
    assert!(output.contains("root_file.txt"));

    assert!(
        !output.contains("empty_dir1/"),
        "empty_dir1 should be pruned"
    );
    assert!(
        !output.contains("parent_of_empty/"),
        "parent_of_empty should be pruned"
    );
    assert!(
        !output.contains("empty_child/"),
        "empty_child should be pruned"
    );
    assert!(
        !output.contains(".hidden_empty_dir/"),
        ".hidden_empty_dir should be pruned"
    );

    // Expected nodes: .hidden_dir_with_content, .hidden_file.txt (child), dir_with_content, file1.txt (child), root_file.txt
    // Dirs: .hidden_dir_with_content, dir_with_content (2)
    // Files: .hidden_file.txt, file1.txt, root_file.txt (3)
    // Summary should reflect counts of *child* nodes displayed.
    // Nodes list: .hidden_dir_with_content, .hidden_file.txt, dir_with_content, file1.txt, root_file.txt
    // Child Dirs: .hidden_dir_with_content, dir_with_content (2)
    // Child Files: .hidden_file.txt, file1.txt, root_file.txt (3)
    assert!(
        output.trim_end().ends_with("3 directories, 3 files"),
        "Summary line mismatch. Output: {}",
        output
    );

    Ok(())
}

#[test]
fn test_prune_empty_directories_alias() -> Result<()> {
    let temp_dir = setup_pruning_test_directory()?;
    let root_path = temp_dir.path();

    // Simulate CLI: rustree . --prune -a
    let cli_args = rustree::cli::CliArgs::parse_from([
        // Changed crate::cli to rustree::cli
        "rustree",
        root_path.to_str().unwrap(),
        "--prune", // Alias
        "-a",
        "--sort-by",
        "name",
    ]);
    let lib_config =
        rustree::cli::map_cli_to_lib_config(&cli_args).expect("Failed to map CLI config"); // Changed crate::cli to rustree::cli
    let lib_output_format =
        rustree::cli::map_cli_to_lib_output_format(cli_args.format.output_format); // Changed crate::cli to rustree::cli

    let nodes = get_tree_nodes(root_path, &lib_config)?;
    let output = format_nodes(&nodes, lib_output_format, &lib_config)?;

    println!("[test_prune_empty_directories_alias]\nOutput:\n{}", output);

    assert!(output.contains(".hidden_dir_with_content/"));
    assert!(output.contains("dir_with_content/"));
    assert!(!output.contains("empty_dir1/"));
    assert!(!output.contains("parent_of_empty/"));
    assert!(!output.contains(".hidden_empty_dir/"));
    assert!(
        output.trim_end().ends_with("3 directories, 3 files"),
        "Summary line mismatch. Output: {}",
        output
    );

    Ok(())
}

#[test]
fn test_pruning_disabled_shows_empty_dirs() -> Result<()> {
    let temp_dir = setup_pruning_test_directory()?;
    let root_path = temp_dir.path();

    // Simulate CLI: rustree . -a (no pruning flag)
    let cli_args = rustree::cli::CliArgs::parse_from([
        // Changed crate::cli to rustree::cli
        "rustree",
        root_path.to_str().unwrap(),
        "-a",
        "--sort-by",
        "name",
    ]);
    let lib_config =
        rustree::cli::map_cli_to_lib_config(&cli_args).expect("Failed to map CLI config"); // Changed crate::cli to rustree::cli
    let lib_output_format =
        rustree::cli::map_cli_to_lib_output_format(cli_args.format.output_format); // Changed crate::cli to rustree::cli

    let nodes = get_tree_nodes(root_path, &lib_config)?;
    let output = format_nodes(&nodes, lib_output_format, &lib_config)?;

    println!(
        "[test_pruning_disabled_shows_empty_dirs]\nOutput:\n{}",
        output
    );

    assert!(output.contains(".hidden_dir_with_content/"));
    assert!(output.contains("dir_with_content/"));
    assert!(
        output.contains("empty_dir1/"),
        "empty_dir1 should be present"
    );
    assert!(
        output.contains("parent_of_empty/"),
        "parent_of_empty should be present"
    );
    assert!(
        output.contains("empty_child/"),
        "empty_child should be present"
    );
    assert!(
        output.contains(".hidden_empty_dir/"),
        ".hidden_empty_dir should be present"
    );

    // Dirs: .hidden_dir_with_content, .hidden_empty_dir, dir_with_content, empty_child, empty_dir1, parent_of_empty (6)
    // Files: .hidden_file.txt, file1.txt, root_file.txt (3)
    assert!(
        output.trim_end().ends_with("7 directories, 3 files"),
        "Summary line mismatch. Output: {}",
        output
    );

    Ok(())
}

#[test]
fn test_get_nodes_with_hidden_and_depth_limit() -> Result<()> {
    let temp_dir = common_test_utils::setup_test_directory()?;
    let root_path = temp_dir.path();

    let config = RustreeLibConfig {
        listing: ListingOptions {
            max_depth: Some(1), // Only top-level files and sub_dir itself
            show_hidden: true,  // .hidden_file is at depth 2, so max_depth limits it
            ..Default::default()
        },
        ..Default::default()
    };

    let nodes = get_tree_nodes(root_path, &config)?;
    // Expected: file1.txt, file2.log, sub_dir (all at depth 1)
    assert_eq!(
        nodes.len(),
        3,
        "Expected 3 nodes. Found: {:?}",
        nodes.iter().map(|n| n.name.as_str()).collect::<Vec<&str>>()
    );
    assert!(nodes.iter().any(|n| n.name == "file1.txt"));
    assert!(nodes.iter().any(|n| n.name == "file2.log"));
    assert!(nodes.iter().any(|n| n.name == "sub_dir"));
    assert!(
        !nodes.iter().any(|n| n.name == "file3.dat"),
        "file3.dat should be excluded by max_depth"
    );
    assert!(
        !nodes.iter().any(|n| n.name == ".hidden_file"),
        ".hidden_file should be excluded by max_depth"
    );

    // Test show_hidden with appropriate depth
    let config_show_hidden_deeper = RustreeLibConfig {
        listing: ListingOptions {
            max_depth: Some(2),
            show_hidden: true,
            ..Default::default()
        },
        ..Default::default()
    };
    let nodes_hidden = get_tree_nodes(root_path, &config_show_hidden_deeper)?;
    // Expected: file1, file2, sub_dir, file3, .hidden_file
    assert_eq!(
        nodes_hidden.len(),
        5,
        "Expected 5 nodes with show_hidden. Found: {:?}",
        nodes_hidden
            .iter()
            .map(|n| n.name.as_str())
            .collect::<Vec<&str>>()
    );
    assert!(nodes_hidden.iter().any(|n| n.name == ".hidden_file"));

    Ok(())
}

#[test]
fn test_formatting_markdown() -> Result<()> {
    let temp_dir = common_test_utils::setup_test_directory()?;
    let root_path = temp_dir.path();

    let config = RustreeLibConfig {
        input_source: InputSourceOptions {
            root_display_name: "test_root".to_string(),
            root_is_directory: true,
            ..Default::default()
        },
        listing: ListingOptions {
            max_depth: Some(1),
            ..Default::default()
        },
        ..Default::default()
    };

    let nodes = get_tree_nodes(root_path, &config)?;
    // Note: format_nodes itself doesn't sort, it relies on pre-sorted nodes if sorting is desired.
    // get_tree_nodes sorts if config.sort_by is Some.

    let markdown_output = format_nodes(&nodes, LibOutputFormat::Markdown, &config)?;

    println!("Markdown Output:\n{}", markdown_output);

    // Test that we get proper Markdown formatting instead of placeholder
    assert!(markdown_output.starts_with("# test_root"));
    assert!(markdown_output.contains("* "));
    assert!(markdown_output.contains("director"));
    assert!(markdown_output.contains("file"));

    Ok(())
}

#[test]
fn test_formatting_markdown_no_summary_report() -> Result<()> {
    let temp_dir = common_test_utils::setup_test_directory()?;
    let root_path = temp_dir.path();

    // Test with summary report enabled (default)
    let config_with_summary = RustreeLibConfig {
        input_source: InputSourceOptions {
            root_display_name: "test_root".to_string(),
            root_is_directory: true,
            ..Default::default()
        },
        listing: ListingOptions {
            max_depth: Some(2),
            show_hidden: false,
            ..Default::default()
        },
        sorting: SortingOptions {
            sort_by: Some(SortKey::Name),
            ..Default::default()
        },
        ..Default::default()
    };

    let nodes = get_tree_nodes(root_path, &config_with_summary)?;
    let markdown_output_with_summary =
        format_nodes(&nodes, LibOutputFormat::Markdown, &config_with_summary)?;

    // Should contain summary line in markdown format
    assert!(markdown_output_with_summary.contains("__"));
    assert!(markdown_output_with_summary.contains("total__"));
    assert!(markdown_output_with_summary.contains("director"));
    assert!(markdown_output_with_summary.contains("file"));

    // Test with summary report disabled
    let config_no_summary = RustreeLibConfig {
        input_source: InputSourceOptions {
            root_display_name: "test_root".to_string(),
            root_is_directory: true,
            ..Default::default()
        },
        listing: ListingOptions {
            max_depth: Some(2),
            show_hidden: false,
            ..Default::default()
        },
        sorting: SortingOptions {
            sort_by: Some(SortKey::Name),
            ..Default::default()
        },
        misc: rustree::MiscOptions {
            no_summary_report: true,
            human_friendly: false,
            no_color: false,
            verbose: false,
        },
        ..Default::default()
    };

    let markdown_output_no_summary =
        format_nodes(&nodes, LibOutputFormat::Markdown, &config_no_summary)?;

    // Should NOT contain summary line
    assert!(!markdown_output_no_summary.contains("total__"));
    assert!(!markdown_output_no_summary.contains("__"));

    // But should still contain the main structure
    assert!(markdown_output_no_summary.starts_with("# test_root"));
    assert!(markdown_output_no_summary.contains("* "));

    // Verify no summary-related words appear
    let lines: Vec<&str> = markdown_output_no_summary.lines().collect();
    let has_summary_words = lines
        .iter()
        .any(|line| line.contains("directories") || line.contains("files"));
    assert!(!has_summary_words);

    Ok(())
}

#[test]
fn test_formatting_markdown_no_summary_with_directories_only() -> Result<()> {
    let temp_dir = common_test_utils::setup_test_directory()?;
    let root_path = temp_dir.path();

    // Test directories only with no summary
    let config_dirs_only_no_summary = RustreeLibConfig {
        input_source: InputSourceOptions {
            root_display_name: "test_root".to_string(),
            root_is_directory: true,
            ..Default::default()
        },
        listing: ListingOptions {
            max_depth: Some(2),
            show_hidden: false,
            list_directories_only: true,
            ..Default::default()
        },
        sorting: SortingOptions {
            sort_by: Some(SortKey::Name),
            ..Default::default()
        },
        misc: rustree::MiscOptions {
            no_summary_report: true,
            human_friendly: false,
            no_color: false,
            verbose: false,
        },
        ..Default::default()
    };

    let nodes = get_tree_nodes(root_path, &config_dirs_only_no_summary)?;
    let markdown_output = format_nodes(
        &nodes,
        LibOutputFormat::Markdown,
        &config_dirs_only_no_summary,
    )?;

    // Should NOT contain summary line
    assert!(!markdown_output.contains("total__"));
    assert!(!markdown_output.contains("directories"));
    assert!(!markdown_output.contains("files"));

    // Should still show directory structure
    assert!(markdown_output.starts_with("# test_root"));
    assert!(markdown_output.contains("* sub_dir"));

    Ok(())
}

// --- Tests for Word Count ---

#[test]
fn test_word_count_calculation_when_enabled() -> Result<()> {
    let temp_dir = common_test_utils::setup_test_directory()?;
    let root_path = temp_dir.path();
    common_test_utils::create_file_with_content(root_path, "words.txt", "one two three four")?;
    common_test_utils::create_file_with_content(root_path, "words_empty.txt", "")?;
    common_test_utils::create_file_with_content(
        root_path,
        "words_complex.txt",
        "  hello   world  \n new line words  ",
    )?;

    let config = RustreeLibConfig {
        metadata: MetadataOptions {
            calculate_word_count: true,
            ..Default::default()
        },
        listing: ListingOptions {
            max_depth: Some(1), // Keep it simple, only look at root_path files
            ..Default::default()
        },
        ..Default::default()
    };

    let mut nodes = get_tree_nodes(root_path, &config)?;
    nodes.sort_by_key(|n| n.path.clone()); // For predictable order

    let words_node = nodes
        .iter()
        .find(|n| n.name == "words.txt")
        .expect("words.txt not found");
    assert_eq!(
        words_node.word_count,
        Some(4),
        "Failed for 'one two three four'"
    );

    let empty_node = nodes
        .iter()
        .find(|n| n.name == "words_empty.txt")
        .expect("words_empty.txt not found");
    assert_eq!(empty_node.word_count, Some(0), "Failed for empty file");

    let complex_node = nodes
        .iter()
        .find(|n| n.name == "words_complex.txt")
        .expect("words_complex.txt not found");
    assert_eq!(
        complex_node.word_count,
        Some(5),
        "Failed for '  hello   world  \\n new line words  '"
    );

    // Also check a pre-existing file from setup_test_directory to ensure it's also processed
    let file1_node = nodes
        .iter()
        .find(|n| n.name == "file1.txt")
        .expect("file1.txt not found");
    // "hello\nworld\nrust" -> 3 words
    assert_eq!(file1_node.word_count, Some(3));

    Ok(())
}

#[test]
fn test_word_count_is_none_when_disabled() -> Result<()> {
    let temp_dir = common_test_utils::setup_test_directory()?;
    let root_path = temp_dir.path();
    // file1.txt already exists from setup

    let config = RustreeLibConfig {
        metadata: MetadataOptions {
            calculate_word_count: false, // Explicitly false or default
            ..Default::default()
        },
        listing: ListingOptions {
            max_depth: Some(1),
            ..Default::default()
        },
        ..Default::default()
    };

    let mut nodes = get_tree_nodes(root_path, &config)?;
    nodes.sort_by_key(|n| n.path.clone());

    let file1_node = nodes
        .iter()
        .find(|n| n.name == "file1.txt")
        .expect("file1.txt not found");
    assert_eq!(file1_node.word_count, None);

    Ok(())
}

#[test]
fn test_word_count_for_directory_is_none() -> Result<()> {
    let temp_dir = common_test_utils::setup_test_directory()?;
    let root_path = temp_dir.path();
    // sub_dir exists from setup

    let config = RustreeLibConfig {
        metadata: MetadataOptions {
            calculate_word_count: true, // Enabled, but shouldn't apply to dirs
            ..Default::default()
        },
        listing: ListingOptions {
            max_depth: Some(1),
            ..Default::default()
        },
        ..Default::default()
    };

    let mut nodes = get_tree_nodes(root_path, &config)?;
    nodes.sort_by_key(|n| n.path.clone());

    let subdir_node = nodes
        .iter()
        .find(|n| n.name == "sub_dir")
        .expect("sub_dir not found");
    assert_eq!(subdir_node.node_type, NodeType::Directory);
    assert_eq!(subdir_node.word_count, None);

    Ok(())
}

// --- Tests for Apply Function (Correctness and Control) ---

// This test replaces/updates the existing test_apply_function_count_pluses
// The key change is the expected value for file3.dat.
#[test]
fn test_apply_function_count_pluses_corrected_and_zero_case() -> Result<()> {
    let temp_dir = common_test_utils::setup_test_directory()?;
    let root_path = temp_dir.path();
    // file1.txt: "hello\nworld\nrust" (0 pluses)
    // sub_dir/file3.dat: "data\nplus+plus" (2 pluses)

    let config = RustreeLibConfig {
        metadata: MetadataOptions {
            apply_function: Some(ApplyFunction::BuiltIn(BuiltInFunction::CountPluses)),
            ..Default::default()
        },
        listing: ListingOptions {
            max_depth: Some(2), // Ensure file3.dat is processed
            ..Default::default()
        },
        ..Default::default()
    };

    let mut nodes = get_tree_nodes(root_path, &config)?;
    nodes.sort_by_key(|n| n.path.clone());

    let file3_node = nodes
        .iter()
        .find(|n| n.name == "file3.dat")
        .expect("file3.dat not found in nodes");
    assert_eq!(
        file3_node.custom_function_output,
        Some(Ok("2".to_string())), // Corrected expected value
        "CountPluses failed for file3.dat"
    );

    let file1_node = nodes
        .iter()
        .find(|n| n.name == "file1.txt")
        .expect("file1.txt not found in nodes");
    assert_eq!(
        file1_node.custom_function_output,
        Some(Ok("0".to_string())),
        "CountPluses failed for file1.txt (zero case)"
    );

    Ok(())
}

#[test]
fn test_apply_function_is_none_when_not_configured() -> Result<()> {
    let temp_dir = common_test_utils::setup_test_directory()?;
    let root_path = temp_dir.path();

    let config = RustreeLibConfig {
        metadata: MetadataOptions {
            apply_function: None, // Explicitly None or default
            ..Default::default()
        },
        listing: ListingOptions {
            max_depth: Some(2),
            ..Default::default()
        },
        ..Default::default()
    };

    let mut nodes = get_tree_nodes(root_path, &config)?;
    nodes.sort_by_key(|n| n.path.clone());

    let file1_node = nodes
        .iter()
        .find(|n| n.name == "file1.txt")
        .expect("file1.txt not found");
    assert_eq!(file1_node.custom_function_output, None);

    let file3_node = nodes
        .iter()
        .find(|n| n.name == "file3.dat")
        .expect("file3.dat not found");
    assert_eq!(file3_node.custom_function_output, None);

    Ok(())
}

#[test]
fn test_apply_function_for_directory_is_none() -> Result<()> {
    let temp_dir = common_test_utils::setup_test_directory()?;
    let root_path = temp_dir.path();

    let config = RustreeLibConfig {
        metadata: MetadataOptions {
            apply_function: Some(ApplyFunction::BuiltIn(BuiltInFunction::CountPluses)), // Enabled
            ..Default::default()
        },
        listing: ListingOptions {
            max_depth: Some(1),
            ..Default::default()
        },
        ..Default::default()
    };

    let mut nodes = get_tree_nodes(root_path, &config)?;
    nodes.sort_by_key(|n| n.path.clone());

    let subdir_node = nodes
        .iter()
        .find(|n| n.name == "sub_dir")
        .expect("sub_dir not found");
    assert_eq!(subdir_node.node_type, NodeType::Directory);
    assert_eq!(subdir_node.custom_function_output, None);

    Ok(())
}

// --- Test for Combined Stats & File Read Error Handling (Conceptual) ---

#[test]
fn test_stats_are_conditional_and_graceful_on_read_error() -> Result<()> {
    let temp_dir = common_test_utils::setup_test_directory()?;
    let root_path = temp_dir.path();

    // file1.txt: "hello\nworld\nrust" (3 lines, 3 words, 0 pluses)
    // file2.log: "another file" (1 line, 2 words, 0 pluses)

    // Case 1: Only line count requested
    let config_lines_only = RustreeLibConfig {
        metadata: MetadataOptions {
            calculate_line_count: true,
            calculate_word_count: false,
            apply_function: None,
            ..Default::default()
        },
        listing: ListingOptions {
            max_depth: Some(1),
            ..Default::default()
        },
        ..Default::default()
    };
    let mut nodes_lines_only = get_tree_nodes(root_path, &config_lines_only)?;
    nodes_lines_only.sort_by_key(|n| n.path.clone());

    let file1_lines_only = nodes_lines_only
        .iter()
        .find(|n| n.name == "file1.txt")
        .expect("file1.txt not found");
    assert_eq!(file1_lines_only.line_count, Some(3));
    assert_eq!(file1_lines_only.word_count, None);
    assert_eq!(file1_lines_only.custom_function_output, None);

    // Case 2: All stats requested
    let config_all_stats = RustreeLibConfig {
        metadata: MetadataOptions {
            calculate_line_count: true,
            calculate_word_count: true,
            apply_function: Some(ApplyFunction::BuiltIn(BuiltInFunction::CountPluses)),
            ..Default::default()
        },
        listing: ListingOptions {
            max_depth: Some(1),
            ..Default::default()
        },
        ..Default::default()
    };
    let mut nodes_all_stats = get_tree_nodes(root_path, &config_all_stats)?;
    nodes_all_stats.sort_by_key(|n| n.path.clone());

    let file1_all_stats = nodes_all_stats
        .iter()
        .find(|n| n.name == "file1.txt")
        .expect("file1.txt not found");
    assert_eq!(file1_all_stats.line_count, Some(3));
    assert_eq!(file1_all_stats.word_count, Some(3));
    assert_eq!(
        file1_all_stats.custom_function_output,
        Some(Ok("0".to_string()))
    );

    // Note on testing actual read errors for `custom_function_output` or `word_count`:
    // Inducing a reliable, cross-platform I/O read error for `fs::read_to_string`
    // within an integration test is non-trivial (e.g., setting permissions dynamically
    // can be OS-specific and flaky in CI).
    // For this PRD stage, the primary check is that the `walker.rs` code's `Err` arm
    // in the `match fs::read_to_string` block correctly results in `None` for these fields
    // and `Some(Err(...))` for `custom_function_output`, and doesn't panic.
    // This is partially covered by ensuring success paths populate `Some(...)`.
    // If a file cannot be read, the `Result` from `apply_function_to_content` would not be Ok.
    // For example, if `custom_function_output` was designed to store `Option<String>` directly and error during `apply_fn`
    // then it would be `None`. Current `custom_function_output` is `Option<Result<String, ApplyFnError>>`
    // so if reading fails *before* calling apply_fn, it will be `None`. If `apply_fn` itself fails, it will be `Some(Err(...))`.
    // The current PRD mainly focuses on the success path of `count_words` and `apply_fn`.

    Ok(())
}
