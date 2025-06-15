// src/cli/diff.rs

//! CLI arguments for diff functionality.

use clap::Args;
use std::path::PathBuf;

/// Arguments related to diff functionality.
#[derive(Args, Debug, Clone)]
pub struct DiffArgs {
    /// Compare current directory structure with a saved snapshot file.
    /// The snapshot file should contain tree output in JSON format from a previous run.
    #[arg(
        long = "diff",
        value_name = "FILE",
        help = "Compare with snapshot file (JSON format)"
    )]
    pub diff_file: Option<PathBuf>,

    /// Show only specific types of changes. Comma-separated list.
    /// Possible values: added, removed, modified, moved, type_changed, unchanged
    #[arg(
        long = "show-only",
        value_delimiter = ',',
        help = "Show only specific change types (comma-separated)"
    )]
    pub show_only: Vec<String>,

    /// Disable move detection, treating moves as separate add+remove operations.
    #[arg(long = "ignore-moves", help = "Don't detect file moves/renames")]
    pub ignore_moves: bool,

    /// Similarity threshold for move detection (0.0 to 1.0).
    /// Higher values require more similarity to consider files as moved.
    #[arg(
        long = "move-threshold",
        value_name = "THRESHOLD",
        default_value = "0.8",
        help = "Similarity threshold for move detection (0.0-1.0)"
    )]
    pub move_threshold: f64,

    /// Include unchanged files in the output.
    #[arg(long = "show-unchanged", help = "Include unchanged files in output")]
    pub show_unchanged: bool,

    /// Show only summary statistics, not detailed changes.
    #[arg(long = "stats-only", help = "Show only summary statistics")]
    pub stats_only: bool,

    /// Minimum size difference to report for file changes (in bytes).
    #[arg(
        long = "size-threshold",
        value_name = "BYTES",
        help = "Minimum size change to report"
    )]
    pub size_threshold: Option<u64>,

    /// Minimum time difference to report for file changes (in seconds).
    #[arg(
        long = "time-threshold",
        value_name = "SECONDS",
        help = "Minimum time change to report"
    )]
    pub time_threshold: Option<u64>,
}

impl Default for DiffArgs {
    fn default() -> Self {
        Self {
            diff_file: None,
            show_only: Vec::new(),
            ignore_moves: false,
            move_threshold: 0.8,
            show_unchanged: false,
            stats_only: false,
            size_threshold: None,
            time_threshold: None,
        }
    }
}

impl DiffArgs {
    /// Check if diff mode is enabled
    pub fn is_diff_mode(&self) -> bool {
        self.diff_file.is_some()
    }

    /// Get the diff file path if specified
    pub fn get_diff_file(&self) -> Option<&PathBuf> {
        self.diff_file.as_ref()
    }

    /// Check if a specific change type should be shown
    pub fn should_show_change_type(&self, change_type: &str) -> bool {
        if self.show_only.is_empty() {
            true // Show all by default
        } else {
            self.show_only
                .iter()
                .any(|t| t.eq_ignore_ascii_case(change_type))
        }
    }

    /// Validate the move threshold value
    pub fn validate_move_threshold(&self) -> Result<(), String> {
        if self.move_threshold < 0.0 || self.move_threshold > 1.0 {
            Err("Move threshold must be between 0.0 and 1.0".to_string())
        } else {
            Ok(())
        }
    }
}

/// Enum for filtering change types in output
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChangeTypeFilter {
    Added,
    Removed,
    Modified,
    Moved,
    TypeChanged,
    Unchanged,
}

impl std::str::FromStr for ChangeTypeFilter {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "added" | "add" | "+" => Ok(ChangeTypeFilter::Added),
            "removed" | "remove" | "rm" | "-" => Ok(ChangeTypeFilter::Removed),
            "modified" | "modify" | "mod" | "m" => Ok(ChangeTypeFilter::Modified),
            "moved" | "move" | "mv" | "~" => Ok(ChangeTypeFilter::Moved),
            "type_changed" | "type-changed" | "typechanged" | "t" => {
                Ok(ChangeTypeFilter::TypeChanged)
            }
            "unchanged" | "same" => Ok(ChangeTypeFilter::Unchanged),
            _ => Err(format!(
                "Invalid change type filter: '{}'. Valid options: added, removed, modified, moved, type_changed, unchanged",
                s
            )),
        }
    }
}
