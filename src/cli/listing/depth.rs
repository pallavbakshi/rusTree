// src/cli/listing/depth.rs
use clap::Args;

#[derive(Args, Debug)]
pub struct DepthArgs {
    /// Maximum depth to scan into the directory tree. (Original tree: -L)
    /// E.g., `-L 1` shows only direct children.
    #[arg(short = 'L', long = "depth")]
    pub max_depth: Option<usize>,
} 