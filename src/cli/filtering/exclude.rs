// src/cli/filtering/exclude.rs
use clap::Args;

#[derive(Args, Debug)]
pub struct ExcludeArgs {
    /// Do not list those files/directories that match the wild-card pattern. (Original tree: -I)
    /// Can be specified multiple times. Uses glob pattern syntax (see -P).
    #[arg(short = 'I', long = "filter-exclude", action = clap::ArgAction::Append)]
    pub ignore_patterns: Option<Vec<String>>,
}
