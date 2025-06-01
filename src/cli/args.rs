// src/cli/args.rs
use crate::cli::filtering::{exclude, gitignore_rules, include};
use crate::cli::listing::{depth, directory_only, hidden};
use crate::cli::metadata::{date, size, stats};
use crate::cli::misc::llm;
use crate::cli::output::format;
use crate::cli::pruning; // Import the new pruning module
use crate::cli::sorting::order;
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

    #[command(flatten)]
    pub depth: depth::DepthArgs,

    #[command(flatten)]
    pub all_files: hidden::AllFilesArgs,

    #[command(flatten)]
    pub directory_only: directory_only::DirectoryOnlyArgs,

    #[command(flatten)]
    pub size: size::SizeArgs,

    #[command(flatten)]
    pub date: date::DateArgs,

    #[command(flatten)]
    pub file_stats: stats::FileStatsArgs,

    #[command(flatten)]
    pub sort_order: order::SortOrderArgs,

    #[command(flatten)]
    pub include: include::IncludeArgs,

    #[command(flatten)]
    pub exclude: exclude::ExcludeArgs,

    #[command(flatten)]
    pub gitignore: gitignore_rules::GitignoreArgs,

    #[command(flatten)]
    pub format: format::FormatArgs,

    #[command(flatten)]
    pub llm: llm::LlmArgs,

    #[command(flatten)]
    pub pruning: pruning::PruningArgs,
}
