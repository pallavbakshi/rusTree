// tests/ignore_features_tests.rs

use rustree::{get_tree_nodes, RustreeLibConfig, NodeInfo}; // Assuming RustreeLibConfig is directly accessible
use anyhow::Result;
use std::collections::HashSet;
use std::fs::{self, File};
use std::io::Write;
// use std::path::{Path, PathBuf}; // PathBuf might be needed for config
// tempfile::TempDir is used via common_test_utils return type, direct import not needed.

// Assume common module exists and has setup_complex_test_directory or similar
mod common;
// create_file_with_content is used by setup_gitignore_test_dir in common.rs, not directly here.
// common_test_utils itself is needed for setup_gitignore_test_dir.
use common::common_test_utils;

// Helper to get names from nodes for easier assertion
fn get_node_names(nodes: &[NodeInfo]) -> HashSet<String> {
    nodes.iter().map(|n| n.name.clone()).collect()
}

// Using common_test_utils::setup_gitignore_test_dir from common.rs

// --- -I / --ignore-path Tests ---

#[test]
fn test_ignore_single_file_by_name() -> Result<()> {
    let temp_dir = common_test_utils::setup_gitignore_test_dir()?;
    let config = RustreeLibConfig {
        ignore_patterns: Some(vec!["file.txt".to_string()]),
        ..Default::default()
    };
    let nodes = get_tree_nodes(temp_dir.path(), &config)?;
    let names = get_node_names(&nodes);
    assert!(!names.contains("file.txt"), "file.txt should be ignored");
    assert!(names.contains("file.log"), "file.log should still be present (unless gitignored and --gitignore is on by default, which it is not)");
    Ok(())
}

#[test]
fn test_ignore_files_by_wildcard() -> Result<()> {
    let temp_dir = common_test_utils::setup_gitignore_test_dir()?;
    let config = RustreeLibConfig {
        ignore_patterns: Some(vec!["*.log".to_string()]),
        ..Default::default()
    };
    let nodes = get_tree_nodes(temp_dir.path(), &config)?;
    let names = get_node_names(&nodes);
    assert!(!names.contains("file.log"), "*.log should ignore file.log");
    Ok(())
}

#[test]
fn test_ignore_directory_by_name_suffix_slash() -> Result<()> {
    let temp_dir = common_test_utils::setup_gitignore_test_dir()?;
    let config = RustreeLibConfig {
        ignore_patterns: Some(vec!["target/".to_string()]),
        ..Default::default()
    };
    let nodes = get_tree_nodes(temp_dir.path(), &config)?;
    let names = get_node_names(&nodes);
    assert!(!names.contains("target"), "target/ should ignore target directory");
    assert!(!names.contains("app.exe"), "target/ should ignore contents like app.exe");
    Ok(())
}

#[test]
fn test_ignore_directory_by_name_no_suffix_slash() -> Result<()> {
    let temp_dir = common_test_utils::setup_gitignore_test_dir()?;
    // Original tree behavior: -I pattern (no slash) matches both files and dirs by that name.
    // Our glob implementation: "docs" matches a file or dir named "docs".
    // If "docs" is a directory, it and its contents are effectively ignored because
    // the directory entry itself is skipped by the -I filter.
    let config = RustreeLibConfig {
        ignore_patterns: Some(vec!["docs".to_string()]),
        ..Default::default()
    };
    let nodes = get_tree_nodes(temp_dir.path(), &config)?;
    let names = get_node_names(&nodes);
    assert!(!names.contains("docs"), "docs (no slash) should ignore docs directory");
    assert!(!names.contains("api.md"), "docs (no slash) should ignore contents like api.md");
    Ok(())
}

