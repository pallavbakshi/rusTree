// src/cli/output/format.rs
use super::CliOutputFormat;
use clap::Args;

#[derive(Args, Debug)]
pub struct FormatArgs {
    /// Specifies the output format for the tree.
    /// Defaults to "text".
    #[arg(long, default_value = "text")]
    pub output_format: Option<CliOutputFormat>,

    /// Omits printing of the file and directory report at the end of the tree listing.
    #[arg(long)]
    pub no_summary_report: bool,
}
