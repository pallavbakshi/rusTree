//! Core file system traversal functionality.
//!
//! This module contains the main directory walking logic, including WalkBuilder
//! setup, entry processing, and metadata collection.

use crate::core::error::RustreeError;
use crate::core::filter::pattern::{compile_glob_patterns, entry_matches_glob_patterns};
use crate::core::metadata::{file_info, size_calculator};
use crate::core::options::contexts::{OwnedWalkingContext, WalkingContext};
use crate::core::options::{FilteringOptions, ListingOptions, MetadataOptions, RustreeLibConfig};
use crate::core::tree::node::{NodeInfo, NodeType};
use ignore::WalkBuilder;
use std::fs;
use std::path::Path;

/// Walk directory using WalkingContext (Phase 3 - Context Objects)
///
/// This function uses a context structure for cleaner API and better modularity.
pub fn walk_directory_with_context(
    root_path: &Path,
    walking_ctx: &WalkingContext,
) -> Result<Vec<NodeInfo>, RustreeError> {
    walk_directory_with_options(
        root_path,
        walking_ctx.listing,
        walking_ctx.filtering,
        walking_ctx.metadata,
    )
}

/// Walk directory using owned context (GUI-friendly with pattern caching)
///
/// This function is optimized for scenarios where contexts are owned and modified,
/// such as GUI applications. It caches compiled patterns for better performance.
pub fn walk_directory_owned(
    root_path: &Path,
    walking_ctx: &mut OwnedWalkingContext,
) -> Result<Vec<NodeInfo>, RustreeError> {
    // Validate the context first
    walking_ctx
        .validate()
        .map_err(|e| RustreeError::ConfigError(e.to_string()))?;

    // Use the caching functionality for better performance
    let _ignore_patterns = walking_ctx.ignore_patterns()?;
    let _match_patterns = walking_ctx.match_patterns()?;

    // Call the underlying implementation with the owned context's options
    walk_directory_with_options(
        root_path,
        &walking_ctx.listing,
        &walking_ctx.filtering,
        &walking_ctx.metadata,
    )
}

