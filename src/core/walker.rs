// src/core/walker.rs
use crate::config::RustreeLibConfig;
use crate::core::analyzer::{apply_fn, file_stats};
use crate::core::error::RustreeError;
use crate::core::node::{NodeInfo, NodeType};
use glob::MatchOptions;
use ignore::WalkBuilder;
use std::fs;
use std::path::Path;

// Helper struct to hold compiled glob patterns and their properties
#[derive(Clone)]
struct CompiledGlobPattern {
    pattern: glob::Pattern,
    options: MatchOptions, // Stores case sensitivity and other glob matching options
    is_dir_only_match: bool, // True if original pattern string ended with '/'
    is_path_pattern: bool, // True if original pattern string contained '/' or '**'
}

/// Compiles string patterns into `CompiledGlobPattern` structs.
fn compile_glob_patterns(
    patterns_str: &Option<Vec<String>>,
    ignore_case: bool,
    show_hidden: bool, // Used to set require_literal_leading_dot
) -> Result<Option<Vec<CompiledGlobPattern>>, RustreeError> {
    match patterns_str {
        Some(ps_outer) if !ps_outer.is_empty() => {
            let mut compiled_patterns = Vec::new();
            let mut opts = MatchOptions::new();
            opts.case_sensitive = !ignore_case;
            opts.require_literal_separator = true; // Standard glob behavior: '*' doesn't match '/'
            // If show_hidden is true (-a), then '*' should match '.' (require_literal_leading_dot = false).
            // If show_hidden is false (no -a), then '*' should NOT match '.' (require_literal_leading_dot = true).
            opts.require_literal_leading_dot = !show_hidden;

            for p_outer_str in ps_outer {
                for p_inner_str in p_outer_str.split('|') {
                    if p_inner_str.is_empty() {
                        continue;
                    }

                    let is_dir_only = p_inner_str.ends_with('/');
                    let pattern_to_compile = if is_dir_only {
                        p_inner_str.strip_suffix('/').unwrap_or(p_inner_str)
                    } else {
                        p_inner_str
                    };

                    if pattern_to_compile.is_empty() {
                        continue; // Skip empty patterns (e.g., from "/" or "||")
                    }

                    let glob_pattern = glob::Pattern::new(pattern_to_compile)?;
                    let is_path_p = p_inner_str.contains('/') || p_inner_str.contains("**");

                    let mut current_opts = opts; // Copy base options
                    if p_inner_str.contains("**") {
                        current_opts.require_literal_separator = false;
                    }

                    compiled_patterns.push(CompiledGlobPattern {
                        pattern: glob_pattern,
                        options: current_opts, // Use potentially modified opts
                        is_dir_only_match: is_dir_only,
                        is_path_pattern: is_path_p,
                    });
                }
            }
            if compiled_patterns.is_empty() {
                Ok(Some(Vec::new())) // e.g. patterns were ["", "|"], results in matching nothing
            } else {
                Ok(Some(compiled_patterns))
            }
        }
        _ => Ok(None), // No patterns provided, or patterns_str was Some(empty_vec)
    }
}

/// Checks if a `DirEntry` matches any of the compiled glob patterns.
/// Assumes `compiled_patterns` is not empty.
fn entry_matches_glob_patterns(
    entry: &ignore::DirEntry,
    compiled_patterns: &Vec<CompiledGlobPattern>, // Expects a non-empty Vec
    walk_root_path: &Path,                        // The canonicalized root path of the walk
) -> bool {
    let entry_full_path = entry.path();
    let file_name_lossy = entry.file_name().to_string_lossy();
    let is_dir = entry.file_type().is_some_and(|ft| ft.is_dir());

    for p_info in compiled_patterns {
        let matches = if p_info.is_dir_only_match {
            // Pattern like "dir/" - matches directory name
            is_dir
                && p_info
                    .pattern
                    .matches_with(&file_name_lossy, p_info.options)
        } else if p_info.is_path_pattern {
            // Pattern like "src/*.rs" or "**/*.tmp" or "/abs/path/*.txt"
            let pattern_str = p_info.pattern.as_str();
            if Path::new(pattern_str).is_absolute() {
                // For absolute path patterns, match against the full entry path.
                p_info
                    .pattern
                    .matches_path_with(entry_full_path, p_info.options)
            } else {
                // For relative path patterns (including "**" patterns), match against path relative to walk_root_path.
                match entry_full_path.strip_prefix(walk_root_path) {
                    Ok(relative_path) => p_info
                        .pattern
                        .matches_path_with(relative_path, p_info.options),
                    Err(_) => {
                        // This occurs if entry_full_path is not under walk_root_path,
                        // or walk_root_path is not a prefix. This should be rare if walk_root_path
                        // is canonicalized and the walk behaves as expected.
                        // Fallback to matching against the full path, or consider it a non-match.
                        // For stricter relative matching, this should be 'false'.
                        // However, glob matching can be complex. If strip_prefix fails,
                        // it's safer to assume the relative pattern doesn't match.
                        false
                    }
                }
            }
        } else {
            // Basename match, e.g., "*.log"
            p_info
                .pattern
                .matches_with(&file_name_lossy, p_info.options)
        };
        if matches {
            return true;
        }
    }
    false
}

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
        config.filtering.ignore_case_for_patterns,
        config.listing.show_hidden,
    )?;
    let compiled_match_patterns = compile_glob_patterns(
        &config.filtering.match_patterns,
        config.filtering.ignore_case_for_patterns,
        config.listing.show_hidden,
    )?;

    let mut walker_builder = WalkBuilder::new(&canonical_root_path); // Use canonicalized path
    walker_builder.hidden(!config.listing.show_hidden);
    walker_builder.parents(true);
    walker_builder.ignore(false);
    walker_builder.git_global(config.filtering.use_gitignore);
    walker_builder.git_ignore(config.filtering.use_gitignore);
    walker_builder.git_exclude(config.filtering.use_gitignore);
    walker_builder.require_git(false); // Process gitignore files even if not in a git repo (for tests)
    walker_builder.ignore_case_insensitive(config.filtering.ignore_case_for_patterns);

    if let Some(max_d) = config.listing.max_depth {
        walker_builder.max_depth(Some(max_d));
    }

    if let Some(custom_ignore_files) = &config.filtering.git_ignore_files {
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

        if config.listing.list_directories_only && node_type_for_filter != NodeType::Directory {
            continue;
        }

        let final_node_type_for_storage = node_type_for_filter;

        let mut node = NodeInfo {
            path: entry_path_obj.to_path_buf(),
            name,
            node_type: final_node_type_for_storage,
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
            if config.metadata.report_sizes {
                node.size = Some(meta.len());
            }
            if config.metadata.report_modification_time {
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
            // Analysis only for effective files
            if config.metadata.calculate_line_count
                || config.metadata.calculate_word_count
                || config.metadata.apply_function.is_some()
            {
                match fs::read_to_string(&node.path) {
                    // Reads target for symlinks
                    Ok(content) => {
                        if config.metadata.calculate_line_count {
                            node.line_count = Some(file_stats::count_lines_from_string(&content));
                        }
                        if config.metadata.calculate_word_count {
                            node.word_count = Some(file_stats::count_words_from_string(&content));
                        }
                        if let Some(func_type) = &config.metadata.apply_function {
                            node.custom_function_output =
                                Some(apply_fn::apply_function_to_content(&content, func_type));
                        }
                    }
                    Err(_e) => { /* Log error or store in NodeInfo */ }
                }
            }
        }
        intermediate_nodes.push(node);
    }
    Ok(intermediate_nodes)
}
