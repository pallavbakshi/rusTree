// src/cli/metadata/date.rs
use clap::Args;

#[derive(Args, Debug)]
pub struct DateArgs {
    /// Report last modification times for files and directories. (Original tree: -D)
    #[arg(short = 'D', long)]
    pub report_mtime: bool,
} 