// src/config/filtering.rs

use std::path::PathBuf;

/// Options related to filtering files and directories.
///
/// This includes patterns for inclusion/exclusion, gitignore handling,
/// and case sensitivity settings for pattern matching.
#[derive(Debug, Clone, Default)]
pub struct FilteringOptions {
    /// Patterns to filter entries by. Only entries matching any pattern will be shown.
    /// Corresponds to CLI -P/--match-pattern.
    pub match_patterns: Option<Vec<String>>,
    /// Patterns to ignore entries by. Entries matching any pattern will be excluded.
    /// Corresponds to CLI -I/--ignore-path.
    pub ignore_patterns: Option<Vec<String>>,
    /// If `true`, use .gitignore files for filtering.
    pub use_gitignore_rules: bool,
    /// List of custom files to use as gitignore files.
    pub gitignore_file: Option<Vec<PathBuf>>,
    /// If `true`, all pattern matching (-P, -I, gitignore) is case-insensitive.
    pub case_insensitive_filter: bool,
    /// If `true`, prune empty directories after all other filtering.
    pub prune_empty_directories: bool,

    // Apply-function specific filtering
    /// Patterns to include when applying functions. Only files/dirs matching these patterns will have the function applied.
    /// Corresponds to CLI --apply-include.
    pub apply_include_patterns: Option<Vec<String>>,
    /// Patterns to exclude when applying functions. Files/dirs matching these patterns will skip function application.
    /// Corresponds to CLI --apply-exclude.
    pub apply_exclude_patterns: Option<Vec<String>>,
}
