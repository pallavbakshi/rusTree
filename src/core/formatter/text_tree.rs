use super::base::TreeFormatter;
use crate::config::RustreeLibConfig;
use crate::core::error::RustreeError;
use crate::core::node::{NodeInfo, NodeType};
use std::collections::HashMap;
use std::fmt::Write;
use std::path::{Path, PathBuf};
use std::time::UNIX_EPOCH;

/// A formatter that generates a plain text, tree-like representation of the directory structure.
///
/// This is similar to the output of the standard `tree` command.
pub struct TextTreeFormatter;

impl TextTreeFormatter {
    fn format_metadata(node: &NodeInfo, config: &RustreeLibConfig) -> String {
        let mut metadata_str = String::new();

        // FR7: Interaction with Metadata Flags
        // Size: applies to files and directories if config.report_sizes is true.
        // node.size is populated for directories by the walker.
        if config.metadata.report_sizes {
            if let Some(size) = node.size {
                write!(metadata_str, "[{:>7}B] ", size).unwrap();
            } else {
                // This case should ideally not be hit if report_sizes is true,
                // as walker populates size. But as a fallback:
                write!(metadata_str, "[       B] ").unwrap();
            }
        }

        // MTime: applies to all node types if config.report_mtime is true.
        if config.metadata.report_mtime {
            if let Some(mtime) = node.mtime {
                match mtime.duration_since(UNIX_EPOCH) {
                    Ok(duration) => {
                        write!(metadata_str, "[MTime: {:>10}s] ", duration.as_secs()).unwrap()
                    }
                    Err(_) => write!(metadata_str, "[MTime: <error>] ").unwrap(),
                }
            } else {
                write!(metadata_str, "[MTime:            ] ").unwrap(); // Placeholder
            }
        }

        // File-specific metadata: only show if the node is a file.
        // When -d is active, node.node_type will be Directory, so these won't be shown.
        if node.node_type == NodeType::File {
            if config.metadata.calculate_line_count {
                if let Some(lc) = node.line_count {
                    write!(metadata_str, "[L:{:>4}] ", lc).unwrap();
                } else {
                    write!(metadata_str, "[L:    ] ").unwrap(); // Placeholder if None
                }
            }
            if config.metadata.calculate_word_count {
                if let Some(wc) = node.word_count {
                    write!(metadata_str, "[W:{:>4}] ", wc).unwrap();
                } else {
                    write!(metadata_str, "[W:    ] ").unwrap(); // Placeholder if None
                }
            }
            if config.metadata.apply_function.is_some() {
                match &node.custom_function_output {
                    Some(Ok(val)) => write!(metadata_str, "[F: \"{}\"] ", val).unwrap(),
                    Some(Err(_)) => write!(metadata_str, "[F: error] ").unwrap(),
                    None => {
                        // If apply_function was requested but output is None (e.g. read error before apply_fn)
                        write!(metadata_str, "[F: N/A] ").unwrap();
                    }
                }
            }
        }
        metadata_str
    }

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
        config: &RustreeLibConfig,
    ) -> Result<String, RustreeError> {
        let mut output = String::new();

        // Handle root display name with optional size prefix
        if config.metadata.report_sizes {
            if let Some(size) = config.input_source.root_node_size {
                write!(output, "[{:>7}B] ", size)?;
            }
            // If report_sizes is true but root_node_size is None (e.g. metadata error for root),
            // we could print a placeholder like "[       B] ", but original tree doesn't show
            // anything for the root if its size isn't available/applicable.
            // For now, if size is None, we just print the name.
            // The original `tree` command shows size for the root only if -s is active.
        }
        if config.input_source.root_is_directory {
            writeln!(output, "{}/", config.input_source.root_display_name)?;
        } else {
            writeln!(output, "{}", config.input_source.root_display_name)?;
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

            let metadata_string = Self::format_metadata(node, config);
            write!(output, "{}", metadata_string)?;

            write!(output, "{}", node.name)?;
            if node.node_type == NodeType::Directory {
                write!(output, "/")?;
            }
            writeln!(output)?;
        }

        // FR4 & FR7: Summary Line
        let (dir_count, file_count) = if config.listing.list_directories_only {
            // If -d is active, nodes contains child directories.
            // The total directory count includes these children plus the root if it's a directory.
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
                    NodeType::Symlink => { /* Symlinks are not explicitly counted in summary */ }
                }
            }
            (dc, fc)
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
            file_count, // Will be 0 if config.list_directories_only is true
            if file_count == 1 { "" } else { "s" }
        )?;

        Ok(output)
    }
}
