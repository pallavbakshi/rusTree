// src/cli/filtering/gitignore_rules.rs
use clap::Args;
use std::path::PathBuf;

#[derive(Args, Debug)]
pub struct GitignoreArgs {
    /// Uses git .gitignore files for filtering.
    /// Also respects global gitignore and $GIT_DIR/info/exclude.
    #[arg(long = "use-gitignore-rules", aliases = ["gitignore"], help = "Use .gitignore files for filtering. The --gitignore flag is deprecated.")]
    pub use_gitignore_rules: bool,

    /// Use file explicitly as a gitignore file.
    /// Can be specified multiple times.
    #[arg(long = "gitignore-file", value_name = "FILE", action = clap::ArgAction::Append)]
    pub gitignore_file: Option<Vec<PathBuf>>,

    /// Ignore case for -P, -I, --use-gitignore-rules, and --gitignore-file patterns.
    #[arg(long = "case-insensitive-filter")]
    pub case_insensitive_filter: bool,
}
