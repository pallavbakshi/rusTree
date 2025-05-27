// src/cli/args.rs
use clap::Parser;
use std::path::PathBuf;

/// Defines the command-line arguments accepted by the `rustree` executable.
///
/// This struct uses `clap` for parsing and automatically generates help messages.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CliArgs {
    /// The root path to start scanning from.
    /// Defaults to the current directory (`.`).
    #[arg(default_value = ".")]
    pub path: PathBuf,

    /// Maximum depth to scan into the directory tree.
    /// E.g., `-L 1` shows only direct children.
    #[arg(short = 'L', long)]
    pub max_depth: Option<usize>,

    /// Show hidden files and directories (those starting with a `.`).
    /// Equivalent to `tree -a`.
    #[arg(short = 'a', long = "all")]
    pub show_hidden: bool,

    /// Report sizes of files in the output.
    #[arg(short = 's', long)]
    pub report_sizes: bool,

    /// Report last modification times for files and directories.
    /// Equivalent to `tree -D`.
    #[arg(short = 'D', long)]
    pub report_mtime: bool,

    /// Calculate and display line counts for files.
    #[arg(long)]
    pub calculate_lines: bool,

    /// Calculate and display word counts for files.
    #[arg(short = 'w', long)]
    pub calculate_words: bool,

    /// Specifies the key for sorting directory entries.
    /// If not provided, defaults to sorting by name.
    #[arg(long)]
    pub sort_key: Option<CliSortKey>,

    /// Reverse the order of the sort specified by `sort_key`.
    #[arg(short = 'r', long)]
    pub reverse_sort: bool,

    /// Apply a built-in function to file contents and display the result.
    #[arg(long)]
    pub apply_function: Option<CliBuiltInFunction>,

    /// Specifies the output format for the tree.
    /// Defaults to "text".
    #[arg(long, default_value = "text")]
    pub output_format: Option<CliOutputFormat>,

    /// Ask a question to an LLM, providing the `rustree` output as context.
    /// The output will be specially formatted for easy piping to an LLM tool.
    #[arg(long)]
    pub llm_ask: Option<String>,
}

/// Defines the possible keys for sorting directory entries via the CLI.
#[derive(clap::ValueEnum, Clone, Debug, PartialEq, Eq)]
pub enum CliSortKey {
    /// Sort by entry name.
    Name,
    /// Sort by file size.
    Size,
    /// Sort by last modification time.
    MTime,
    /// Sort by word count (for files).
    Words,
    /// Sort by line count (for files).
    Lines,
    /// Sort by the output of a custom applied function.
    Custom,
}

/// Defines the possible output formats selectable via the CLI.
#[derive(clap::ValueEnum, Clone, Debug, PartialEq, Eq)]
pub enum CliOutputFormat {
    /// Plain text, tree-like structure.
    Text,
    /// Markdown list format.
    Markdown,
}

/// Defines built-in functions that can be applied to file contents via the CLI.
#[derive(clap::ValueEnum, Clone, Debug, PartialEq, Eq)]
pub enum CliBuiltInFunction {
    /// Counts occurrences of the '+' character.
    CountPluses,
    // Add other function names here
}