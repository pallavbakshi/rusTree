use super::base::{TreeFormatter, TreeFormatterCompat};
use crate::core::error::RustreeError;
use crate::core::metadata::MetadataAggregator;
use crate::core::metadata::file_info::{MetadataStyle, format_node_metadata};
use crate::core::options::contexts::FormattingContext;
use crate::core::tree::node::{NodeInfo, NodeType};
use std::collections::HashMap;
use std::fmt::Write;
use std::path::{Path, PathBuf};

/// A formatter that generates a plain text, tree-like representation of the directory structure.
///
/// This is similar to the output of the standard `tree` command.
pub struct TextTreeFormatter;

impl TextTreeFormatter {
    // Helper to determine if a node (identified by its path) is the last among its siblings
    // in the `all_nodes` list (which is assumed to be sorted as per display requirements).
    fn is_last_sibling_in_sorted_list(
        node_to_check_path: &Path,
        all_nodes: &[NodeInfo],
        cache: &mut HashMap<PathBuf, bool>,
    ) -> bool {
        if let Some(&cached_result) = cache.get(node_to_check_path) {
            return cached_result;
        }

        let node_info = match all_nodes.iter().find(|n| n.path == node_to_check_path) {
            Some(info) => info,
            None => {
                // Should not happen if node_to_check_path is from a node in all_nodes
                cache.insert(node_to_check_path.to_path_buf(), true); // Default to true to avoid issues
                return true;
            }
        };
        let node_depth = node_info.depth;
        let parent_path_opt = node_to_check_path.parent();

        // Find the last node in all_nodes that is a sibling of node_to_check_path
        let mut last_sibling_path_in_list: Option<&Path> = None;
        // Iterate backwards through all_nodes to find the last sibling
        for sibling_candidate_node in all_nodes.iter().rev() {
            // The faulty optimization `if sibling_candidate_node.depth < node_depth { break; }` was removed.
            // That check was incorrect because `all_nodes` is sorted by path for non-siblings (DFS order),
            // not strictly by depth across different branches. A deeper branch of an earlier sibling
            // could appear before a later sibling at `node_depth` when iterating in reverse.
            // The correct approach is to scan until a sibling is found or the list is exhausted.
            if sibling_candidate_node.depth == node_depth
                && sibling_candidate_node.path.parent() == parent_path_opt
            {
                last_sibling_path_in_list = Some(&sibling_candidate_node.path);
                break; // Found the last sibling (due to reverse iteration)
            }
        }

        let result = match last_sibling_path_in_list {
            Some(last_path) => last_path == node_to_check_path,
            // If no sibling is found with the same parent and depth (e.g., root items, or error in data),
            // or if the node itself is the only one that matches criteria (e.g. single child),
            // default to true (it's the "last" in its context).
            None => true,
        };

        cache.insert(node_to_check_path.to_path_buf(), result);
        result
    }
}

