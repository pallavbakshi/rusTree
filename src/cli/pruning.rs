// src/cli/pruning.rs
use clap::Args;

#[derive(Args, Debug)]
pub struct PruningArgs {
    /// Prune empty directories after all other filtering.
    /// An empty directory is one that contains no files and no non-empty subdirectories
    /// after all other filtering has been applied.
    #[arg(long = "prune-empty-directories", alias = "prune")]
    pub prune_empty_directories: bool,
}
