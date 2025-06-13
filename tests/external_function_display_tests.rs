//! Tests covering the UI behaviour of external command apply-functions.

use rustree::config::metadata::{ExternalFunction, FunctionOutputKind};
use rustree::config::{ListingOptions, MetadataOptions, RustreeLibConfig};
use rustree::core::tree::node::NodeType;
use rustree::{LibOutputFormat, format_nodes, get_tree_nodes};
use std::fs::{self, File};
use std::io::Write;

/// Helper to build a minimal config that only sets the external function.
fn make_config(ext_fn: ExternalFunction) -> RustreeLibConfig {
    RustreeLibConfig {
        metadata: MetadataOptions {
            external_function: Some(ext_fn),
            ..Default::default()
        },
        listing: ListingOptions {
            // ensure deterministic order for tests
            max_depth: None,
            ..Default::default()
        },
        ..Default::default()
    }
}

#[test]
fn test_external_number_aggregation_and_display() {
    let tmp = tempfile::TempDir::new().expect("tmpdir");
    let dir = tmp.path();

    // Create two files: 3 lines + 4 lines => expected total 7.
    let mut f1 = File::create(dir.join("a.txt")).unwrap();
    writeln!(f1, "one\ntwo\nthree").unwrap();

    let mut f2 = File::create(dir.join("b.txt")).unwrap();
    writeln!(f2, "1\n2\n3\n4").unwrap();

    let ext_fn = ExternalFunction {
        cmd_template: "wc -l < {}".to_string(),
        timeout_secs: 5,
        kind: FunctionOutputKind::Number,
    };

    let cfg = make_config(ext_fn);

    let nodes = get_tree_nodes(dir, &cfg).expect("nodes");
    // Ensure we collected output for each file
    for n in &nodes {
        if n.node_type == NodeType::File {
            assert!(matches!(n.custom_function_output, Some(Ok(_))));
        }
    }

    let out = format_nodes(&nodes, LibOutputFormat::Text, &cfg).expect("format");

    assert!(out.contains("[F: \"3\"]"));
    assert!(out.contains("[F: \"4\"]"));
    assert!(out.contains("7 total (custom)"));
}

#[test]
fn test_external_text_cat_style_header_and_content() {
    let tmp = tempfile::TempDir::new().expect("tmpdir");
    let dir = tmp.path();

    let file_path = dir.join("hello.txt");
    fs::write(&file_path, "Hello external world!\n").unwrap();

    let ext_cmd = "cat {}".to_string();
    let ext_fn = ExternalFunction {
        cmd_template: ext_cmd.clone(),
        timeout_secs: 5,
        kind: FunctionOutputKind::Text,
    };

    let cfg = make_config(ext_fn);
    let nodes = get_tree_nodes(dir, &cfg).expect("nodes");
    let out = format_nodes(&nodes, LibOutputFormat::Text, &cfg).expect("format");

    // No inline F markers for text mode
    assert!(!out.contains("[F:"));

    // The custom header should be present
    let expected_header = format!(
        "--- Results of applying '{}' to relevant files ---",
        ext_cmd
    );
    assert!(out.contains(&expected_header));

    // And the content of the file should follow under === <path> ===
    assert!(out.contains("Hello external world!"));
}
