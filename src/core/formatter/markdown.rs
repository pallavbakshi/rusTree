// src/core/formatter/markdown.rs
use super::base::TreeFormatter;
use crate::config::RustreeLibConfig;
use crate::core::error::RustreeError;
use crate::core::metadata::MetadataAggregator;
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

        // Determine the effective root path from the nodes themselves
        let scan_root_path_opt = nodes
            .iter()
            .find(|n| n.depth == 1)
            .and_then(|n| n.path.parent().map(|p| p.to_path_buf()));

        // Convert nodes to markdown list
        for node in nodes {
            // Create indentation based on depth (depth 1 = no extra indent, depth 2 = 2 spaces, etc.)
            let indent = "  ".repeat(node.depth.saturating_sub(1));

            // Get the display name (full path or just name)
            let display_name = if config.listing.show_full_path {
                // For full path, we need to make it relative to the current directory
                if let Some(scan_root) = &scan_root_path_opt {
                    // Make path relative to scan root
                    node.path
                        .strip_prefix(scan_root)
                        .unwrap_or(&node.path)
                        .to_string_lossy()
                        .to_string()
                } else {
                    // Fallback to just the name if no scan root
                    node.name.clone()
                }
            } else {
                node.name.clone()
            };

            // Format the node name with directory indicator
            let name_with_suffix = if node.node_type == NodeType::Directory {
                format!("{}/", display_name)
            } else {
                display_name
            };

            // Add metadata if configured using centralized formatting
            let metadata_str = format_node_metadata(node, config, MetadataStyle::Markdown);

            // Write the markdown list item
            writeln!(output, "{}* {}{}", indent, name_with_suffix, metadata_str)?;
        }

        // Add summary
        if !config.misc.no_summary_report {
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
                // Include root directory in count if it's a directory
                let root_dir_increment = if config.input_source.root_is_directory {
                    1
                } else {
                    0
                };
                (dc + root_dir_increment, fc)
            };

            writeln!(output)?;
            write!(
                output,
                "__{} director{}, {} file{}",
                dir_count,
                if dir_count == 1 { "y" } else { "ies" },
                file_count,
                if file_count == 1 { "" } else { "s" }
            )?;

            // Aggregate metadata and add to summary
            let aggregator = MetadataAggregator::aggregate_from_nodes(nodes, config);
            let summary_additions = aggregator.format_summary_additions();
            if !summary_additions.is_empty() {
                write!(output, "{}", summary_additions)?;
            }

            write!(output, " total__")?;
        }

        Ok(output)
    }
}
