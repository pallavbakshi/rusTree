// src/cli/filtering/size_filter.rs

//! CLI arguments for size-based filtering (`--min-file-size`, `--max-file-size`).

use clap::Args;

#[derive(Args, Debug, Clone)]
pub struct SizeFilterArgs {
    /// Only include files whose size is **at least** this value.
    /// Accepts suffixes K, M, G for kibibytes, mebibytes, gibibytes (base-1024).
    #[arg(long = "min-file-size", value_name = "SIZE")]
    pub min_file_size: Option<String>,

    /// Only include files whose size is **at most** this value.
    /// Accepts suffixes K, M, G for kibibytes, mebibytes, gibibytes (base-1024).
    #[arg(long = "max-file-size", value_name = "SIZE")]
    pub max_file_size: Option<String>,
}
