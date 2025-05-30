// src/cli/mapping.rs - CLI to Library Configuration Mapping
//
// This module provides functions to convert CLI argument structures into
// library configuration structures. It acts as a translation layer between
// the command-line interface and the core library.
use crate::cli::args::CliArgs;
use crate::cli::sorting::CliSortKey;
use crate::cli::output::CliOutputFormat;
use crate::cli::metadata::CliBuiltInFunction;
use rustree::{RustreeLibConfig, LibOutputFormat, SortKey as LibSortKey, BuiltInFunction as LibBuiltInFunction};

/// Maps command-line arguments (`CliArgs`) to the library's configuration structure (`RustreeLibConfig`).
///
/// This function translates CLI flags and options into the format expected by the `rustree` core library.
///
/// # Arguments
///
/// * `cli_args` - A reference to the parsed command-line arguments.
///
/// # Returns
///
/// A `RustreeLibConfig` instance populated from the `cli_args`.
pub fn map_cli_to_lib_config(cli_args: &CliArgs) -> RustreeLibConfig {
    let root_display_name = if cli_args.path.to_string_lossy() == "." {
        ".".to_string()
    } else {
        cli_args.path
            .file_name()
            .unwrap_or_else(|| cli_args.path.as_os_str()) // Fallback for paths like "/" or "C:\"
            .to_string_lossy()
            .into_owned()
    };

    let root_node_size = if cli_args.size.report_sizes {
        std::fs::metadata(&cli_args.path).ok().map(|meta| meta.len())
    } else {
        None
    };

    let root_is_directory = std::fs::metadata(&cli_args.path)
        .map(|meta| meta.is_dir())
        .unwrap_or(false); // Default to false if metadata fails or it's not a dir

    RustreeLibConfig {
        root_display_name,
        root_node_size,
        root_is_directory,
        match_patterns: cli_args.include.match_patterns.clone(),
        max_depth: cli_args.depth.max_depth,
        show_hidden: cli_args.all_files.show_hidden,
        report_sizes: cli_args.size.report_sizes,
        report_permissions: false, // Not exposed in CLI args yet
        report_mtime: cli_args.date.report_mtime,
        calculate_line_count: cli_args.file_stats.calculate_lines,
        calculate_word_count: cli_args.file_stats.calculate_words,
        apply_function: cli_args.file_stats.apply_function.as_ref().map(|f| match f {
            CliBuiltInFunction::CountPluses => LibBuiltInFunction::CountPluses,
        }),
        sort_by: if cli_args.sort_order.unsorted_flag {
            None // -U means no sorting
        } else if cli_args.sort_order.sort_by_mtime_flag {
            Some(LibSortKey::MTime) // -t means sort by MTime
        } else {
            cli_args.sort_order.sort_key.as_ref().map(|sk| match sk { // --sort-key
                CliSortKey::Name => LibSortKey::Name,
                CliSortKey::Size => LibSortKey::Size,
                CliSortKey::MTime => LibSortKey::MTime,
                CliSortKey::Words => LibSortKey::Words,
                CliSortKey::Lines => LibSortKey::Lines,
                CliSortKey::Custom => LibSortKey::Custom,
            }).or(Some(LibSortKey::Name)) // Default to sort by Name if no sort option is specified
        },
        reverse_sort: cli_args.sort_order.reverse_sort,
        list_directories_only: cli_args.directory_only.list_directories_only,
        // Map ignore feature args
        ignore_patterns: cli_args.exclude.ignore_patterns.clone(),
        use_gitignore: cli_args.gitignore.use_gitignore,
        git_ignore_files: cli_args.gitignore.git_ignore_files.clone(),
        ignore_case_for_patterns: cli_args.gitignore.ignore_case_for_patterns,
    }
}

/// Maps the CLI output format enum (`CliOutputFormat`) to the library's output format enum (`LibOutputFormat`).
///
/// # Arguments
///
/// * `cli_output_format` - An `Option` containing the output format specified via CLI.
///
/// # Returns
///
/// The corresponding `LibOutputFormat`. Defaults to `LibOutputFormat::Text` if `None` is provided.
pub fn map_cli_to_lib_output_format(cli_output_format: Option<CliOutputFormat>) -> LibOutputFormat {
    match cli_output_format {
        Some(CliOutputFormat::Markdown) => LibOutputFormat::Markdown,
        Some(CliOutputFormat::Text) | None => LibOutputFormat::Text, // Default to Text
    }
} 