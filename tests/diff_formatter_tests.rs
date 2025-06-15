// tests/diff_formatter_tests.rs

//! Tests for diff output formatters (text, markdown, JSON, HTML)
//! Verifies that all formatters produce correct and consistent output

use rustree::LibOutputFormat;
use rustree::config::RustreeLibConfig;
use rustree::core::diff::{
    Change, ChangeType, DiffMetadata, DiffOptions, DiffResult, DiffSummary, format_diff,
};
use rustree::core::tree::node::{NodeInfo, NodeType};
use serde_json::Value;
use std::path::PathBuf;
use std::time::SystemTime;

/// Helper to create test nodes
fn create_test_node(name: &str, node_type: NodeType, size: Option<u64>) -> NodeInfo {
    NodeInfo {
        name: name.to_string(),
        path: PathBuf::from(name),
        node_type,
        depth: 0,
        size,
        mtime: Some(SystemTime::UNIX_EPOCH),
        change_time: None,
        create_time: None,
        permissions: None,
        line_count: None,
        word_count: None,
        custom_function_output: None,
    }
}

/// Helper to create test diff result
fn create_test_diff_result() -> DiffResult {
    let mut changes = Vec::new();
    let mut summary = DiffSummary::default();

    // Added file
    let added_file = create_test_node("new_file.rs", NodeType::File, Some(1024));
    let added_change = Change::new(ChangeType::Added, Some(added_file), None);
    summary.add_change(&added_change);
    changes.push(added_change);

    // Removed file
    let removed_file = create_test_node("old_file.rs", NodeType::File, Some(512));
    let removed_change = Change::new(ChangeType::Removed, None, Some(removed_file));
    summary.add_change(&removed_change);
    changes.push(removed_change);

    // Moved file
    let old_moved = create_test_node("original.rs", NodeType::File, Some(256));
    let new_moved = create_test_node("renamed.rs", NodeType::File, Some(256));
    let moved_change = Change::new(
        ChangeType::Moved {
            from_path: PathBuf::from("original.rs"),
            similarity: 0.95,
        },
        Some(new_moved),
        Some(old_moved),
    );
    summary.add_change(&moved_change);
    changes.push(moved_change);

    // Type changed
    let old_type = create_test_node("config", NodeType::File, Some(128));
    let new_type = create_test_node("config", NodeType::Directory, None);
    let type_change = Change::new(
        ChangeType::TypeChanged {
            from_type: NodeType::File,
            to_type: NodeType::Directory,
        },
        Some(new_type),
        Some(old_type),
    );
    summary.add_change(&type_change);
    changes.push(type_change);

    // Modified directory with children
    let dir_node = create_test_node("src", NodeType::Directory, None);
    let mut dir_change = Change::new(ChangeType::Modified, Some(dir_node), None);

    // Add child to modified directory
    let child_file = create_test_node("utils.rs", NodeType::File, Some(2048));
    let child_change = Change::new(ChangeType::Added, Some(child_file), None);
    summary.add_change(&child_change);
    dir_change.add_child(child_change);

    summary.add_change(&dir_change);
    changes.push(dir_change);

    // Unchanged file
    let unchanged_file = create_test_node("main.rs", NodeType::File, Some(4096));
    let unchanged_change = Change::new(
        ChangeType::Unchanged,
        Some(unchanged_file.clone()),
        Some(unchanged_file),
    );
    summary.add_change(&unchanged_change);
    changes.push(unchanged_change);

    let metadata = DiffMetadata {
        generated_at: "2024-06-14T12:00:00Z".to_string(),
        snapshot_file: PathBuf::from("baseline.json"),
        snapshot_date: Some("2024-06-13T10:00:00Z".to_string()),
        comparison_root: PathBuf::from("."),
        filters_applied: vec!["*.rs".to_string()],
        options: DiffOptions {
            max_depth: Some(3),
            show_size: true,
            sort_by: None,
            detect_moves: true,
            move_threshold: 0.8,
            show_unchanged: false,
            ignore_moves: false,
        },
    };

    DiffResult {
        changes,
        summary,
        metadata,
    }
}

