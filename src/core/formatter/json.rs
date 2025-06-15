// src/core/formatter/json.rs

//! JSON output formatter (hierarchical).
//!
//! Mimics the structure produced by `tree -J`: a nested hierarchy with
//! `type`, `name`, and, for directories, a `contents` array.  At the end a
//! synthetic `{ "type": "report", ... }` object is appended containing the
//! total directory / file counts so downstream tools can replicate `tree`'s
//! summary line.

use crate::core::error::RustreeError;
use crate::core::formatter::base::{TreeFormatter, TreeFormatterCompat};
use crate::core::options::contexts::FormattingContext;
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
        formatting_ctx: &FormattingContext,
    ) -> Result<String, RustreeError> {
        // Build temporary tree to restore hierarchy
        let mut roots = builder::build_tree(nodes.to_vec())
            .map_err(|e| RustreeError::TreeBuildError(format!("tree build failed: {}", e)))?;

        let mut dirs = 0usize;
        let mut files = 0usize;
        let mut json_roots = Vec::new();

        // Determine apply command string once.
        let apply_cmd_opt: Option<String> =
            formatting_ctx
                .metadata
                .apply_function
                .as_ref()
                .map(|apply_fn| match apply_fn {
                    crate::core::options::ApplyFunction::BuiltIn(builtin) => format!("{builtin:?}"),
                    crate::core::options::ApplyFunction::External(ext) => ext.cmd_template.clone(),
                });

        for root in &mut roots {
            json_roots.push(convert_node(root, &apply_cmd_opt, &mut dirs, &mut files));
        }

        // Wrap under synthetic root directory ("." by default)
        dirs += 1; // count the synthetic root as directory, like GNU tree does
        let root_name = ".".to_string();
        let wrapped_root = JsonValue::Directory {
            name: root_name,
            contents: Some(json_roots),
            apply_command: apply_cmd_opt.clone(),
            apply_command_output: None,
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
        #[serde(skip_serializing_if = "Option::is_none")]
        apply_command: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        apply_command_output: Option<String>,
    },
    #[serde(rename = "file")]
    File {
        name: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        apply_command: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        apply_command_output: Option<String>,
    },
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
    apply_cmd: &Option<String>,
    dir_ctr: &mut usize,
    file_ctr: &mut usize,
) -> JsonValue {
    match node.node_info.node_type {
        NodeType::Directory => {
            *dir_ctr += 1;
            let mut child_vals = Vec::new();
            for child in &mut node.children {
                child_vals.push(convert_node(child, apply_cmd, dir_ctr, file_ctr));
            }
            JsonValue::Directory {
                name: node.node_info.name.clone(),
                contents: if child_vals.is_empty() {
                    None
                } else {
                    Some(child_vals)
                },
                apply_command: apply_cmd.clone(),
                apply_command_output: node
                    .node_info
                    .custom_function_output
                    .as_ref()
                    .and_then(|r| r.as_ref().ok())
                    .cloned(),
            }
        }
        _ => {
            *file_ctr += 1;
            JsonValue::File {
                name: node.node_info.name.clone(),
                apply_command: apply_cmd.clone(),
                apply_command_output: node
                    .node_info
                    .custom_function_output
                    .as_ref()
                    .and_then(|r| r.as_ref().ok())
                    .cloned(),
            }
        }
    }
}

/// Implement backward compatibility trait
impl TreeFormatterCompat for JsonFormatter {}

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
            .format_compat(&nodes, &crate::core::options::RustreeLibConfig::default())
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
