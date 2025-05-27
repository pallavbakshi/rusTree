// src/cli/args.rs
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CliArgs {
    /// The root path to start scanning from
    #[arg(default_value = ".")]
    pub path: PathBuf,

    /// Maximum depth to scan
    #[arg(short = 'L', long)]
    pub max_depth: Option<usize>,

    /// Show hidden files and directories (equivalent to original tree -a)
    #[arg(short = 'a', long = "all")]
    pub show_hidden: bool,

    /// Report sizes of files
    #[arg(short = 's', long)]
    pub report_sizes: bool,

    /// Report modification times (equivalent to original tree -D)
    #[arg(short = 'D', long)]
    pub report_mtime: bool,

    /// Calculate line counts for files
    #[arg(long)]
    pub calculate_lines: bool,

    /// Calculate word counts for files (placeholder, not fully implemented in lib)
    #[arg(short = 'w', long)]
    pub calculate_words: bool,

    /// Sort key
    #[arg(long)]
    pub sort_key: Option<CliSortKey>,

    /// Reverse sort order
    #[arg(short = 'r', long)]
    pub reverse_sort: bool,

    /// Apply a built-in function to file contents
    #[arg(long)]
    pub apply_function: Option<CliBuiltInFunction>,

    /// Output format
    #[arg(long, default_value = "text")]
    pub output_format: Option<CliOutputFormat>, // clap will parse "text" or "markdown"

    /// Ask a question to an LLM (output will be formatted for piping)
    #[arg(long)]
    pub llm_ask: Option<String>,
}

#[derive(clap::ValueEnum, Clone, Debug, PartialEq, Eq)]
pub enum CliSortKey {
    Name,
    Size,
    MTime,
    Words,
    Lines,
    Custom, // If custom function output is sortable
}

#[derive(clap::ValueEnum, Clone, Debug, PartialEq, Eq)]
pub enum CliOutputFormat {
    Text,
    Markdown,
}

#[derive(clap::ValueEnum, Clone, Debug, PartialEq, Eq)]
pub enum CliBuiltInFunction {
    CountPluses,
    // Add other function names here
}