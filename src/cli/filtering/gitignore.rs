// src/cli/filtering/gitignore.rs
use clap::Args;
use std::path::PathBuf;

#[derive(Args, Debug)]
pub struct GitignoreArgs {
    /// Uses git .gitignore files for filtering.
    /// Also respects global gitignore and $GIT_DIR/info/exclude.
    #[arg(long = "gitignore")]
    pub use_gitignore: bool,

    /// Use file explicitly as a gitignore file.
    /// Can be specified multiple times.
    #[arg(long, value_name = "FILE", action = clap::ArgAction::Append)]
    pub git_ignore_files: Option<Vec<PathBuf>>,

    /// Ignore case for -P, -I, --gitignore, and --gitfile patterns.
    #[arg(long = "ignore-case")]
    pub ignore_case_for_patterns: bool,
}
