// src/cli/mapping.rs - CLI to Library Configuration Mapping
//
// This module provides functions to convert CLI argument structures into
// library configuration structures. It acts as a translation layer between
// the command-line interface and the core library.
use crate::cli::args::CliArgs;
use crate::cli::metadata::CliBuiltInFunction;
use crate::cli::output::CliOutputFormat;
use crate::cli::sorting::CliSortKey;
use crate::core::diff::changes::DiffOptions;

// Corrected imports using explicit paths from crate::config
use crate::config::BuiltInFunction as LibBuiltInFunction;
use crate::config::FilteringOptions;
use crate::config::HtmlOptions;
use crate::config::InputSourceOptions;
use crate::config::ListingOptions;
use crate::config::MetadataOptions;
use crate::config::MiscOptions;
use crate::config::SortKey as LibSortKey;
use crate::config::SortingOptions;
use crate::config::llm::LlmConfigError;
use crate::config::metadata::{
    ExternalFunction as LibExternalFunction, FunctionOutputKind as LibFunctionOutputKind,
};
use crate::config::output_format::OutputFormat as LibOutputFormat;
use crate::config::sorting::DirectoryFileOrder;
use crate::config::{RustreeLibConfig, load_merged_config};

/// Error type for CLI mapping operations
#[derive(Debug)]
pub enum CliMappingError {
    /// IO error when reading pattern files
    Io(std::io::Error),
    /// LLM configuration error
    LlmConfig(LlmConfigError),
}

impl std::fmt::Display for CliMappingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CliMappingError::Io(err) => write!(f, "Error reading pattern files: {}", err),
            CliMappingError::LlmConfig(err) => write!(f, "LLM configuration error: {}", err),
        }
    }
}

impl std::error::Error for CliMappingError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            CliMappingError::Io(err) => Some(err),
            CliMappingError::LlmConfig(err) => Some(err),
        }
    }
}

impl From<std::io::Error> for CliMappingError {
    fn from(err: std::io::Error) -> Self {
        CliMappingError::Io(err)
    }
}

impl From<LlmConfigError> for CliMappingError {
    fn from(err: LlmConfigError) -> Self {
        CliMappingError::LlmConfig(err)
    }
}

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
/// A `RustreeLibConfig` instance populated from the `cli_args`, or an error if pattern files cannot be read.
pub fn map_cli_to_lib_config(cli_args: &CliArgs) -> Result<RustreeLibConfig, CliMappingError> {
    // ------------------------------------------------------------------
    //  A. Build config based solely on CLI flags (legacy behaviour)
    // ------------------------------------------------------------------

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

    let mut cfg = RustreeLibConfig {
        input_source: InputSourceOptions {
            root_display_name,
            root_node_size,
            root_is_directory,
        },
        listing: ListingOptions {
            max_depth: cli_args.depth.max_depth,
            show_hidden: cli_args.all_files.show_hidden,
            list_directories_only: cli_args.directory_only.list_directories_only,
            show_full_path: cli_args.full_path.show_full_path,
        },
        filtering: FilteringOptions {
            match_patterns: cli_args.include.get_all_match_patterns()?,
            ignore_patterns: cli_args.exclude.get_all_ignore_patterns()?,
            use_gitignore_rules: cli_args.gitignore.use_gitignore_rules,
            gitignore_file: cli_args.gitignore.gitignore_file.clone(),
            case_insensitive_filter: cli_args.gitignore.case_insensitive_filter,
            prune_empty_directories: cli_args.pruning.prune_empty_directories,
            apply_include_patterns: cli_args.apply_function_filter.get_all_include_patterns()?,
            apply_exclude_patterns: cli_args.apply_function_filter.get_all_exclude_patterns()?,

            // Size filters will be parsed below
            min_file_size: parse_size_arg(&cli_args.size_filter.min_file_size)?,
            max_file_size: parse_size_arg(&cli_args.size_filter.max_file_size)?,
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
            directory_file_order: if cli_args.sort_order.dirs_first {
                DirectoryFileOrder::DirsFirst
            } else if cli_args.sort_order.files_first {
                DirectoryFileOrder::FilesFirst
            } else {
                DirectoryFileOrder::Default
            },
        },
        metadata: MetadataOptions {
            show_size_bytes: cli_args.size.show_size_bytes,
            report_permissions: false, // Not exposed in CLI args yet
            show_last_modified: cli_args.date.show_last_modified
                && !cli_args.sort_order.legacy_sort_change_time, // If -D is present AND -c is NOT
            report_change_time: cli_args.sort_order.legacy_sort_change_time
                && cli_args.date.show_last_modified, // -c with -D implies reporting ctime for display
            report_creation_time: false, // Currently no CLI flag for reporting creation time, but can be added later
            calculate_line_count: cli_args.file_stats.calculate_lines,
            calculate_word_count: cli_args.file_stats.calculate_words,
            apply_function: {
                // Handle built-in functions
                if let Some(f) = &cli_args.file_stats.apply_function {
                    if cli_args.file_stats.apply_function_cmd.is_some() {
                        return Err(CliMappingError::Io(std::io::Error::new(
                            std::io::ErrorKind::InvalidInput,
                            "Cannot specify both --apply-function and --apply-function-cmd",
                        )));
                    }
                    let builtin = match f {
                        CliBuiltInFunction::CountPluses => LibBuiltInFunction::CountPluses,
                        CliBuiltInFunction::Cat => LibBuiltInFunction::Cat,
                        CliBuiltInFunction::CountFiles => LibBuiltInFunction::CountFiles,
                        CliBuiltInFunction::CountDirs => LibBuiltInFunction::CountDirs,
                        CliBuiltInFunction::SizeTotal => LibBuiltInFunction::SizeTotal,
                        CliBuiltInFunction::DirStats => LibBuiltInFunction::DirStats,
                    };
                    Some(crate::core::options::ApplyFunction::BuiltIn(builtin))
                } else if let Some(cmd) = &cli_args.file_stats.apply_function_cmd {
                    // Handle external command functions
                    let kind = match cli_args
                        .file_stats
                        .apply_function_cmd_kind
                        .to_ascii_lowercase()
                        .as_str()
                    {
                        "number" | "num" | "count" => LibFunctionOutputKind::Number,
                        "bytes" | "byte" | "size" => LibFunctionOutputKind::Bytes,
                        _ => LibFunctionOutputKind::Text,
                    };

                    Some(crate::core::options::ApplyFunction::External(
                        LibExternalFunction {
                            cmd_template: cmd.clone(),
                            timeout_secs: cli_args.file_stats.apply_function_timeout,
                            kind,
                        },
                    ))
                } else {
                    None
                }
            },
            human_readable_size: cli_args.llm.human_friendly,
        },
        misc: MiscOptions {
            no_summary_report: cli_args.format.no_summary_report,
            human_friendly: cli_args.llm.human_friendly,
            no_color: false, // TODO: Add CLI flag for this if needed
            verbose: cli_args.verbose,
        },

        html: HtmlOptions {
            base_href: cli_args.html_output.html_base_href.clone(),
            strip_first_component: cli_args.html_output.html_strip_first_component,
            custom_intro: cli_args.html_output.html_intro_file.clone(),
            custom_outro: cli_args.html_output.html_outro_file.clone(),
            include_links: !cli_args.html_output.html_no_links,
        },
        llm: crate::config::LlmOptions::from_cli_args(&cli_args.llm)?,
    };

    // ------------------------------------------------------------------
    //  B. Load TOML configuration files and merge (Phase-3 feature)
    // ------------------------------------------------------------------

    match load_merged_config(&cli_args.config_file, !cli_args.no_config) {
        Ok((partial, _)) => {
            partial.merge_into(&mut cfg);
        }
        Err(e) => {
            return Err(CliMappingError::Io(std::io::Error::other(e.to_string())));
        }
    }

    Ok(cfg)
}

