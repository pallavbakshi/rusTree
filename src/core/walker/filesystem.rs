//! Core file system traversal functionality.
//!
//! This module contains the main directory walking logic, including WalkBuilder
//! setup, entry processing, and metadata collection.

use crate::config::RustreeLibConfig;
use crate::core::error::RustreeError;
use crate::core::filter::pattern::{compile_glob_patterns, entry_matches_glob_patterns};
use crate::core::metadata::{file_info, size_calculator};
use crate::core::tree::node::{NodeInfo, NodeType};
use ignore::WalkBuilder;
use std::fs;
use std::path::Path;

pub fn walk_directory(
    root_path: &Path,
    config: &RustreeLibConfig,
) -> Result<Vec<NodeInfo>, RustreeError> {
    let mut intermediate_nodes = Vec::new();

    // Canonicalize root_path for consistent path operations
    let canonical_root_path = match fs::canonicalize(root_path) {
        Ok(p) => p,
        Err(e)
            if e.kind() == std::io::ErrorKind::NotFound && root_path.to_string_lossy() == "." =>
        {
            // Special case: if root_path is "." and canonicalize fails (e.g. current dir deleted during run),
            // try to use the original path. This is mostly for robustness in edge cases.
            // For most scenarios, if canonicalize fails, it's a hard error.
            // However, the tests often use ".", and if the temp dir is cleaned up too soon by another process,
            // this could be an issue. For now, let's proceed with canonicalization and let it fail if appropriate.
            // Re-evaluating: if canonicalize fails, it's a fundamental issue. Let it propagate.
            return Err(RustreeError::Io(e));
        }
        Err(e) => return Err(RustreeError::Io(e)),
    };

    let final_compiled_ignore_patterns = compile_glob_patterns(
        &config.filtering.ignore_patterns,
        config.filtering.case_insensitive_filter,
        config.listing.show_hidden,
    )?;
    let compiled_match_patterns = compile_glob_patterns(
        &config.filtering.match_patterns,
        config.filtering.case_insensitive_filter,
        config.listing.show_hidden,
    )?;

    let mut walker_builder = WalkBuilder::new(&canonical_root_path); // Use canonicalized path
    walker_builder.hidden(!config.listing.show_hidden);
    walker_builder.parents(true);
    walker_builder.ignore(false);
    walker_builder.git_global(config.filtering.use_gitignore_rules);
    walker_builder.git_ignore(config.filtering.use_gitignore_rules);
    walker_builder.git_exclude(config.filtering.use_gitignore_rules);
    walker_builder.require_git(false); // Process gitignore files even if not in a git repo (for tests)
    walker_builder.ignore_case_insensitive(config.filtering.case_insensitive_filter);

    if let Some(max_d) = config.listing.max_depth {
        walker_builder.max_depth(Some(max_d));
    }

    if let Some(custom_ignore_files) = &config.filtering.gitignore_file {
        for file_path in custom_ignore_files {
            walker_builder.add_custom_ignore_filename(file_path);
        }
    }

    // Apply -I patterns using filter_entry to prune the walk
    if let Some(ref patterns_vec) = final_compiled_ignore_patterns {
        if !patterns_vec.is_empty() {
            let patterns_for_closure = patterns_vec.clone();
            // Clone canonical_root_path for the closure, as it needs to own its captured variables or have 'static lifetime
            let root_path_for_closure = canonical_root_path.clone();
            walker_builder.filter_entry(move |entry| {
                if entry.depth() == 0 {
                    return true;
                }
                !entry_matches_glob_patterns(entry, &patterns_for_closure, &root_path_for_closure)
            });
        }
    }

    for entry_result in walker_builder.build() {
        let entry = match entry_result {
            Ok(e) => e,
            Err(e) => return Err(RustreeError::IgnoreError(e)),
        };

        // Skip the root path itself (depth 0)
        // This check is technically redundant if filter_entry also has it,
        // but harmless and ensures root is never processed here.
        if entry.depth() == 0 {
            continue;
        }

        // -I (--ignore-path) patterns are now handled by walker_builder.filter_entry

        // 2. Apply -P (--match-pattern) patterns
        // If config.match_patterns is Some (i.e., -P was used), then files/symlinks must match.
        // Directories are not filtered by -P at this stage.
        let should_be_skipped_by_p_pattern = match &compiled_match_patterns {
            Some(patterns) => {
                // -P was used
                if patterns.is_empty() {
                    // e.g. -P "" or -P "|", which means "match nothing"
                    true // Skip everything, because nothing can match empty patterns
                } else if let Some(file_type) = entry.file_type() {
                    if file_type.is_file() || file_type.is_symlink() {
                        // Files and symlinks must match
                        !entry_matches_glob_patterns(&entry, patterns, &canonical_root_path) // Skip if it does NOT match
                    } else {
                        false // It's a directory, don't skip based on -P here
                    }
                } else {
                    true // Cannot determine file type, skip
                }
            }
            None => false, // -P was not used, so don't skip based on -P patterns
        };
        if should_be_skipped_by_p_pattern {
            continue;
        }

        let entry_path_obj = entry.path();
        let name = entry.file_name().to_string_lossy().into_owned();
        // rustree depth is 1 for direct children, which matches entry.depth() from ignore crate (after skipping depth 0)
        let depth = entry.depth();
        let current_entry_file_type = entry.file_type(); // Option<std::fs::FileType>

        let (node_type_for_filter, resolved_metadata_for_node): (
            NodeType,
            Option<std::fs::Metadata>,
        ) = if current_entry_file_type.is_some_and(|ft| ft.is_dir()) {
            (NodeType::Directory, entry.metadata().ok())
        } else if current_entry_file_type.is_some_and(|ft| ft.is_file()) {
            (NodeType::File, entry.metadata().ok())
        } else if current_entry_file_type.is_some_and(|ft| ft.is_symlink()) {
            match fs::metadata(entry_path_obj) {
                // Follow symlink
                Ok(target_meta) => {
                    if target_meta.is_dir() {
                        (NodeType::Directory, Some(target_meta))
                    } else if target_meta.is_file() {
                        (NodeType::File, Some(target_meta))
                    } else {
                        (NodeType::Symlink, Some(target_meta))
                    } // Target is not file/dir
                }
                Err(_) => (NodeType::Symlink, None), // Broken symlink
            }
        } else {
            continue; // Not a dir, file, or symlink
        };

        // The list_directories_only filter is now applied in lib.rs after pruning.
        // let final_node_type_for_storage = node_type_for_filter; // This was used before, now node_type_for_filter is directly used.

        let mut node = NodeInfo {
            path: entry_path_obj.to_path_buf(),
            name,
            node_type: node_type_for_filter, // Use the resolved node_type_for_filter
            depth,
            size: None,
            permissions: None,
            mtime: None,
            change_time: None,
            create_time: None,
            line_count: None,
            word_count: None,
            custom_function_output: None,
        };

        if let Some(meta) = resolved_metadata_for_node {
            if config.metadata.show_size_bytes
                || config.filtering.min_file_size.is_some()
                || config.filtering.max_file_size.is_some()
            {
                node.size = Some(meta.len());
            }
            if config.metadata.show_last_modified {
                node.mtime = meta.modified().ok();
            }
            if config.metadata.report_change_time {
                // Note: ctime is Unix-specific and represents the time when file metadata was last changed.
                // It is not directly available via std::fs::Metadata on all platforms.
                // A cross-platform solution would require platform-specific APIs.
                #[cfg(unix)]
                {
                    use std::os::unix::fs::MetadataExt;
                    use std::time::{Duration, UNIX_EPOCH};
                    let ctime_secs = meta.ctime();
                    // Only process non-negative timestamps to avoid conversion issues
                    if ctime_secs >= 0 {
                        // Safe conversion from i64 to u64 since we checked it's non-negative
                        let secs = ctime_secs as u64;
                        node.change_time = UNIX_EPOCH.checked_add(Duration::from_secs(secs));
                    }
                    // If ctime is negative (before Unix epoch) or checked_add fails, change_time remains None
                }
                #[cfg(windows)]
                {
                    // Windows does not have a direct equivalent to Unix ctime (metadata change time).
                    // Windows creation time semantics differ significantly from Unix ctime, so we set
                    // this to None rather than providing misleading information.
                    node.change_time = None;
                }
                #[cfg(not(any(unix, windows)))]
                {
                    node.change_time = None; // Placeholder for other OS where ctime isn't easily accessible
                }
            }
            if config.metadata.report_creation_time {
                // Note: creation_time is not universally available (e.g., Linux filesystems often don't store it).
                // `std::fs::Metadata::created()` is the portable way but can return an error.
                node.create_time = meta.created().ok();
            }
        }

        if node.node_type == NodeType::File {
            // === 1. Optional in-memory content processing (lines/words, built-ins that need content)
            if config.metadata.calculate_line_count
                || config.metadata.calculate_word_count
                || config.metadata.apply_function.is_some()
            {
                if let Ok(content) = fs::read_to_string(&node.path) {
                    if config.metadata.calculate_line_count {
                        node.line_count = Some(size_calculator::count_lines_from_string(&content));
                    }
                    if config.metadata.calculate_word_count {
                        node.word_count = Some(size_calculator::count_words_from_string(&content));
                    }

                    if let Some(func_type) = &config.metadata.apply_function {
                        if is_file_function(func_type)
                            && should_apply_function_to_file(&node, config)
                        {
                            node.custom_function_output =
                                Some(file_info::apply_builtin_to_file(&node.path, func_type));
                        }
                    }
                }
            }

            // === 2. External command processing (does not require file content)
            if node.custom_function_output.is_none() {
                if let Some(ext_fn) = &config.metadata.external_function {
                    if should_apply_function_to_file(&node, config) {
                        node.custom_function_output =
                            Some(file_info::apply_external_to_file(&node.path, ext_fn));
                    }
                }
            }
        }
        intermediate_nodes.push(node);
    }
    Ok(intermediate_nodes)
}

