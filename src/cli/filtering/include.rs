// src/cli/filtering/include.rs
use clap::Args;

#[derive(Args, Debug)]
pub struct IncludeArgs {
    /// List only those files that match the wild-card pattern. (Original tree: -P)
    /// Can be specified multiple times.
    /// See `glob` crate documentation for pattern syntax.
    /// `|` can be used within a pattern for alternation, e.g., "*.txt|*.md".
    /// A `/` at the end of a pattern matches directories only, e.g., "docs/".
    #[arg(short = 'P', long = "filter-include", action = clap::ArgAction::Append)]
    pub match_patterns: Option<Vec<String>>,
}
