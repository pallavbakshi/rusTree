// src/core/walker.rs
use crate::core::config::RustreeLibConfig;
use crate::core::node::{NodeInfo, NodeType};
use crate::core::error::RustreeError;
use crate::core::analyzer::{apply_fn, file_stats};
use std::path::Path;
use std::fs;
use walkdir::WalkDir;

/// Traverses a directory structure starting from `root_path` and collects information
/// about each entry based on the provided `config`.
///
/// This function uses `walkdir::WalkDir` for traversal and populates `NodeInfo`
/// structs for each valid entry encountered. It applies filtering (e.g., `max_depth`,
/// `show_hidden`) and performs analysis (e.g., size, line/word counts, custom functions)
/// as specified in the `config`.
///
/// # Arguments
///
/// * `root_path` - The [`Path`] to the root directory to start scanning from.
/// * `config` - The [`RustreeLibConfig`] containing options for traversal and analysis.
///
/// # Returns
///
/// A `Result` containing a `Vec<NodeInfo>` on success, where each `NodeInfo`
/// represents a file system entry. Returns a [`RustreeError`] on failure, such as
/// I/O errors or issues during traversal.
///
/// # Notes
///
/// - The `root_path` itself is not included in the returned list of nodes; traversal
///   starts with its children (due to `min_depth(1)`).
/// - Hidden file filtering and max depth are applied. For more efficient skipping of
///   deep or hidden directories, `WalkDir`'s `filter_entry` or `max_depth` methods
///   could be used directly in future enhancements.
pub fn walk_directory(
    root_path: &Path,
    config: &RustreeLibConfig,
) -> Result<Vec<NodeInfo>, RustreeError> {
    let mut nodes = Vec::new();
    let walker = WalkDir::new(root_path).min_depth(1); // min_depth(1) to exclude root_path itself from NodeInfo list

    for entry_result in walker {
        let entry = entry_result?; // Propagate WalkDir errors

        let depth = entry.depth();
        if let Some(max_depth) = config.max_depth {
            // entry.depth() is 0 for root, 1 for children of root.
            // If min_depth(1) is used, first entry.depth() will be 1.
            // So, if max_depth is 1, we only want entries with entry.depth() == 1.
            if depth > max_depth {
                if entry.file_type().is_dir() {
                    // If entry is a directory and it's too deep, skip its contents
                    // entry.skip_subtree() is not available on DirEntry, need to configure WalkDir
                    // For now, just filter by depth. WalkDir needs to be configured with max_depth for efficiency.
                    // Or, handle it here:
                    // walker.skip_current_dir(); // This is not available on the iterator directly
                    // For simplicity, we'll just filter. For performance, WalkDir's max_depth should be used.
                }
                continue;
            }
        }


        let path = entry.path().to_path_buf();
        let name = path.file_name().unwrap_or_default().to_string_lossy().into_owned();

        if !config.show_hidden && name.starts_with('.') {
            // If it's a directory and hidden, we need to tell WalkDir to skip it.
            // This basic filter here will hide it from results, but WalkDir might still traverse.
            // For true skipping, WalkDir's filter_entry would be better.
            if entry.file_type().is_dir() {
                 // How to tell WalkDir to skip from here?
                 // For now, this just filters the output.
            }
            continue;
        }
        
        let metadata = entry.metadata()?;
        let node_type = if metadata.is_dir() {
            NodeType::Directory
        } else if metadata.is_file() {
            NodeType::File
        } else if metadata.file_type().is_symlink() {
            NodeType::Symlink
        } else {
            continue; // Skip other types
        };

        let mut node = NodeInfo {
            path,
            name,
            node_type,
            depth,
            size: if config.report_sizes { Some(metadata.len()) } else { None },
            permissions: None, // Placeholder
            mtime: if config.report_mtime { metadata.modified().ok() } else { None },
            line_count: None,
            word_count: None,
            custom_function_output: None,
        };

        if node.node_type == NodeType::File {
            if config.calculate_line_count || config.calculate_word_count || config.apply_function.is_some() {
                // Read file content only if needed
                match fs::read_to_string(&node.path) {
                    Ok(content) => {
                        if config.calculate_line_count {
                            node.line_count = Some(file_stats::count_lines_from_string(&content));
                        }
                        if config.calculate_word_count {
                            node.word_count = Some(file_stats::count_words_from_string(&content));
                        }
                        if let Some(func_type) = &config.apply_function {
                            node.custom_function_output = Some(apply_fn::apply_function_to_content(&content, func_type));
                        }
                    }
                    Err(_e) => {
                        // Could log this error or store it in NodeInfo
                        // For now, fields remain None if read fails
                    }
                }
            }
        }
        nodes.push(node);
    }
    Ok(nodes)
}