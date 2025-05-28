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
    let mut walker_builder = WalkDir::new(root_path).min_depth(1);

    if let Some(d) = config.max_depth {
        // WalkDir's max_depth is relative to the root of the walk.
        // Since we use min_depth(1), an entry at our depth 1 is WalkDir's depth 1.
        // So, config.max_depth directly maps to WalkDir's max_depth.
        walker_builder = walker_builder.max_depth(d);
    }

    // Apply entry filtering for hidden files.
    // This closure is called for each entry. If it returns true, the entry is processed.
    // If it returns false for a directory, that directory's contents are skipped by WalkDir.
    let walker_iter = walker_builder.into_iter().filter_entry(|e| {
        // If show_hidden is true, always process the entry.
        if config.show_hidden {
            return true;
        }
        // If show_hidden is false, check if the file name starts with a dot.
        // Allow the walk to proceed (return true) if the entry is NOT hidden.
        // Return false if it IS hidden, to filter it and prevent descent if it's a directory.
        !e.file_name()
            .to_string_lossy()
            .starts_with('.')
    });

    for entry_result in walker_iter {
        let entry = entry_result?; // Propagate WalkDir errors

        let entry_path_obj = entry.path(); // entry.path() is &Path to the current item (file, dir, or symlink)
        let name = entry.file_name().to_string_lossy().into_owned();
        let depth = entry.depth();
        let current_entry_file_type = entry.file_type(); // Type of the entry itself (e.g. symlink, dir, file)

        // Determine the effective NodeType for filtering and the metadata to use for NodeInfo.
        // For symlinks, this involves resolving the target.
        let (node_type_for_filter, resolved_metadata_for_node): (NodeType, Option<std::fs::Metadata>) =
            if current_entry_file_type.is_dir() {
                // Actual directory
                (NodeType::Directory, entry.metadata().ok()) // entry.metadata() is dir's own metadata
            } else if current_entry_file_type.is_file() {
                // Actual file
                (NodeType::File, entry.metadata().ok()) // entry.metadata() is file's own metadata
            } else if current_entry_file_type.is_symlink() {
                // It's a symlink. Determine its effective type based on its target.
                // fs::metadata(entry_path_obj) follows the symlink to get target's metadata.
                match fs::metadata(entry_path_obj) {
                    Ok(target_meta) => { // Successfully followed symlink and got target's metadata
                        if target_meta.is_dir() {
                            (NodeType::Directory, Some(target_meta)) // Effective type is Directory
                        } else if target_meta.is_file() {
                            (NodeType::File, Some(target_meta))      // Effective type is File
                        } else {
                            // Target is neither a directory nor a file (e.g. symlink to socket/fifo)
                            // This symlink won't be treated as a directory for -d.
                            (NodeType::Symlink, Some(target_meta)) // Store as Symlink, with target's meta if available
                        }
                    }
                    Err(_) => { // fs::metadata failed: broken symlink or permission error on target
                        (NodeType::Symlink, None) // Effective type is Symlink (broken), no target metadata
                    }
                }
            } else {
                // Not a dir, file, or symlink (e.g. FIFO, char device). Skip.
                continue;
            };

        // Filtering Logic for -d (list_directories_only)
        // node_type_for_filter is NodeType::Directory if it's an actual dir OR a symlink to an actual dir.
        if config.list_directories_only && node_type_for_filter != NodeType::Directory {
            continue;
        }

        // The NodeType stored in NodeInfo should reflect its effective type after symlink resolution for -d.
        // This ensures formatter treats symlinks to dirs as dirs (e.g. for trailing slash).
        let final_node_type_for_storage = node_type_for_filter;

        let mut node = NodeInfo {
            path: entry_path_obj.to_path_buf(), // Path of the entry itself (file, dir, or symlink)
            name,
            node_type: final_node_type_for_storage,
            depth,
            size: None,
            permissions: None, // Placeholder
            mtime: None,
            line_count: None,
            word_count: None,
            custom_function_output: None,
        };

        // Populate size/mtime from resolved_metadata_for_node.
        // For actual dirs/files, this is their own metadata.
        // For symlinks, this is their target's metadata (if resolvable and relevant).
        if let Some(meta) = resolved_metadata_for_node {
            if config.report_sizes { node.size = Some(meta.len()); }
            if config.report_mtime { node.mtime = meta.modified().ok(); }
        }

        // File content analysis:
        // This should only happen if the node is effectively a File.
        // If config.list_directories_only is true, final_node_type_for_storage cannot be File here.
        if node.node_type == NodeType::File {
            if config.calculate_line_count || config.calculate_word_count || config.apply_function.is_some() {
                // fs::read_to_string on a symlink path will read the target file's content.
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