// src/cli/metadata/size.rs
use clap::Args;

#[derive(Args, Debug)]
pub struct SizeArgs {
    /// Report sizes of files in the output. (Original tree: -s)
    #[arg(short = 's', long = "show-size-bytes")]
    pub show_size_bytes: bool,
}
