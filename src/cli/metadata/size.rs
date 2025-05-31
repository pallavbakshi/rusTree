// src/cli/metadata/size.rs
use clap::Args;

#[derive(Args, Debug)]
pub struct SizeArgs {
    /// Report sizes of files in the output. (Original tree: -s)
    #[arg(short = 's', long)]
    pub report_sizes: bool,
}