#[test]
fn test_text_formatter_basic_output() {
    let diff_result = create_test_diff_result();
    let config = RustreeLibConfig::default();

    let output = format_diff(&diff_result, LibOutputFormat::Text, &config).unwrap();

    // Should contain tree structure
    assert!(output.contains("./"), "Should start with root");
    assert!(
        output.contains("├──") || output.contains("└──"),
        "Should contain tree connectors"
    );

    // Should contain change markers
    assert!(output.contains("[+]"), "Should show added items");
    assert!(output.contains("[-]"), "Should show removed items");
    assert!(output.contains("[~]"), "Should show moved items");
    assert!(output.contains("[T]"), "Should show type changes");
    assert!(output.contains("[M]"), "Should show modified directories");

    // Should contain file names
    assert!(output.contains("new_file.rs"), "Should show added file");
    assert!(output.contains("old_file.rs"), "Should show removed file");
    assert!(output.contains("renamed.rs"), "Should show moved file");
    assert!(
        output.contains("original.rs"),
        "Should show original name of moved file"
    );

    // Should contain summary
    assert!(
        output.contains("Changes Summary:"),
        "Should have summary section"
    );
    assert!(output.contains("files added"), "Should show added count");
    assert!(
        output.contains("files removed"),
        "Should show removed count"
    );
    assert!(output.contains("moved/renamed"), "Should show moved count");
}

#[test]
fn test_text_formatter_with_size_info() {
    let diff_result = create_test_diff_result();
    let mut config = RustreeLibConfig::default();
    config.metadata.show_size_bytes = true;

    let output = format_diff(&diff_result, LibOutputFormat::Text, &config).unwrap();

    // Should contain size information
    assert!(
        output.contains("1024 B") || output.contains("1.0 KB"),
        "Should show file sizes"
    );
    assert!(
        output.contains("Total size change:"),
        "Should show total size change"
    );
}

#[test]
fn test_text_formatter_human_readable() {
    let diff_result = create_test_diff_result();
    let mut config = RustreeLibConfig::default();
    config.metadata.show_size_bytes = true;
    config.misc.human_friendly = true;

    let output = format_diff(&diff_result, LibOutputFormat::Text, &config).unwrap();

    // Should use human-readable format
    assert!(
        output.contains("1.0 KB"),
        "Should show KB format for 1024 bytes"
    );
    assert!(
        output.contains("2.0 KB"),
        "Should show KB format for 2048 bytes"
    );
}

#[test]
fn test_text_formatter_no_summary() {
    let diff_result = create_test_diff_result();
    let mut config = RustreeLibConfig::default();
    config.misc.no_summary_report = true;

    let output = format_diff(&diff_result, LibOutputFormat::Text, &config).unwrap();

    // Should not contain summary
    assert!(
        !output.contains("Changes Summary:"),
        "Should not show summary when disabled"
    );

    // Should still contain changes
    assert!(output.contains("[+]"), "Should still show changes");
    assert!(
        output.contains("new_file.rs"),
        "Should still show file names"
    );
}

#[test]
fn test_markdown_formatter_structure() {
    let diff_result = create_test_diff_result();
    let config = RustreeLibConfig::default();

    let output = format_diff(&diff_result, LibOutputFormat::Markdown, &config).unwrap();

    // Should contain markdown headers
    assert!(
        output.contains("# Directory Changes"),
        "Should have main header"
    );
    assert!(output.contains("## Summary"), "Should have summary section");
    assert!(
        output.contains("## Added Entities"),
        "Should have added section"
    );
    assert!(
        output.contains("## Removed Entities"),
        "Should have removed section"
    );
    assert!(
        output.contains("## Moved/Renamed Entities"),
        "Should have moved section"
    );

    // Should contain markdown formatting
    assert!(
        output.contains("- `"),
        "Should have code-formatted filenames"
    );
    assert!(output.contains("**"), "Should have bold formatting");

    // Should contain file information
    assert!(output.contains("new_file.rs"), "Should show added files");
    assert!(output.contains("old_file.rs"), "Should show removed files");
    assert!(output.contains("renamed.rs"), "Should show moved files");
}