/// Checks if a function is a file-specific function.
fn is_file_function(func: &crate::config::metadata::BuiltInFunction) -> bool {
    matches!(
        func,
        crate::config::metadata::BuiltInFunction::CountPluses
            | crate::config::metadata::BuiltInFunction::Cat
    )
}

/// Checks if a function should be applied to a specific file based on filtering patterns.
fn should_apply_function_to_file(node: &NodeInfo, config: &RustreeLibConfig) -> bool {
    use crate::core::filter::pattern::{compile_glob_patterns, entry_matches_path_with_patterns};

    // Check apply_exclude_patterns first - if it matches, skip
    if let Some(exclude_patterns) = &config.filtering.apply_exclude_patterns {
        if !exclude_patterns.is_empty() {
            if let Ok(Some(patterns)) = compile_glob_patterns(
                &Some(exclude_patterns.clone()),
                config.filtering.case_insensitive_filter,
                config.listing.show_hidden,
            ) {
                if entry_matches_path_with_patterns(&node.path, &patterns) {
                    return false; // Skip this node
                }
            }
        }
    }

    // Check apply_include_patterns - if specified, node must match
    if let Some(include_patterns) = &config.filtering.apply_include_patterns {
        if !include_patterns.is_empty() {
            if let Ok(Some(patterns)) = compile_glob_patterns(
                &Some(include_patterns.clone()),
                config.filtering.case_insensitive_filter,
                config.listing.show_hidden,
            ) {
                return entry_matches_path_with_patterns(&node.path, &patterns);
            }
            // If we have include patterns but compilation failed, don't apply
            return false;
        }
    }

    // If no include patterns specified, or node passed all checks, apply the function
    true
}
