// src/cli/listing/directory_only.rs
use clap::Args;

#[derive(Args, Debug)]
pub struct DirectoryOnlyArgs {
    /// List directories only. (Original tree: -d)
    #[arg(short = 'd', long = "directory-only")]
    pub list_directories_only: bool,
} 