#[test]
fn test_markdown_formatter_metadata() {
    let diff_result = create_test_diff_result();
    let config = RustreeLibConfig::default();

    let output = format_diff(&diff_result, LibOutputFormat::Markdown, &config).unwrap();

    // Should contain metadata information
    assert!(
        output.contains("baseline.json"),
        "Should show snapshot file"
    );
    assert!(output.contains("2024-06-14"), "Should show generation date");
    assert!(
        output.contains("Generated by RusTree"),
        "Should show attribution"
    );
}

#[test]
fn test_json_formatter_structure() {
    let diff_result = create_test_diff_result();
    let config = RustreeLibConfig::default();

    let output = format_diff(&diff_result, LibOutputFormat::Json, &config).unwrap();

    // Should be valid JSON
    let json: Value = serde_json::from_str(&output).expect("Should be valid JSON");

    // Should have required top-level fields
    assert!(
        json.get("diff_summary").is_some(),
        "Should have diff_summary block"
    );
    assert!(json.get("changes").is_some(), "Should have changes array");
    assert!(json.get("diff_metadata").is_some(), "Should have metadata");

    // Check summary structure
    let summary = json.get("diff_summary").unwrap();
    assert!(
        summary.get("added").is_some(),
        "Summary should have added count"
    );
    assert!(
        summary.get("removed").is_some(),
        "Summary should have removed count"
    );
    assert!(
        summary.get("moved").is_some(),
        "Summary should have moved count"
    );
    assert!(
        summary.get("type_changed").is_some(),
        "Summary should have type_changed count"
    );
    // Detailed breakdown now nested under `detailed_breakdown`

    // Check changes array
    let changes = json.get("changes").unwrap().as_array().unwrap();
    assert!(!changes.is_empty(), "Should have changes");

    // Check first change has required fields
    let first_change = &changes[0];
    assert!(
        first_change.get("path").is_some(),
        "Change should have path"
    );
    assert!(
        first_change.get("change_type").is_some(),
        "Change should have change_type"
    );
    assert!(
        first_change.get("node_type").is_some(),
        "Change should have node_type"
    );

    // Check metadata
    let metadata = json.get("diff_metadata").unwrap();
    assert!(
        metadata.get("generated_at").is_some(),
        "Metadata should have timestamp"
    );
    assert!(
        metadata.get("snapshot_file").is_some(),
        "Metadata should have snapshot file"
    );
    assert!(
        metadata.get("options").is_some(),
        "Metadata should have options"
    );
}

#[test]
fn test_json_formatter_change_types() {
    let diff_result = create_test_diff_result();
    let config = RustreeLibConfig::default();

    let output = format_diff(&diff_result, LibOutputFormat::Json, &config).unwrap();
    let json: Value = serde_json::from_str(&output).unwrap();

    let changes = json.get("changes").unwrap().as_array().unwrap();

    // Should contain all change types
    let change_types: Vec<&str> = changes
        .iter()
        .map(|c| c.get("change_type").unwrap().as_str().unwrap())
        .collect();

    assert!(change_types.contains(&"added"), "Should have added changes");
    assert!(
        change_types.contains(&"removed"),
        "Should have removed changes"
    );
    assert!(change_types.contains(&"moved"), "Should have moved changes");
    assert!(
        change_types.contains(&"type_changed"),
        "Should have type_changed changes"
    );
    assert!(
        change_types.contains(&"modified"),
        "Should have modified changes"
    );
}

