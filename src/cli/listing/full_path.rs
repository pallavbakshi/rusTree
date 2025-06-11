use clap::Args;

#[derive(Args, Debug)]
pub struct FullPathArgs {
    /// Print the full path prefix for each file (Original tree: -f)
    #[arg(short = 'f', long = "full-path")]
    pub show_full_path: bool,
}