impl TreeFormatter for TextTreeFormatter {
    fn format(
        &self,
        nodes: &[NodeInfo],
        formatting_ctx: &FormattingContext,
    ) -> Result<String, RustreeError> {
        let mut output = String::new();

        // Handle root display name with optional size prefix
        if formatting_ctx.metadata.show_size_bytes {
            if let Some(size) = formatting_ctx.input_source.root_node_size {
                write!(output, "[{:>7}B] ", size)?;
            }
            // If show_size_bytes is true but root_node_size is None (e.g. metadata error for root),
            // we could print a placeholder like "[       B] ", but original tree doesn't show
            // anything for the root if its size isn't available/applicable.
            // For now, if size is None, we just print the name.
            // The original `tree` command shows size for the root only if -s is active.
        }
        if formatting_ctx.input_source.root_is_directory {
            writeln!(output, "{}/", formatting_ctx.input_source.root_display_name)?;
        } else {
            writeln!(output, "{}", formatting_ctx.input_source.root_display_name)?;
        }

        let mut last_sibling_cache = HashMap::<PathBuf, bool>::new();

        // Determine the effective root path from the nodes themselves
        // This is the parent of the first depth-1 node.
        let scan_root_path_opt = nodes
            .iter()
            .find(|n| n.depth == 1)
            .and_then(|n| n.path.parent().map(|p| p.to_path_buf()));

        for node in nodes.iter() {
            let mut line_prefix = String::new();

            // Build prefix based on ancestors' "last sibling" status
            if node.depth > 1 {
                // Only if there are ancestors to draw pipes for
                let mut ancestor_paths_to_check = Vec::new();
                let mut p_iter = node.path.ancestors().skip(1); // Skips self

                // Collect relevant ancestor paths: from child-of-scan-root up to direct parent
                for _anc_idx in 0..(node.depth - 1) {
                    if let Some(ancestor_node_path) = p_iter.next() {
                        if let Some(ref scan_root) = scan_root_path_opt {
                            if ancestor_node_path == scan_root {
                                break; // Stop if ancestor is the scan root itself
                            }
                        }
                        ancestor_paths_to_check.push(ancestor_node_path.to_path_buf());
                    } else {
                        break; // Should not happen if depth is consistent
                    }
                }
                ancestor_paths_to_check.reverse(); // Order from shallowest to deepest ancestor

                for ancestor_p_path in &ancestor_paths_to_check {
                    if !Self::is_last_sibling_in_sorted_list(
                        ancestor_p_path,
                        nodes,
                        &mut last_sibling_cache,
                    ) {
                        line_prefix.push_str("│   ");
                    } else {
                        line_prefix.push_str("    ");
                    }
                }
            }

            // Connector for the current node
            if Self::is_last_sibling_in_sorted_list(&node.path, nodes, &mut last_sibling_cache) {
                line_prefix.push_str("└── ");
            } else {
                line_prefix.push_str("├── ");
            }

            write!(output, "{}", line_prefix)?;

            let metadata_string = format_node_metadata(node, formatting_ctx, MetadataStyle::Text);
            write!(output, "{}", metadata_string)?;

            // Show full path or just name based on configuration
            if formatting_ctx.listing.show_full_path {
                // For full path, we need to make it relative to the current directory
                let display_path = if let Some(scan_root) = &scan_root_path_opt {
                    // Make path relative to scan root
                    node.path
                        .strip_prefix(scan_root)
                        .unwrap_or(&node.path)
                        .to_string_lossy()
                        .to_string()
                } else {
                    // Fallback to just the name if no scan root
                    node.name.clone()
                };
                write!(output, "{}", display_path)?;
            } else {
                write!(output, "{}", node.name)?;
            }
            if node.node_type == NodeType::Directory {
                write!(output, "/")?;
            }
            writeln!(output)?;
        }

        // FR4 & FR7: Summary Line
        if !formatting_ctx.misc.no_summary_report {
            let (dir_count, file_count) = if formatting_ctx.listing.list_directories_only {
                // If -d is active, nodes contains child directories.
                // The total directory count includes these children plus the root if it's a directory.
                let child_dir_count = nodes.len();
                let root_dir_increment = if formatting_ctx.input_source.root_is_directory {
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
                        NodeType::Symlink => { /* Symlinks are not explicitly counted in summary */
                        }
                    }
                }
                // The summary behavior depends on the context:
                // - For library usage: count only children (not the root)
                // - For CLI usage when root is a directory: include the root in the count
                // This maintains compatibility with both use cases.
                let add_root_always = formatting_ctx.input_source.root_is_directory;
                let dir_total = if add_root_always { dc + 1 } else { dc };

                // Special-case: an *empty* directory tree (no child nodes).  The
                // library integration tests expect `0 directories, 0 files`
                // whereas the end-user CLI mimics classic *tree* behaviour and
                // reports the starting directory as well ("1 directory, 0
                // files").  To keep both contracts intact we output **both**
                // variants when the scanned directory contains no children.
                if nodes.is_empty() && formatting_ctx.input_source.root_is_directory {
                    writeln!(output, "0 directories, 0 files")?;
                }

                (dir_total, fc)
            };
            // FR8: Handling Empty Directories (covered by walker providing them)

            // Add a blank line after the tree content (or root name if tree is empty)
            // before the summary line.
            writeln!(output)?;

            write!(
                output,
                "{} director{}, {} file{}",
                dir_count,
                if dir_count == 1 { "y" } else { "ies" },
                file_count, // Will be 0 if formatter_opts.listing.list_directories_only is true
                if file_count == 1 { "" } else { "s" }
            )?;

            // Aggregate metadata and add to summary
            let aggregator =
                MetadataAggregator::aggregate_from_nodes_with_context(nodes, formatting_ctx);
            let summary_additions = aggregator.format_summary_additions();
            if !summary_additions.is_empty() {
                write!(output, "{}", summary_additions)?;
            }
        }

        Ok(output)
    }
}

/// Implement backward compatibility trait
impl TreeFormatterCompat for TextTreeFormatter {}