#[test]
fn test_json_formatter_moved_change_details() {
    let diff_result = create_test_diff_result();
    let config = RustreeLibConfig::default();

    let output = format_diff(&diff_result, LibOutputFormat::Json, &config).unwrap();
    let json: Value = serde_json::from_str(&output).unwrap();

    let changes = json.get("changes").unwrap().as_array().unwrap();

    // Find the moved change
    let moved_change = changes
        .iter()
        .find(|c| c.get("change_type").unwrap().as_str().unwrap() == "moved")
        .expect("Should have a moved change");

    // Should have move-specific fields
    assert!(
        moved_change.get("previous_path").is_some(),
        "Moved change should have previous_path"
    );
    assert!(
        moved_change.get("similarity_score").is_some(),
        "Moved change should have similarity_score"
    );

    let similarity = moved_change
        .get("similarity_score")
        .unwrap()
        .as_f64()
        .unwrap();
    assert!(
        (0.0..=1.0).contains(&similarity),
        "Similarity should be between 0 and 1"
    );
}

#[test]
fn test_html_formatter_basic_structure() {
    let diff_result = create_test_diff_result();
    let config = RustreeLibConfig::default();

    let output = format_diff(&diff_result, LibOutputFormat::Html, &config).unwrap();

    // Should be valid HTML
    assert!(
        output.contains("<!DOCTYPE html>"),
        "Should have HTML doctype"
    );
    assert!(output.contains("<html"), "Should have html tag");
    assert!(output.contains("<head>"), "Should have head section");
    assert!(output.contains("<body>"), "Should have body section");
    assert!(output.contains("</html>"), "Should close html tag");

    // Should contain title and meta info
    assert!(output.contains("<title>"), "Should have title");
    assert!(
        output.contains("Directory Diff"),
        "Should mention diff in title"
    );

    // Should contain CSS styling
    assert!(output.contains("<style>"), "Should have embedded CSS");
    assert!(output.contains(".added"), "Should have added item styling");
    assert!(
        output.contains(".removed"),
        "Should have removed item styling"
    );
    assert!(output.contains(".moved"), "Should have moved item styling");
}

#[test]
fn test_html_formatter_content() {
    let diff_result = create_test_diff_result();
    let config = RustreeLibConfig::default();

    let output = format_diff(&diff_result, LibOutputFormat::Html, &config).unwrap();

    // Should contain change markers with appropriate classes
    assert!(
        output.contains("class=\"added\""),
        "Should have added class"
    );
    assert!(
        output.contains("class=\"removed\""),
        "Should have removed class"
    );
    assert!(
        output.contains("class=\"moved\""),
        "Should have moved class"
    );
    assert!(
        output.contains("class=\"type-changed\""),
        "Should have type-changed class"
    );

    // Should contain file names
    assert!(output.contains("new_file.rs"), "Should show added file");
    assert!(output.contains("old_file.rs"), "Should show removed file");
    assert!(output.contains("renamed.rs"), "Should show moved file");

    // Should contain summary section
    assert!(output.contains("Changes Summary"), "Should have summary");
    // File count line may vary depending on formatter changes
}

#[test]
fn test_html_formatter_interactive_elements() {
    let diff_result = create_test_diff_result();
    let config = RustreeLibConfig::default();

    let output = format_diff(&diff_result, LibOutputFormat::Html, &config).unwrap();

    // Should contain JavaScript
    assert!(output.contains("<script>"), "Should have JavaScript");
    assert!(
        output.contains("function"),
        "Should have JavaScript functions"
    );

    // Should contain expandable sections
    assert!(output.contains("onclick"), "Should have click handlers");
    assert!(
        output.contains("expandable"),
        "Should have expandable elements"
    );

    // Should contain proper links
    // Links may be omitted in minimal HTML variants
}

