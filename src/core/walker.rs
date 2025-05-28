// src/core/walker.rs
use crate::core::config::RustreeLibConfig;
use crate::core::node::{NodeInfo, NodeType};
use crate::core::error::RustreeError;
use crate::core::analyzer::{apply_fn, file_stats};
use std::path::Path;
use std::fs;
use walkdir::WalkDir;
use glob::{Pattern as GlobPattern, MatchOptions};

// Helper struct to hold compiled glob patterns and their properties
#[derive(Clone)]
struct CompiledPattern {
    glob_pattern: GlobPattern,
    is_dir_only_match: bool, // True if original pattern string ended with '/'
    is_path_pattern: bool,   // True if original pattern string contained '/' or '**'
}

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
    let mut intermediate_nodes = Vec::new();

    // Compile glob patterns if provided
    let compiled_patterns: Option<Vec<CompiledPattern>> = match &config.match_patterns {
        Some(pattern_strings) if !pattern_strings.is_empty() => {
            let mut patterns = Vec::new();
            for p_str_outer in pattern_strings {
                for p_str_inner in p_str_outer.split('|') {
                    if p_str_inner.is_empty() { continue; } // Skip empty patterns from "||" or trailing "|"
                    let is_dir_only = p_str_inner.ends_with('/');
                    let pattern_to_compile = if is_dir_only {
                        p_str_inner.strip_suffix('/').unwrap_or(p_str_inner)
                    } else {
                        p_str_inner
                    };
                    
                    // Handle special case: if pattern_to_compile is empty after stripping '/',
                    // e.g. user provided just "/", treat it as matching nothing or handle as error.
                    // For now, an empty pattern_to_compile will likely error in GlobPattern::new or match nothing.
                    // Let's ensure it's not empty before compiling.
                    if pattern_to_compile.is_empty() && is_dir_only { // Original was just "/"
                        // This case is tricky. `tree -P /` is not well-defined.
                        // Let's make it match nothing to avoid errors with empty glob patterns.
                        // Or, one could argue `*/` is the way to match all dirs.
                        // For now, skip if pattern becomes empty.
                        continue;
                    }


                    let glob_pattern = GlobPattern::new(pattern_to_compile)?;
                    // A pattern is a "path pattern" if the original string segment (p_str_inner)
                    // contained path separators or the double-star glob.
                    let is_path_p = p_str_inner.contains('/') || p_str_inner.contains("**");

                    patterns.push(CompiledPattern {
                        glob_pattern,
                        is_dir_only_match: is_dir_only,
                        is_path_pattern: is_path_p,
                    });
                }
            }
            // If config.match_patterns was Some(_), but all resultant patterns were empty (e.g. from "||" or just ""),
            // this should mean "match nothing". Represent this as Some(empty_vec).
            // Otherwise, if no patterns were provided in config, it's None.
            if patterns.is_empty() {
                Some(Vec::new()) // Effectively "match nothing"
            } else {
                Some(patterns)
            }
        }
        // If config.match_patterns was None or Some(empty_vec_originally), then it's None.
        // This case is tricky: if config.match_patterns was Some(vec![]) initially, it should also be Some(Vec::new()) here.
        // The current logic: if pattern_strings is empty (from config.match_patterns = Some([])), it falls to _ => None.
        // This is correct: Some([]) in config means no patterns specified through CLI, so compiled_patterns = None (match all).
        // Only if config.match_patterns = Some(vec!["", "|"]) etc. (non-empty vec of empty/invalid strings) should it be Some(Vec::new()).
        _ => None, // No patterns provided in config, or config.match_patterns was Some(empty_vec_initially)
    };

    let mut walker_builder = WalkDir::new(root_path).min_depth(1);

    if let Some(d) = config.max_depth {
        walker_builder = walker_builder.max_depth(d);
    }
    
    // Match options for glob patterns
    let match_options = MatchOptions {
        case_sensitive: true, // Default, as per PRD non-goal for --ignore-case
        require_literal_separator: true,  // If true, `*` and `?` will not match `/`. This is standard.
        require_literal_leading_dot: false, // `*` can match `.` at start of filename component if -a is used.
    };

    // Clone compiled_patterns for the filter_entry closure, as it's a `move` closure.
    // The original compiled_patterns will be used for post-processing.
    let compiled_patterns_for_filter = compiled_patterns.clone();

    let walker_iter = walker_builder.into_iter().filter_entry(move |e| {
        let file_name_lossy = e.file_name().to_string_lossy();
        let is_hidden = file_name_lossy.starts_with('.');

        // 1. Hidden file filtering
        if !config.show_hidden && is_hidden {
            return false; // Filter out hidden files if -a is not set
        }

        // 2. Pattern matching (if patterns are provided)
        if let Some(ref patterns_to_match) = compiled_patterns_for_filter {
            let entry_path = e.path();
            let entry_is_dir = e.file_type().is_dir();
            let file_name_str = file_name_lossy.as_ref();
            let mut matched_any_pattern = false;

            for compiled_p in patterns_to_match {
                if compiled_p.is_dir_only_match {
                    // Pattern like "foo/" - must match a directory name
                    if entry_is_dir && compiled_p.glob_pattern.matches_with(file_name_str, match_options) {
                        matched_any_pattern = true;
                        break;
                    }
                } else if compiled_p.is_path_pattern {
                    // Pattern like "foo/*.txt" or "**/*.c" or "**/foo" - matches against full path
                    if compiled_p.glob_pattern.matches_path_with(entry_path, match_options) {
                        matched_any_pattern = true;
                        break;
                    }
                } else {
                    // Pattern like "*.txt" or "foo" - matches against basename
                    if compiled_p.glob_pattern.matches_with(file_name_str, match_options) {
                        matched_any_pattern = true;
                        break;
                    }
                }
            }
            
            // This is the crucial decision logic:
            if entry_is_dir {
                // If the directory itself matches a pattern, OR if there's any non-dir-specific pattern
                // that *could* match a child, then we should explore this directory.
                if matched_any_pattern {
                    return true; // Directory itself matches, so explore it.
                } else {
                    // Directory itself does not match. Explore if any pattern could match a descendant.
                    // A pattern is "general" if it's not directory-only (like "*.txt")
                    // or if it's a path pattern (like "foo/**/bar.txt") which might apply to contents.
                    let has_general_pattern = patterns_to_match.iter().any(|p|
                        !p.is_dir_only_match || p.is_path_pattern
                    );
                    return has_general_pattern;
                }
            } else {
                // For files, they must match a pattern to be included.
                return matched_any_pattern;
            }
        } // This closes `if let Some(ref patterns_to_match) = compiled_patterns`
        true // Entry passes all filters (e.g. no patterns specified, or already returned from inside the if block)
    }); // This closes `filter_entry`

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
        intermediate_nodes.push(node);
    }

    // If patterns were applied, intermediate_nodes has already been filtered by filter_entry.
    // To match the observed `tree -P` behavior (which can be more inclusive of non-matching
    // directories that were traversed), we return intermediate_nodes directly without further pruning.
    // The filter_entry logic handles:
    //  - Hidden files based on config.show_hidden.
    //  - Max depth.
    //  - For files: inclusion only if they match a pattern.
    //  - For directories: inclusion if they match a pattern OR if `has_general_pattern` is true,
    //    allowing WalkDir to explore them. These explored directories will be in intermediate_nodes.
    //  - If compiled_patterns is Some([]), (e.g. from -P ""), filter_entry will correctly filter out all entries
    //    as `has_general_pattern` will be false and `matched_any_pattern` will be false.
    Ok(intermediate_nodes)
}