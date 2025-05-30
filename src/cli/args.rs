// src/cli/args.rs
use clap::Parser;
use std::path::PathBuf;
use crate::cli::sorting::order;
use crate::cli::metadata::{size, date, stats};
use crate::cli::listing::{depth, hidden, directory_only};
use crate::cli::filtering::{include, exclude, gitignore};
use crate::cli::output::format;
use crate::cli::misc::llm;

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
    pub gitignore: gitignore::GitignoreArgs,

    #[command(flatten)]
    pub format: format::FormatArgs,

    #[command(flatten)]
    pub llm: llm::LlmArgs,
} 