/// Walk directory using specific option structs (Phase 1 - Parameter Objects)
///
/// This function takes only the specific options needed for directory walking,
/// reducing coupling to the full configuration structure.
pub fn walk_directory_with_options(
    root_path: &Path,
    listing_opts: &ListingOptions,
    filtering_opts: &FilteringOptions,
    metadata_opts: &MetadataOptions,
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
        &filtering_opts.ignore_patterns,
        filtering_opts.case_insensitive_filter,
        listing_opts.show_hidden,
    )?;
    let compiled_match_patterns = compile_glob_patterns(
        &filtering_opts.match_patterns,
        filtering_opts.case_insensitive_filter,
        listing_opts.show_hidden,
    )?;

    let mut walker_builder = WalkBuilder::new(&canonical_root_path); // Use canonicalized path
    walker_builder.hidden(!listing_opts.show_hidden);
    walker_builder.parents(true);
    walker_builder.ignore(false);
    walker_builder.git_global(filtering_opts.use_gitignore_rules);
    walker_builder.git_ignore(filtering_opts.use_gitignore_rules);
    walker_builder.git_exclude(filtering_opts.use_gitignore_rules);
    walker_builder.require_git(false); // Process gitignore files even if not in a git repo (for tests)
    walker_builder.ignore_case_insensitive(filtering_opts.case_insensitive_filter);

    if let Some(max_d) = listing_opts.max_depth {
        walker_builder.max_depth(Some(max_d));
    }

    if let Some(custom_ignore_files) = &filtering_opts.gitignore_file {
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
            if metadata_opts.show_size_bytes
                || filtering_opts.min_file_size.is_some()
                || filtering_opts.max_file_size.is_some()
            {
                node.size = Some(meta.len());
            }
            if metadata_opts.show_last_modified {
                node.mtime = meta.modified().ok();
            }
            if metadata_opts.report_change_time {
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
            if metadata_opts.report_creation_time {
                // Note: creation_time is not universally available (e.g., Linux filesystems often don't store it).
                // `std::fs::Metadata::created()` is the portable way but can return an error.
                node.create_time = meta.created().ok();
            }
        }

        if node.node_type == NodeType::File {
            // === 1. Optional in-memory content processing (lines/words, built-ins that need content)
            let needs_builtin_content = metadata_opts
                .apply_function
                .as_ref()
                .map(|apply_fn| matches!(apply_fn, crate::core::options::ApplyFunction::BuiltIn(_)))
                .unwrap_or(false);

            if metadata_opts.calculate_line_count
                || metadata_opts.calculate_word_count
                || needs_builtin_content
            {
                if let Ok(content) = fs::read_to_string(&node.path) {
                    if metadata_opts.calculate_line_count {
                        node.line_count = Some(size_calculator::count_lines_from_string(&content));
                    }
                    if metadata_opts.calculate_word_count {
                        node.word_count = Some(size_calculator::count_words_from_string(&content));
                    }

                    if let Some(crate::core::options::ApplyFunction::BuiltIn(func_type)) =
                        &metadata_opts.apply_function
                    {
                        if is_file_function(func_type)
                            && should_apply_function_to_file_with_options(
                                &node,
                                listing_opts,
                                filtering_opts,
                                &canonical_root_path,
                            )
                        {
                            node.custom_function_output =
                                Some(file_info::apply_builtin_to_file(&node.path, func_type));
                        }
                    }
                }
            }

            // === 2. External command processing (does not require file content)
            if node.custom_function_output.is_none() {
                if let Some(crate::core::options::ApplyFunction::External(ext_fn)) =
                    &metadata_opts.apply_function
                {
                    if should_apply_function_to_file_with_options(
                        &node,
                        listing_opts,
                        filtering_opts,
                        &canonical_root_path,
                    ) {
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
fn is_file_function(func: &crate::core::options::BuiltInFunction) -> bool {
    matches!(
        func,
        crate::core::options::BuiltInFunction::CountPluses
            | crate::core::options::BuiltInFunction::Cat
    )
}

/// Checks if a function should be applied to a specific file based on filtering patterns (parameter objects version).
fn should_apply_function_to_file_with_options(
    node: &NodeInfo,
    listing_opts: &ListingOptions,
    filtering_opts: &FilteringOptions,
    walk_root: &Path,
) -> bool {
    use crate::core::filter::pattern::{
        compile_glob_patterns, entry_matches_path_with_patterns_relative,
    };

    // Check apply_exclude_patterns first - if it matches, skip
    if let Some(exclude_patterns) = &filtering_opts.apply_exclude_patterns {
        if !exclude_patterns.is_empty() {
            if let Ok(Some(patterns)) = compile_glob_patterns(
                &Some(exclude_patterns.clone()),
                filtering_opts.case_insensitive_filter,
                listing_opts.show_hidden,
            ) {
                if entry_matches_path_with_patterns_relative(&node.path, &patterns, walk_root) {
                    return false; // Skip this node
                }
            }
        }
    }

    // Check apply_include_patterns - if specified, node must match
    if let Some(include_patterns) = &filtering_opts.apply_include_patterns {
        // If include patterns are specified (even if empty), use them as a filter
        if include_patterns.is_empty() {
            // Empty include patterns means match nothing
            return false;
        }

        if let Ok(Some(patterns)) = compile_glob_patterns(
            &Some(include_patterns.clone()),
            filtering_opts.case_insensitive_filter,
            listing_opts.show_hidden,
        ) {
            return entry_matches_path_with_patterns_relative(&node.path, &patterns, walk_root);
        }
        // If we have include patterns but compilation failed, don't apply
        return false;
    }

    // If no include patterns specified (None), apply the function to all
    true
}

/// Checks if a function should be applied to a specific file based on filtering patterns (backward compatibility).
/// Note: This function uses the current working directory as the walk root, which may not be correct
/// when scanning directories outside the CWD. For correct behavior, use should_apply_function_to_file_with_options.
#[allow(dead_code)]
fn should_apply_function_to_file(node: &NodeInfo, config: &RustreeLibConfig) -> bool {
    // For backward compatibility, use the current working directory as walk root
    // This is not ideal but maintains the existing API
    match std::env::current_dir() {
        Ok(cwd) => should_apply_function_to_file_with_options(
            node,
            &config.listing,
            &config.filtering,
            &cwd,
        ),
        Err(_) => {
            // If we can't get the current directory, fall back to allowing all
            // (same behavior as when pattern matching fails)
            true
        }
    }
}

/// Walk directory using full config (backward compatibility)
///
/// This function maintains the original API while internally using the new
/// parameter-based function.
pub fn walk_directory(
    root_path: &Path,
    config: &RustreeLibConfig,
) -> Result<Vec<NodeInfo>, RustreeError> {
    walk_directory_with_options(
        root_path,
        &config.listing,
        &config.filtering,
        &config.metadata,
    )
}
