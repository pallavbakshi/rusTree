// src/core/formatter/markdown.rs
use super::base::TreeFormatter;
use crate::config::RustreeLibConfig;
use crate::core::error::RustreeError;
use crate::core::metadata::file_info::{MetadataStyle, format_node_metadata};
use crate::core::tree::node::{NodeInfo, NodeType};
use std::fmt::Write;

/// A formatter that generates a Markdown list representation of the directory structure.
///
/// The output is a nested Markdown list using `*` for unordered lists, with proper
/// indentation to represent the tree hierarchy. Files and directories are
/// distinguished by trailing `/` for directories.
pub struct MarkdownFormatter;

impl TreeFormatter for MarkdownFormatter {
    fn format(
        &self,
        nodes: &[NodeInfo],
        config: &RustreeLibConfig,
    ) -> Result<String, RustreeError> {
        let mut output = String::new();

        // Add the root header
        writeln!(output, "# {}", config.input_source.root_display_name)?;
        writeln!(output)?;

        // Convert nodes to markdown list
        for node in nodes {
            // Create indentation based on depth (depth 1 = no extra indent, depth 2 = 2 spaces, etc.)
            let indent = "  ".repeat(node.depth.saturating_sub(1));

            // Format the node name with directory indicator
            let name_with_suffix = if node.node_type == NodeType::Directory {
                format!("{}/", node.name)
            } else {
                node.name.clone()
            };

            // Add metadata if configured using centralized formatting
            let metadata_str = format_node_metadata(node, config, MetadataStyle::Markdown);

            // Write the markdown list item
            writeln!(output, "{}* {}{}", indent, name_with_suffix, metadata_str)?;
        }

        // Add summary
        let (dir_count, file_count) = if config.listing.list_directories_only {
            let child_dir_count = nodes.len();
            let root_dir_increment = if config.input_source.root_is_directory {
                1
            } else {
                0
            };
            (child_dir_count + root_dir_increment, 0)
        } else {
            let mut dc = 0;
            let mut fc = 0;
            for node in nodes {
                match node.node_type {
                    NodeType::Directory => dc += 1,
                    NodeType::File => fc += 1,
                    NodeType::Symlink => { /* Not counted in summary */ }
                }
            }
            (dc, fc)
        };

        writeln!(output)?;
        write!(
            output,
            "__{} director{}, {} file{} total__",
            dir_count,
            if dir_count == 1 { "y" } else { "ies" },
            file_count,
            if file_count == 1 { "" } else { "s" }
        )?;

        Ok(output)
    }
}
