// src/cli/mapping.rs - CLI to Library Configuration Mapping
//
// This module provides functions to convert CLI argument structures into
// library configuration structures. It acts as a translation layer between
// the command-line interface and the core library.
use crate::cli::args::CliArgs;
use crate::cli::metadata::CliBuiltInFunction;
use crate::cli::output::CliOutputFormat;
use crate::cli::sorting::CliSortKey;
use rustree::{
    BuiltInFunction as LibBuiltInFunction, FilteringOptions, InputSourceOptions, LibOutputFormat,
    ListingOptions, MetadataOptions, MiscOptions, RustreeLibConfig, SortKey as LibSortKey,
    SortingOptions,
};

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
        cli_args
            .path
            .file_name()
            .unwrap_or_else(|| cli_args.path.as_os_str()) // Fallback for paths like "/" or "C:\"
            .to_string_lossy()
            .into_owned()
    };

    let root_node_size = if cli_args.size.show_size_bytes {
        std::fs::metadata(&cli_args.path)
            .ok()
            .map(|meta| meta.len())
    } else {
        None
    };

    let root_is_directory = std::fs::metadata(&cli_args.path)
        .map(|meta| meta.is_dir())
        .unwrap_or(false); // Default to false if metadata fails or it's not a dir

    RustreeLibConfig {
        input_source: InputSourceOptions {
            root_display_name,
            root_node_size,
            root_is_directory,
        },
        listing: ListingOptions {
            max_depth: cli_args.depth.max_depth,
            show_hidden: cli_args.all_files.show_hidden,
            list_directories_only: cli_args.directory_only.list_directories_only,
        },
        filtering: FilteringOptions {
            match_patterns: cli_args.include.match_patterns.clone(),
            ignore_patterns: cli_args.exclude.ignore_patterns.clone(),
            use_gitignore_rules: cli_args.gitignore.use_gitignore_rules,
            gitignore_file: cli_args.gitignore.gitignore_file.clone(),
            case_insensitive_filter: cli_args.gitignore.case_insensitive_filter,
        },
        sorting: SortingOptions {
            sort_by: if cli_args.sort_order.legacy_no_sort {
                None // -U means no sorting
            } else if cli_args.sort_order.legacy_sort_version {
                Some(LibSortKey::Version) // -v means sort by Version
            } else if cli_args.sort_order.legacy_sort_mtime {
                Some(LibSortKey::MTime) // -t means sort by MTime
            } else if cli_args.sort_order.legacy_sort_change_time {
                Some(LibSortKey::ChangeTime) // -c means sort by ChangeTime
            } else {
                cli_args
                    .sort_order
                    .sort_by
                    .as_ref()
                    .map(|sk| match sk {
                        CliSortKey::Name => LibSortKey::Name,
                        CliSortKey::Version => LibSortKey::Version,
                        CliSortKey::Size => LibSortKey::Size,
                        CliSortKey::MTime => LibSortKey::MTime,
                        CliSortKey::ChangeTime => LibSortKey::ChangeTime,
                        CliSortKey::CreateTime => LibSortKey::CreateTime,
                        CliSortKey::Words => LibSortKey::Words,
                        CliSortKey::Lines => LibSortKey::Lines,
                        CliSortKey::Custom => LibSortKey::Custom,
                        CliSortKey::None => LibSortKey::None,
                    })
                    .or(Some(LibSortKey::Name)) // Default to sort by Name if no sort option is specified
            },
            reverse_sort: cli_args.sort_order.reverse_sort,
            files_before_directories: true, // Default to traditional behavior
        },
        metadata: MetadataOptions {
            show_size_bytes: cli_args.size.show_size_bytes,
            report_permissions: false, // Not exposed in CLI args yet
            show_last_modified: cli_args.date.show_last_modified && !cli_args.sort_order.legacy_sort_change_time, // If -D is present AND -c is NOT
            report_change_time: cli_args.sort_order.legacy_sort_change_time && cli_args.date.show_last_modified, // -c with -D implies reporting ctime for display
            report_creation_time: false, // Currently no CLI flag for reporting creation time, but can be added later
            calculate_line_count: cli_args.file_stats.calculate_lines,
            calculate_word_count: cli_args.file_stats.calculate_words,
            apply_function: cli_args
                .file_stats
                .apply_function
                .as_ref()
                .map(|f| match f {
                    CliBuiltInFunction::CountPluses => LibBuiltInFunction::CountPluses,
                }),
        },
        misc: MiscOptions::default(),
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
