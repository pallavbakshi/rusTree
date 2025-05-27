// src/cli/handler.rs
use crate::cli::args::{CliArgs, CliSortKey, CliOutputFormat, CliBuiltInFunction};
use rustree::core::config::RustreeLibConfig;
use rustree::core::formatter::OutputFormat as LibOutputFormat;
use rustree::core::sorter::SortKey as LibSortKey;
use rustree::core::analyzer::apply_fn::BuiltInFunction as LibBuiltInFunction;

pub fn map_cli_to_lib_config(cli_args: &CliArgs) -> RustreeLibConfig {
    RustreeLibConfig {
        max_depth: cli_args.max_depth,
        show_hidden: cli_args.show_hidden,
        report_sizes: cli_args.report_sizes,
        report_permissions: false, // Not exposed in CLI args yet
        report_mtime: cli_args.report_mtime,
        calculate_line_count: cli_args.calculate_lines,
        calculate_word_count: cli_args.calculate_words,
        apply_function: cli_args.apply_function.as_ref().map(|f| match f {
            CliBuiltInFunction::CountPluses => LibBuiltInFunction::CountPluses,
        }),
        sort_by: cli_args.sort_key.as_ref().map(|sk| match sk {
            CliSortKey::Name => LibSortKey::Name,
            CliSortKey::Size => LibSortKey::Size,
            CliSortKey::MTime => LibSortKey::MTime,
            CliSortKey::Words => LibSortKey::Words,
            CliSortKey::Lines => LibSortKey::Lines,
            CliSortKey::Custom => LibSortKey::Custom,
        }),
        reverse_sort: cli_args.reverse_sort,
    }
}

pub fn map_cli_to_lib_output_format(cli_output_format: Option<CliOutputFormat>) -> LibOutputFormat {
    match cli_output_format {
        Some(CliOutputFormat::Markdown) => LibOutputFormat::Markdown,
        Some(CliOutputFormat::Text) | None => LibOutputFormat::Text, // Default to Text
    }
}