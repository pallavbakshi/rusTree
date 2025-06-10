// src/cli/sorting/order.rs
use crate::cli::sorting::CliSortKey;
use clap::Args;

#[derive(Args, Debug)]
pub struct SortOrderArgs {
    /// Sort by entry name, version, size, modification time, change time, creation time, lines, words, custom, or none.
    /// E.g., `--sort-by size` or `-S m`.
    /// Conflicts with -v, -t, -c, -U.
    #[arg(long = "sort-by", short = 'S', value_name = "FIELD", conflicts_with_all = ["legacy_sort_version", "legacy_sort_mtime", "legacy_sort_change_time", "legacy_no_sort"])]
    pub sort_by: Option<CliSortKey>,

    // Legacy flags for backward compatibility
    /// Sort by version. (Original tree: -v)
    /// Conflicts with --sort-by, -t, -c, -U.
    #[arg(short = 'v', conflicts_with_all = ["sort_by", "legacy_sort_mtime", "legacy_sort_change_time", "legacy_no_sort"])]
    pub legacy_sort_version: bool,

    /// Sort by modification time. (Original tree: -t)
    /// Conflicts with --sort-by, -v, -c, -U.
    #[arg(short = 't', conflicts_with_all = ["sort_by", "legacy_sort_version", "legacy_sort_change_time", "legacy_no_sort"])]
    pub legacy_sort_mtime: bool,

    /// Sort by change time. (Original tree: -c)
    /// Also modifies --date to show change time if both -c and -D are present.
    /// Conflicts with --sort-by, -v, -t, -U.
    #[arg(short = 'c', conflicts_with_all = ["sort_by", "legacy_sort_version", "legacy_sort_mtime", "legacy_no_sort"])]
    pub legacy_sort_change_time: bool,

    /// Do not sort; list files in directory order. (Original tree: -U)
    /// Conflicts with --sort-by, -v, -t, -c, -r.
    #[arg(short = 'U', long, conflicts_with_all = ["sort_by", "legacy_sort_version", "legacy_sort_mtime", "legacy_sort_change_time", "reverse_sort"])]
    pub legacy_no_sort: bool,

    /// Reverse the order of the active sort. (Original tree: -r)
    /// Incompatible with -U/--unsorted.
    #[arg(short = 'r', long)]
    pub reverse_sort: bool,

    /// List directories before files. More readable.
    /// Conflicts with --files-first.
    #[arg(long = "dirs-first", conflicts_with = "files_first")]
    pub dirs_first: bool,

    /// List files before directories. More readable.
    /// Conflicts with --dirs-first.
    #[arg(long = "files-first", conflicts_with = "dirs_first")]
    pub files_first: bool,
}