#[test]
fn test_multiple_ignore_flags() -> Result<()> {
    let temp_dir = common_test_utils::setup_gitignore_test_dir()?;
    let config = RustreeLibConfig {
        ignore_patterns: Some(vec!["*.log".to_string(), "docs/".to_string()]),
        ..Default::default()
    };
    let nodes = get_tree_nodes(temp_dir.path(), &config)?;
    let names = get_node_names(&nodes);
    assert!(!names.contains("file.log"), "file.log should be ignored by *.log");
    assert!(!names.contains("docs"), "docs directory should be ignored by docs/");
    assert!(!names.contains("api.md"), "api.md in docs/ should be ignored");
    Ok(())
}

#[test]
fn test_ignore_path_long_flag() -> Result<()> {
    // This is a CLI parsing test, implicitly tested by using ignore_patterns in RustreeLibConfig
    // For library unit test, it's same as test_ignore_files_by_wildcard
    let temp_dir = common_test_utils::setup_gitignore_test_dir()?;
    let config = RustreeLibConfig {
        ignore_patterns: Some(vec!["*.log".to_string()]), // Simulates --ignore-path *.log
        ..Default::default()
    };
    let nodes = get_tree_nodes(temp_dir.path(), &config)?;
    let names = get_node_names(&nodes);
    assert!(!names.contains("file.log"), "file.log should be ignored");
    Ok(())
}

#[test]
fn test_ignore_with_pipe_alternation() -> Result<()> {
    let temp_dir = common_test_utils::setup_gitignore_test_dir()?;
    let config = RustreeLibConfig {
        ignore_patterns: Some(vec!["*.log|*.temp".to_string()]),
        ..Default::default()
    };
    let nodes = get_tree_nodes(temp_dir.path(), &config)?;
    let names = get_node_names(&nodes);
    assert!(!names.contains("file.log"), "file.log should be ignored by *.log|*.temp");
    assert!(!names.contains("module.temp"), "module.temp should be ignored by *.log|*.temp");
    assert!(names.contains("main.rs"), "main.rs should still be present");
    Ok(())
}

#[test]
fn test_ignore_interaction_with_p_match() -> Result<()> {
    let temp_dir = common_test_utils::setup_gitignore_test_dir()?;
    let config = RustreeLibConfig {
        ignore_patterns: Some(vec!["*.log".to_string()]),
        match_patterns: Some(vec!["*.log".to_string()]), // Attempt to match .log files
        ..Default::default()
    };
    let nodes = get_tree_nodes(temp_dir.path(), &config)?;
    let names = get_node_names(&nodes);
    // Even if -P *.log tries to match, -I *.log should take precedence.
    assert!(!names.contains("file.log"), ".log files should NOT be present (ignore takes precedence)");
    Ok(())
}

#[test]
fn test_ignore_path_pattern_relative_to_subdir() -> Result<()> {
    let temp_dir = common_test_utils::setup_gitignore_test_dir()?;
    let config = RustreeLibConfig {
        ignore_patterns: Some(vec!["src/*.temp".to_string()]),
        ..Default::default()
    };
    let nodes = get_tree_nodes(temp_dir.path(), &config)?;
    let names = get_node_names(&nodes);
    assert!(!names.iter().any(|p| p.ends_with("module.temp")), "src/module.temp should not be present");
    assert!(names.contains("main.rs"), "src/main.rs should be present");
    Ok(())
}

#[test]
fn test_ignore_doublestar_pattern() -> Result<()> {
    let temp_dir = common_test_utils::setup_gitignore_test_dir()?;
    let config = RustreeLibConfig {
        ignore_patterns: Some(vec!["**/*.temp".to_string()]),
        ..Default::default()
    };
    let nodes = get_tree_nodes(temp_dir.path(), &config)?;
    let names = get_node_names(&nodes);
    
    assert!(!names.iter().any(|p| p.ends_with("module.temp")), "src/module.temp should not be present due to **/*.temp");
    Ok(())
}


// --- --gitignore Tests ---

