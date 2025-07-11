// src/cli/args.rs
use crate::cli::diff;
use crate::cli::filtering::{
    apply_function, exclude, gitignore_rules, include, pruning, size_filter,
};
use crate::cli::input;
use crate::cli::listing::{depth, directory_only, full_path, hidden};
use crate::cli::llm;
use crate::cli::metadata::{date, size, stats};
use crate::cli::output::format;
use crate::cli::sorting::order;
use clap::Parser;
use clap_complete::Shell;
use std::path::PathBuf;

/// Defines the command-line arguments accepted by the `rustree` executable.
///
/// This struct uses `clap` for parsing and automatically generates help messages.
#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about,
    long_about = None,
    color = clap::ColorChoice::Auto
)]
pub struct CliArgs {
    /// The root path to start scanning from.
    /// Defaults to the current directory (`.`).
    #[arg(default_value = ".")]
    pub path: PathBuf,

    // Utility options
    /// Generate shell completion script for the specified shell and exit.
    #[arg(
        long = "generate-completions",
        value_enum,
        help_heading = "Utility Options",
        value_name = "SHELL"
    )]
    pub generate_completions: Option<Shell>,

    /// Print a commented template configuration file (.rustree/config.toml) and exit.
    #[arg(
        long = "generate-config",
        help_heading = "Utility Options",
        conflicts_with = "generate_completions",
        default_value_t = false
    )]
    pub generate_config: bool,

    /// Print the fully merged configuration before executing.
    #[arg(
        long,
        global = true,
        help_heading = "Utility Options",
        default_value_t = false
    )]
    pub verbose: bool,

    /// Load an additional TOML configuration file (can be repeated; last one wins).
    #[arg(
        long = "config-file",
        value_name = "FILE",
        num_args = 1..,
        help_heading = "Utility Options"
    )]
    pub config_file: Vec<PathBuf>,

    /// Disable automatic discovery of project/global config files.
    #[arg(
        long = "no-config",
        help_heading = "Utility Options",
        default_value_t = false
    )]
    pub no_config: bool,

    // Input Options
    #[command(flatten, next_help_heading = "\x1b[1;36mInput Options\x1b[0m")]
    pub input: input::InputArgs,

    // Listing Options
    #[command(flatten, next_help_heading = "\x1b[1;36mListing Options\x1b[0m")]
    pub depth: depth::DepthArgs,

    #[command(flatten)]
    pub all_files: hidden::AllFilesArgs,

    #[command(flatten)]
    pub directory_only: directory_only::DirectoryOnlyArgs,

    #[command(flatten)]
    pub full_path: full_path::FullPathArgs,

    // Metadata Options
    #[command(flatten, next_help_heading = "\x1b[1;35mMetadata Options\x1b[0m")]
    pub size: size::SizeArgs,

    #[command(flatten)]
    pub date: date::DateArgs,

    #[command(flatten)]
    pub file_stats: stats::FileStatsArgs,

    // Sorting Options
    #[command(flatten, next_help_heading = "\x1b[1;34mSorting Options\x1b[0m")]
    pub sort_order: order::SortOrderArgs,

    // Filtering Options
    #[command(flatten, next_help_heading = "\x1b[1;33mFiltering Options\x1b[0m")]
    pub include: include::IncludeArgs,

    #[command(flatten)]
    pub exclude: exclude::ExcludeArgs,

    #[command(flatten)]
    pub gitignore: gitignore_rules::GitignoreArgs,

    #[command(flatten)]
    pub size_filter: size_filter::SizeFilterArgs,

    // Apply-functions patterns
    #[command(flatten, next_help_heading = "\x1b[1;32mApply Functions\x1b[0m")]
    pub apply_function_filter: apply_function::ApplyFunctionFilterArgs,

    // Output Options
    #[command(flatten, next_help_heading = "\x1b[1;37mOutput Options\x1b[0m")]
    pub format: format::FormatArgs,

    #[command(flatten)]
    pub html_output: crate::cli::output::html::HtmlOutputArgs,

    // LLM Options
    #[command(flatten, next_help_heading = "\x1b[1;31mLLM Options\x1b[0m")]
    pub llm: llm::LlmArgs,

    // Diff Options
    #[command(flatten, next_help_heading = "\x1b[1;36mDiff Options\x1b[0m")]
    pub diff: diff::DiffArgs,

    #[command(flatten, next_help_heading = "\x1b[1;33mFiltering Options\x1b[0m")]
    pub pruning: pruning::PruningArgs,
}
