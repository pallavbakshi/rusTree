// src/cli/output/format.rs
use super::CliOutputFormat;
use clap::Args;

#[derive(Args, Debug)]
pub struct FormatArgs {
    /// Specifies the output format for the tree.
    /// Defaults to "text".
    #[arg(long, default_value = "text")]
    pub output_format: Option<CliOutputFormat>,
}