#[test]
fn test_gitignore_basic_root() -> Result<()> {
    let temp_dir = common_test_utils::setup_gitignore_test_dir()?;
    let config = RustreeLibConfig {
        use_gitignore: true,
        ..Default::default()
    };
    let nodes = get_tree_nodes(temp_dir.path(), &config)?;
    let names = get_node_names(&nodes);

    assert!(!names.contains("file.log"), "file.log should be gitignored");
    assert!(!names.contains("target"), "target dir should be gitignored");
    assert!(!names.contains("app.exe"), "app.exe in target should be gitignored");
    
    // The ignore crate respects case sensitivity for gitignore patterns.
    // The pattern "IMAGE.PNG" in .gitignore will only match files named exactly "IMAGE.PNG".
    // Since the actual file is named "image.PNG" (different case), it should NOT be ignored.
    // Note: On case-insensitive filesystems, only one of the two files can exist,
    // and the second file creation overwrites the first, so we end up with "image.PNG".
    assert!(names.contains("image.PNG"), "image.PNG should NOT be gitignored (case mismatch with IMAGE.PNG pattern)");
    
    assert!(names.contains("file.txt"), "file.txt should be present");
    assert!(names.contains("docs"), "docs dir should be present");
    assert!(names.contains("api.md"), "api.md in docs should be present");
    assert!(names.contains("src"), "src dir should be present");
    Ok(())
}

#[test]
fn test_gitignore_nested() -> Result<()> {
    let temp_dir = common_test_utils::setup_gitignore_test_dir()?;
    let config = RustreeLibConfig {
        use_gitignore: true,
        ..Default::default()
    };
    let nodes = get_tree_nodes(temp_dir.path(), &config)?;
    let names = get_node_names(&nodes);
    assert!(!names.contains("module.temp"), "src/module.temp should be gitignored by src/.gitignore");
    assert!(names.contains("main.rs"), "src/main.rs should be present");
    Ok(())
}

#[test]
fn test_gitignore_with_show_hidden() -> Result<()> {
    let temp_dir = common_test_utils::setup_gitignore_test_dir()?;
    // Add .secret_file to .gitignore
    let mut root_gitignore = fs::OpenOptions::new().append(true).open(temp_dir.path().join(".gitignore"))?;
    writeln!(root_gitignore, ".secret_file")?;
    drop(root_gitignore);

    let config = RustreeLibConfig {
        use_gitignore: true,
        show_hidden: true, // -a
        ..Default::default()
    };
    let nodes = get_tree_nodes(temp_dir.path(), &config)?;
    let names = get_node_names(&nodes);

    assert!(!names.contains(".secret_file"), ".secret_file should NOT be listed (ignored by .gitignore)");
    assert!(names.contains(".hidden_dir"), ".hidden_dir IS listed (not ignored by .gitignore and -a is on)");
    assert!(names.contains("content.txt"), "content.txt in .hidden_dir IS listed");
    Ok(())
}

#[test]
#[ignore] // Ignoring for now due to complexity of mocking git environment
fn test_gitignore_global() -> Result<()> {
    // Setup: Mock a global gitignore file (e.g. by setting HOME and creating .config/git/ignore)
    // Config with --gitignore
    // Assert global rules are respected
    todo!();
}

#[test]
#[ignore] // Ignoring for now due to complexity of mocking git environment
fn test_gitignore_repo_specific_info_exclude() -> Result<()> {
    // Setup: Mock a .git/info/exclude file
    // Config with --gitignore
    // Assert repo-specific rules are respected
    todo!();
}


// --- --gitfile Tests ---

