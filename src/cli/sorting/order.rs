// src/cli/sorting/order.rs
use clap::Args;
use crate::cli::sorting::CliSortKey;

#[derive(Args, Debug)]
pub struct SortOrderArgs {
    /// Sort by modification time. (Original tree: -t)
    /// Conflicts with --sort-key and -U/--unsorted.
    #[arg(short = 't', long = "sort-by-mtime", conflicts_with_all = ["sort_key", "unsorted_flag"])]
    pub sort_by_mtime_flag: bool,

    /// Do not sort; list files in directory order. (Original tree: -U)
    /// Conflicts with --sort-key, -t/--sort-by-mtime, and -r/--reverse-sort.
    #[arg(short = 'U', long, conflicts_with_all = ["sort_key", "sort_by_mtime_flag", "reverse_sort"])]
    pub unsorted_flag: bool,

    /// Specifies the key for sorting directory entries (e.g., name, size, mtime).
    /// Conflicts with -t/--sort-by-mtime and -U/--unsorted.
    /// If no sort option (-t, -U, --sort-key) is given, defaults to sorting by name.
    #[arg(long, conflicts_with_all = ["sort_by_mtime_flag", "unsorted_flag"])]
    pub sort_key: Option<CliSortKey>,

    /// Reverse the order of the active sort. (Original tree: -r)
    /// Incompatible with -U/--unsorted.
    #[arg(short = 'r', long)]
    pub reverse_sort: bool,
} 