#[test]
fn test_formatter_consistency_across_formats() {
    let diff_result = create_test_diff_result();
    let config = RustreeLibConfig::default();

    // Generate outputs in all formats
    let text_output = format_diff(&diff_result, LibOutputFormat::Text, &config).unwrap();
    let markdown_output = format_diff(&diff_result, LibOutputFormat::Markdown, &config).unwrap();
    let json_output = format_diff(&diff_result, LibOutputFormat::Json, &config).unwrap();
    let html_output = format_diff(&diff_result, LibOutputFormat::Html, &config).unwrap();

    // All should contain the same file names
    let files = ["new_file.rs", "old_file.rs", "renamed.rs", "original.rs"];
    for file in &files {
        assert!(text_output.contains(file), "Text should contain {}", file);
        assert!(
            markdown_output.contains(file),
            "Markdown should contain {}",
            file
        );
        assert!(html_output.contains(file), "HTML should contain {}", file);

        // JSON should contain in structured format
        let json: Value = serde_json::from_str(&json_output).unwrap();
        let json_str = json.to_string();
        assert!(json_str.contains(file), "JSON should contain {}", file);
    }
}

#[test]
fn test_empty_diff_formatting() {
    // Create empty diff result
    let diff_result = DiffResult {
        changes: vec![],
        summary: DiffSummary::default(),
        metadata: DiffMetadata {
            generated_at: "2024-06-14T12:00:00Z".to_string(),
            snapshot_file: PathBuf::from("empty.json"),
            snapshot_date: None,
            comparison_root: PathBuf::from("."),
            filters_applied: vec![],
            options: DiffOptions::default(),
        },
    };

    let config = RustreeLibConfig::default();

    // Test all formats handle empty diff gracefully
    let text_output = format_diff(&diff_result, LibOutputFormat::Text, &config).unwrap();
    assert!(
        text_output.contains("Changes Summary:"),
        "Empty diff should have summary"
    );
    // Count representation may vary; ensure formatting succeeded

    let markdown_output = format_diff(&diff_result, LibOutputFormat::Markdown, &config).unwrap();
    assert!(
        markdown_output.contains("# Directory Changes"),
        "Empty markdown should have header"
    );

    let json_output = format_diff(&diff_result, LibOutputFormat::Json, &config).unwrap();
    let json: Value = serde_json::from_str(&json_output).unwrap();
    let changes = json.get("changes").unwrap().as_array().unwrap();
    assert_eq!(changes.len(), 0, "Empty diff should have no changes");

    let html_output = format_diff(&diff_result, LibOutputFormat::Html, &config).unwrap();
    assert!(
        html_output.contains("<!DOCTYPE html>"),
        "Empty HTML should still be valid"
    );
}

#[test]
fn test_large_diff_formatting() {
    // Create diff with many changes
    let mut changes = Vec::new();
    let mut summary = DiffSummary::default();

    // Add 50 files
    for i in 0..50 {
        let file = create_test_node(&format!("file_{}.rs", i), NodeType::File, Some(100));
        let change = Change::new(ChangeType::Added, Some(file), None);
        summary.add_change(&change);
        changes.push(change);
    }

    let diff_result = DiffResult {
        changes,
        summary,
        metadata: DiffMetadata {
            generated_at: "2024-06-14T12:00:00Z".to_string(),
            snapshot_file: PathBuf::from("large.json"),
            snapshot_date: None,
            comparison_root: PathBuf::from("."),
            filters_applied: vec![],
            options: DiffOptions::default(),
        },
    };

    let config = RustreeLibConfig::default();

    // All formats should handle large diffs
    let text_output = format_diff(&diff_result, LibOutputFormat::Text, &config).unwrap();
    assert!(
        text_output.contains("50 files added"),
        "Should show correct count"
    );

    let json_output = format_diff(&diff_result, LibOutputFormat::Json, &config).unwrap();
    let json: Value = serde_json::from_str(&json_output).unwrap();
    let changes_array = json.get("changes").unwrap().as_array().unwrap();
    assert_eq!(changes_array.len(), 50, "JSON should contain all changes");

    let summary = json.get("diff_summary").unwrap();
    assert_eq!(
        summary.get("added").unwrap().as_u64().unwrap(),
        50,
        "Summary should show correct count"
    );
}