#[test]
fn test_gitfile_single_custom_ignore() -> Result<()> {
    let temp_dir = common_test_utils::setup_gitignore_test_dir()?;
    let custom_ignore_path = temp_dir.path().join("custom.ignore");
    let mut custom_ignore_file = File::create(&custom_ignore_path)?;
    writeln!(custom_ignore_file, "docs/")?;
    writeln!(custom_ignore_file, "*.rs")?;
    drop(custom_ignore_file);

    let config = RustreeLibConfig {
        git_ignore_files: Some(vec![custom_ignore_path]),
        // use_gitignore: false, // Ensure only custom file is used
        ..Default::default()
    };
    let nodes = get_tree_nodes(temp_dir.path(), &config)?;
    let names = get_node_names(&nodes);

    assert!(!names.contains("docs"), "docs/ should be ignored by custom.ignore");
    assert!(!names.contains("api.md"), "api.md in docs/ should be ignored");
    assert!(!names.contains("main.rs"), "main.rs should be ignored by custom.ignore");
    assert!(names.contains("file.txt"), "file.txt should be present");
    // file.log is not ignored by custom.ignore, and .gitignore is not active by default
    assert!(names.contains("file.log"), "file.log should be present");
    Ok(())
}

#[test]
fn test_gitfile_multiple_custom_ignores() -> Result<()> {
    let temp_dir = common_test_utils::setup_gitignore_test_dir()?;
    let custom_ignore1_path = temp_dir.path().join("custom1.ignore");
    let mut custom_ignore1_file = File::create(&custom_ignore1_path)?;
    writeln!(custom_ignore1_file, "docs/")?;
    drop(custom_ignore1_file);

    let custom_ignore2_path = temp_dir.path().join("custom2.ignore");
    let mut custom_ignore2_file = File::create(&custom_ignore2_path)?;
    writeln!(custom_ignore2_file, "*.rs")?;
    drop(custom_ignore2_file);

    let config = RustreeLibConfig {
        git_ignore_files: Some(vec![custom_ignore1_path, custom_ignore2_path]),
        ..Default::default()
    };
    let nodes = get_tree_nodes(temp_dir.path(), &config)?;
    let names = get_node_names(&nodes);
    assert!(!names.contains("docs"), "docs/ should be ignored by custom1.ignore");
    assert!(!names.contains("main.rs"), "main.rs should be ignored by custom2.ignore");
    Ok(())
}

#[test]
fn test_gitfile_path_relativity() -> Result<()> {
    // PRD: "patterns in files specified by --gitfile are matched as if the --gitfile
    // was located at the root of the scan."
    let temp_dir = common_test_utils::setup_gitignore_test_dir()?;
    let custom_ignore_path = temp_dir.path().join("custom.ignore");
    let mut custom_ignore_file = File::create(&custom_ignore_path)?;
    // This pattern should match src/main.rs if paths are relative to scan root
    writeln!(custom_ignore_file, "src/main.rs")?;
    drop(custom_ignore_file);

    let config = RustreeLibConfig {
        git_ignore_files: Some(vec![custom_ignore_path]),
        ..Default::default()
    };
    let nodes = get_tree_nodes(temp_dir.path(), &config)?;
    let names = get_node_names(&nodes);
    assert!(!names.contains("main.rs"), "src/main.rs should be ignored by custom.ignore with relative path");
    Ok(())
}


// --- --ignore-case Tests ---

#[test]
fn test_ignore_case_with_i_flag() -> Result<()> {
    let temp_dir = common_test_utils::setup_gitignore_test_dir()?; // Has image.png and image.PNG
    let config = RustreeLibConfig {
        ignore_patterns: Some(vec!["image.png".to_string()]),
        ignore_case_for_patterns: true,
        ..Default::default()
    };
    let nodes = get_tree_nodes(temp_dir.path(), &config)?;
    let names = get_node_names(&nodes);
    assert!(!names.contains("image.png"), "image.png should be ignored (case-insensitive)");
    assert!(!names.contains("image.PNG"), "image.PNG should be ignored (case-insensitive)");
    Ok(())
}

#[test]
fn test_ignore_case_with_gitignore() -> Result<()> {
    let temp_dir = common_test_utils::setup_gitignore_test_dir()?; // .gitignore has "IMAGE.PNG"
    let config = RustreeLibConfig {
        use_gitignore: true,
        ignore_case_for_patterns: true,
        ..Default::default()
    };
    let nodes = get_tree_nodes(temp_dir.path(), &config)?;
    let names = get_node_names(&nodes);
    // .gitignore has "IMAGE.PNG". With --ignore-case, this should match "image.png".
    assert!(!names.contains("image.png"), "image.png (lowercase) should be gitignored due to case-insensitive match with IMAGE.PNG");
    assert!(!names.contains("image.PNG"), "image.PNG should also be gitignored");
    Ok(())
}

