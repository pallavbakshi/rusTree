// src/cli/metadata/stats.rs
use crate::cli::metadata::CliBuiltInFunction;
use clap::Args;

#[derive(Args, Debug)]
pub struct FileStatsArgs {
    /// Calculate and display line counts for files.
    #[arg(long)]
    pub calculate_lines: bool,

    /// Calculate and display word counts for files.
    #[arg(short = 'w', long)]
    pub calculate_words: bool,

    /// Apply a built-in function to file contents and display the result.
    #[arg(long)]
    pub apply_function: Option<CliBuiltInFunction>,

    /// Apply an external command to file contents; mutually exclusive with
    /// `--apply-function`.
    #[arg(long = "apply-function-cmd", value_name = "CMD")]
    pub apply_function_cmd: Option<String>,

    /// Specify the result kind for the external command: "number", "bytes", or "text".
    /// Defaults to "text".
    #[arg(
        long = "apply-function-cmd-kind",
        value_name = "KIND",
        default_value = "text"
    )]
    pub apply_function_cmd_kind: String,

    /// Timeout in seconds for the external command (default 5 seconds).
    #[arg(long = "apply-timeout", default_value_t = 5)]
    pub apply_function_timeout: u64,
}
