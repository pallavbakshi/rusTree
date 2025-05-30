// src/cli/output/format.rs
use clap::Args;
use super::CliOutputFormat;

#[derive(Args, Debug)]
pub struct FormatArgs {
    /// Specifies the output format for the tree.
    /// Defaults to "text".
    #[arg(long, default_value = "text")]
    pub output_format: Option<CliOutputFormat>,
} 