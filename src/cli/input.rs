// src/cli/input.rs

//! CLI arguments for input file parsing functionality.

use crate::core::input::InputFormat;
use clap::Args;
use std::path::PathBuf;

#[derive(Args, Debug)]
pub struct InputArgs {
    /// Read tree structure from a previously generated tree file instead of scanning the filesystem.
    /// The file should contain tree output in one of the supported formats (text, markdown, JSON, HTML).
    #[arg(long = "from-tree-file", value_name = "FILE", conflicts_with = "path")]
    pub from_tree_file: Option<PathBuf>,

    /// Specify the format of the input file. If not specified, the format will be auto-detected.
    /// Possible values: text, markdown, json, html, auto
    #[arg(
        long = "input-format",
        value_name = "FORMAT",
        default_value = "auto",
        requires = "from_tree_file"
    )]
    pub input_format: InputFormat,
}

impl InputArgs {
    /// Check if we're reading from a tree file instead of scanning filesystem
    pub fn is_from_file(&self) -> bool {
        self.from_tree_file.is_some()
    }

    /// Get the tree file path if specified
    pub fn get_tree_file(&self) -> Option<&PathBuf> {
        self.from_tree_file.as_ref()
    }

    /// Get the input format
    pub fn get_input_format(&self) -> InputFormat {
        self.input_format.clone()
    }
}
