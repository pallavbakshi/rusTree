// src/cli/metadata/stats.rs
use clap::Args;
use crate::cli::metadata::CliBuiltInFunction;

#[derive(Args, Debug)]
pub struct FileStatsArgs {
    /// Calculate and display line counts for files.
    #[arg(long)]
    pub calculate_lines: bool,

    /// Calculate and display word counts for files.
    #[arg(short = 'w', long)]
    pub calculate_words: bool,

    /// Apply a built-in function to file contents and display the result.
    #[arg(long)]
    pub apply_function: Option<CliBuiltInFunction>,
} 