//! Pattern matching and glob filtering functionality.
//!
//! This module provides functionality for compiling and matching glob patterns
//! against file system entries, supporting various pattern types and options.

use crate::core::error::RustreeError;
use glob::MatchOptions;
use std::path::Path;

/// Helper struct to hold compiled glob patterns and their properties.
#[derive(Clone)]
pub struct CompiledGlobPattern {
    pub pattern: glob::Pattern,
    pub options: MatchOptions, // Stores case sensitivity and other glob matching options
    pub is_dir_only_match: bool, // True if original pattern string ended with '/'
    pub is_path_pattern: bool, // True if original pattern string contained '/' or '**'
}

/// Compiles string patterns into `CompiledGlobPattern` structs.
pub fn compile_glob_patterns(
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
/// Returns false if no patterns are provided.
pub fn entry_matches_glob_patterns(
    entry: &ignore::DirEntry,
    compiled_patterns: &Vec<CompiledGlobPattern>,
    walk_root_path: &Path, // The canonicalized root path of the walk
) -> bool {
    // Validate that we have patterns to match against
    if compiled_patterns.is_empty() {
        return false; // No patterns means no matches
    }

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