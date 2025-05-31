// src/cli/metadata/date.rs
use clap::Args;

#[derive(Args, Debug)]
pub struct DateArgs {
    /// Report last modified dates for files and directories. (Original tree: -D)
    /// If -c is also used, this flag will display change times instead.
    #[arg(short = 'D', long = "show-last-modified")]
    pub show_last_modified: bool,
}