#[test]
fn test_ignore_case_with_gitfile() -> Result<()> {
    let temp_dir = common_test_utils::setup_gitignore_test_dir()?;
    let custom_ignore_path = temp_dir.path().join("custom_case.ignore");
    let mut custom_ignore_file = File::create(&custom_ignore_path)?;
    writeln!(custom_ignore_file, "IMAGE.PNG")?;
    drop(custom_ignore_file);

    let config = RustreeLibConfig {
        git_ignore_files: Some(vec![custom_ignore_path]),
        ignore_case_for_patterns: true,
        ..Default::default()
    };
    let nodes = get_tree_nodes(temp_dir.path(), &config)?;
    let names = get_node_names(&nodes);
    assert!(!names.contains("image.png"), "image.png (lowercase) should be ignored by custom_case.ignore (case-insensitive)");
    assert!(!names.contains("image.PNG"), "image.PNG should also be ignored");
    Ok(())
}


// --- Combination Tests ---

#[test]
fn test_combination_i_and_gitignore() -> Result<()> {
    let temp_dir = common_test_utils::setup_gitignore_test_dir()?;
    // .gitignore ignores *.log and target/ and IMAGE.PNG
    let config = RustreeLibConfig {
        ignore_patterns: Some(vec!["docs/".to_string()]), // -I docs/
        use_gitignore: true, // --gitignore
        ..Default::default()
    };
    let nodes = get_tree_nodes(temp_dir.path(), &config)?;
    let names = get_node_names(&nodes);
    assert!(!names.contains("file.log"), "file.log should be ignored by .gitignore");
    assert!(!names.contains("docs"), "docs/ should be ignored by -I");
    assert!(!names.contains("api.md"), "api.md in docs/ should be ignored");
    assert!(!names.contains("target"), "target/ should be ignored by .gitignore");
    Ok(())
}

#[test]
fn test_combination_gitfile_and_gitignore() -> Result<()> {
    let temp_dir = common_test_utils::setup_gitignore_test_dir()?;
    let custom_ignore_path = temp_dir.path().join("custom.ignore");
    let mut custom_ignore_file = File::create(&custom_ignore_path)?;
    writeln!(custom_ignore_file, "*.rs")?; // Ignore .rs files
    drop(custom_ignore_file);

    let config = RustreeLibConfig {
        git_ignore_files: Some(vec![custom_ignore_path]),
        use_gitignore: true,
        ..Default::default()
    };
    let nodes = get_tree_nodes(temp_dir.path(), &config)?;
    let names = get_node_names(&nodes);
    assert!(!names.contains("file.log"), "file.log should be ignored by .gitignore");
    assert!(!names.contains("main.rs"), "main.rs should be ignored by custom.ignore");
    Ok(())
}

#[test]
fn test_combination_all_ignore_mechanisms() -> Result<()> {
    let temp_dir = common_test_utils::setup_gitignore_test_dir()?;
    let custom_ignore_path = temp_dir.path().join("custom.ignore");
    let mut custom_ignore_file = File::create(&custom_ignore_path)?;
    writeln!(custom_ignore_file, "*.rs")?; // Ignore .rs files from custom
    drop(custom_ignore_file);

    let config = RustreeLibConfig {
        ignore_patterns: Some(vec!["docs/".to_string()]), // -I docs/
        use_gitignore: true, // --gitignore (ignores *.log, target/, IMAGE.PNG)
        git_ignore_files: Some(vec![custom_ignore_path]), // --gitfile (ignores *.rs)
        ..Default::default()
    };
    let nodes = get_tree_nodes(temp_dir.path(), &config)?;
    let names = get_node_names(&nodes);
    assert!(!names.contains("docs"), "docs/ should be ignored by -I");
    assert!(!names.contains("file.log"), "file.log should be ignored by .gitignore");
    assert!(!names.contains("main.rs"), "main.rs should be ignored by custom.ignore");
    assert!(names.contains("file.txt"), "file.txt should still be present");
    Ok(())
}


