// src/core/formatter/json.rs

//! JSON output formatter (hierarchical).
//!
//! Mimics the structure produced by `tree -J`: a nested hierarchy with
//! `type`, `name`, and, for directories, a `contents` array.  At the end a
//! synthetic `{ "type": "report", ... }` object is appended containing the
//! total directory / file counts so downstream tools can replicate `tree`'s
//! summary line.

use crate::config::RustreeLibConfig;
use crate::core::error::RustreeError;
use crate::core::formatter::base::TreeFormatter;
use crate::core::tree::{
    builder,
    node::{NodeInfo, NodeType},
};

use serde::Serialize;

pub struct JsonFormatter;

impl TreeFormatter for JsonFormatter {
    fn format(
        &self,
        nodes: &[NodeInfo],
        _config: &RustreeLibConfig,
    ) -> Result<String, RustreeError> {
        // Build temporary tree to restore hierarchy
        let mut roots = builder::build_tree(nodes.to_vec())
            .map_err(|e| RustreeError::TreeBuildError(format!("tree build failed: {}", e)))?;

        let mut dirs = 0usize;
        let mut files = 0usize;
        let mut json_roots = Vec::new();

        for root in &mut roots {
            json_roots.push(convert_node(root, &mut dirs, &mut files));
        }

        // Wrap under synthetic root directory ("." by default)
        dirs += 1; // count the synthetic root as directory, like GNU tree does
        let root_name = ".".to_string();
        let wrapped_root = JsonValue::Directory {
            name: root_name,
            contents: Some(json_roots),
        };

        let output_vec = vec![
            wrapped_root,
            JsonValue::Report(JsonReport {
                directories: dirs,
                files,
            }),
        ];

        serde_json::to_string_pretty(&output_vec)
            .map_err(|e| RustreeError::TreeBuildError(format!("JSON serialization failed: {}", e)))
    }
}

/// Internal serialisable representation.
#[derive(Serialize)]
#[serde(tag = "type")]
enum JsonValue {
    #[serde(rename = "directory")]
    Directory {
        name: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        contents: Option<Vec<JsonValue>>,
    },
    #[serde(rename = "file")]
    File { name: String },
    #[serde(rename = "report")]
    Report(JsonReport),
}

#[derive(Serialize)]
struct JsonReport {
    directories: usize,
    files: usize,
}

fn convert_node(
    node: &mut builder::TempNode,
    dir_ctr: &mut usize,
    file_ctr: &mut usize,
) -> JsonValue {
    match node.node_info.node_type {
        NodeType::Directory => {
            *dir_ctr += 1;
            let mut child_vals = Vec::new();
            for child in &mut node.children {
                child_vals.push(convert_node(child, dir_ctr, file_ctr));
            }
            JsonValue::Directory {
                name: node.node_info.name.clone(),
                contents: if child_vals.is_empty() {
                    None
                } else {
                    Some(child_vals)
                },
            }
        }
        _ => {
            *file_ctr += 1;
            JsonValue::File {
                name: node.node_info.name.clone(),
            }
        }
    }
}

// --------------------------------------------------
// Tests
// --------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::tree::node::NodeInfo;
    use std::path::PathBuf;

    #[test]
    fn hierarchical_json_matches_expected_shape() {
        let nodes = vec![
            NodeInfo {
                path: PathBuf::from("root"),
                name: "root".into(),
                node_type: NodeType::Directory,
                depth: 0,
                size: None,
                permissions: None,
                mtime: None,
                change_time: None,
                create_time: None,
                line_count: None,
                word_count: None,
                custom_function_output: None,
            },
            NodeInfo {
                path: PathBuf::from("root/file.txt"),
                name: "file.txt".into(),
                node_type: NodeType::File,
                depth: 1,
                size: None,
                permissions: None,
                mtime: None,
                change_time: None,
                create_time: None,
                line_count: None,
                word_count: None,
                custom_function_output: None,
            },
        ];

        let json_str = JsonFormatter
            .format(&nodes, &RustreeLibConfig::default())
            .unwrap();

        let v: serde_json::Value = serde_json::from_str(&json_str).unwrap();
        assert!(v.is_array());
        assert_eq!(v.as_array().unwrap().len(), 2); // synthetic root + report

        // Root object is directory named "."
        assert_eq!(v[0]["type"], "directory");
        assert_eq!(v[0]["name"], ".");
        // first child dir 'root'
        assert_eq!(v[0]["contents"][0]["name"], "root");

        // Report object
        assert_eq!(v[1]["type"], "report");
        assert_eq!(v[1]["directories"], 2); // synthetic root + actual dir
        assert_eq!(v[1]["files"], 1);
    }
}