/// Converts a human-readable size string (e.g. "12K", "3M", "1G") into bytes.
/// The conversion uses base-1024 (1K = 1024 bytes).
fn parse_size_arg(arg: &Option<String>) -> Result<Option<u64>, std::io::Error> {
    match arg {
        None => Ok(None),
        Some(raw) => {
            let bytes = parse_size_string(raw).map_err(|e| {
                std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!("Invalid size specification '{}': {}", raw, e),
                )
            })?;
            Ok(Some(bytes))
        }
    }
}

fn parse_size_string(s: &str) -> Result<u64, &'static str> {
    if s.is_empty() {
        return Err("empty string");
    }

    let (num_part, unit_part) = s.trim().split_at(
        s.trim()
            .char_indices()
            .take_while(|(_, c)| c.is_ascii_digit())
            .map(|(i, _)| i + 1)
            .last()
            .unwrap_or(0),
    );

    let value: u64 = num_part.parse().map_err(|_| "failed to parse number")?;

    // No suffix => bytes
    if unit_part.is_empty() {
        return Ok(value);
    }

    let factor: u64 = match unit_part.trim().to_ascii_lowercase().as_str() {
        "k" | "kb" => 1024,
        "m" | "mb" => 1024 * 1024,
        "g" | "gb" => 1024 * 1024 * 1024,
        _ => return Err("unrecognized size suffix"),
    };

    Ok(value.saturating_mul(factor))
}

/// Maps the CLI output format enum (`CliOutputFormat`) to the library's output format enum (`LibOutputFormat`).
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
        Some(CliOutputFormat::Json) => LibOutputFormat::Json,
        Some(CliOutputFormat::Html) => LibOutputFormat::Html,
        Some(CliOutputFormat::Text) | None => LibOutputFormat::Text, // Default to Text
    }
}

/// Maps CLI diff arguments to DiffOptions.
pub fn map_cli_to_diff_options(cli_args: &CliArgs, config: &RustreeLibConfig) -> DiffOptions {
    DiffOptions {
        max_depth: config.listing.max_depth,
        show_size: config.metadata.show_size_bytes,
        sort_by: config.sorting.sort_by.as_ref().map(|s| format!("{:?}", s)),
        detect_moves: !cli_args.diff.ignore_moves,
        move_threshold: cli_args.diff.move_threshold,
        show_unchanged: cli_args.diff.show_unchanged,
        ignore_moves: cli_args.diff.ignore_moves,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_size_string_no_suffix() {
        assert_eq!(parse_size_string("123").unwrap(), 123);
    }

    #[test]
    fn test_parse_size_string_kib() {
        assert_eq!(parse_size_string("1K").unwrap(), 1024);
        assert_eq!(parse_size_string("2k").unwrap(), 2048);
    }

    #[test]
    fn test_parse_size_string_mib() {
        assert_eq!(parse_size_string("1M").unwrap(), 1024 * 1024);
    }

    #[test]
    fn test_parse_size_string_gib() {
        assert_eq!(parse_size_string("1G").unwrap(), 1024 * 1024 * 1024);
    }

    #[test]
    fn test_parse_size_string_invalid() {
        assert!(parse_size_string("12X").is_err());
        assert!(parse_size_string("").is_err());
    }
}