// --- Edge Case Tests ---

#[test]
fn test_ignore_empty_pattern_string() -> Result<()> {
    let temp_dir = common_test_utils::setup_gitignore_test_dir()?;
    let initial_nodes_config = RustreeLibConfig { ..Default::default() };
    let initial_nodes = get_tree_nodes(temp_dir.path(), &initial_nodes_config)?;
    let initial_names_count = get_node_names(&initial_nodes).len();

    let config = RustreeLibConfig {
        ignore_patterns: Some(vec!["".to_string()]), // -I ""
        ..Default::default()
    };
    let nodes = get_tree_nodes(temp_dir.path(), &config)?;
    let names = get_node_names(&nodes);
    // Empty glob pattern matches nothing. So, no files should be ignored due to this pattern.
    assert_eq!(names.len(), initial_names_count, "Empty pattern string should not ignore any files");
    Ok(())
}

#[test]
fn test_ignore_pattern_matches_nothing() -> Result<()> {
    let temp_dir = common_test_utils::setup_gitignore_test_dir()?;
    let initial_nodes_config = RustreeLibConfig { ..Default::default() };
    let initial_nodes = get_tree_nodes(temp_dir.path(), &initial_nodes_config)?;
    let initial_names_count = get_node_names(&initial_nodes).len();

    let config = RustreeLibConfig {
        ignore_patterns: Some(vec!["this_pattern_matches_nothing_123".to_string()]),
        ..Default::default()
    };
    let nodes = get_tree_nodes(temp_dir.path(), &config)?;
    let names = get_node_names(&nodes);
    assert_eq!(names.len(), initial_names_count, "Pattern matching nothing should not ignore any files");
    Ok(())
}

#[test]
fn test_ignore_applies_to_hidden_files_when_a_is_used() -> Result<()> {
    let temp_dir = common_test_utils::setup_gitignore_test_dir()?; // Has .secret_file, .hidden_dir
    let config = RustreeLibConfig {
        ignore_patterns: Some(vec![".secret*".to_string()]),
        show_hidden: true, // -a
        ..Default::default()
    };
    let nodes = get_tree_nodes(temp_dir.path(), &config)?;
    let names = get_node_names(&nodes);
    assert!(!names.contains(".secret_file"), ".secret_file should NOT be present (ignored by -I .secret*)");
    assert!(names.contains(".hidden_dir"), ".hidden_dir IS present (not matched by .secret* and -a is on)");
    Ok(())
}

#[test]
fn test_gitignore_ignores_hidden_files_even_if_a_is_used() -> Result<()> {
    let temp_dir = common_test_utils::setup_gitignore_test_dir()?;
    let mut root_gitignore = fs::OpenOptions::new().append(true).open(temp_dir.path().join(".gitignore"))?;
    writeln!(root_gitignore, ".secret_file")?;
    writeln!(root_gitignore, ".hidden_dir/")?; // Note: .gitignore usually matches .hidden_dir for dir patterns
    drop(root_gitignore);

    let config = RustreeLibConfig {
        use_gitignore: true,
        show_hidden: true, // -a
        ..Default::default()
    };
    let nodes = get_tree_nodes(temp_dir.path(), &config)?;
    let names = get_node_names(&nodes);
    assert!(!names.contains(".secret_file"), ".secret_file should NOT be present (gitignored)");
    assert!(!names.contains(".hidden_dir"), ".hidden_dir should NOT be present (gitignored)");
    assert!(!names.contains("content.txt"), "content.txt in .hidden_dir should NOT be present (gitignored)");
    Ok(())
}